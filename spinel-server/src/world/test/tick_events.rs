use crate::entity::{Entity, EntityPosition, GenericEntity};
use crate::events::instance_tick::InstanceTickEvent;
use crate::events::instance_tick_end::InstanceTickEndEvent;
use crate::server::MinecraftServer;
use spinel_macros::event_listener;
use spinel_network::types::Identifier;
use spinel_registry::{EntityType, Registries};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use uuid::Uuid;

static INSTANCE_TICK_EVENT_TEST_ENABLED: AtomicBool = AtomicBool::new(false);
static INSTANCE_TICK_EVENT_TEST_SEQUENCE: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());
static INSTANCE_TICK_EVENT_TEST_WORLD: Mutex<Option<Uuid>> = Mutex::new(None);
static INSTANCE_TICK_EVENT_TEST_LOCK: Mutex<()> = Mutex::new(());

#[event_listener]
fn instance_tick_test_listener(event: &mut InstanceTickEvent, _server: &mut MinecraftServer) {
    if !INSTANCE_TICK_EVENT_TEST_ENABLED.load(Ordering::SeqCst) {
        return;
    }
    let world = event.world();
    if *INSTANCE_TICK_EVENT_TEST_WORLD.lock().unwrap() != Some(world.uuid()) {
        return;
    }
    let creature_ticks = world
        .creatures()
        .first()
        .map(|entity| entity.ticks())
        .unwrap_or_default();
    INSTANCE_TICK_EVENT_TEST_SEQUENCE
        .lock()
        .unwrap()
        .push(if creature_ticks == 0 {
            "tick_before_entity"
        } else {
            "tick_after_entity"
        });
}

#[event_listener]
fn instance_tick_end_test_listener(
    event: &mut InstanceTickEndEvent,
    _server: &mut MinecraftServer,
) {
    if !INSTANCE_TICK_EVENT_TEST_ENABLED.load(Ordering::SeqCst) {
        return;
    }
    let world = event.world();
    if *INSTANCE_TICK_EVENT_TEST_WORLD.lock().unwrap() != Some(world.uuid()) {
        return;
    }
    let creature_ticks = world
        .creatures()
        .first()
        .map(|entity| entity.ticks())
        .unwrap_or_default();
    INSTANCE_TICK_EVENT_TEST_SEQUENCE
        .lock()
        .unwrap()
        .push(if creature_ticks == 1 {
            "tick_end_after_entity"
        } else {
            "tick_end_before_entity"
        });
}

#[test]
fn instance_tick_events_surround_world_tick_work() {
    let _lock = INSTANCE_TICK_EVENT_TEST_LOCK.lock().unwrap();
    INSTANCE_TICK_EVENT_TEST_SEQUENCE.lock().unwrap().clear();
    INSTANCE_TICK_EVENT_TEST_ENABLED.store(true, Ordering::SeqCst);
    let mut server = MinecraftServer::new();
    let world_uuid = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    *INSTANCE_TICK_EVENT_TEST_WORLD.lock().unwrap() = Some(world_uuid);
    server
        .world_manager
        .world_mut(world_uuid)
        .unwrap()
        .add_entity(Entity::Generic(positioned_entity()));
    let registries = Registries::new_vanilla();
    let server_ptr = &mut server as *mut MinecraftServer as usize;

    server.world_manager.tick(&registries, server_ptr);

    INSTANCE_TICK_EVENT_TEST_ENABLED.store(false, Ordering::SeqCst);
    *INSTANCE_TICK_EVENT_TEST_WORLD.lock().unwrap() = None;
    assert_eq!(
        INSTANCE_TICK_EVENT_TEST_SEQUENCE.lock().unwrap().as_slice(),
        ["tick_before_entity", "tick_end_after_entity"]
    );
}

fn positioned_entity() -> GenericEntity {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0));
    entity
}
