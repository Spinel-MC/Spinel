use crate::entity::ItemEntity;
use spinel_network::types::entity_metadata::MetadataValue;
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
    item_entity.set_pickup_delay(Duration::from_secs(1));
    assert!(!item_entity.is_pickable());
    item_entity.set_pickable(false);
    item_entity.set_pickup_delay(Duration::ZERO);
    assert!(!item_entity.is_pickable());
}
