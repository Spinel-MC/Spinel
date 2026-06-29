use super::super::world::World;
use crate::entity::{Entity, EntityId, EntityPosition, Player, PlayerChunk};
use crate::events::player_move::PlayerMoveEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use crate::world::{
    Biome, Block, BlockPosition, Chunk, ChunkLoader, ChunkPosition, GenerateChunkError,
    SetChunkBlockResult,
};
use spinel_core::network::clientbound::play::chunk_batch_finished::ChunkBatchFinishedPacket;
use spinel_core::network::clientbound::play::chunk_batch_start::ChunkBatchStartPacket;
use spinel_core::network::clientbound::play::chunk_data::ChunkDataAndUpdateLightPacket;
use spinel_core::network::clientbound::play::disconnect::PlayDisconnectPacket;
use spinel_core::network::clientbound::play::forget_level_chunk::ForgetLevelChunkPacket;
use spinel_core::network::clientbound::play::light_update::LightUpdatePacket;
use spinel_core::network::clientbound::play::remove_entities::RemoveEntitiesPacket;
use spinel_core::network::clientbound::play::set_chunk_cache_center::SetChunkCacheCenterPacket;
use spinel_core::network::clientbound::play::set_player_inventory::SetPlayerInventoryPacket;
use spinel_core::network::clientbound::play::sync_player_pos::SyncPlayerPositionPacket;
use spinel_macros::event_listener;
use spinel_network::types::Identifier;
use spinel_network::{ConnectionState, DataType, VarIntWrapper};
use spinel_registry::EntityType;
use spinel_registry::ItemStack;
use spinel_registry::Material;
use spinel_registry::Registries;
use spinel_registry::dimension_type::DimensionType;
use std::io::{self, Cursor, Error, ErrorKind, Read};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use uuid::Uuid;

const PLAYER_MOVE_TEST_PASSTHROUGH: i32 = 0;
const PLAYER_MOVE_TEST_CANCEL: i32 = 1;
const PLAYER_MOVE_TEST_VIEW: i32 = 2;
const PLAYER_MOVE_TEST_COORDINATES: i32 = 3;
const PLAYER_MOVE_TEST_TELEPORT: i32 = 4;

static PLAYER_MOVE_TEST_BEHAVIOR: AtomicI32 = AtomicI32::new(PLAYER_MOVE_TEST_PASSTHROUGH);
static PLAYER_MOVE_TEST_LOCK: Mutex<()> = Mutex::new(());

#[event_listener]
fn player_move_test_listener(event: &mut PlayerMoveEvent, _server: &mut MinecraftServer) {
    match PLAYER_MOVE_TEST_BEHAVIOR.load(Ordering::SeqCst) {
        PLAYER_MOVE_TEST_CANCEL => event.set_cancelled(true),
        PLAYER_MOVE_TEST_VIEW => {
            let packet_position = event.new_position();
            event.set_new_position(EntityPosition::new(
                packet_position.get_x(),
                packet_position.get_y(),
                packet_position.get_z(),
                90.0,
                45.0,
            ));
        }
        PLAYER_MOVE_TEST_COORDINATES => {
            let packet_position = event.new_position();
            event.set_new_position(EntityPosition::new(
                packet_position.get_x() + 1.0,
                packet_position.get_y(),
                packet_position.get_z(),
                packet_position.get_yaw(),
                packet_position.get_pitch(),
            ));
        }
        PLAYER_MOVE_TEST_TELEPORT => event
            .player()
            .set_position(EntityPosition::new(8.0, 64.0, 8.0, 0.0, 0.0)),
        _ => {}
    }
}

struct PlayerMoveBehaviorScope<'a> {
    _lock: std::sync::MutexGuard<'a, ()>,
}

impl Drop for PlayerMoveBehaviorScope<'_> {
    fn drop(&mut self) {
        PLAYER_MOVE_TEST_BEHAVIOR.store(PLAYER_MOVE_TEST_PASSTHROUGH, Ordering::SeqCst);
    }
}

struct FailingChunkLoader;

impl ChunkLoader for FailingChunkLoader {
    fn load_chunk(&self, _position: ChunkPosition) -> io::Result<Option<Chunk>> {
        Err(Error::new(ErrorKind::Other, "load failed"))
    }

    fn save_chunk(&self, _chunk: &Chunk) -> io::Result<()> {
        Err(Error::new(ErrorKind::Other, "save failed"))
    }

    fn unload_chunk(&self, _chunk: &mut Chunk) -> io::Result<()> {
        Err(Error::new(ErrorKind::Other, "unload failed"))
    }
}

struct StoredChunkLoader {
    generation_callback_count: Arc<AtomicUsize>,
}

impl ChunkLoader for StoredChunkLoader {
    fn load_chunk(&self, position: ChunkPosition) -> io::Result<Option<Chunk>> {
        let generation_callback_count = Arc::clone(&self.generation_callback_count);
        let mut chunk = Chunk::new(position);
        chunk.set_block(BlockPosition::new(0, 0, 0), Block::BEDROCK);
        chunk.set_generation_callback(move |_| {
            generation_callback_count.fetch_add(1, Ordering::SeqCst);
        });
        Ok(Some(chunk))
    }

    fn save_chunk(&self, _chunk: &Chunk) -> io::Result<()> {
        Ok(())
    }

    fn unload_chunk(&self, _chunk: &mut Chunk) -> io::Result<()> {
        Ok(())
    }
}

fn player_move_behavior_scope(behavior: i32) -> PlayerMoveBehaviorScope<'static> {
    let lock = PLAYER_MOVE_TEST_LOCK.lock().unwrap();
    PLAYER_MOVE_TEST_BEHAVIOR.store(behavior, Ordering::SeqCst);
    PlayerMoveBehaviorScope { _lock: lock }
}

fn test_client_pair() -> (Client, TcpStream) {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let stream = std::net::TcpStream::connect(addr).unwrap();
    let (peer_stream, _) = listener.accept().unwrap();
    peer_stream
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    let mut client = Client::new(stream, addr);
    client.state = ConnectionState::Play;
    (client, peer_stream)
}

fn read_packet_frame(peer_stream: &mut TcpStream) -> (i32, Vec<u8>) {
    let frame_length = VarIntWrapper::decode(peer_stream).unwrap().0 as usize;
    let mut frame = vec![0; frame_length];
    peer_stream.read_exact(&mut frame).unwrap();
    let mut frame_cursor = Cursor::new(frame);
    let packet_id = VarIntWrapper::decode(&mut frame_cursor).unwrap().0;
    let payload_start = frame_cursor.position() as usize;
    let payload = frame_cursor.into_inner()[payload_start..].to_vec();
    (packet_id, payload)
}

