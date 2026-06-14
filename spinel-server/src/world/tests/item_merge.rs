use crate::entity::{Entity, EntityId, EntityPosition, ItemEntity};
use crate::events::entity_item_merge::EntityItemMergeEvent;
use crate::server::MinecraftServer;
use crate::world::World;
use spinel_macros::event_listener;
use spinel_network::types::Identifier;
use spinel_registry::{ItemStack, Material, Registries};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

static ITEM_MERGE_TEST_LOCK: Mutex<()> = Mutex::new(());
static ITEM_MERGE_TEST_SOURCE: Mutex<Option<EntityId>> = Mutex::new(None);
static ITEM_MERGE_TEST_CANCELLED: AtomicBool = AtomicBool::new(false);
static ITEM_MERGE_TEST_MUTATES_RESULT: AtomicBool = AtomicBool::new(false);
static ITEM_MERGE_EVENT_ENTITY_ACCESSOR_MATCHED: AtomicBool = AtomicBool::new(false);

#[event_listener]
fn item_merge_test_listener(event: &mut EntityItemMergeEvent, _server: &mut MinecraftServer) {
    if ITEM_MERGE_TEST_SOURCE.lock().unwrap().is_none() {
        return;
    }
    let entity_id = event.entity_id();
    let merged_entity_id = event.merged_entity_id();
    let event_entity_id = event.entity().entity_id();
    let event_merged_entity_id = event.merged_entity().entity_id();
    if event_entity_id == entity_id && event_merged_entity_id == merged_entity_id {
        ITEM_MERGE_EVENT_ENTITY_ACCESSOR_MATCHED.store(true, Ordering::SeqCst);
    }
    event.set_cancelled(ITEM_MERGE_TEST_CANCELLED.load(Ordering::SeqCst));
    if ITEM_MERGE_TEST_MUTATES_RESULT.load(Ordering::SeqCst) {
        event.set_result(event.result().with_amount(7));
    }
}

#[test]
fn item_entities_merge_similar_stacks_and_reject_overfilled_results() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    world.set_world_age(10).unwrap();
    let source_id = add_item(&mut world, Material::DIAMOND, 5, 0.0);
    let merged_id = add_item(&mut world, Material::DIAMOND, 6, 0.5);
    let overfilled_source_id = add_item(&mut world, Material::EMERALD, 40, 10.0);
    let overfilled_merged_id = add_item(&mut world, Material::EMERALD, 30, 10.5);

    world.tick_with_registries(&Registries::new_vanilla());

    assert_eq!(item_amount(&world, source_id), Some(11));
    assert!(world.entity_by_id(merged_id).is_none());
    assert_eq!(item_amount(&world, overfilled_source_id), Some(40));
    assert_eq!(item_amount(&world, overfilled_merged_id), Some(30));
}

#[test]
fn item_merge_event_can_cancel_and_mutate_the_result() {
    let _lock = ITEM_MERGE_TEST_LOCK.lock().unwrap();
    reset_item_merge_test_state();
    let mut server = MinecraftServer::new();
    let world_uuid = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    let source_id;
    let merged_id;
    {
        let world = server.world_manager.world_mut(world_uuid).unwrap();
        world.set_world_age(10).unwrap();
        source_id = add_item(world, Material::DIAMOND, 2, 0.0);
        merged_id = add_item(world, Material::DIAMOND, 3, 0.5);
    }
    *ITEM_MERGE_TEST_SOURCE.lock().unwrap() = Some(source_id);
    ITEM_MERGE_TEST_CANCELLED.store(true, Ordering::SeqCst);
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);

    world.tick_with_registries(&Registries::new_vanilla());

    assert_eq!(item_amount(world, source_id), Some(2));
    assert_eq!(item_amount(world, merged_id), Some(3));
    assert!(ITEM_MERGE_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));
    ITEM_MERGE_TEST_CANCELLED.store(false, Ordering::SeqCst);
    ITEM_MERGE_TEST_MUTATES_RESULT.store(true, Ordering::SeqCst);
    ITEM_MERGE_EVENT_ENTITY_ACCESSOR_MATCHED.store(false, Ordering::SeqCst);
    world.set_world_age(20).unwrap();

    world.tick_with_registries(&Registries::new_vanilla());

    assert_eq!(item_amount(world, source_id), Some(7));
    assert!(world.entity_by_id(merged_id).is_none());
    assert!(ITEM_MERGE_EVENT_ENTITY_ACCESSOR_MATCHED.load(Ordering::SeqCst));
    reset_item_merge_test_state();
}

fn add_item(world: &mut World, material: Material, amount: i32, x: f64) -> EntityId {
    let mut item_entity = ItemEntity::new(ItemStack::of(material).with_amount(amount));
    item_entity.spawn(EntityPosition::new(x, 64.0, 0.0, 0.0, 0.0));
    let entity_id = item_entity.entity_id();
    world.add_entity(Entity::Item(item_entity));
    entity_id
}

fn item_amount(world: &World, entity_id: EntityId) -> Option<i32> {
    world
        .entity_by_id(entity_id)
        .and_then(|entity| match entity {
            Entity::Item(item_entity) => Some(item_entity.item_stack().amount()),
            _ => None,
        })
}

fn reset_item_merge_test_state() {
    *ITEM_MERGE_TEST_SOURCE.lock().unwrap() = None;
    ITEM_MERGE_TEST_CANCELLED.store(false, Ordering::SeqCst);
    ITEM_MERGE_TEST_MUTATES_RESULT.store(false, Ordering::SeqCst);
    ITEM_MERGE_EVENT_ENTITY_ACCESSOR_MATCHED.store(false, Ordering::SeqCst);
}
