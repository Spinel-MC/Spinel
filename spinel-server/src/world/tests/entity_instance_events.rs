use crate::entity::{Entity, EntityId, GenericEntity};
use crate::events::add_entity_to_instance::AddEntityToInstanceEvent;
use crate::events::entity_despawn::EntityDespawnEvent;
use crate::events::entity_spawn::EntitySpawnEvent;
use crate::events::remove_entity_from_instance::RemoveEntityFromInstanceEvent;
use crate::server::MinecraftServer;
use crate::world::{World, WorldManager};
use spinel_macros::event_listener;
use spinel_network::types::Identifier;
use spinel_registry::EntityType;
use std::io::ErrorKind;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

static ENTITY_INSTANCE_EVENT_TEST_LOCK: Mutex<()> = Mutex::new(());
static ENTITY_INSTANCE_EVENT_TARGET: Mutex<Option<EntityId>> = Mutex::new(None);
static ENTITY_INSTANCE_EVENT_LOG: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());
static ENTITY_INSTANCE_EVENT_CANCEL_ADD: AtomicBool = AtomicBool::new(false);

#[event_listener]
fn entity_add_to_instance_listener(
    event: &mut AddEntityToInstanceEvent,
    _server: &mut MinecraftServer,
) {
    if *ENTITY_INSTANCE_EVENT_TARGET.lock().unwrap() != Some(event.get_entity_id()) {
        return;
    }
    ENTITY_INSTANCE_EVENT_LOG.lock().unwrap().push("add");
    event.set_cancelled(ENTITY_INSTANCE_EVENT_CANCEL_ADD.load(Ordering::SeqCst));
}

#[event_listener]
fn entity_spawn_listener(event: &mut EntitySpawnEvent, _server: &mut MinecraftServer) {
    if *ENTITY_INSTANCE_EVENT_TARGET.lock().unwrap() != Some(event.get_entity_id()) {
        return;
    }
    ENTITY_INSTANCE_EVENT_LOG.lock().unwrap().push("spawn");
}

#[event_listener]
fn entity_despawn_listener(event: &mut EntityDespawnEvent, _server: &mut MinecraftServer) {
    if *ENTITY_INSTANCE_EVENT_TARGET.lock().unwrap() != Some(event.get_entity_id()) {
        return;
    }
    ENTITY_INSTANCE_EVENT_LOG.lock().unwrap().push("despawn");
}

#[event_listener]
fn entity_remove_from_instance_listener(
    event: &mut RemoveEntityFromInstanceEvent,
    _server: &mut MinecraftServer,
) {
    if *ENTITY_INSTANCE_EVENT_TARGET.lock().unwrap() != Some(event.get_entity_id()) {
        return;
    }
    ENTITY_INSTANCE_EVENT_LOG.lock().unwrap().push("remove");
}

#[test]
fn world_add_entity_dispatches_add_and_spawn_events_in_minestom_order() {
    let _lock = ENTITY_INSTANCE_EVENT_TEST_LOCK.lock().unwrap();
    reset_entity_instance_event_state();
    let mut server = MinecraftServer::new();
    let mut world = event_world("entity_add_spawn", &mut server);
    let entity = Entity::Generic(GenericEntity::new(EntityType::ZOMBIE));
    let entity_id = entity.get_entity_id();
    *ENTITY_INSTANCE_EVENT_TARGET.lock().unwrap() = Some(entity_id);

    assert!(world.add_entity(entity));

    assert_eq!(recorded_entity_instance_events(), ["add", "spawn"]);
    assert_eq!(world.get_entity(entity_id).unwrap().get_world(), Some(world.uuid()));
}

#[test]
fn world_add_entity_cancellation_blocks_insert_and_spawn_event() {
    let _lock = ENTITY_INSTANCE_EVENT_TEST_LOCK.lock().unwrap();
    reset_entity_instance_event_state();
    let mut server = MinecraftServer::new();
    let mut world = event_world("entity_add_cancel", &mut server);
    let entity = Entity::Generic(GenericEntity::new(EntityType::ZOMBIE));
    let entity_id = entity.get_entity_id();
    *ENTITY_INSTANCE_EVENT_TARGET.lock().unwrap() = Some(entity_id);
    ENTITY_INSTANCE_EVENT_CANCEL_ADD.store(true, Ordering::SeqCst);

    assert!(!world.add_entity(entity));

    assert_eq!(recorded_entity_instance_events(), ["add"]);
    assert!(world.get_entity(entity_id).is_none());
}