fn read_available_packet_frames(peer_stream: &mut TcpStream) -> Vec<(i32, Vec<u8>)> {
    let previous_timeout = peer_stream.read_timeout().unwrap();
    peer_stream
        .set_read_timeout(Some(Duration::from_millis(25)))
        .unwrap();
    let mut packet_frames = Vec::new();
    loop {
        match VarIntWrapper::decode(peer_stream) {
            Ok(frame_length) => {
                let mut frame = vec![0; frame_length.0 as usize];
                peer_stream.read_exact(&mut frame).unwrap();
                let mut frame_cursor = Cursor::new(frame);
                let packet_id = VarIntWrapper::decode(&mut frame_cursor).unwrap().0;
                let payload_start = frame_cursor.position() as usize;
                let payload = frame_cursor.into_inner()[payload_start..].to_vec();
                packet_frames.push((packet_id, payload));
            }
            Err(_) => break,
        }
    }
    peer_stream.set_read_timeout(previous_timeout).unwrap();
    packet_frames
}

fn world_with_entered_player(client: &mut Client) -> World {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut player = Player::new(Uuid::nil(), "Player".to_string(), 0, client.addr);
    player.set_client(client);
    player.assign_world(world.uuid());
    player.set_position(EntityPosition::new(0.0, 64.0, 0.0, 0.0, 0.0));
    player.mark_entered_world();
    world.add_entity(Entity::Player(player));
    world
}

fn attach_server_to_client(server: &mut MinecraftServer, client: &mut Client) {
    client.server_ptr = Some(server as *mut MinecraftServer as usize);
}

fn player_position(world: &World, client: &Client) -> EntityPosition {
    world.player_by_addr(&client.addr).unwrap().get_position()
}

#[test]
fn configuration_player_inventory_changes_wait_for_play_login() {
    let registries = Registries::new_vanilla();
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Configuration;
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut player = Player::new(Uuid::nil(), "Player".to_string(), 0, client.addr);
    player.set_client(&mut client);
    assert!(
        player
            .get_inventory()
            .add_item_stack(ItemStack::of(Material::DIAMOND_PICKAXE))
    );
    world.add_entity(Entity::Player(player));

    world.tick_with_registries(&registries);

    let packet_ids = read_available_packet_frames(&mut peer_stream)
        .into_iter()
        .map(|(packet_id, _)| packet_id)
        .collect::<Vec<_>>();
    assert!(!packet_ids.contains(&SetPlayerInventoryPacket::get_id()));
}
#[test]
fn optional_chunk_load_respects_auto_chunk_loading_like_minestom() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let chunk_position = ChunkPosition::new(2, 3);

    world.enable_auto_chunk_load(false);

    assert!(world.load_optional_chunk(chunk_position).is_none());
    assert!(world.chunk(chunk_position).is_none());

    world.enable_auto_chunk_load(true);

    assert!(world.load_optional_chunk(chunk_position).is_some());
    assert!(world.chunk(chunk_position).is_some());
}

#[test]
fn chunk_unload_missing_chunk_is_minestom_noop() {
    let mut world = World::new(Identifier::minecraft("overworld"));

    assert!(!world.unload_chunk(ChunkPosition::new(4, 5)).unwrap());
}

#[test]
fn explicit_chunk_unload_sends_forget_packet_to_loaded_viewer() {
    let (mut client, mut peer_stream) = test_client_pair();
    let mut world = world_with_entered_player(&mut client);
    let registries = Registries::new_vanilla();

    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    let packet = world
        .chunk(ChunkPosition::new(0, 0))
        .unwrap()
        .full_data_packet(&registries)
        .unwrap();
    let player = world.player_by_addr_mut(&client.addr).unwrap();
    player.send_chunk(packet);
    player.send_pending_chunks().unwrap();
    let _ = read_packet_frame(&mut peer_stream);
    let _ = read_packet_frame(&mut peer_stream);
    let _ = read_packet_frame(&mut peer_stream);
    let _ = read_packet_frame(&mut peer_stream);

    assert!(world.unload_chunk(ChunkPosition::new(0, 0)).unwrap());

    let (packet_id, _) = read_packet_frame(&mut peer_stream);

    assert_eq!(packet_id, ForgetLevelChunkPacket::get_id());
}

#[test]
#[ignore]
fn measure_fast_movement_load_vs_throttled_send_cost() {
    let (mut client, mut peer_stream) = test_client_pair();
    let mut world = world_with_entered_player(&mut client);
    let registries = Registries::new_vanilla();
    world.set_view_distance(4);
    world.set_generator(|unit| {
        unit.modifier().fill_height(-64, 64, Block::STONE);
    });
    let drain_thread = thread::spawn(move || {
        let mut drained_bytes = 0usize;
        let mut buffer = [0u8; 8192];
        loop {
            match peer_stream.read(&mut buffer) {
                Ok(0) => return drained_bytes,
                Ok(bytes_read) => drained_bytes += bytes_read,
                Err(error)
                    if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) =>
                {
                    return drained_bytes;
                }
                Err(_) => return drained_bytes,
            }
        }
    });

    let movement_start = Instant::now();
    world
        .move_player(&mut client, 320.0, 64.0, 0.0, true, &registries)
        .unwrap();
    let movement_elapsed = movement_start.elapsed();
    let loaded_chunks = world.chunks().count();
    let queued_before_tick = world
        .player_by_addr(&client.addr)
        .unwrap()
        .get_queued_chunk_count();

    let tick_start = Instant::now();
    world.tick_with_registries(&registries);
    let tick_elapsed = tick_start.elapsed();
    let player = world.player_by_addr(&client.addr).unwrap();
    let queued_after_tick = player.get_queued_chunk_count();
    let chunk_batch_lead = player.get_chunk_batch_lead();
    let target_chunks_per_tick = player.get_target_chunks_per_tick();
    drop(world);
    drop(client);
    let drained_bytes = drain_thread.join().unwrap();

    println!(
        "loaded_chunks={} queued_before_tick={} move_load={:?} first_throttled_tick={:?} queued_after_tick={} chunk_batch_lead={} target_chunks_per_tick={} drained_bytes={}",
        loaded_chunks,
        queued_before_tick,
        movement_elapsed,
        tick_elapsed,
        queued_after_tick,
        chunk_batch_lead,
        target_chunks_per_tick,
        drained_bytes
    );
}

