use crate::data_components::vanilla_components::{
    ATTRIBUTE_MODIFIERS, DAMAGE, GLIDER, ITEM_NAME, MAX_DAMAGE, MAX_STACK_SIZE, USE_COOLDOWN,
    WEAPON,
};
use crate::{
    DataComponentDescriptor, DataComponentMap, DecodeDataComponentMapError, EntityPacketType,
    EntityType, ItemStack, Material, RegistryKey, UnitComponent,
};
use spinel_nbt::{Nbt, NbtCompound, Tag, TagReadable};
use spinel_utils::component::variant::ComponentType;

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
fn entity_type_registry_matches_minestom_generated_id_order() {
    assert_eq!(EntityType::ACACIA_BOAT.id(), 0);
    assert_eq!(EntityType::PLAYER.id(), 155);
    assert_eq!(EntityType::FISHING_BOBBER.id(), 156);
    assert_eq!(
        EntityType::from_key("minecraft:wolf"),
        Some(EntityType::WOLF)
    );
    assert_eq!(EntityType::from_id(139), Some(EntityType::VILLAGER));
    assert_eq!(EntityType::ALL.len(), 157);
    assert_eq!(EntityType::PLAYER.packet_type(), EntityPacketType::Player);
    assert_eq!(EntityType::ZOMBIE.packet_type(), EntityPacketType::Living);
    assert_eq!(EntityType::ARROW.packet_type(), EntityPacketType::Entity);
    assert_eq!(EntityType::PLAYER.width(), 0.6);
    assert_eq!(EntityType::PLAYER.height(), 1.8);
    assert_eq!(EntityType::PLAYER.eye_height(), 1.62);
    assert_eq!(EntityType::PLAYER.client_tracking_range(), 32);
    assert!(!EntityType::PLAYER.fire_immune());
    assert_eq!(
        EntityType::PLAYER.entity_attachment("name_tag"),
        Some([0.0, 1.7999999523162842, 0.0])
    );
    assert_eq!(EntityType::PLAYER.bounding_box().width(), 0.6);
    assert_eq!(EntityType::PLAYER.bounding_box().height(), 1.8);
    assert_eq!(EntityType::PLAYER.bounding_box().depth(), 0.6);
    assert_eq!(
        EntityType::static_registry().get_id(&RegistryKey::new(EntityType::PLAYER.key())),
        Some(155)
    );
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
fn material_prototype_uses_extracted_use_cooldown_component() {
    let item_stack = ItemStack::of(Material::ENDER_PEARL);
    let use_cooldown = item_stack.get(USE_COOLDOWN).unwrap();

    assert_eq!(use_cooldown.seconds(), 1.0);
    assert_eq!(use_cooldown.cooldown_group(), None);
    assert_eq!(use_cooldown.ticks(), 20);
}

#[test]
fn data_component_descriptors_match_minestom_registry_metadata() {
    assert_eq!(DataComponentDescriptor::values().len(), 104);
    assert_eq!(
        DataComponentDescriptor::from_key("minecraft:max_stack_size")
            .unwrap()
            .id(),
        1
    );
    assert!(
        !DataComponentDescriptor::from_key("minecraft:creative_slot_lock")
            .unwrap()
            .is_serialized()
    );
    assert!(
        !DataComponentDescriptor::from_key("minecraft:lock")
            .unwrap()
            .is_synced()
    );
    assert!(
        !DataComponentDescriptor::from_key("minecraft:map_post_processing")
            .unwrap()
            .is_serialized()
    );
}

#[test]
fn component_nbt_map_decoding_keeps_absolute_values_and_ignores_patch_removals() {
    let mut compound = NbtCompound::new();
    compound.insert("minecraft:max_stack_size".to_string(), Nbt::Int(16));
    compound.insert("!minecraft:damage".to_string(), NbtCompound::new());

    let components = DataComponentMap::from_nbt_map(compound).unwrap();

    assert_eq!(
        components.get(&DataComponentMap::new(), MAX_STACK_SIZE),
        Some(16)
    );
    assert_eq!(components.removed_component_ids(), Vec::<i32>::new());
}

#[test]
fn component_nbt_patch_decoding_keeps_removals_and_rejects_unknown_keys() {
    let mut compound = NbtCompound::new();
    compound.insert("!minecraft:max_stack_size".to_string(), NbtCompound::new());
    compound.insert("minecraft:unknown_component".to_string(), Nbt::Int(1));

    let error = DataComponentMap::from_nbt_patch(compound).unwrap_err();

    assert_eq!(
        error,
        DecodeDataComponentMapError::UnknownComponentKey {
            component_key: "minecraft:unknown_component".to_string()
        }
    );
}

#[test]
fn material_prototype_is_generated_from_component_entries() {
    let prototype = Material::DIAMOND_SWORD.prototype();

    assert!(prototype.has(&DataComponentMap::new(), MAX_STACK_SIZE));
    assert_eq!(
        prototype.get(&DataComponentMap::new(), MAX_STACK_SIZE),
        Some(1)
    );
}

#[test]
fn material_prototype_uses_minestom_default_data_components() {
    let pickaxe = ItemStack::of(Material::DIAMOND_PICKAXE);

    assert_eq!(pickaxe.max_stack_size(), 1);
    assert_eq!(pickaxe.get(MAX_DAMAGE), Some(1561));
    assert!(pickaxe.has(WEAPON));
    assert!(pickaxe.has(ATTRIBUTE_MODIFIERS));
    assert!(item_stack_has_attack_speed_modifier(
        &pickaxe,
        -2.799999952316284
    ));
}

#[test]
fn material_text_components_decode_from_minestom_component_json() {
    let item_name = ItemStack::of(Material::DIAMOND_PICKAXE)
        .get(ITEM_NAME)
        .unwrap();

    assert_eq!(
        item_name.content,
        ComponentType::Translatable {
            key: "item.minecraft.diamond_pickaxe".to_string(),
            fallback: None,
            args: Vec::new()
        }
    );
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
fn data_component_map_builder_matches_minestom_staged_build_shape() {
    let mut builder = DataComponentMap::builder();
    builder.set(MAX_STACK_SIZE, 12).set_unit(GLIDER);
    let component_map = builder.build();

    assert_eq!(builder.get(MAX_STACK_SIZE), Some(12));
    assert!(builder.has(GLIDER));
    assert_eq!(component_map.get_component(MAX_STACK_SIZE), Some(12));
    assert!(component_map.has_component(GLIDER));
}

#[test]
fn data_component_patch_builder_preserves_explicit_removals() {
    let mut patch_builder = DataComponentMap::patch_builder();
    patch_builder
        .set(MAX_DAMAGE, 10)
        .set_unit(GLIDER)
        .remove(MAX_STACK_SIZE);
    let component_patch = patch_builder.build();

    assert_eq!(patch_builder.get(MAX_DAMAGE), Some(10));
    assert!(patch_builder.has(GLIDER));
    assert_eq!(
        component_patch.removed_component_ids(),
        vec![MAX_STACK_SIZE.id()]
    );
}

#[test]
fn component_map_to_builders_copy_current_patch_entries() {
    let component_patch = DataComponentMap::new()
        .with(MAX_DAMAGE, 10)
        .with(GLIDER, UnitComponent)
        .remove(MAX_STACK_SIZE);

    assert_eq!(component_patch.to_builder().build(), component_patch);
    assert_eq!(component_patch.to_patch_builder().build(), component_patch);
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

fn item_stack_has_attack_speed_modifier(item_stack: &ItemStack, amount: f64) -> bool {
    item_stack
        .get(ATTRIBUTE_MODIFIERS)
        .map(|attribute_list| {
            attribute_list.modifiers().iter().any(|modifier| {
                modifier.attribute_type().to_string() == "minecraft:attack_speed"
                    && (modifier.amount() - amount).abs() < f64::EPSILON
            })
        })
        .unwrap_or(false)
}
