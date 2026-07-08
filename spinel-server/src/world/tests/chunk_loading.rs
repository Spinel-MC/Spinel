use super::super::instance::World;
use crate::entity::{Entity, EntityPosition, Player};
use crate::network::client::instance::Client;
use crate::world::{Block, BlockPosition, Chunk, ChunkLoader, ChunkPosition, GenerateChunkError};
use spinel_core::network::clientbound::play::chunk_batch_finished::ChunkBatchFinishedPacket;
use spinel_core::network::clientbound::play::chunk_batch_start::ChunkBatchStartPacket;
use spinel_core::network::clientbound::play::chunk_data::ChunkDataAndUpdateLightPacket;
use spinel_core::network::clientbound::play::forget_level_chunk::ForgetLevelChunkPacket;
use spinel_core::network::clientbound::play::sync_player_pos::SyncPlayerPositionPacket;
use spinel_network::types::Identifier;
use spinel_network::{ConnectionState, DataType, VarIntWrapper};
use spinel_registry::Registries;
use std::io::{self, Cursor, Error, ErrorKind, Read};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use uuid::Uuid;

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

struct GatedParallelChunkLoader {
    can_load: Arc<AtomicBool>,
}

impl ChunkLoader for GatedParallelChunkLoader {
    fn load_chunk(&self, _position: ChunkPosition) -> io::Result<Option<Chunk>> {
        while !self.can_load.load(Ordering::SeqCst) {
            thread::yield_now();
        }
        Ok(None)
    }

    fn save_chunk(&self, _chunk: &Chunk) -> io::Result<()> {
        Ok(())
    }

    fn unload_chunk(&self, _chunk: &mut Chunk) -> io::Result<()> {
        Ok(())
    }

    fn supports_parallel_loading(&self) -> bool {
        true
    }
}

#[test]
fn optional_chunk_load_respects_auto_chunk_loading_like_minestom() {
    let mut world = test_world();
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
    let mut world = test_world();

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
fn chunk_loader_errors_propagate_through_fallible_load_api() {
    let mut world = test_world();
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
    let mut world = test_world();
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
    let mut world = test_world();
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
fn explicit_generator_runs_for_loaded_non_generating_chunk() {
    let generation_callback_count = Arc::new(AtomicUsize::new(0));
    let mut world = test_world();
    let position = ChunkPosition::new(0, 0);
    world.set_chunk_loader(StoredChunkLoader {
        generation_callback_count: Arc::clone(&generation_callback_count),
    });

    world.load_chunk(position).unwrap();
    world.set_generator(|unit| {
        unit.modifier()
            .set_block(BlockPosition::new(0, 0, 0), Block::STONE);
    });
    world.generate_chunk_result(position).unwrap();

    let chunk = world.chunk(position).unwrap();
    assert_eq!(chunk.block(BlockPosition::new(0, 0, 0)), Block::STONE);
    assert_eq!(generation_callback_count.load(Ordering::SeqCst), 0);
}

#[test]
fn generation_errors_propagate_through_fallible_chunk_load_api() {
    let mut world = test_world();
    world.set_fallible_generator(|_| -> Result<(), GenerateChunkError> {
        Err(GenerateChunkError::GeneratorFailed {
            reason: "boom".to_string(),
        })
    });

    let load_error = match world.load_chunk(ChunkPosition::new(1, 1)) {
        Ok(_) => panic!("generation error should propagate"),
        Err(error) => error,
    };

    assert_eq!(load_error.kind(), ErrorKind::Other);
    assert!(load_error.to_string().contains("boom"));
    assert!(world.chunk(ChunkPosition::new(1, 1)).is_some());
}

#[test]
fn enter_player_queues_the_initial_spawn_view_without_blocking_on_chunk_loads() {
    let generation_callback_count = Arc::new(AtomicUsize::new(0));
    let registries = Registries::new_vanilla();
    let (mut client, mut peer_stream) = test_client_pair();
    let mut world = test_world();
    world.set_view_distance(1);
    world.set_chunk_loader(StoredChunkLoader {
        generation_callback_count,
    });
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
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

    assert!(
        world
            .player_by_addr(&client.addr)
            .is_some_and(|player| player.has_entered_world())
    );
    assert!(world.chunks().count() < initial_spawn_chunk_count);
    let initial_packet_ids = read_available_packet_frames(&mut peer_stream)
        .into_iter()
        .map(|(packet_id, _)| packet_id)
        .collect::<Vec<_>>();
    assert!(initial_packet_ids.contains(&ChunkDataAndUpdateLightPacket::get_id()));
}

#[test]
fn enter_player_with_parallel_loader_enters_before_initial_chunks_finish_loading() {
    let registries = Registries::new_vanilla();
    let (mut client, mut peer_stream) = test_client_pair();
    let can_load = Arc::new(AtomicBool::new(false));
    let mut world = test_world();
    world.set_view_distance(1);
    world.set_chunk_loader(GatedParallelChunkLoader {
        can_load: Arc::clone(&can_load),
    });
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
    assert!(!initial_packet_ids.contains(&ChunkDataAndUpdateLightPacket::get_id()));
    assert!(!initial_packet_ids.contains(&SyncPlayerPositionPacket::get_id()));
    assert!(
        world
            .player_by_addr(&client.addr)
            .is_some_and(|player| player.has_entered_world())
    );
    can_load.store(true, Ordering::SeqCst);
    world.process_completed_chunk_loads().unwrap();
}

fn test_world() -> World {
    World::new_with_dimension_name(
        uuid::Uuid::new_v4(),
        spinel_registry::dimension_type::DimensionType::OVERWORLD,
        Identifier::minecraft("overworld"),
    )
}

fn test_client_pair() -> (Client, TcpStream) {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let stream = std::net::TcpStream::connect(addr).unwrap();
    let (peer_stream, _) = listener.accept().unwrap();
    peer_stream
        .set_read_timeout(Some(std::time::Duration::from_secs(1)))
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

fn world_with_entered_player(client: &mut Client) -> World {
    let mut world = test_world();
    let mut player = Player::new(Uuid::nil(), "Player".to_string(), 0, client.addr);
    player.set_client(client);
    player.assign_world(world.uuid());
    player.set_position(EntityPosition::new(0.0, 64.0, 0.0, 0.0, 0.0));
    player.mark_entered_world();
    world.add_entity(Entity::Player(player));
    world
}
fn read_available_packet_frames(peer_stream: &mut TcpStream) -> Vec<(i32, Vec<u8>)> {
    let previous_timeout = peer_stream.read_timeout().unwrap();
    peer_stream
        .set_read_timeout(Some(std::time::Duration::from_millis(25)))
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