#[test]
#[ignore]
fn measure_fast_client_movement_chunk_generation_per_second() {
    let (mut client, mut peer_stream) = test_client_pair();
    let mut world = world_with_entered_player(&mut client);
    let registries = Registries::new_vanilla();
    let generated_chunks = Arc::new(AtomicUsize::new(0));
    let generated_chunk_counter = Arc::clone(&generated_chunks);
    world.set_view_distance(4);
    world.set_generator(move |unit| {
        generated_chunk_counter.fetch_add(1, Ordering::SeqCst);
        unit.modifier().fill_height(0, 3, Block::GRASS_BLOCK);
    });
    let drain_thread = thread::spawn(move || {
        let mut drained_bytes = 0usize;
        let mut buffer = [0u8; 16384];
        loop {
            match peer_stream.read(&mut buffer) {
                Ok(0) => return drained_bytes,
                Ok(bytes_read) => drained_bytes += bytes_read,
                Err(error)
                    if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) =>
                {
                    return drained_bytes;
                }
                Err(_) => return drained_bytes,
            }
        }
    });

    let movement_count = 40;
    let movement_start = Instant::now();
    for movement_step in 1..=movement_count {
        world
            .move_player(
                &mut client,
                movement_step as f64 * 24.0,
                64.0,
                0.0,
                true,
                &registries,
            )
            .unwrap();
        world
            .player_by_addr_mut(&client.addr)
            .unwrap()
            .on_chunk_batch_received(64.0);
        world.tick_with_registries(&registries);
    }
    let movement_elapsed = movement_start.elapsed();
    let generated_chunk_count = generated_chunks.load(Ordering::SeqCst);
    let generated_chunks_per_second = generated_chunk_count as f64 / movement_elapsed.as_secs_f64();
    let loaded_chunks = world.chunks().count();
    let player = world.player_by_addr(&client.addr).unwrap();
    let queued_chunks = player.get_queued_chunk_count();
    let chunk_batch_lead = player.get_chunk_batch_lead();
    let target_chunks_per_tick = player.get_target_chunks_per_tick();
    drop(world);
    drop(client);
    let drained_bytes = drain_thread.join().unwrap();

    println!(
        "movement_count={} generated_chunks={} elapsed={:?} generated_chunks_per_second={:.2} loaded_chunks={} queued_chunks={} chunk_batch_lead={} target_chunks_per_tick={} drained_bytes={}",
        movement_count,
        generated_chunk_count,
        movement_elapsed,
        generated_chunks_per_second,
        loaded_chunks,
        queued_chunks,
        chunk_batch_lead,
        target_chunks_per_tick,
        drained_bytes
    );
}

#[test]
fn too_large_player_coordinate_kicks_player_like_minestom() {
    let (mut client, mut peer_stream) = test_client_pair();
    let mut world = world_with_entered_player(&mut client);
    let registries = Registries::new_vanilla();

    world
        .move_player(&mut client, 30_000_001.0, 64.0, 0.0, true, &registries)
        .unwrap();

    let (packet_id, _) = read_packet_frame(&mut peer_stream);

    assert_eq!(packet_id, PlayDisconnectPacket::get_id());
    assert!(!client.is_online());
}

#[test]
fn movement_is_suppressed_while_teleport_confirmation_is_pending() {
    let (mut client, mut peer_stream) = test_client_pair();
    let mut world = world_with_entered_player(&mut client);
    let registries = Registries::new_vanilla();

    world
        .player_by_addr_mut(&client.addr)
        .unwrap()
        .get_next_teleport_id();
    world
        .move_player(&mut client, 1.0, 64.0, 0.0, true, &registries)
        .unwrap();

    peer_stream
        .set_read_timeout(Some(Duration::from_millis(25)))
        .unwrap();
    let mut trailing_packet_byte = [0u8; 1];

    assert_eq!(player_position(&world, &client).get_x(), 0.0);
    assert!(peer_stream.read(&mut trailing_packet_byte).is_err());
}

#[test]
fn movement_into_unloaded_destination_chunk_teleports_player_back() {
    let (mut client, mut peer_stream) = test_client_pair();
    let mut world = world_with_entered_player(&mut client);
    let registries = Registries::new_vanilla();

    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    world.enable_auto_chunk_load(false);
    world
        .move_player(&mut client, 16.0, 64.0, 0.0, true, &registries)
        .unwrap();

    let (packet_id, _) = read_packet_frame(&mut peer_stream);

    assert_eq!(packet_id, SyncPlayerPositionPacket::get_id());
    assert_eq!(player_position(&world, &client).get_x(), 0.0);
}

#[test]
fn incremental_walking_andsprinting_cross_east_chunk_borders_without_correction() {
    let (mut client, _peer_stream) = test_client_pair();
    client.enable_outbound_packet_queue();
    let mut world = world_with_entered_player(&mut client);
    let registries = Registries::new_vanilla();

    world
        .move_player(&mut client, 15.8, 64.0, 0.0, true, &registries)
        .unwrap();
    world
        .move_player(&mut client, 16.05, 64.0, 0.0, true, &registries)
        .unwrap();
    world
        .player_by_addr_mut(&client.addr)
        .unwrap()
        .set_sprinting(true);
    world
        .move_player(&mut client, 31.8, 64.0, 0.0, true, &registries)
        .unwrap();
    world
        .move_player(&mut client, 32.15, 64.0, 0.0, true, &registries)
        .unwrap();

    assert_eq!(player_position(&world, &client).get_x(), 32.15);
    assert!(
        !client
            .queued_outbound_packet_ids()
            .contains(&SyncPlayerPositionPacket::get_id())
    );
}

#[test]
fn backward_diagonal_movement_crosses_negative_chunk_borders_without_correction() {
    let (mut client, _peer_stream) = test_client_pair();
    client.enable_outbound_packet_queue();
    let mut world = world_with_entered_player(&mut client);
    let registries = Registries::new_vanilla();

    world
        .move_player(&mut client, -15.8, 64.0, -15.8, true, &registries)
        .unwrap();
    world
        .move_player(&mut client, -16.2, 64.0, -16.2, true, &registries)
        .unwrap();

    assert_eq!(
        player_position(&world, &client),
        EntityPosition::new(-16.2, 64.0, -16.2, 0.0, 0.0)
    );
    assert!(
        !client
            .queued_outbound_packet_ids()
            .contains(&SyncPlayerPositionPacket::get_id())
    );
}

