use crate::entity::{Entity, EntityId, EntityPosition, GenericEntity, Player};
use crate::events::entity_fire_extinguish::EntityFireExtinguishEvent;
use crate::events::entity_set_fire::EntitySetFireEvent;
use crate::server::MinecraftServer;
use spinel_macros::event_listener;
use spinel_network::types::Identifier;
use spinel_registry::{EntityType, Registries};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

static LIVING_FIRE_TEST_LOCK: Mutex<()> = Mutex::new(());
static LIVING_FIRE_TEST_ENTITY: Mutex<Option<EntityId>> = Mutex::new(None);
static LIVING_FIRE_SET_EVENT_ENABLED: AtomicBool = AtomicBool::new(false);
static LIVING_FIRE_SET_EVENT_TICKS: AtomicI32 = AtomicI32::new(0);
static LIVING_FIRE_EXTINGUISH_EVENT_ENABLED: AtomicBool = AtomicBool::new(false);
static LIVING_FIRE_EXTINGUISH_NATURAL: AtomicBool = AtomicBool::new(false);
static LIVING_FIRE_SET_EVENT_ENTITY_ACCESSOR_MATCHED: AtomicBool = AtomicBool::new(false);
static LIVING_FIRE_EXTINGUISH_EVENT_ENTITY_ACCESSOR_MATCHED: AtomicBool = AtomicBool::new(false);

#[event_listener]
fn living_fire_set_listener(event: &mut EntitySetFireEvent, _server: &mut MinecraftServer) {
    if !LIVING_FIRE_SET_EVENT_ENABLED.load(Ordering::SeqCst) {
        return;
    }
    if *LIVING_FIRE_TEST_ENTITY.lock().unwrap() != Some(event.get_entity_id()) {
        return;
    }
    let event_entity_id = event.get_entity_id();
    if event.get_entity().get_entity_id() == event_entity_id {
        LIVING_FIRE_SET_EVENT_ENTITY_ACCESSOR_MATCHED.store(true, Ordering::SeqCst);
    }
    event.set_fire_ticks(LIVING_FIRE_SET_EVENT_TICKS.load(Ordering::SeqCst));
}

#[event_listener]
fn living_fire_extinguish_listener(
    event: &mut EntityFireExtinguishEvent,
    _server: &mut MinecraftServer,
) {
    if !LIVING_FIRE_EXTINGUISH_EVENT_ENABLED.load(Ordering::SeqCst) {
        return;
    }
    if *LIVING_FIRE_TEST_ENTITY.lock().unwrap() != Some(event.get_entity_id()) {
        return;
    }
    let event_entity_id = event.get_entity_id();
    if event.get_entity().get_entity_id() == event_entity_id {
        LIVING_FIRE_EXTINGUISH_EVENT_ENTITY_ACCESSOR_MATCHED.store(true, Ordering::SeqCst);
    }
    LIVING_FIRE_EXTINGUISH_NATURAL.store(event.is_natural(), Ordering::SeqCst);
    event.set_cancelled(true);
}

#[test]
fn world_set_entity_fire_ticks_dispatches_minestom_set_fire_event_mutation() {
    let _lock = LIVING_FIRE_TEST_LOCK.lock().unwrap();
    reset_living_fire_test_state();
    let mut server = server_with_living_entity();
    let entity_id = tracked_living_entity_id(&server);
    *LIVING_FIRE_TEST_ENTITY.lock().unwrap() = Some(entity_id);
    LIVING_FIRE_SET_EVENT_ENABLED.store(true, Ordering::SeqCst);
    LIVING_FIRE_SET_EVENT_TICKS.store(3, Ordering::SeqCst);

    let world_uuid = server.world_manager.world_uuids()[0];
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);
    assert!(world.set_entity_fire_ticks(entity_id, 20));

    let entity = tracked_living_entity(&server);
    assert_eq!(entity.get_fire_ticks(), 3);
    assert!(entity.is_on_fire());
    assert!(LIVING_FIRE_SET_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));
    reset_living_fire_test_state();
}

#[test]
fn world_set_player_fire_ticks_uses_the_same_minestom_set_fire_path() {
    let _lock = LIVING_FIRE_TEST_LOCK.lock().unwrap();
    reset_living_fire_test_state();
    let mut server = server_with_player();
    let entity_id = tracked_player_entity_id(&server);
    *LIVING_FIRE_TEST_ENTITY.lock().unwrap() = Some(entity_id);
    LIVING_FIRE_SET_EVENT_ENABLED.store(true, Ordering::SeqCst);
    LIVING_FIRE_SET_EVENT_TICKS.store(4, Ordering::SeqCst);

    let world_uuid = server.world_manager.world_uuids()[0];
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);
    assert!(world.set_entity_fire_ticks(entity_id, 20));

    let player = tracked_player(&server);
    assert_eq!(player.get_fire_ticks(), 4);
    assert!(player.is_on_fire());
    assert!(LIVING_FIRE_SET_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));
    reset_living_fire_test_state();
}

