use crate::entity::ItemEntity;
use crate::world::{Block, BlockPosition, World};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::{Identifier, Vector3d, Velocity};
use spinel_registry::Material;
use std::time::Duration;

#[test]
fn item_entity_owns_stack_metadata_and_pickup_delay() {
    let mut item_entity = ItemEntity::new(spinel_registry::ItemStack::of(Material::DIAMOND));

    assert_eq!(item_entity.item_stack().material(), &Material::DIAMOND);
    assert!(matches!(
        item_entity
            .metadata()
            .value(&crate::entity::metadata::definitions::item_stack()),
        MetadataValue::Slot(_)
    ));
    assert_eq!(item_entity.spawn_packet().data, 1);
    let velocity = Velocity(Vector3d {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    });
    item_entity.set_velocity(velocity);
    assert_eq!(
        item_entity.spawn_packet().velocity,
        Velocity(Vector3d {
            x: 0.05,
            y: 0.1,
            z: 0.15,
        })
    );
    item_entity.set_pickup_delay(Duration::from_secs(1));
    assert!(!item_entity.is_pickable());
    item_entity.set_pickable(false);
    item_entity.set_pickup_delay(Duration::ZERO);
    assert!(!item_entity.is_pickable());
}

#[test]
fn item_entity_landing_synchronizes_position_and_velocity_immediately() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    world
        .set_block(BlockPosition::new(0, 63, 0), Block::STONE)
        .unwrap();
    let snapshot = world.update_snapshot();
    let mut item_entity = ItemEntity::new(spinel_registry::ItemStack::of(Material::DIAMOND));
    item_entity.set_position(crate::entity::EntityPosition::new(0.5, 64.1, 0.5, 0.0, 0.0));
    item_entity.set_velocity(Velocity(Vector3d {
        x: 0.0,
        y: -2.0,
        z: 0.0,
    }));

    item_entity.movement_tick(&snapshot);
    item_entity.tick();

    assert!(item_entity.is_on_ground());
    assert!(item_entity.scheduled_position_sync_packet().is_some());
}