#[test]
fn chunk_unload_and_entity_removal_packets_are_scoped_to_actual_viewers() {
    let (mut viewer_client, _viewer_peer) = test_client_pair();
    let (mut non_viewer_client, _non_viewer_peer) = test_client_pair();
    viewer_client.enable_outbound_packet_queue();
    non_viewer_client.enable_outbound_packet_queue();
    let mut world = World::new(Identifier::minecraft("overworld"));
    let chunk_position = ChunkPosition::new(2, 0);
    let mut viewer = Player::new(Uuid::new_v4(), "Viewer".to_string(), 0, viewer_client.addr);
    let mut non_viewer = Player::new(
        Uuid::new_v4(),
        "NonViewer".to_string(),
        0,
        non_viewer_client.addr,
    );
    viewer.set_client(&mut viewer_client);
    non_viewer.set_client(&mut non_viewer_client);
    viewer.mark_entered_world();
    non_viewer.mark_entered_world();
    viewer.mark_chunk_sent_to_client(PlayerChunk::new(2, 0));
    let viewer_id = viewer.get_entity_id();
    let mut entity = Entity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(32.0, 64.0, 0.0, 0.0, 0.0));
    entity.get_view_mut().manual_add(viewer_id);
    world.load_chunk(chunk_position).unwrap();
    world.add_entity(entity);
    world.add_entity(Entity::Player(viewer));
    world.add_entity(Entity::Player(non_viewer));
    viewer_client.discard_queued_outbound_packets();
    non_viewer_client.discard_queued_outbound_packets();

    assert!(world.unload_chunk(chunk_position).unwrap());

    let viewer_packets = viewer_client.queued_outbound_packet_ids();
    assert!(viewer_packets.contains(&ForgetLevelChunkPacket::get_id()));
    assert!(viewer_packets.contains(&RemoveEntitiesPacket::get_id()));
    assert!(non_viewer_client.queued_outbound_packet_ids().is_empty());
}

#[test]
fn cancelled_player_move_event_sends_correction_packet() {
    let _scope = player_move_behavior_scope(PLAYER_MOVE_TEST_CANCEL);
    let (mut client, mut peer_stream) = test_client_pair();
    let mut server = MinecraftServer::new();
    attach_server_to_client(&mut server, &mut client);
    let mut world = world_with_entered_player(&mut client);
    let registries = Registries::new_vanilla();

    world
        .move_player(&mut client, 1.0, 64.0, 0.0, true, &registries)
        .unwrap();

    let (packet_id, _) = read_packet_frame(&mut peer_stream);

    assert_eq!(packet_id, SyncPlayerPositionPacket::get_id());
    assert_eq!(player_position(&world, &client).get_x(), 0.0);
}

#[test]
fn player_move_event_mutatd_yaw_and_pitch_update_player_view() {
    let _scope = player_move_behavior_scope(PLAYER_MOVE_TEST_VIEW);
    let (mut client, mut peer_stream) = test_client_pair();
    let mut server = MinecraftServer::new();
    attach_server_to_client(&mut server, &mut client);
    let mut world = world_with_entered_player(&mut client);
    let registries = Registries::new_vanilla();

    world
        .move_player_with_view(&mut client, 1.0, 64.0, 0.0, 0.0, 0.0, true, &registries)
        .unwrap();

    peer_stream
        .set_read_timeout(Some(Duration::from_millis(25)))
        .unwrap();
    let mut trailing_packet_byte = [0u8; 1];
    let position = player_position(&world, &client);

    assert_eq!(position.get_x(), 1.0);
    assert_eq!(position.get_yaw(), 90.0);
    assert_eq!(position.get_pitch(), 45.0);
    assert!(peer_stream.read(&mut trailing_packet_byte).is_err());
}

#[test]
fn position_and_rotation_movement_queues_chunks_only_after_async_load_completion() {
    let (mut client, _peer_stream) = test_client_pair();
    let mut world = world_with_entered_player(&mut client);
    let registries = Registries::new_vanilla();
    world.set_generator(|_| {
        std::thread::sleep(Duration::from_millis(50));
    });

    world
        .move_player_with_view(&mut client, 16.0, 64.0, 0.0, 90.0, 45.0, true, &registries)
        .unwrap();

    assert_eq!(world.chunks().count(), 0);
    assert_eq!(
        world
            .player_by_addr(&client.addr)
            .unwrap()
            .get_queued_chunk_count(),
        0
    );

    let completion_deadline = Instant::now() + Duration::from_secs(2);
    while world
        .player_by_addr(&client.addr)
        .is_some_and(|player| player.get_queued_chunk_count() == 0)
    {
        assert!(Instant::now() < completion_deadline);
        world.tick_with_registries(&registries);
        std::thread::yield_now();
    }

    assert!(
        world
            .player_by_addr(&client.addr)
            .is_some_and(|player| player.get_queued_chunk_count() > 0)
    );
}

#[test]
fn recent_reverse_transition_does_not_schedule_chunk_loads_like_minestom() {
    let (mut client, _peer_stream) = test_client_pair();
    let mut world = world_with_entered_player(&mut client);
    let registries = Registries::new_vanilla();
    world.set_view_distance(1);
    world.set_generator(|_| {
        std::thread::sleep(Duration::from_millis(25));
    });

    world
        .move_player_with_view(&mut client, 16.0, 64.0, 0.0, 90.0, 0.0, true, &registries)
        .unwrap();
    world
        .move_player(&mut client, 32.0, 64.0, 0.0, true, &registries)
        .unwrap();
    world
        .move_player(&mut client, 16.0, 64.0, 0.0, true, &registries)
        .unwrap();

    assert_eq!(
        world
            .player_by_addr(&client.addr)
            .unwrap()
            .get_queued_chunk_count(),
        0
    );

    let completion_deadline = Instant::now() + Duration::from_secs(2);
    while world.chunks().count() < 10 {
        assert!(Instant::now() < completion_deadline);
        world.process_completed_chunk_loads().unwrap();
        std::thread::yield_now();
    }

    assert_eq!(
        world
            .player_by_addr(&client.addr)
            .unwrap()
            .get_queued_chunk_count(),
        10
    );
}

