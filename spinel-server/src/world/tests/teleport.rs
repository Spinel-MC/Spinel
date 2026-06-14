use crate::entity::{Entity, EntityId, EntityPosition, GenericEntity, Player};
use crate::events::entity_teleport::EntityTeleportEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use crate::world::{Chunk, ChunkLoader, ChunkPosition, World};
use spinel_core::network::clientbound::play::entity_position_sync::EntityPositionSyncPacket;
use spinel_macros::event_listener;
use spinel_network::ConnectionState;
use spinel_network::types::{Identifier, TeleportFlags, Vector3d, Velocity};
use spinel_registry::EntityType;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use uuid::Uuid;

static TELEPORT_TEST_LOCK: Mutex<()> = Mutex::new(());
static TELEPORT_EVENT_STATE: Mutex<Option<TeleportEventState>> = Mutex::new(None);
static TELEPORT_EXPECTED_PREVIOUS_POSITION: Mutex<Option<EntityPosition>> = Mutex::new(None);
static TELEPORT_EVENT_OBSERVED_PREVIOUS_POSITION: AtomicBool = AtomicBool::new(false);
static TELEPORT_EVENT_ENTITY_ACCESSOR_MATCHED: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy)]
struct TeleportEventState {
    entity_id: EntityId,
    teleport_position: EntityPosition,
    new_position: EntityPosition,
    relative_flags: TeleportFlags,
}

struct DeferredTeleportChunkLoader {
    can_finish: Arc<AtomicBool>,
}

impl ChunkLoader for DeferredTeleportChunkLoader {
    fn load_chunk(&self, _position: ChunkPosition) -> io::Result<Option<Chunk>> {
        while !self.can_finish.load(Ordering::SeqCst) {
            std::thread::yield_now();
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

#[event_listener]
fn entity_teleport_listener(event: &mut EntityTeleportEvent, server: &mut MinecraftServer) {
    let event_entity_id = event.entity_id();
    if event.entity().entity_id() == event_entity_id {
        TELEPORT_EVENT_ENTITY_ACCESSOR_MATCHED.store(true, Ordering::SeqCst);
    }
    let current_position = server
        .world_manager
        .worlds()
        .iter()
        .find_map(|world| world.entity_by_id(event.entity_id()))
        .map(Entity::position);
    TELEPORT_EVENT_OBSERVED_PREVIOUS_POSITION.store(
        current_position == *TELEPORT_EXPECTED_PREVIOUS_POSITION.lock().unwrap(),
        Ordering::SeqCst,
    );
    *TELEPORT_EVENT_STATE.lock().unwrap() = Some(TeleportEventState {
        entity_id: event.entity_id(),
        teleport_position: event.teleport_position(),
        new_position: event.new_position(),
        relative_flags: event.relative_flags(),
    });
}

#[test]
fn world_player_teleport_dispatches_immutable_event_before_state_mutation() {
    let _lock = TELEPORT_TEST_LOCK.lock().unwrap();
    *TELEPORT_EVENT_STATE.lock().unwrap() = None;
    *TELEPORT_EXPECTED_PREVIOUS_POSITION.lock().unwrap() =
        Some(EntityPosition::new(1.0, 64.0, 1.0, 10.0, 20.0));
    TELEPORT_EVENT_OBSERVED_PREVIOUS_POSITION.store(false, Ordering::SeqCst);
    TELEPORT_EVENT_ENTITY_ACCESSOR_MATCHED.store(false, Ordering::SeqCst);

    let mut server = MinecraftServer::new();
    let world_uuid = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    let player_uuid = Uuid::new_v4();
    let mut player = Player::new(
        player_uuid,
        "TeleportPlayer".to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_position(EntityPosition::new(1.0, 64.0, 1.0, 10.0, 20.0));
    let entity_id = player.entity_id();
    server
        .world_manager
        .world_mut(world_uuid)
        .unwrap()
        .add_entity(Entity::Player(player));
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let requested_position = EntityPosition::new(2.0, 3.0, 4.0, 5.0, 6.0);
    let requested_flags = TeleportFlags::from_bitmask(TeleportFlags::X | TeleportFlags::Y_ROTATION);
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);

    let teleport = world
        .teleport_player(
            player_uuid,
            requested_position,
            None,
            requested_flags,
            false,
        )
        .unwrap()
        .unwrap();

    let event = TELEPORT_EVENT_STATE.lock().unwrap().unwrap();
    let expected_flags = requested_flags.with(TeleportFlags::DELTA_COORD);
    let expected_position = EntityPosition::new(3.0, 3.0, 4.0, 15.0, 6.0);
    assert_eq!(event.entity_id, entity_id);
    assert_eq!(event.teleport_position, requested_position);
    assert_eq!(event.new_position, expected_position);
    assert_eq!(event.relative_flags, expected_flags);
    assert!(TELEPORT_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));
    assert!(TELEPORT_EVENT_OBSERVED_PREVIOUS_POSITION.load(Ordering::SeqCst));
    assert_eq!(teleport.position(), expected_position);
    assert_eq!(
        server
            .world_manager
            .world(world_uuid)
            .unwrap()
            .entity_by_id(entity_id)
            .unwrap()
            .position(),
        expected_position
    );
}

#[test]
fn world_generic_entity_teleport_updates_viewers_after_event_and_state_change() {
    let _lock = TELEPORT_TEST_LOCK.lock().unwrap();
    *TELEPORT_EVENT_STATE.lock().unwrap() = None;
    let previous_position = EntityPosition::new(1.0, 64.0, 1.0, 10.0, 20.0);
    *TELEPORT_EXPECTED_PREVIOUS_POSITION.lock().unwrap() = Some(previous_position);
    TELEPORT_EVENT_OBSERVED_PREVIOUS_POSITION.store(false, Ordering::SeqCst);
    TELEPORT_EVENT_ENTITY_ACCESSOR_MATCHED.store(false, Ordering::SeqCst);

    let mut server = MinecraftServer::new();
    let world_uuid = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let mut viewer = Player::new(
        Uuid::new_v4(),
        "TeleportViewer".to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), viewer_client.addr.port()),
    );
    viewer.set_client(&mut viewer_client);
    viewer.mark_entered_world();
    let viewer_id = viewer.entity_id();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(previous_position);
    entity.set_velocity(Velocity(Vector3d {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    }));
    entity.view_mut().set_auto_viewable(false);
    let entity_id = entity.entity_id();
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.add_entity(Entity::Player(viewer));
    world.add_entity(Entity::Generic(entity));
    world.add_entity_viewer(entity_id, viewer_id).unwrap();
    viewer_client.discard_queued_outbound_packets();
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);

    let teleport = world
        .teleport_entity(
            entity_id,
            EntityPosition::new(8.0, 70.0, 9.0, 30.0, 40.0),
            None,
            TeleportFlags::absolute(),
        )
        .unwrap()
        .unwrap();

    assert!(TELEPORT_EVENT_OBSERVED_PREVIOUS_POSITION.load(Ordering::SeqCst));
    assert!(TELEPORT_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));
    assert_eq!(
        teleport.velocity(),
        Velocity(Vector3d {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        })
    );
    assert_eq!(
        server
            .world_manager
            .world(world_uuid)
            .unwrap()
            .entity_by_id(entity_id)
            .unwrap()
            .position(),
        teleport.position()
    );
    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![EntityPositionSyncPacket::get_id()]
    );
}

