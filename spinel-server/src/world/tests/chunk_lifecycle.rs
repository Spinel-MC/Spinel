use crate::entity::player::{Player, PlayerChunk};
use crate::entity::{Entity, EntityPosition};
use crate::events::world_chunk_load::WorldChunkLoadEvent;
use crate::events::world_chunk_unload::WorldChunkUnloadEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use crate::world::{Block, BlockPosition, Chunk, ChunkLoader, ChunkPosition};
use spinel_core::network::clientbound::play::forget_level_chunk::ForgetLevelChunkPacket;
use spinel_macros::event_listener;
use spinel_network::ConnectionState;
use spinel_network::types::{Identifier, TeleportFlags};
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

static CHUNK_LIFECYCLE_TEST_LOCK: Mutex<()> = Mutex::new(());
static CHUNK_LIFECYCLE_WORLD: Mutex<Option<Uuid>> = Mutex::new(None);
static CHUNK_LIFECYCLE_SEQUENCE: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());

#[event_listener]
fn chunk_load_lifecycle_listener(event: &mut WorldChunkLoadEvent, _server: &mut MinecraftServer) {
    let position = event.chunk_position();
    let world = event.world();
    if *CHUNK_LIFECYCLE_WORLD.lock().unwrap() != Some(world.uuid()) {
        return;
    }
    let chunk = world.chunk(position).unwrap();
    assert!(chunk.should_generate());
    assert_eq!(chunk.block(BlockPosition::new(1, 64, 1)), Block::STONE);
    CHUNK_LIFECYCLE_SEQUENCE.lock().unwrap().push("load_event");
}

#[event_listener]
fn chunk_unload_lifecycle_listener(
    event: &mut WorldChunkUnloadEvent,
    _server: &mut MinecraftServer,
) {
    let position = event.chunk_position();
    let world = event.world();
    if *CHUNK_LIFECYCLE_WORLD.lock().unwrap() != Some(world.uuid()) {
        return;
    }
    assert!(world.chunk(position).is_some_and(Chunk::is_loaded));
    let forget_packet_was_queued = world.players().any(|player| {
        player.get_client().is_some_and(|client| {
            client
                .queued_outbound_packet_ids()
                .contains(&ForgetLevelChunkPacket::get_id())
        })
    });
    assert!(forget_packet_was_queued);
    CHUNK_LIFECYCLE_SEQUENCE
        .lock()
        .unwrap()
        .push("unload_event");
}

struct LifecycleChunkLoader;

impl ChunkLoader for LifecycleChunkLoader {
    fn load_chunk(&self, _position: ChunkPosition) -> io::Result<Option<Chunk>> {
        Ok(None)
    }

    fn save_chunk(&self, _chunk: &Chunk) -> io::Result<()> {
        Ok(())
    }

    fn unload_chunk(&self, chunk: &mut Chunk) -> io::Result<()> {
        assert!(!chunk.is_loaded());
        CHUNK_LIFECYCLE_SEQUENCE
            .lock()
            .unwrap()
            .push("loader_unload");
        Ok(())
    }
}

struct TeleportChunkLoader {
    loaded_positions: Arc<Mutex<Vec<ChunkPosition>>>,
    failing_position: Option<ChunkPosition>,
}

impl ChunkLoader for TeleportChunkLoader {
    fn load_chunk(&self, position: ChunkPosition) -> io::Result<Option<Chunk>> {
        self.loaded_positions.lock().unwrap().push(position);
        if self.failing_position == Some(position) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "teleport chunk load failed",
            ));
        }
        Ok(None)
    }

    fn save_chunk(&self, _chunk: &Chunk) -> io::Result<()> {
        Ok(())
    }

    fn unload_chunk(&self, _chunk: &mut Chunk) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn chunk_load_event_observes_cached_generated_chunk_after_generation_callback() {
    let _lock = CHUNK_LIFECYCLE_TEST_LOCK.lock().unwrap();
    let mut server = MinecraftServer::new();
    let world_uuid = server
        .world_manager
        .create_world(Identifier::minecraft("chunk_lifecycle_load"));
    *CHUNK_LIFECYCLE_WORLD.lock().unwrap() = Some(world_uuid);
    CHUNK_LIFECYCLE_SEQUENCE.lock().unwrap().clear();
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);
    world.set_chunk_supplier(|position| {
        let mut chunk = Chunk::new(position);
        chunk.set_generation_callback(|_| {
            CHUNK_LIFECYCLE_SEQUENCE
                .lock()
                .unwrap()
                .push("generation_callback");
        });
        chunk
    });
    world.set_generator(|unit| {
        unit.modifier()
            .set_block(BlockPosition::new(1, 64, 1), Block::STONE);
    });

    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();

    assert_eq!(
        CHUNK_LIFECYCLE_SEQUENCE.lock().unwrap().as_slice(),
        ["generation_callback", "load_event"]
    );
    reset_lifecycle_state();
}