#[test]
fn rapid_forward_reverse_stop_loads_and_lights_the_standing_chunk() {
    let (mut client, _peer_stream) = test_client_pair();
    client.enable_outbound_packet_queue();
    let registries = Registries::new_vanilla();
    let destination_chunk = ChunkPosition::new(10, 0);
    let mut world = World::new(Identifier::minecraft("overworld"));
    world.set_view_distance(1);
    let mut player = Player::new(Uuid::nil(), "Player".to_string(), 0, client.addr);
    player.set_client(&mut client);
    world.add_entity(Entity::Player(player));
    world.enter_player(&mut client, 20, &registries).unwrap();

    let entry_deadline = Instant::now() + Duration::from_secs(3);
    while world
        .player_by_addr(&client.addr)
        .is_some_and(|player| !player.has_entered_world())
    {
        assert!(Instant::now() < entry_deadline);
        world.tick_with_registries(&registries);
        std::thread::yield_now();
    }
    let initial_delivery_deadline = Instant::now() + Duration::from_secs(10);
    while world
        .player_by_addr(&client.addr)
        .is_some_and(|player| player.get_queued_chunk_count() > 0)
    {
        assert!(
            Instant::now() < initial_delivery_deadline,
            "queued_chunks={} chunk_batch_lead={}",
            world
                .player_by_addr(&client.addr)
                .unwrap()
                .get_queued_chunk_count(),
            world
                .player_by_addr(&client.addr)
                .unwrap()
                .get_chunk_batch_lead()
        );
        if world
            .player_by_addr(&client.addr)
            .is_some_and(|player| player.get_chunk_batch_lead() > 0)
        {
            world
                .player_by_addr_mut(&client.addr)
                .unwrap()
                .on_chunk_batch_received(64.0);
        }
        world.tick_with_registries(&registries);
        std::thread::yield_now();
    }
    world.set_chunk_supplier(Chunk::new_lighting);
    world.set_generator(|unit| {
        std::thread::sleep(Duration::from_millis(5));
        unit.modifier().fill_height(0, 4, Block::GRASS_BLOCK);
    });

    world
        .move_player(&mut client, 160.0, 64.0, 0.0, true, &registries)
        .unwrap();
    world
        .move_player(&mut client, 0.0, 64.0, 0.0, true, &registries)
        .unwrap();
    world
        .move_player(&mut client, 160.0, 64.0, 0.0, true, &registries)
        .unwrap();

    let completion_deadline = Instant::now() + Duration::from_secs(3);
    while Instant::now() < completion_deadline {
        if world
            .player_by_addr(&client.addr)
            .is_some_and(|player| player.get_chunk_batch_lead() > 0)
        {
            world
                .player_by_addr_mut(&client.addr)
                .unwrap()
                .on_chunk_batch_received(64.0);
        }
        world.tick_with_registries(&registries);
        let destination_is_lit =
            client
                .queued_outbound_packet_payloads()
                .into_iter()
                .any(|(packet_id, payload)| {
                    if packet_id == ChunkDataAndUpdateLightPacket::get_id() {
                        return ChunkDataAndUpdateLightPacket::decode(&mut Cursor::new(payload))
                            .ok()
                            .is_some_and(|packet| {
                                packet.chunk_x == destination_chunk.x
                                    && packet.chunk_z == destination_chunk.z
                                    && !packet.light_data.sky_light_mask.is_empty()
                                    && !packet.light_data.sky_light_arrays.is_empty()
                            });
                    }
                    if packet_id != LightUpdatePacket::get_id() {
                        return false;
                    }
                    LightUpdatePacket::decode(&mut Cursor::new(payload))
                        .ok()
                        .is_some_and(|packet| {
                            packet.chunk_x == destination_chunk.x
                                && packet.chunk_z == destination_chunk.z
                                && !packet.light_data.sky_light_mask.is_empty()
                                && !packet.light_data.sky_light_arrays.is_empty()
                        })
                });
        if destination_is_lit {
            break;
        }
        std::thread::yield_now();
    }

    let packets = client.queued_outbound_packet_payloads();
    let final_cache_center = packets
        .iter()
        .filter(|(packet_id, _)| *packet_id == SetChunkCacheCenterPacket::get_id())
        .filter_map(|(_, payload)| {
            SetChunkCacheCenterPacket::decode(&mut Cursor::new(payload)).ok()
        })
        .last()
        .map(|packet| ChunkPosition::new(packet.x, packet.z));
    let destination_light_data = packets
        .iter()
        .filter(|(packet_id, _)| *packet_id == ChunkDataAndUpdateLightPacket::get_id())
        .filter_map(|(_, payload)| {
            ChunkDataAndUpdateLightPacket::decode(&mut Cursor::new(payload)).ok()
        })
        .find(|packet| {
            packet.chunk_x == destination_chunk.x && packet.chunk_z == destination_chunk.z
        })
        .map(|packet| packet.light_data);
    let destination_light_update = packets
        .iter()
        .filter(|(packet_id, _)| *packet_id == LightUpdatePacket::get_id())
        .filter_map(|(_, payload)| LightUpdatePacket::decode(&mut Cursor::new(payload)).ok())
        .find(|packet| {
            packet.chunk_x == destination_chunk.x && packet.chunk_z == destination_chunk.z
        })
        .map(|packet| packet.light_data);
    let destination_has_skylight =
        destination_light_data.as_ref().is_some_and(|light_data| {
            !light_data.sky_light_mask.is_empty() && !light_data.sky_light_arrays.is_empty()
        }) || destination_light_update.as_ref().is_some_and(|light_data| {
            !light_data.sky_light_mask.is_empty() && !light_data.sky_light_arrays.is_empty()
        });

    assert_eq!(player_position(&world, &client).get_x(), 160.0);
    assert!(
        final_cache_center == Some(destination_chunk) && destination_has_skylight,
        "standing_chunk={destination_chunk:?} final_cache_center={final_cache_center:?} destination_light_data={destination_light_data:?} destination_light_update={destination_light_update:?}"
    );
}

#[test]
fn player_move_event_mutated_coordinates_teleport_player_to_event_position() {
    let _scope = player_move_behavior_scope(PLAYER_MOVE_TEST_COORDINATES);
    let (mut client, mut peer_stream) = test_client_pair();
    let mut server = MinecraftServer::new();
    attach_server_to_client(&mut server, &mut client);
    let mut world = world_with_entered_player(&mut client);
    let registries = Registries::new_vanilla();

    world
        .move_player(&mut client, 1.0, 64.0, 0.0, true, &registries)
        .unwrap();

    let (packet_id, _) = read_packet_frame(&mut peer_stream);

    assert_eq!(packet_id, SyncPlayerPositionPacket::get_id());
    assert_eq!(player_position(&world, &client).get_x(), 2.0);
}

#[test]
fn player_move_event_triggred_teleport_stops_original_movement() {
    let _scope = player_move_behavior_scope(PLAYER_MOVE_TEST_TELEPORT);
    let (mut client, mut peer_stream) = test_client_pair();
    let mut server = MinecraftServer::new();
    attach_server_to_client(&mut server, &mut client);
    let mut world = world_with_entered_player(&mut client);
    let registries = Registries::new_vanilla();

    world
        .move_player(&mut client, 1.0, 64.0, 0.0, true, &registries)
        .unwrap();

    peer_stream
        .set_read_timeout(Some(Duration::from_millis(25)))
        .unwrap();
    let mut trailing_packet_byte = [0u8; 1];

    assert_eq!(player_position(&world, &client).get_x(), 8.0);
    assert!(peer_stream.read(&mut trailing_packet_byte).is_err());
}

#[test]
fn chunk_loader_errors_propagate_through_fallible_load_api() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    world.set_chunk_loader(FailingChunkLoader);

    let load_error = match world.load_chunk(ChunkPosition::new(1, 1)) {
        Ok(_) => panic!("loader error should propagate"),
        Err(error) => error,
    };

    assert_eq!(load_error.kind(), ErrorKind::Other);
    assert!(world.chunk(ChunkPosition::new(1, 1)).is_none());
}

#[test]
fn loader_miss_uses_world_chunk_supplier_like_minestom() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    world.set_chunk_supplier(|_| Chunk::new(ChunkPosition::new(7, -9)));

    let chunk = world.load_chunk(ChunkPosition::new(1, 1)).unwrap();

    assert_eq!(chunk.x(), 7);
    assert_eq!(chunk.z(), -9);
    assert_eq!(
        world
            .chunk_supplier()
            .create_chunk(ChunkPosition::new(3, 4))
            .x(),
        7
    );
}

