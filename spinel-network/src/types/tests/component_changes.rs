use super::super::component_changes::ComponentChanges;
use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use spinel_nbt::NbtCompound;
use spinel_registry::data_components::UnitComponent;
use spinel_registry::data_components::vanilla_components::DAMAGE_TYPE;
use spinel_registry::data_components::vanilla_components::{
    ATTRIBUTE_MODIFIERS, BREAK_SOUND, BUNDLE_CONTENTS, CUSTOM_NAME, ENTITY_DATA, FOOD, GLIDER,
    INSTRUMENT, INTANGIBLE_PROJECTILE, LORE, MAX_STACK_SIZE, POTION_CONTENTS, PROFILE,
    TOOLTIP_STYLE, USE_COOLDOWN, USE_EFFECTS, USE_REMAINDER, VILLAGER_VARIANT,
};
use spinel_registry::{
    AttributeList, AttributeModifierDisplay, AttributeModifierEntry, AttributeOperation,
    BuiltinSoundEvent, DataComponentMap, EquipmentSlotGroup, Food, GameProfileProperty, Identifier,
    InstrumentComponent, ItemStack, Material, PotionContents, ResolvableProfile, TypedCustomData,
    UseCooldown, UseEffects,
};
use spinel_utils::component::Component;

fn decode_single_component_change(component_changes: &ComponentChanges) -> ComponentChanges {
    let mut payload = Vec::new();

    component_changes.encode(&mut payload).unwrap();

    ComponentChanges::decode(&mut payload.as_slice()).unwrap()
}

#[test]
fn synced_unit_component_writes_empty_payload_entry() {
    let component_patch = DataComponentMap::new().with(GLIDER, UnitComponent);
    let component_changes = ComponentChanges::from(&component_patch);

    assert_eq!(component_changes.added.len(), 1);
    assert_eq!(component_changes.added[0].type_id, GLIDER.id());
    assert_eq!(component_changes.added[0].data, Vec::<u8>::new());
}

#[test]
fn serialized_only_component_is_not_sent_in_component_changes() {
    let component_patch = DataComponentMap::new().with(INTANGIBLE_PROJECTILE, UnitComponent);
    let component_changes = ComponentChanges::from(&component_patch);

    assert!(component_changes.added.is_empty());
    assert!(component_changes.removed.is_empty());
}

#[test]
fn string_component_encoder_writes_minestom_string_payload_shape() {
    let component_patch = DataComponentMap::new().with(TOOLTIP_STYLE, "abc".to_string());
    let mut payload = Vec::new();

    ComponentChanges::from(&component_patch)
        .encode(&mut payload)
        .unwrap();

    assert_eq!(
        payload,
        vec![1, 0, TOOLTIP_STYLE.id() as u8, 3, b'a', b'b', b'c']
    );
}

#[test]
fn item_text_components_use_vanilla_registry_nbt_packet_codec() {
    let custom_name = Component::text("Named item").build();
    let lore_line = Component::text("Lore line").build();
    let component_patch = DataComponentMap::new()
        .with(CUSTOM_NAME, custom_name)
        .with(LORE, vec![lore_line]);
    let component_changes = ComponentChanges::from(&component_patch);
    let custom_name_payload = component_changes
        .added
        .iter()
        .find(|component| component.type_id == CUSTOM_NAME.id())
        .unwrap();
    let lore_payload = component_changes
        .added
        .iter()
        .find(|component| component.type_id == LORE.id())
        .unwrap();

    assert_eq!(custom_name_payload.data.first(), Some(&10));
    assert_eq!(lore_payload.data.get(1), Some(&10));
}

#[test]
fn dynamic_registry_key_component_writes_registry_id_payload() {
    let component_patch =
        DataComponentMap::new().with(DAMAGE_TYPE, Identifier::minecraft("in_fire"));
    let component_changes = ComponentChanges::from(&component_patch);

    assert_eq!(component_changes.added.len(), 1);
    assert_eq!(component_changes.added[0].type_id, DAMAGE_TYPE.id());
    assert!(!component_changes.added[0].data.is_empty());
}

#[test]
fn villager_variant_component_writes_minestom_enum_ordinal() {
    let component_patch =
        DataComponentMap::new().with(VILLAGER_VARIANT, spinel_registry::VillagerType::PLAINS);
    let component_changes = ComponentChanges::from(&component_patch);

    assert_eq!(component_changes.added.len(), 1);
    assert_eq!(component_changes.added[0].data, vec![2]);
}