#[test]
fn chunk_unload_orders_forget_event_cache_removal_state_and_loader_hook() {
    let _lock = CHUNK_LIFECYCLE_TEST_LOCK.lock().unwrap();
    let mut server = MinecraftServer::new();
    let world_uuid = server
        .world_manager
        .create_world(Identifier::minecraft("chunk_lifecycle_unload"));
    let mut client = queued_client();
    let mut player = Player::new(Uuid::new_v4(), "ChunkViewer".to_owned(), 0, client.addr);
    player.set_client(&mut client);
    player.mark_entered_world();
    player.mark_chunk_sent_to_client(PlayerChunk::new(0, 0));
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.set_chunk_loader(LifecycleChunkLoader);
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    world.add_entity(Entity::Player(player));
    world.use_server_event_dispatcher(server_ptr);
    *CHUNK_LIFECYCLE_WORLD.lock().unwrap() = Some(world_uuid);
    CHUNK_LIFECYCLE_SEQUENCE.lock().unwrap().clear();

    assert!(world.unload_chunk(ChunkPosition::new(0, 0)).unwrap());

    assert!(world.chunk(ChunkPosition::new(0, 0)).is_none());
    assert_eq!(
        CHUNK_LIFECYCLE_SEQUENCE.lock().unwrap().as_slice(),
        ["unload_event", "loader_unload"]
    );
    reset_lifecycle_state();
}

#[test]
fn explicit_teleport_chunks_finish_loading_before_position_sync() {
    let mut failing_world =
        crate::world::World::new(Identifier::minecraft("failing_teleport_chunks"));
    let mut failing_client = queued_client();
    let failing_player_uuid = Uuid::new_v4();
    let mut failing_player = Player::new(
        failing_player_uuid,
        "FailingTeleport".to_owned(),
        0,
        failing_client.addr,
    );
    failing_player.set_client(&mut failing_client);
    failing_player.mark_entered_world();
    failing_world.add_entity(Entity::Player(failing_player));
    let failing_position = ChunkPosition::new(3, 0);
    failing_world.set_chunk_loader(TeleportChunkLoader {
        loaded_positions: Arc::new(Mutex::new(Vec::new())),
        failing_position: Some(failing_position),
    });
    let explicit_chunks = vec![ChunkPosition::new(2, 0).index(), failing_position.index()];

    assert!(
        failing_world
            .teleport_player(
                failing_player_uuid,
                EntityPosition::new(32.0, 64.0, 0.0, 0.0, 0.0),
                Some(explicit_chunks.clone()),
                TeleportFlags::absolute(),
                true,
            )
            .is_err()
    );
    assert!(failing_client.queued_outbound_packet_ids().is_empty());

    let loaded_positions = Arc::new(Mutex::new(Vec::new()));
    let mut world = crate::world::World::new(Identifier::minecraft("teleport_chunks"));
    let mut client = queued_client();
    let player_uuid = Uuid::new_v4();
    let mut player = Player::new(player_uuid, "Teleport".to_owned(), 0, client.addr);
    player.set_client(&mut client);
    player.mark_entered_world();
    world.add_entity(Entity::Player(player));
    world.set_chunk_loader(TeleportChunkLoader {
        loaded_positions: Arc::clone(&loaded_positions),
        failing_position: None,
    });

    world
        .teleport_player(
            player_uuid,
            EntityPosition::new(32.0, 64.0, 0.0, 0.0, 0.0),
            Some(explicit_chunks),
            TeleportFlags::absolute(),
            false,
        )
        .unwrap()
        .unwrap();

    assert_eq!(
        loaded_positions.lock().unwrap().as_slice(),
        [ChunkPosition::new(2, 0), ChunkPosition::new(3, 0)]
    );
    assert!(world.is_chunk_loaded(ChunkPosition::new(2, 0)));
    assert!(world.is_chunk_loaded(ChunkPosition::new(3, 0)));
    let packet_ids = client.queued_outbound_packet_ids();
    assert_eq!(
        packet_ids.first().copied(),
        Some(
            spinel_core::network::clientbound::play::set_chunk_cache_center::SetChunkCacheCenterPacket::get_id()
        )
    );
    assert_eq!(
        packet_ids.last().copied(),
        Some(
            spinel_core::network::clientbound::play::sync_player_pos::SyncPlayerPositionPacket::get_id()
        )
    );
    assert!(
        packet_ids[1..packet_ids.len() - 1]
            .iter()
            .all(|packet_id| *packet_id == ForgetLevelChunkPacket::get_id())
    );
    let player = world.player_by_uuid(player_uuid).unwrap();
    assert_eq!(player.get_position().get_x(), 32.0);
    assert_eq!(player.get_last_sent_teleport_id(), 0);
}

fn queued_client() -> Client {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let stream = std::net::TcpStream::connect(addr).unwrap();
    let _peer = listener.accept().unwrap();
    let mut client = Client::new(stream, addr);
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    client
}

fn reset_lifecycle_state() {
    *CHUNK_LIFECYCLE_WORLD.lock().unwrap() = None;
    CHUNK_LIFECYCLE_SEQUENCE.lock().unwrap().clear();
}