#[test]
fn stored_chunk_skips_generation_and_generation_callback() {
    let generation_callback_count = Arc::new(AtomicUsize::new(0));
    let mut world = World::new(Identifier::minecraft("overworld"));
    world.set_chunk_loader(StoredChunkLoader {
        generation_callback_count: Arc::clone(&generation_callback_count),
    });
    world.set_generator(|unit| {
        unit.modifier()
            .set_block(BlockPosition::new(0, 0, 0), Block::STONE);
    });

    let chunk = world.load_chunk(ChunkPosition::new(0, 0)).unwrap();

    assert_eq!(chunk.block(BlockPosition::new(0, 0, 0)), Block::BEDROCK);
    assert_eq!(generation_callback_count.load(Ordering::SeqCst), 0);
    assert!(chunk.should_generate());
}

#[test]
fn enter_player_loads_the_initial_spawn_view_without_one_chunk_per_tick_throttling() {
    let generation_callback_count = Arc::new(AtomicUsize::new(0));
    let registries = Registries::new_vanilla();
    let (mut client, _peer_stream) = test_client_pair();
    let mut world = World::new(Identifier::minecraft("overworld"));
    world.set_view_distance(1);
    world.set_chunk_loader(StoredChunkLoader {
        generation_callback_count,
    });
    world.add_entity(Entity::Player(Player::new(
        Uuid::new_v4(),
        "ChunkQueue".to_string(),
        0,
        client.addr,
    )));
    let initial_spawn_chunk_count = world
        .player_by_addr(&client.addr)
        .unwrap()
        .spawn_chunks(world.view_distance())
        .len();

    world.enter_player(&mut client, 20, &registries).unwrap();
    world.tick_with_registries(&registries);

    assert_eq!(world.chunks().count(), initial_spawn_chunk_count);
}

#[test]
fn enter_player_waits_for_loaded_chunks_before_sending_initial_chunk_batch() {
    let registries = Registries::new_vanilla();
    let (mut client, mut peer_stream) = test_client_pair();
    let mut world = World::new(Identifier::minecraft("overworld"));
    world.add_entity(Entity::Player(Player::new(
        Uuid::new_v4(),
        "ChunkQueue".to_string(),
        0,
        client.addr,
    )));

    world.enter_player(&mut client, 20, &registries).unwrap();

    let initial_packet_ids = read_available_packet_frames(&mut peer_stream)
        .into_iter()
        .map(|(packet_id, _)| packet_id)
        .collect::<Vec<_>>();

    assert!(!initial_packet_ids.contains(&ChunkBatchStartPacket::get_id()));
    assert!(!initial_packet_ids.contains(&ChunkBatchFinishedPacket::get_id()));
    assert!(!initial_packet_ids.contains(&SyncPlayerPositionPacket::get_id()));

    let entry_deadline = Instant::now() + Duration::from_secs(2);
    while world
        .player_by_addr(&client.addr)
        .is_some_and(|player| !player.has_entered_world())
    {
        assert!(Instant::now() < entry_deadline);
        world.tick_with_registries(&registries);
        std::thread::yield_now();
    }

    let tick_packet_ids = read_available_packet_frames(&mut peer_stream)
        .into_iter()
        .map(|(packet_id, _)| packet_id)
        .collect::<Vec<_>>();
    let chunk_batch_start_index = tick_packet_ids
        .iter()
        .position(|packet_id| *packet_id == ChunkBatchStartPacket::get_id())
        .unwrap();
    let chunk_batch_finished_index = tick_packet_ids
        .iter()
        .position(|packet_id| *packet_id == ChunkBatchFinishedPacket::get_id())
        .unwrap();

    assert!(
        tick_packet_ids[chunk_batch_start_index + 1..chunk_batch_finished_index]
            .iter()
            .all(|packet_id| *packet_id == ChunkDataAndUpdateLightPacket::get_id())
    );
    assert!(chunk_batch_finished_index > chunk_batch_start_index + 1);
    assert_eq!(
        tick_packet_ids[chunk_batch_finished_index + 1],
        SyncPlayerPositionPacket::get_id()
    );
}

#[test]
fn explicit_generator_runs_for_loaded_non_generating_chunk() {
    let generation_callback_count = Arc::new(AtomicUsize::new(0));
    let supplier_callback_count = Arc::clone(&generation_callback_count);
    let mut world = World::new(Identifier::minecraft("overworld"));
    world.set_chunk_supplier(move |position| {
        let callback_count = Arc::clone(&supplier_callback_count);
        let mut chunk = Chunk::new_with_generation(position, false);
        chunk.set_generation_callback(move |_| {
            callback_count.fetch_add(1, Ordering::SeqCst);
        });
        chunk
    });
    let position = ChunkPosition::new(0, 0);
    world.load_chunk(position).unwrap();
    assert_eq!(generation_callback_count.load(Ordering::SeqCst), 1);
    let explicit_generator = |unit: &mut crate::world::GenerationUnit| {
        unit.modifier()
            .set_block(BlockPosition::new(0, 0, 0), Block::STONE);
    };

    assert!(
        world
            .generate_chunk_with_result(position, &explicit_generator)
            .unwrap()
    );
    assert_eq!(
        world
            .chunk(position)
            .unwrap()
            .block(BlockPosition::new(0, 0, 0)),
        Block::STONE
    );
    assert!(!world.chunk(position).unwrap().should_generate());
    assert_eq!(generation_callback_count.load(Ordering::SeqCst), 1);
}

#[test]
fn generation_errors_propagate_through_fallible_chunk_load_api() {
    let generation_callback_count = Arc::new(AtomicUsize::new(0));
    let supplier_callback_count = Arc::clone(&generation_callback_count);
    let mut world = World::new(Identifier::minecraft("overworld"));
    world.set_chunk_supplier(move |position| {
        let callback_count = Arc::clone(&supplier_callback_count);
        let mut chunk = Chunk::new(position);
        chunk.set_generation_callback(move |_| {
            callback_count.fetch_add(1, Ordering::SeqCst);
        });
        chunk
    });
    world.set_fallible_generator(|_| {
        Err(GenerateChunkError::GeneratorFailed {
            reason: "test generator failed".to_string(),
        })
    });

    let load_error = match world.load_chunk(ChunkPosition::new(1, 1)) {
        Ok(_) => panic!("generator error should propagate"),
        Err(error) => error,
    };

    assert_eq!(load_error.kind(), ErrorKind::Other);
    assert!(world.chunk(ChunkPosition::new(1, 1)).is_some());
    assert!(
        world
            .chunk(ChunkPosition::new(1, 1))
            .is_some_and(Chunk::should_generate)
    );
    assert_eq!(generation_callback_count.load(Ordering::SeqCst), 1);
}

