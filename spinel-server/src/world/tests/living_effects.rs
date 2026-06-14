use crate::entity::player::{Player, PlayerChunk};
use crate::entity::{Entity, EntityId, EntityPosition, GenericEntity, TimedPotionEffect};
use crate::events::entity_potion_add::EntityPotionAddEvent;
use crate::events::entity_potion_remove::EntityPotionRemoveEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::clientbound::play::entity_effect::EntityEffectPacket;
use spinel_core::network::clientbound::play::remove_entity_effect::RemoveEntityEffectPacket;
use spinel_macros::event_listener;
use spinel_network::ConnectionState;
use spinel_network::types::Identifier;
use spinel_registry::{EntityType, Registries};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use uuid::Uuid;

static LIVING_EFFECT_TEST_LOCK: Mutex<()> = Mutex::new(());
static LIVING_EFFECT_TEST_ENTITY: Mutex<Option<EntityId>> = Mutex::new(None);
static LIVING_EFFECT_ADD_CANCELLED: AtomicBool = AtomicBool::new(false);
static LIVING_EFFECT_REMOVE_COUNT: AtomicU32 = AtomicU32::new(0);
static LIVING_EFFECT_ADD_EVENT_ENTITY_ACCESSOR_MATCHED: AtomicBool = AtomicBool::new(false);
static LIVING_EFFECT_REMOVE_EVENT_ENTITY_ACCESSOR_MATCHED: AtomicBool = AtomicBool::new(false);

#[event_listener]
fn living_effect_add_listener(event: &mut EntityPotionAddEvent, _server: &mut MinecraftServer) {
    if *LIVING_EFFECT_TEST_ENTITY.lock().unwrap() != Some(event.entity_id()) {
        return;
    }
    let event_entity_id = event.entity_id();
    if event.entity().entity_id() == event_entity_id {
        LIVING_EFFECT_ADD_EVENT_ENTITY_ACCESSOR_MATCHED.store(true, Ordering::SeqCst);
    }
    event.set_cancelled(LIVING_EFFECT_ADD_CANCELLED.load(Ordering::SeqCst));
}

#[event_listener]
fn living_effect_remove_listener(
    event: &mut EntityPotionRemoveEvent,
    _server: &mut MinecraftServer,
) {
    if *LIVING_EFFECT_TEST_ENTITY.lock().unwrap() != Some(event.entity_id()) {
        return;
    }
    let event_entity_id = event.entity_id();
    if event.entity().entity_id() == event_entity_id {
        LIVING_EFFECT_REMOVE_EVENT_ENTITY_ACCESSOR_MATCHED.store(true, Ordering::SeqCst);
    }
    LIVING_EFFECT_REMOVE_COUNT.fetch_add(1, Ordering::SeqCst);
}

#[test]
fn world_add_entity_effect_cancellation_preserves_effect_state_and_sends_no_packet() {
    let _lock = LIVING_EFFECT_TEST_LOCK.lock().unwrap();
    reset_living_effect_test_state();
    let (mut server, entity_id, viewer_client) = server_with_living_entity_and_viewer();
    *LIVING_EFFECT_TEST_ENTITY.lock().unwrap() = Some(entity_id);
    LIVING_EFFECT_ADD_CANCELLED.store(true, Ordering::SeqCst);
    let world_uuid = server.world_manager.instance_uuids()[0];
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);

    assert!(
        !world
            .add_entity_effect(entity_id, TimedPotionEffect::new(1, 2, 40, 6, 0))
            .unwrap()
    );
    assert!(!world.entity_by_id(entity_id).unwrap().effect(1).is_some());
    assert!(viewer_client.queued_outbound_packet_ids().is_empty());
    assert!(LIVING_EFFECT_ADD_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));
    reset_living_effect_test_state();
}

#[test]
fn world_add_entity_effect_replacement_removes_previous_effect_before_add_packet() {
    let _lock = LIVING_EFFECT_TEST_LOCK.lock().unwrap();
    reset_living_effect_test_state();
    let (mut server, entity_id, viewer_client) = server_with_living_entity_and_viewer();
    *LIVING_EFFECT_TEST_ENTITY.lock().unwrap() = Some(entity_id);
    let world_uuid = server.world_manager.instance_uuids()[0];
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);

    assert!(
        world
            .add_entity_effect(entity_id, TimedPotionEffect::new(1, 0, 40, 0, 0))
            .unwrap()
    );
    assert!(
        world
            .add_entity_effect(entity_id, TimedPotionEffect::new(1, 3, 60, 0, 0))
            .unwrap()
    );

    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![
            EntityEffectPacket::get_id(),
            RemoveEntityEffectPacket::get_id(),
            EntityEffectPacket::get_id(),
        ]
    );
    assert_eq!(LIVING_EFFECT_REMOVE_COUNT.load(Ordering::SeqCst), 1);
    assert!(LIVING_EFFECT_ADD_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));
    assert!(LIVING_EFFECT_REMOVE_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));
    assert_eq!(
        world
            .entity_by_id(entity_id)
            .unwrap()
            .effect(1)
            .unwrap()
            .amplifier(),
        3
    );
    reset_living_effect_test_state();
}

#[test]
fn world_living_tick_expires_effect_and_dispatches_remove_packet_and_event() {
    let _lock = LIVING_EFFECT_TEST_LOCK.lock().unwrap();
    reset_living_effect_test_state();
    let (mut server, entity_id, viewer_client) = server_with_living_entity_and_viewer();
    *LIVING_EFFECT_TEST_ENTITY.lock().unwrap() = Some(entity_id);
    let world_uuid = server.world_manager.instance_uuids()[0];
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);
    assert!(
        world
            .add_entity_effect(entity_id, TimedPotionEffect::new(1, 0, 1, 0, 0))
            .unwrap()
    );

    world.tick_with_registries(&Registries::new_vanilla());

    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![
            EntityEffectPacket::get_id(),
            RemoveEntityEffectPacket::get_id(),
        ]
    );
    assert_eq!(LIVING_EFFECT_REMOVE_COUNT.load(Ordering::SeqCst), 1);
    assert!(LIVING_EFFECT_ADD_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));
    assert!(LIVING_EFFECT_REMOVE_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));
    assert!(world.entity_by_id(entity_id).unwrap().effect(1).is_none());
    reset_living_effect_test_state();
}

fn server_with_living_entity_and_viewer() -> (MinecraftServer, EntityId, Box<Client>) {
    let mut server = MinecraftServer::new();
    let world_uuid = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    let mut viewer_client = Box::new(queued_client());
    let mut viewer = Player::new(
        Uuid::new_v4(),
        "EffectViewer".to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), viewer_client.addr.port()),
    );
    viewer.set_client(&mut viewer_client);
    viewer.mark_entered_world();
    viewer.mark_chunk_sent_to_client(PlayerChunk::new(0, 0));
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0));
    let entity_id = entity.entity_id();
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.add_entity(Entity::Generic(entity));
    world.add_entity(Entity::Player(viewer));
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();
    (server, entity_id, viewer_client)
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

fn reset_living_effect_test_state() {
    *LIVING_EFFECT_TEST_ENTITY.lock().unwrap() = None;
    LIVING_EFFECT_ADD_CANCELLED.store(false, Ordering::SeqCst);
    LIVING_EFFECT_REMOVE_COUNT.store(0, Ordering::SeqCst);
    LIVING_EFFECT_ADD_EVENT_ENTITY_ACCESSOR_MATCHED.store(false, Ordering::SeqCst);
    LIVING_EFFECT_REMOVE_EVENT_ENTITY_ACCESSOR_MATCHED.store(false, Ordering::SeqCst);
}