#[test]
fn instrument_component_writes_registry_key_either_branch() {
    let component_patch = DataComponentMap::new().with(
        INSTRUMENT,
        InstrumentComponent::new(Identifier::minecraft("ponder_goat_horn")),
    );
    let component_changes = ComponentChanges::from(&component_patch);

    assert_eq!(component_changes.added.len(), 1);
    assert_eq!(component_changes.added[0].data[0], 0);
}

#[test]
fn potion_contents_component_writes_optional_fields_and_static_potion_id() {
    let component_patch = DataComponentMap::new().with(
        POTION_CONTENTS,
        PotionContents::new(Some(Identifier::minecraft("water")), None, vec![], None),
    );
    let component_changes = ComponentChanges::from(&component_patch);

    assert_eq!(component_changes.added.len(), 1);
    assert_eq!(component_changes.added[0].data, vec![1, 0, 0, 0, 0]);
}

#[test]
fn entity_data_component_writes_entity_type_id_then_custom_nbt() {
    let component_patch = DataComponentMap::new().with(
        ENTITY_DATA,
        TypedCustomData::new(Identifier::minecraft("wolf"), NbtCompound::new()),
    );
    let component_changes = ComponentChanges::from(&component_patch);

    assert_eq!(component_changes.added.len(), 1);
    assert_eq!(component_changes.added[0].type_id, ENTITY_DATA.id());
    assert_eq!(component_changes.added[0].data[0], 148);
}

#[test]
fn builtin_sound_event_component_writes_minestom_builtin_id_branch() {
    let sound_identifier = Identifier::minecraft("entity.generic.eat");
    let builtin_sound_event = BuiltinSoundEvent::from_key(&sound_identifier).unwrap();
    let component_patch = DataComponentMap::new().with(BREAK_SOUND, sound_identifier);
    let component_changes = ComponentChanges::from(&component_patch);
    let mut expected_payload = Vec::new();

    VarIntWrapper(builtin_sound_event.id() + 1)
        .encode(&mut expected_payload)
        .unwrap();

    assert_eq!(component_changes.added.len(), 1);
    assert_eq!(component_changes.added[0].data, expected_payload);
}

#[test]
fn custom_sound_event_component_writes_minestom_named_branch() {
    let component_patch =
        DataComponentMap::new().with(BREAK_SOUND, Identifier::new("custom", "snap"));
    let component_changes = ComponentChanges::from(&component_patch);
    let mut expected_payload = Vec::new();

    VarIntWrapper(0).encode(&mut expected_payload).unwrap();
    "custom:snap"
        .to_string()
        .encode(&mut expected_payload)
        .unwrap();
    Option::<f32>::None.encode(&mut expected_payload).unwrap();

    assert_eq!(component_changes.added.len(), 1);
    assert_eq!(component_changes.added[0].data, expected_payload);
}

#[test]
fn var_int_component_encoder_writes_minestom_var_int_payload_shape() {
    let component_patch = DataComponentMap::new().with(MAX_STACK_SIZE, 16);
    let component_changes = ComponentChanges::from(&component_patch);
    let mut expected_payload = Vec::new();

    VarIntWrapper(16).encode(&mut expected_payload).unwrap();

    assert_eq!(component_changes.added.len(), 1);
    assert_eq!(component_changes.added[0].data, expected_payload);
}

#[test]
fn non_empty_component_changes_decode_preserves_synced_payload_bytes() {
    let mut payload = Vec::new();

    VarIntWrapper(1).encode(&mut payload).unwrap();
    VarIntWrapper(0).encode(&mut payload).unwrap();
    VarIntWrapper(MAX_STACK_SIZE.id())
        .encode(&mut payload)
        .unwrap();
    VarIntWrapper(16).encode(&mut payload).unwrap();

    let component_changes = ComponentChanges::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(component_changes.added.len(), 1);
    assert_eq!(component_changes.added[0].type_id, MAX_STACK_SIZE.id());
    assert_eq!(component_changes.added[0].data, vec![16]);
}

