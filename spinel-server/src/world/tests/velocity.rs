use crate::entity::{Entity, EntityId, EntityPosition, GenericEntity, Player};
use crate::events::entity_velocity::EntityVelocityEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::clientbound::play::entity_velocity::EntityVelocityPacket;
use spinel_macros::event_listener;
use spinel_network::ConnectionState;
use spinel_network::types::{Identifier, Vector3d, Velocity};
use spinel_registry::EntityType;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use uuid::Uuid;

static VELOCITY_TEST_LOCK: Mutex<()> = Mutex::new(());
static VELOCITY_TEST_ENTITY: Mutex<Option<EntityId>> = Mutex::new(None);
static VELOCITY_EVENT_CANCELLED: AtomicBool = AtomicBool::new(false);
static VELOCITY_EVENT_ENTITY_ACCESSOR_MATCHED: AtomicBool = AtomicBool::new(false);

#[event_listener]
fn entity_velocity_listener(event: &mut EntityVelocityEvent, _server: &mut MinecraftServer) {
    if *VELOCITY_TEST_ENTITY.lock().unwrap() != Some(event.get_entity_id()) {
        return;
    }
    let event_entity_id = event.get_entity_id();
    if event.get_entity().get_entity_id() == event_entity_id {
        VELOCITY_EVENT_ENTITY_ACCESSOR_MATCHED.store(true, Ordering::SeqCst);
    }
    event.set_velocity(Velocity(Vector3d {
        x: 4.0,
        y: 5.0,
        z: 6.0,
    }));
    event.set_cancelled(VELOCITY_EVENT_CANCELLED.load(Ordering::SeqCst));
}

#[test]
fn world_set_entity_velocity_applies_event_mutation_and_sends_packet_to_viewers() {
    let _lock = VELOCITY_TEST_LOCK
        .lock()
        .unwrap_or_else(|poisoned_lock| poisoned_lock.into_inner());
    VELOCITY_EVENT_CANCELLED.store(false, Ordering::SeqCst);
    VELOCITY_EVENT_ENTITY_ACCESSOR_MATCHED.store(false, Ordering::SeqCst);
    let (mut server, viewer_client, entity_id) = velocity_server();
    *VELOCITY_TEST_ENTITY.lock().unwrap() = Some(entity_id);
    let world_uuid = server.world_manager.world_uuids()[0];
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);

    assert!(
        world
            .set_entity_velocity(
                entity_id,
                Velocity(Vector3d {
                    x: 1.0,
                    y: 2.0,
                    z: 3.0,
                }),
            )
            .unwrap()
    );

    assert_eq!(
        world.get_entity(entity_id).unwrap().get_velocity(),
        Velocity(Vector3d {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        })
    );
    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![EntityVelocityPacket::get_id()]
    );
    assert!(VELOCITY_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));
    *VELOCITY_TEST_ENTITY.lock().unwrap() = None;
}

#[test]
fn world_set_entity_velocity_cancellation_preserves_state_and_sends_no_packet() {
    let _lock = VELOCITY_TEST_LOCK
        .lock()
        .unwrap_or_else(|poisoned_lock| poisoned_lock.into_inner());
    VELOCITY_EVENT_CANCELLED.store(true, Ordering::SeqCst);
    VELOCITY_EVENT_ENTITY_ACCESSOR_MATCHED.store(false, Ordering::SeqCst);
    let (mut server, viewer_client, entity_id) = velocity_server();
    *VELOCITY_TEST_ENTITY.lock().unwrap() = Some(entity_id);
    let world_uuid = server.world_manager.world_uuids()[0];
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);

    assert!(
        !world
            .set_entity_velocity(
                entity_id,
                Velocity(Vector3d {
                    x: 1.0,
                    y: 2.0,
                    z: 3.0,
                }),
            )
            .unwrap()
    );

    assert_eq!(
        world.get_entity(entity_id).unwrap().get_velocity(),
        Velocity(Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        })
    );
    assert!(viewer_client.queued_outbound_packet_ids().is_empty());
    assert!(VELOCITY_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));
    VELOCITY_EVENT_CANCELLED.store(false, Ordering::SeqCst);
    *VELOCITY_TEST_ENTITY.lock().unwrap() = None;
}

#[test]
fn world_set_player_velocity_sends_packet_to_the_player() {
    let _lock = VELOCITY_TEST_LOCK
        .lock()
        .unwrap_or_else(|poisoned_lock| poisoned_lock.into_inner());
    VELOCITY_EVENT_CANCELLED.store(false, Ordering::SeqCst);
    let mut server = MinecraftServer::new();
    let world_uuid = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    let mut player_client = Box::new(queued_client());
    let mut player = Player::new(
        Uuid::new_v4(),
        "VelocityPlayer".to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), player_client.addr.port()),
    );
    player.set_client(player_client.as_mut());
    player.mark_entered_world();
    let player_id = player.get_entity_id();
    *VELOCITY_TEST_ENTITY.lock().unwrap() = Some(player_id);
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);
    world.add_entity(Entity::Player(player));
    player_client.discard_queued_outbound_packets();

    assert!(
        world
            .set_entity_velocity(
                player_id,
                Velocity(Vector3d {
                    x: 1.0,
                    y: 2.0,
                    z: 3.0,
                }),
            )
            .unwrap()
    );

    assert_eq!(
        player_client.queued_outbound_packet_ids(),
        vec![EntityVelocityPacket::get_id()]
    );
    *VELOCITY_TEST_ENTITY.lock().unwrap() = None;
}

fn velocity_server() -> (MinecraftServer, Box<Client>, EntityId) {
    let mut server = MinecraftServer::new();
    let world_uuid = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    let mut viewer_client = Box::new(queued_client());
    let mut viewer = Player::new(
        Uuid::new_v4(),
        "VelocityViewer".to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), viewer_client.addr.port()),
    );
    viewer.set_client(viewer_client.as_mut());
    viewer.mark_entered_world();
    let viewer_id = viewer.get_entity_id();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0));
    entity.get_view_mut().set_auto_viewable(false);
    let entity_id = entity.get_entity_id();
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.add_entity(Entity::Player(viewer));
    world.add_entity(Entity::Generic(entity));
    world.add_entity_viewer(entity_id, viewer_id).unwrap();
    viewer_client.discard_queued_outbound_packets();
    (server, viewer_client, entity_id)
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
