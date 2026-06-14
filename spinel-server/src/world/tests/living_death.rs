use crate::entity::player::{Player, PlayerChunk, PlayerPose};
use crate::entity::{Entity, EntityId, EntityPosition, GenericEntity};
use crate::events::entity_death::EntityDeathEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use crate::world::World;
use spinel_core::network::clientbound::play::entity_status::EntityStatusPacket;
use spinel_core::network::clientbound::play::set_passengers::SetPassengersPacket;
use spinel_macros::event_listener;
use spinel_network::ConnectionState;
use spinel_network::types::{Identifier, Vector3d, Velocity};
use spinel_registry::EntityType;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use uuid::Uuid;

static LIVING_DEATH_TEST_LOCK: Mutex<()> = Mutex::new(());
static LIVING_DEATH_TEST_ENTITY: Mutex<Option<EntityId>> = Mutex::new(None);
static LIVING_DEATH_EVENT_FIRED: AtomicBool = AtomicBool::new(false);
static LIVING_DEATH_EVENT_ENTITY_ACCESSOR_MATCHED: AtomicBool = AtomicBool::new(false);

#[event_listener]
fn living_death_listener(event: &mut EntityDeathEvent, _server: &mut MinecraftServer) {
    if *LIVING_DEATH_TEST_ENTITY.lock().unwrap() != Some(event.entity_id()) {
        return;
    }
    let event_entity_id = event.entity_id();
    if event.entity().entity_id() == event_entity_id {
        LIVING_DEATH_EVENT_ENTITY_ACCESSOR_MATCHED.store(true, Ordering::SeqCst);
    }
    LIVING_DEATH_EVENT_FIRED.store(true, Ordering::SeqCst);
}

#[test]
fn world_kill_entity_sends_death_status_and_applies_generic_death_state_once() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let mut viewer = Player::new(
        Uuid::new_v4(),
        "DeathViewer".to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), viewer_client.addr.port()),
    );
    viewer.set_client(&mut viewer_client);
    viewer.mark_entered_world();
    viewer.mark_chunk_sent_to_client(PlayerChunk::new(0, 0));
    let mut entity = positioned_living_entity();
    let entity_id = entity.entity_id();
    let passenger_id = EntityId::from_raw(999);
    entity.add_passenger(passenger_id);
    entity.set_velocity(Velocity(Vector3d {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    }));

    world.add_entity(Entity::Generic(entity));
    world.add_entity(Entity::Player(viewer));
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();

    assert!(world.kill_entity(entity_id).unwrap());
    assert!(!world.kill_entity(entity_id).unwrap());

    let entity = world.creatures()[0];
    assert!(entity.is_dead());
    assert_eq!(entity.pose(), 6);
    assert_eq!(entity.velocity().0.x, 0.0);
    assert_eq!(entity.velocity().0.y, 0.0);
    assert_eq!(entity.velocity().0.z, 0.0);
    assert!(entity.passengers().is_empty());
    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![EntityStatusPacket::get_id(), SetPassengersPacket::get_id(),]
    );
}

#[test]
fn world_kill_entity_dispatches_entity_death_event_after_living_state_changes() {
    let _lock = LIVING_DEATH_TEST_LOCK.lock().unwrap();
    reset_living_death_test_state();
    let mut server = server_with_living_entity();
    let entity_id = tracked_living_entity_id(&server);
    *LIVING_DEATH_TEST_ENTITY.lock().unwrap() = Some(entity_id);

    let world_uuid = server.world_manager.instance_uuids()[0];
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);
    assert!(world.kill_entity(entity_id).unwrap());

    assert!(LIVING_DEATH_EVENT_FIRED.load(Ordering::SeqCst));
    assert!(LIVING_DEATH_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));
    assert!(tracked_living_entity(&server).is_dead());
    reset_living_death_test_state();
}

#[test]
fn world_kill_entity_applies_player_dying_pose_and_dead_state_once() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let player = Player::new(
        Uuid::new_v4(),
        "DeathPlayer".to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let player_id = player.entity_id();
    world.add_entity(Entity::Player(player));

    assert!(world.kill_entity(player_id).unwrap());
    assert!(!world.kill_entity(player_id).unwrap());

    let player = world.players().next().unwrap();
    assert!(player.is_dead());
    assert_eq!(player.pose(), PlayerPose::Dying);
}

fn server_with_living_entity() -> MinecraftServer {
    let mut server = MinecraftServer::new();
    let world_uuid = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    server
        .world_manager
        .world_mut(world_uuid)
        .unwrap()
        .add_entity(Entity::Generic(positioned_living_entity()));
    server
}

fn positioned_living_entity() -> GenericEntity {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0));
    entity
}

fn tracked_living_entity_id(server: &MinecraftServer) -> EntityId {
    tracked_living_entity(server).entity_id()
}

fn tracked_living_entity(server: &MinecraftServer) -> &GenericEntity {
    server.world_manager.worlds()[0].creatures()[0]
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

fn reset_living_death_test_state() {
    *LIVING_DEATH_TEST_ENTITY.lock().unwrap() = None;
    LIVING_DEATH_EVENT_FIRED.store(false, Ordering::SeqCst);
    LIVING_DEATH_EVENT_ENTITY_ACCESSOR_MATCHED.store(false, Ordering::SeqCst);
}