#[test]
fn use_cooldown_component_decode_preserves_minestom_network_payload() {
    let component_patch = DataComponentMap::new().with(
        USE_COOLDOWN,
        UseCooldown::new(1.5, Some("minecraft:ender_pearl".to_string())),
    );
    let component_changes = ComponentChanges::from(&component_patch);

    let decoded_component_changes = decode_single_component_change(&component_changes);

    assert_eq!(decoded_component_changes, component_changes);
}

#[test]
fn food_component_decode_preserves_minestom_network_payload() {
    let component_patch = DataComponentMap::new().with(FOOD, Food::new(4, 0.3, true));
    let component_changes = ComponentChanges::from(&component_patch);

    let decoded_component_changes = decode_single_component_change(&component_changes);

    assert_eq!(decoded_component_changes, component_changes);
}

#[test]
fn use_effects_component_decode_preserves_minestom_network_payload() {
    let component_patch =
        DataComponentMap::new().with(USE_EFFECTS, UseEffects::new(true, false, 0.5));
    let component_changes = ComponentChanges::from(&component_patch);

    let decoded_component_changes = decode_single_component_change(&component_changes);

    assert_eq!(decoded_component_changes, component_changes);
}

#[test]
fn attribute_modifiers_component_decode_preserves_minestom_network_payload() {
    let attribute_modifiers = AttributeList::new(vec![AttributeModifierEntry::new(
        Identifier::minecraft("attack_speed"),
        Identifier::minecraft("base_attack_speed"),
        -2.8,
        AttributeOperation::AddValue,
        EquipmentSlotGroup::MainHand,
        AttributeModifierDisplay::Default,
    )]);
    let component_patch = DataComponentMap::new().with(ATTRIBUTE_MODIFIERS, attribute_modifiers);
    let component_changes = ComponentChanges::from(&component_patch);

    let decoded_component_changes = decode_single_component_change(&component_changes);

    assert_eq!(decoded_component_changes, component_changes);
}

#[test]
fn profile_component_decode_preserves_minestom_network_payload() {
    let profile = ResolvableProfile::new(
        Some("Wayne".to_string()),
        Some("00000000-0000-0000-0000-000000000001".to_string()),
        vec![GameProfileProperty::new(
            "textures".to_string(),
            "abc".to_string(),
            Some("sig".to_string()),
        )],
        Some("minecraft:textures/entity/player/wide/steve".to_string()),
    );
    let component_patch = DataComponentMap::new().with(PROFILE, profile);
    let component_changes = ComponentChanges::from(&component_patch);

    let decoded_component_changes = decode_single_component_change(&component_changes);

    assert_eq!(decoded_component_changes, component_changes);
}

#[test]
fn use_remainder_component_decode_preserves_nested_item_stack_payload() {
    let nested_item_stack = ItemStack::of(Material::STONE).with_amount(2);
    let component_patch = DataComponentMap::new().with(USE_REMAINDER, nested_item_stack);
    let component_changes = ComponentChanges::from(&component_patch);

    let decoded_component_changes = decode_single_component_change(&component_changes);

    assert_eq!(decoded_component_changes, component_changes);
}

#[test]
fn bundle_contents_component_decode_preserves_nested_item_stack_list_payload() {
    let nested_item_stack = ItemStack::of(Material::STONE).with_amount(2);
    let component_patch = DataComponentMap::new().with(BUNDLE_CONTENTS, vec![nested_item_stack]);
    let component_changes = ComponentChanges::from(&component_patch);

    let decoded_component_changes = decode_single_component_change(&component_changes);

    assert_eq!(decoded_component_changes, component_changes);
}

#[test]
fn component_changes_decode_rejects_unknown_component_id() {
    let mut payload = Vec::new();

    VarIntWrapper(1).encode(&mut payload).unwrap();
    VarIntWrapper(0).encode(&mut payload).unwrap();
    VarIntWrapper(999_999).encode(&mut payload).unwrap();

    let error = ComponentChanges::decode(&mut payload.as_slice()).unwrap_err();

    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
}

#[test]
fn component_changes_decode_rejects_maps_larger_than_minestom_limit() {
    let mut payload = Vec::new();

    VarIntWrapper(257).encode(&mut payload).unwrap();
    VarIntWrapper(0).encode(&mut payload).unwrap();

    let error = ComponentChanges::decode(&mut payload.as_slice()).unwrap_err();

    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
}