#[test]
fn chunk_state_accessors_match_minestom_chunk_api_capability() {
    let mut chunk = Chunk::new(ChunkPosition::new(2, -3));
    let copied_chunk = chunk.copy_for_position(ChunkPosition::new(4, 5));
    let copied_lighting_chunk =
        Chunk::new_lighting(ChunkPosition::new(6, 7)).copy_for_position(ChunkPosition::new(8, 9));
    let registries = Registries::new_vanilla();

    assert_ne!(chunk.identifier(), copied_chunk.identifier());
    assert!(!copied_chunk.is_lighting_chunk());
    assert!(copied_lighting_chunk.is_lighting_chunk());
    assert_eq!(chunk.x(), 2);
    assert_eq!(chunk.z(), -3);
    assert_eq!(chunk.min_section(), -4);
    assert_eq!(chunk.max_section(), 20);
    assert_eq!(
        chunk.world_position(),
        crate::world::BlockPosition::new(32, 0, -48)
    );
    assert!(chunk.section_at_block_y(-64).is_some());
    assert!(chunk.section_at_block_y(319).is_some());
    assert!(chunk.section_at_block_y(320).is_none());
    assert!(chunk.section(-4).is_some());
    assert!(chunk.section(20).is_none());

    assert_eq!(
        chunk.try_set_block(crate::world::BlockPosition::new(1, 64, 1), Block::STONE),
        SetChunkBlockResult::Changed
    );
    assert!(chunk.is_invalidated());
    let chunk_packet = chunk.full_data_packet(&registries).unwrap();

    assert_eq!(chunk_packet.chunk_x, 2);
    assert_eq!(chunk_packet.chunk_z, -3);
    assert_eq!(chunk_packet.chunk_data.sections.len(), 24);
    chunk.reset();

    assert_eq!(
        chunk.block(crate::world::BlockPosition::new(1, 64, 1)),
        Block::AIR
    );
    assert!(chunk.is_invalidated());
    assert!(chunk.should_generate());
    chunk.on_generate();
    assert!(chunk.should_generate());

    chunk.set_read_only(true);

    assert_eq!(
        chunk.try_set_block(crate::world::BlockPosition::new(1, 64, 1), Block::STONE),
        SetChunkBlockResult::ReadOnly
    );
    assert!(chunk.section_mut(4).is_none());
}

#[test]
fn chunk_biome_accessors_invalidate_cached_chunk_data() {
    let registries = Registries::new_vanilla();
    let mut chunk = Chunk::new(ChunkPosition::new(0, 0));

    let empty_chunk_data = chunk.data(&registries).unwrap();

    assert_eq!(
        chunk.biome(crate::world::BlockPosition::new(0, 64, 0)),
        spinel_registry::Biome::PLAINS
    );
    assert!(chunk.set_biome(
        crate::world::BlockPosition::new(0, 64, 0),
        spinel_registry::Biome::BADLANDS
    ));

    let changed_chunk_data = chunk.data(&registries).unwrap();
    let empty_biomes = &empty_chunk_data.sections[8].biomes;
    let changed_biomes = &changed_chunk_data.sections[8].biomes;

    assert_ne!(empty_biomes.bits_per_entry, changed_biomes.bits_per_entry);
    assert_ne!(empty_biomes.palette, changed_biomes.palette);

    chunk.fill_biome(spinel_registry::Biome::BAMBOO_JUNGLE);

    assert_eq!(
        chunk.biome(crate::world::BlockPosition::new(4, 64, 4)),
        spinel_registry::Biome::BAMBOO_JUNGLE
    );
}

#[test]
fn chunk_cache_and_heightmaps_invalidate_after_block_mutation() {
    let registries = Registries::new_vanilla();
    let mut chunk = Chunk::new(ChunkPosition::new(0, 0));

    let empty_chunk_data = chunk.data(&registries).unwrap();

    assert_eq!(empty_chunk_data.sections[4].block_count, 0);

    assert_eq!(
        chunk.try_set_block(crate::world::BlockPosition::new(1, 64, 1), Block::STONE),
        SetChunkBlockResult::Changed
    );

    let changed_chunk_data = chunk.data(&registries).unwrap();

    assert_eq!(changed_chunk_data.sections[8].block_count, 1);
    let empty_heightmap_data = empty_chunk_data
        .heightmaps
        .iter()
        .flat_map(|heightmap| heightmap.data.iter())
        .copied()
        .collect::<Vec<_>>();
    let changed_heightmap_data = changed_chunk_data
        .heightmaps
        .iter()
        .flat_map(|heightmap| heightmap.data.iter())
        .copied()
        .collect::<Vec<_>>();

    assert_ne!(empty_heightmap_data, changed_heightmap_data);
}

#[test]
fn chunk_loads_heightmaps_from_nbt_like_minestom() {
    let mut chunk = Chunk::new(ChunkPosition::new(0, 0));
    let mut heightmaps = spinel_nbt::NbtCompound::new();
    heightmaps.insert(
        "MOTION_BLOCKING".to_string(),
        spinel_nbt::Nbt::LongArray(vec![1; 37].into_boxed_slice()),
    );

    chunk.load_heightmaps_from_nbt(&heightmaps);

    assert_eq!(
        &chunk.motion_blocking_heightmap()[0..36],
        vec![1; 36].as_slice()
    );
}

#[test]
fn chunk_viewer_membership_matches_minestom_no_op_edges() {
    let mut chunk = Chunk::new(ChunkPosition::new(0, 0));
    let viewer = EntityId::next();

    assert!(chunk.add_viewer(viewer));
    assert!(!chunk.add_viewer(viewer));
    assert_eq!(
        chunk.viewers().collect::<Vec<_>>(),
        vec![viewer.get_value()]
    );
    assert!(chunk.remove_viewer(viewer));
    assert!(!chunk.remove_viewer(viewer));
    assert!(chunk.viewers().next().is_none());
}

#[test]
fn empty_chunk_light_data_uses_empty_section_masks_like_minestom() {
    let chunk = Chunk::new(ChunkPosition::new(0, 0));
    let light_data = chunk.light_data();

    assert!(light_data.sky_light_mask.is_empty());
    assert!(light_data.block_light_mask.is_empty());
    assert_eq!(light_data.empty_sky_light_mask, vec![0x01ff_fffe]);
    assert_eq!(light_data.empty_block_light_mask, vec![0x01ff_fffe]);
    assert!(light_data.sky_light_arrays.is_empty());
    assert!(light_data.block_light_arrays.is_empty());
}

