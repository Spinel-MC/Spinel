use crate::data_components::vanilla_components::{DAMAGE, MAX_DAMAGE, MAX_STACK_SIZE};
use crate::{DataComponentMap, ItemStack, Material};
use spinel_nbt::{Tag, TagReadable};

#[test]
fn generated_materials_have_vanilla_ids_and_block_links() {
    assert_eq!(Material::BEDROCK.id(), 58);
    assert_eq!(Material::from_key("bedrock"), Some(Material::BEDROCK));
    assert_eq!(
        Material::from_id(Material::DIAMOND.id()),
        Some(Material::DIAMOND)
    );
    assert!(Material::BEDROCK.is_block());
    assert!(!Material::DIAMOND.is_block());
}

#[test]
fn generated_materials_use_extracted_vanilla_max_stack_sizes() {
    assert_eq!(Material::DIAMOND.max_stack_size(), 64);
    assert_eq!(Material::DIAMOND_SWORD.max_stack_size(), 1);
    assert_eq!(Material::WATER_BUCKET.max_stack_size(), 1);
    assert_eq!(Material::SNOWBALL.max_stack_size(), 16);
    assert_eq!(Material::ENDER_PEARL.max_stack_size(), 16);
}

#[test]
fn item_stack_amounts_air_and_custom_data_match_minestom_shape() {
    let tag = Tag::<String>::string("owner");
    let stack = ItemStack::of(Material::DIAMOND)
        .with_amount(3)
        .with_tag(&tag, Some("Wayne".to_string()));
    let nbt = stack.to_item_nbt();
    let decoded = ItemStack::from_item_nbt(nbt).unwrap();

    assert_eq!(ItemStack::of(Material::AIR).amount(), 0);
    assert_eq!(stack.amount(), 3);
    assert!(stack.is_similar(&decoded));
    assert_eq!(decoded.get_tag(&tag), Some("Wayne".to_string()));
}

#[test]
fn item_stack_amount_api_matches_minestom_shape() {
    let stack = ItemStack::of(Material::DIAMOND).with_amount(3);
    let mut builder = ItemStack::builder(Material::DIAMOND);

    builder.amount(4);

    assert_eq!(ItemStack::of(Material::DIAMOND).amount(), 1);
    assert_eq!(stack.amount(), 3);
    assert_eq!(ItemStack::of(Material::AIR).with_amount(3).amount(), 0);
    assert_eq!(builder.build().amount(), 4);
}

#[test]
fn item_stack_transform_amount_matches_minestom_operator_shape() {
    let stack = ItemStack::of(Material::DIAMOND)
        .with_amount(3)
        .transform_amount(|amount| amount + 10);

    assert_eq!(stack.amount(), 13);
}

#[test]
fn item_stack_components_resolve_patch_over_material_prototype() {
    let default_stack = ItemStack::of(Material::DIAMOND);
    let patched_stack = default_stack.with(MAX_STACK_SIZE, 12);
    let restored_stack = patched_stack.with(MAX_STACK_SIZE, 64);

    assert!(default_stack.has(MAX_STACK_SIZE));
    assert_eq!(default_stack.max_stack_size(), 64);
    assert_eq!(patched_stack.max_stack_size(), 12);
    assert!(restored_stack.component_patch().is_empty());
}

#[test]
fn item_stack_component_removal_overrides_material_prototype() {
    let stack = ItemStack::of(Material::DIAMOND).without(MAX_STACK_SIZE);
    let component_patch = stack.component_patch().to_nbt_patch();

    assert!(!stack.has(MAX_STACK_SIZE));
    assert_eq!(stack.max_stack_size(), 64);
    assert!(component_patch.contains_key("!minecraft:max_stack_size"));
}

#[test]
fn item_stack_component_nbt_round_trip_keeps_added_and_removed_patch_entries() {
    let stack = ItemStack::of(Material::DIAMOND)
        .with(MAX_DAMAGE, 10)
        .with(DAMAGE, 4)
        .without(MAX_STACK_SIZE);
    let decoded = ItemStack::from_item_nbt(stack.to_item_nbt()).unwrap();

    assert_eq!(decoded.get(MAX_DAMAGE), Some(10));
    assert_eq!(decoded.get(DAMAGE), Some(4));
    assert!(!decoded.has(MAX_STACK_SIZE));
    assert!(stack.is_similar(&decoded));
}

#[test]
fn component_map_diff_removes_patch_entries_matching_the_material_prototype() {
    let mut patch = DataComponentMap::new();
    patch.set(MAX_STACK_SIZE, 64);

    assert!(DataComponentMap::diff(&Material::DIAMOND.prototype(), &patch).is_empty());
}

#[test]
fn item_stack_damage_requires_existing_damage_component() {
    let undamageable_stack = ItemStack::of(Material::DIAMOND);
    let damageable_stack = ItemStack::of(Material::DIAMOND)
        .with(MAX_DAMAGE, 10)
        .with(DAMAGE, 4);

    assert!(undamageable_stack.damage(3).is_similar(&undamageable_stack));
    assert_eq!(damageable_stack.damage(3).get(DAMAGE), Some(7));
    assert!(damageable_stack.damage(6).is_air());
}

#[test]
fn item_stack_with_material_restores_positive_amount_for_non_air() {
    let stack = ItemStack::air().with_material(Material::DIAMOND);

    assert_eq!(stack.material(), &Material::DIAMOND);
    assert_eq!(stack.amount(), 1);
}
