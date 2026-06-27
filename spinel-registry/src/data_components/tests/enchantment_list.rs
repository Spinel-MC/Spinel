use super::super::enchantment_list::EnchantmentList;
use crate::data_components::vanilla_components::{ENCHANTMENTS, STORED_ENCHANTMENTS};
use crate::enchantment::Enchantment;
use crate::{DataComponentMap, DataComponentValue, Identifier, ItemStack, Material, RegistryKey};
use spinel_nbt::{Nbt, NbtCompound};

fn enchantment_key(path: &str) -> RegistryKey<Enchantment> {
    RegistryKey::new(Identifier::minecraft(path))
}

#[test]
fn enchantment_list_immutable_operations_preserve_source_list() {
    let sharpness = enchantment_key("sharpness");
    let unbreaking = enchantment_key("unbreaking");
    let enchantments = EnchantmentList::from_enchantment(sharpness.clone(), 3);
    let with_unbreaking = enchantments.with(unbreaking.clone(), 2);
    let without_sharpness = with_unbreaking.remove(&sharpness);

    assert!(enchantments.has(&sharpness));
    assert!(!enchantments.has(&unbreaking));
    assert_eq!(enchantments.level(&sharpness), 3);
    assert_eq!(enchantments.level(&unbreaking), 0);
    assert_eq!(with_unbreaking.get_enchantments().len(), 2);
    assert!(!without_sharpness.has(&sharpness));
    assert!(without_sharpness.has(&unbreaking));
}

#[test]
fn enchantment_list_component_nbt_round_trips_registry_keys_and_levels() {
    let enchantments = EnchantmentList::from_enchantment(enchantment_key("sharpness"), 5)
        .with(enchantment_key("mending"), 1);

    let encoded = enchantments.to_component_nbt();
    assert_eq!(
        EnchantmentList::from_component_nbt(&encoded),
        Some(enchantments)
    );
}

#[test]
fn enchantment_list_component_nbt_rejects_non_integer_levels_and_bad_keys() {
    let mut non_integer_level = NbtCompound::new();
    non_integer_level.insert(
        "minecraft:sharpness".to_string(),
        Nbt::String("5".to_string()),
    );
    assert_eq!(
        EnchantmentList::from_component_nbt(&Nbt::Compound(non_integer_level)),
        None
    );

    let mut invalid_key = NbtCompound::new();
    invalid_key.insert("bad key".to_string(), Nbt::Int(1));
    assert_eq!(
        EnchantmentList::from_component_nbt(&Nbt::Compound(invalid_key)),
        None
    );
}

#[test]
fn enchantment_list_copy_preserves_registry_key_type_without_aliasing_source_map() {
    let sharpness = enchantment_key("sharpness");
    let mut source = std::collections::HashMap::new();
    source.insert(sharpness.clone(), 4);
    let enchantments = EnchantmentList::new(source.clone());
    source.insert(enchantment_key("mending"), 1);

    assert_eq!(enchantments.get_enchantments().len(), 1);
    assert_eq!(enchantments.level(&sharpness), 4);
}

#[test]
fn enchantment_component_constants_round_trip_through_item_stack_patch_nbt() {
    let enchantments = EnchantmentList::from_enchantment(enchantment_key("sharpness"), 5);
    let stored_enchantments = EnchantmentList::from_enchantment(enchantment_key("mending"), 1);
    let item_stack = ItemStack::of(Material::DIAMOND_SWORD)
        .with(ENCHANTMENTS, enchantments.clone())
        .with(STORED_ENCHANTMENTS, stored_enchantments.clone());

    assert_eq!(item_stack.get(ENCHANTMENTS), Some(enchantments.clone()));
    assert_eq!(
        item_stack.get(STORED_ENCHANTMENTS),
        Some(stored_enchantments.clone())
    );

    let encoded_patch = item_stack.component_patch().to_nbt_patch();
    assert!(encoded_patch.contains_key("minecraft:enchantments"));
    assert!(encoded_patch.contains_key("minecraft:stored_enchantments"));

    let decoded_patch = DataComponentMap::from_nbt_patch(encoded_patch).unwrap();
    assert_eq!(
        decoded_patch.get_component(ENCHANTMENTS),
        Some(enchantments)
    );
    assert_eq!(
        decoded_patch.get_component(STORED_ENCHANTMENTS),
        Some(stored_enchantments)
    );
}