#[test]
fn default_generated_chunk_uses_dynamic_chunk_empty_light_masks() {
    let registries = Registries::new_vanilla();
    let mut world = World::new(Identifier::minecraft("dynamic_chunk_default"));
    world.set_generator(|unit| {
        unit.modifier().fill_height(0, 4, Block::STONE);
    });

    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();

    let packet = world
        .chunk(ChunkPosition::new(0, 0))
        .unwrap()
        .full_data_packet(&registries)
        .unwrap();

    assert!(packet.light_data.sky_light_mask.is_empty());
    assert!(packet.light_data.block_light_mask.is_empty());
    assert_eq!(packet.light_data.empty_sky_light_mask, vec![0x01ff_fffe]);
    assert_eq!(packet.light_data.empty_block_light_mask, vec![0x01ff_fffe]);
    assert!(packet.light_data.sky_light_arrays.is_empty());
    assert!(packet.light_data.block_light_arrays.is_empty());
}

#[test]
fn lighting_chunk_supplier_preserves_full_light_data() {
    let registries = Registries::new_vanilla();
    let mut world = World::new(Identifier::minecraft("lighting_chunk_supplier"));
    world.set_chunk_supplier(Chunk::new_lighting);
    world.set_generator(|unit| {
        unit.modifier().fill_height(0, 4, Block::STONE);
    });

    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    world.relight_chunks(&[ChunkPosition::new(0, 0)]);

    let packet = world
        .chunk(ChunkPosition::new(0, 0))
        .unwrap()
        .full_data_packet(&registries)
        .unwrap();

    assert!(!packet.light_data.sky_light_mask.is_empty());
    assert!(!packet.light_data.sky_light_arrays.is_empty());
}

#[test]
fn added_entities_record_their_current_world_membership() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let world_uuid = world.uuid;
    let entity = Entity::new(EntityType::ZOMBIE);

    world.add_entity(entity);

    assert_eq!(
        world.entities().next().unwrap().get_world(),
        Some(world_uuid)
    );
}

#[test]
fn world_dimension_registration_and_void_api_match_minestom_world_surface() {
    let dimension_type = DimensionType::THE_NETHER;
    let cached_dimension_type = DimensionType::builder()
        .vertical_bounds(-32, 256, 128)
        .build();
    let world = World::new_with_dimension(
        Identifier::minecraft("the_nether"),
        dimension_type.clone(),
        cached_dimension_type,
    );

    assert!(!world.is_registered());
    assert_eq!(world.get_dimension_type(), &dimension_type);
    assert_eq!(world.cached_dimension_type().min_y, -32);
    assert_eq!(world.dimension_name(), &Identifier::minecraft("the_nether"));
    assert!(world.is_in_void(EntityPosition::new(0.0, -97.0, 0.0, 0.0, 0.0)));
    assert!(!world.is_in_void(EntityPosition::new(0.0, -96.0, 0.0, 0.0, 0.0)));
}

#[test]
fn world_entity_and_player_lookup_api_matches_minestom_world_surface() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let generic_entity = Entity::new(EntityType::ZOMBIE);
    let generic_entity_id = generic_entity.get_entity_id();
    let generic_entity_uuid = generic_entity.get_uuid();
    let player_socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565);
    let player = Player::new(Uuid::nil(), "Player".to_string(), 0, player_socket);
    let player_id = player.get_entity_id();
    let player_uuid = player.get_uuid();

    world.add_entity(generic_entity);
    world.add_entity(Entity::Player(player));

    assert_eq!(world.entities().count(), 2);
    assert_eq!(
        world.get_entity(generic_entity_id).map(Entity::get_uuid),
        Some(generic_entity_uuid)
    );
    assert_eq!(
        world.entity_by_uuid(player_uuid).map(Entity::get_entity_id),
        Some(player_id)
    );
    assert_eq!(world.players().count(), 1);
    assert_eq!(
        world.player_by_uuid(player_uuid).map(Player::get_entity_id),
        Some(player_id)
    );
}

#[test]
fn world_chunk_and_block_api_match_loaded_chunk_semantics() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let chunk_position = ChunkPosition::new(0, 0);

    assert_eq!(world.uuid(), world.uuid);
    assert_eq!(world.name(), &Identifier::minecraft("overworld"));
    assert!(!world.is_chunk_loaded(chunk_position));
    assert!(world.chunk_at(0.0, 0.0).is_none());
    assert!(
        world
            .loaded_block_at(crate::world::BlockPosition::new(1, 64, 1))
            .is_none()
    );

    let block = world
        .block_at(crate::world::BlockPosition::new(1, 64, 1))
        .unwrap();

    assert_eq!(block, Block::AIR);
    assert_eq!(
        world.biome_at(BlockPosition::new(1, 64, 1)).unwrap(),
        Biome::PLAINS
    );
    assert!(
        world
            .set_block(BlockPosition::new(1, 64, 1), Block::STONE)
            .unwrap()
    );
    assert!(world.is_chunk_loaded(chunk_position));
    assert!(world.chunk_at(0.0, 0.0).is_some());
    assert_eq!(world.chunks().count(), 1);
    assert_eq!(
        world.loaded_block_at(crate::world::BlockPosition::new(1, 64, 1)),
        Some(Block::STONE)
    );
    assert!(world.block_position_is_loaded(crate::world::BlockPosition::new(1, 64, 1)));
}

#[test]
fn world_time_api_matches_minestom_defaults_and_validation() {
    let mut world = World::new(Identifier::minecraft("overworld"));

    assert_eq!(world.world_age(), 0);
    assert_eq!(world.time(), 0);
    assert_eq!(world.time_rate(), 1);
    assert_eq!(world.time_synchronization_ticks(), 20);
    assert_eq!(world.view_distance(), 8);
    assert_eq!(world.time_packet().world_age, 0);
    assert_eq!(world.time_packet().time, 0);
    assert!(world.time_packet().tick_day_time);

    world.set_world_age(40).unwrap();
    world.set_time(6000).unwrap();
    world.set_time_rate(0).unwrap();
    world.set_time_synchronization_ticks(0).unwrap();
    world.set_view_distance(12);

    assert_eq!(world.world_age(), 40);
    assert_eq!(world.time(), 6000);
    assert_eq!(world.time_rate(), 0);
    assert_eq!(world.time_synchronization_ticks(), 0);
    assert_eq!(world.view_distance(), 12);
    assert!(!world.time_packet().tick_day_time);
    assert!(world.set_time_rate(-1).is_err());
    assert!(world.set_time_synchronization_ticks(-1).is_err());
}

#[test]
fn world_tick_advances_time_like_minestom() {
    let mut world = World::new(Identifier::minecraft("overworld"));

    world.tick();

    assert_eq!(world.world_age(), 1);
    assert_eq!(world.time(), 1);

    world.set_time_rate(0).unwrap();
    world.tick();

    assert_eq!(world.world_age(), 2);
    assert_eq!(world.time(), 1);
}