#[test]
fn world_living_tick_dispatches_minestom_natural_fire_extinguish_event() {
    let _lock = LIVING_FIRE_TEST_LOCK.lock().unwrap();
    reset_living_fire_test_state();
    let mut server = server_with_living_entity();
    let entity_id = tracked_living_entity_id(&server);
    *LIVING_FIRE_TEST_ENTITY.lock().unwrap() = Some(entity_id);
    LIVING_FIRE_EXTINGUISH_EVENT_ENABLED.store(true, Ordering::SeqCst);
    let world_uuid = server.world_manager.world_uuids()[0];
    {
        let server_ptr = &mut server as *mut MinecraftServer as usize;
        let world = server.world_manager.world_mut(world_uuid).unwrap();
        world.use_server_event_dispatcher(server_ptr);
        assert!(world.set_entity_fire_ticks(entity_id, 1));
    }

    let registries = Registries::new_vanilla();
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    server.world_manager.tick(&registries, server_ptr);

    let entity = tracked_living_entity(&server);
    assert_eq!(entity.get_fire_ticks(), 0);
    assert!(entity.is_on_fire());
    assert!(LIVING_FIRE_EXTINGUISH_NATURAL.load(Ordering::SeqCst));
    assert!(LIVING_FIRE_EXTINGUISH_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));
    reset_living_fire_test_state();
}

#[test]
fn world_player_living_tick_dispatches_minestom_natural_fire_extinguish_event() {
    let _lock = LIVING_FIRE_TEST_LOCK.lock().unwrap();
    reset_living_fire_test_state();
    let mut server = server_with_player();
    let entity_id = tracked_player_entity_id(&server);
    *LIVING_FIRE_TEST_ENTITY.lock().unwrap() = Some(entity_id);
    LIVING_FIRE_EXTINGUISH_EVENT_ENABLED.store(true, Ordering::SeqCst);
    let world_uuid = server.world_manager.world_uuids()[0];
    {
        let server_ptr = &mut server as *mut MinecraftServer as usize;
        let world = server.world_manager.world_mut(world_uuid).unwrap();
        world.use_server_event_dispatcher(server_ptr);
        assert!(world.set_entity_fire_ticks(entity_id, 1));
    }

    let registries = Registries::new_vanilla();
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    server.world_manager.tick(&registries, server_ptr);

    let player = tracked_player(&server);
    assert_eq!(player.get_fire_ticks(), 0);
    assert!(player.is_on_fire());
    assert!(LIVING_FIRE_EXTINGUISH_NATURAL.load(Ordering::SeqCst));
    assert!(LIVING_FIRE_EXTINGUISH_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));
    reset_living_fire_test_state();
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

fn server_with_player() -> MinecraftServer {
    let mut server = MinecraftServer::new();
    let world_uuid = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    server
        .world_manager
        .world_mut(world_uuid)
        .unwrap()
        .add_entity(Entity::Player(Player::new(
            uuid::Uuid::new_v4(),
            "FirePlayer".to_string(),
            0,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
        )));
    server
}

fn positioned_living_entity() -> GenericEntity {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0));
    entity
}

fn tracked_living_entity_id(server: &MinecraftServer) -> EntityId {
    tracked_living_entity(server).get_entity_id()
}

fn tracked_living_entity(server: &MinecraftServer) -> &GenericEntity {
    server.world_manager.worlds()[0].creatures()[0]
}

fn tracked_player_entity_id(server: &MinecraftServer) -> EntityId {
    tracked_player(server).get_entity_id()
}

fn tracked_player(server: &MinecraftServer) -> &Player {
    server.world_manager.worlds()[0].players().next().unwrap()
}

fn reset_living_fire_test_state() {
    *LIVING_FIRE_TEST_ENTITY.lock().unwrap() = None;
    LIVING_FIRE_SET_EVENT_ENABLED.store(false, Ordering::SeqCst);
    LIVING_FIRE_SET_EVENT_TICKS.store(0, Ordering::SeqCst);
    LIVING_FIRE_EXTINGUISH_EVENT_ENABLED.store(false, Ordering::SeqCst);
    LIVING_FIRE_EXTINGUISH_NATURAL.store(false, Ordering::SeqCst);
    LIVING_FIRE_SET_EVENT_ENTITY_ACCESSOR_MATCHED.store(false, Ordering::SeqCst);
    LIVING_FIRE_EXTINGUISH_EVENT_ENTITY_ACCESSOR_MATCHED.store(false, Ordering::SeqCst);
}