#[test]
fn world_take_entity_dispatches_despawn_before_remove_from_instance() {
    let _lock = ENTITY_INSTANCE_EVENT_TEST_LOCK.lock().unwrap();
    reset_entity_instance_event_state();
    let mut server = MinecraftServer::new();
    let mut world = event_world("entity_take", &mut server);
    let entity = Entity::Generic(GenericEntity::new(EntityType::ZOMBIE));
    let entity_id = entity.get_entity_id();
    world.add_entity(entity);
    reset_entity_instance_event_log(entity_id);

    assert!(world.take_entity(entity_id).is_some());

    assert_eq!(recorded_entity_instance_events(), ["despawn", "remove"]);
}

#[test]
fn world_transfer_extraction_dispatches_remove_without_despawn() {
    let _lock = ENTITY_INSTANCE_EVENT_TEST_LOCK.lock().unwrap();
    reset_entity_instance_event_state();
    let mut server = MinecraftServer::new();
    let mut source_world = event_world("entity_transfer_source", &mut server);
    let mut target_world = event_world("entity_transfer_target", &mut server);
    let entity = Entity::Generic(GenericEntity::new(EntityType::ZOMBIE));
    let entity_id = entity.get_entity_id();
    source_world.add_entity(entity);
    reset_entity_instance_event_log(entity_id);

    let entity = source_world.take_entity_from_instance(entity_id).unwrap();
    assert!(target_world.add_entity(entity));

    assert_eq!(
        recorded_entity_instance_events(),
        ["remove", "add", "spawn"]
    );
    assert_eq!(
        target_world.get_entity(entity_id).unwrap().get_world(),
        Some(target_world.uuid())
    );
}

#[test]
fn world_manager_cancelled_transfer_keeps_entity_in_source_world() {
    let _lock = ENTITY_INSTANCE_EVENT_TEST_LOCK.lock().unwrap();
    reset_entity_instance_event_state();
    ENTITY_INSTANCE_EVENT_CANCEL_ADD.store(true, Ordering::SeqCst);
    let mut server = MinecraftServer::new();
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let mut worlds = WorldManager::new();
    let source_world = worlds.create_world(Identifier::minecraft("transfer_cancel_source"));
    let target_world = worlds.create_world(Identifier::minecraft("transfer_cancel_target"));
    let entity = Entity::Generic(GenericEntity::new(EntityType::ZOMBIE));
    let entity_id = entity.get_entity_id();
    assert!(worlds.add_entity(source_world, entity));
    worlds
        .world_mut(source_world)
        .unwrap()
        .use_server_event_dispatcher(server_ptr);
    worlds
        .world_mut(target_world)
        .unwrap()
        .use_server_event_dispatcher(server_ptr);
    reset_entity_instance_event_log(entity_id);
    ENTITY_INSTANCE_EVENT_CANCEL_ADD.store(true, Ordering::SeqCst);

    let result = worlds.set_entity_world_at_position(
        entity_id,
        target_world,
        crate::entity::EntityPosition::new(4.0, 64.0, 4.0, 0.0, 0.0),
    );

    assert_eq!(result.unwrap_err().kind(), ErrorKind::Interrupted);
    assert!(
        worlds
            .world(source_world)
            .unwrap()
            .get_entity(entity_id)
            .is_some()
    );
    assert!(
        worlds
            .world(target_world)
            .unwrap()
            .get_entity(entity_id)
            .is_none()
    );
    assert_eq!(recorded_entity_instance_events(), ["add"]);
}

fn event_world(name: &str, server: &mut MinecraftServer) -> World {
    let mut world = World::new(Identifier::minecraft(name));
    world.use_server_event_dispatcher(server as *mut MinecraftServer as usize);
    world
}

fn reset_entity_instance_event_state() {
    *ENTITY_INSTANCE_EVENT_TARGET.lock().unwrap() = None;
    ENTITY_INSTANCE_EVENT_LOG.lock().unwrap().clear();
    ENTITY_INSTANCE_EVENT_CANCEL_ADD.store(false, Ordering::SeqCst);
}

fn reset_entity_instance_event_log(entity_id: EntityId) {
    *ENTITY_INSTANCE_EVENT_TARGET.lock().unwrap() = Some(entity_id);
    ENTITY_INSTANCE_EVENT_LOG.lock().unwrap().clear();
    ENTITY_INSTANCE_EVENT_CANCEL_ADD.store(false, Ordering::SeqCst);
}

fn recorded_entity_instance_events() -> Vec<&'static str> {
    ENTITY_INSTANCE_EVENT_LOG.lock().unwrap().clone()
}