#[test]
fn entity_teleport_future_applies_state_only_when_completed() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let previous_position = EntityPosition::new(1.0, 64.0, 1.0, 10.0, 20.0);
    let destination = EntityPosition::new(8.0, 70.0, 9.0, 30.0, 40.0);
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(previous_position);
    let entity_id = entity.entity_id();
    world.add_entity(Entity::Generic(entity));

    let mut ticket = world
        .teleport_entity_future(
            entity_id,
            destination,
            None,
            TeleportFlags::absolute(),
            true,
        )
        .unwrap()
        .unwrap();

    assert!(!ticket.is_completed());
    assert_eq!(
        world.entity_by_id(entity_id).unwrap().position(),
        previous_position
    );

    let completed_teleport = world
        .complete_entity_teleport(&mut ticket)
        .unwrap()
        .unwrap();

    assert!(ticket.is_completed());
    assert_eq!(completed_teleport.position(), destination);
    assert_eq!(
        world.entity_by_id(entity_id).unwrap().position(),
        destination
    );
}

#[test]
fn entity_teleport_future_waits_for_parallel_destination_chunk_loading() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let can_finish = Arc::new(AtomicBool::new(false));
    world.set_chunk_loader(DeferredTeleportChunkLoader {
        can_finish: Arc::clone(&can_finish),
    });
    let previous_position = EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0);
    let destination = EntityPosition::new(32.0, 70.0, 1.0, 0.0, 0.0);
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(previous_position);
    let entity_id = entity.entity_id();
    world.add_entity(Entity::Generic(entity));

    let mut ticket = world
        .teleport_entity_future(
            entity_id,
            destination,
            None,
            TeleportFlags::absolute(),
            true,
        )
        .unwrap()
        .unwrap();

    assert!(
        world
            .complete_entity_teleport(&mut ticket)
            .unwrap()
            .is_none()
    );
    assert_eq!(
        world.entity_by_id(entity_id).unwrap().position(),
        previous_position
    );

    can_finish.store(true, Ordering::SeqCst);
    let completed_teleport = loop {
        if let Some(teleport) = world.complete_entity_teleport(&mut ticket).unwrap() {
            break teleport;
        }
        std::thread::sleep(Duration::from_millis(1));
    };

    assert_eq!(completed_teleport.position(), destination);
    assert_eq!(
        world.entity_by_id(entity_id).unwrap().position(),
        destination
    );
}

fn queued_client() -> Client {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let stream = std::net::TcpStream::connect(addr).unwrap();
    let (_peer_stream, _) = listener.accept().unwrap();
    let mut client = Client::new(stream, addr);
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    client
}
