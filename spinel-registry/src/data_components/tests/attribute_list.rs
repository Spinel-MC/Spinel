use super::super::attribute_list::{
    AttributeList, AttributeModifier, AttributeModifierDisplay, AttributeModifierEntry,
    AttributeOperation, EquipmentSlotGroup,
};
use crate::DataComponentValue;
use spinel_nbt::{Nbt, NbtCompound};

fn modifier_entry(id: &str) -> AttributeModifierEntry {
    AttributeModifierEntry::new(
        "minecraft:attack_speed".parse().unwrap(),
        AttributeModifier::from(id, -2.0, AttributeOperation::AddValue).unwrap(),
        EquipmentSlotGroup::MainHand,
        AttributeModifierDisplay::Default,
    )
}

#[test]
fn attribute_modifier_maps_identifier_amount_and_operation() {
    let modifier = AttributeModifier::from(
        "minecraft:base_attack_speed",
        -2.0,
        AttributeOperation::AddValue,
    )
    .unwrap();

    assert_eq!(modifier.get_id().to_string(), "minecraft:base_attack_speed");
    assert_eq!(modifier.get_amount(), -2.0);
    assert_eq!(modifier.get_operation(), AttributeOperation::AddValue);
    assert_eq!(
        AttributeOperation::from_protocol_id(0),
        Some(AttributeOperation::AddValue)
    );
    assert_eq!(
        AttributeOperation::from_protocol_id(1),
        Some(AttributeOperation::AddMultipliedBase)
    );
    assert_eq!(
        AttributeOperation::from_protocol_id(2),
        Some(AttributeOperation::AddMultipliedTotal)
    );
    assert_eq!(AttributeOperation::from_protocol_id(3), None);
}

#[test]
fn attribute_list_immutable_operations_remove_only_one_equal_entry() {
    let modifier = modifier_entry("minecraft:test_speed");
    let list = AttributeList::from_modifier(modifier.clone()).with(modifier.clone());
    let removed = list.remove(&modifier);

    assert_eq!(list.get_modifiers().len(), 2);
    assert_eq!(removed.get_modifiers().len(), 1);
    assert_eq!(removed.get_modifiers()[0], modifier);
}

#[test]
fn attribute_list_round_trips_default_hidden_and_override_display() {
    let hidden = AttributeModifierEntry::new(
        "minecraft:attack_damage".parse().unwrap(),
        AttributeModifier::from(
            "minecraft:hidden_attack_damage",
            3.0,
            AttributeOperation::AddMultipliedBase,
        )
        .unwrap(),
        EquipmentSlotGroup::Any,
        AttributeModifierDisplay::Hidden,
    );
    let overridden = AttributeModifierEntry::new(
        "minecraft:attack_speed".parse().unwrap(),
        AttributeModifier::from(
            "minecraft:override_attack_speed",
            0.5,
            AttributeOperation::AddMultipliedTotal,
        )
        .unwrap(),
        EquipmentSlotGroup::OffHand,
        AttributeModifierDisplay::Override(Nbt::String("override".to_string())),
    );
    let attributes = AttributeList::from_modifier(hidden).with(overridden);

    let encoded = attributes.to_component_nbt();
    assert_eq!(
        AttributeList::from_component_nbt(&encoded),
        Some(attributes)
    );
}

#[test]
fn attribute_list_defaults_any_slot_and_default_display_when_nbt_fields_are_absent() {
    let modifier = AttributeModifierEntry::new(
        "minecraft:attack_speed".parse().unwrap(),
        AttributeModifier::from(
            "minecraft:any_attack_speed",
            1.0,
            AttributeOperation::AddValue,
        )
        .unwrap(),
        EquipmentSlotGroup::Any,
        AttributeModifierDisplay::Default,
    );
    let attributes = AttributeList::from_modifier(modifier);
    let encoded = attributes.to_component_nbt();
    let Nbt::List(entries) = &encoded else {
        panic!("expected attribute modifier list");
    };
    let Nbt::Compound(entry) = &entries[0] else {
        panic!("expected attribute modifier compound");
    };

    assert!(!entry.contains_key("slot"));
    assert!(!entry.contains_key("display"));
    let decoded = AttributeList::from_component_nbt(&encoded).unwrap();
    assert_eq!(
        decoded.get_modifiers()[0].get_slot(),
        EquipmentSlotGroup::Any
    );
    assert_eq!(
        decoded.get_modifiers()[0].get_display(),
        &AttributeModifierDisplay::Default
    );
}

#[test]
fn attribute_list_rejects_unknown_operation_slot_and_display_kind() {
    let mut invalid_operation = valid_attribute_modifier_nbt();
    invalid_operation.insert(
        "operation".to_string(),
        Nbt::String("multiply_everything".to_string()),
    );
    assert_eq!(
        AttributeList::from_component_nbt(&Nbt::List(Box::new([Nbt::Compound(invalid_operation)]))),
        None
    );

    let mut invalid_slot = valid_attribute_modifier_nbt();
    invalid_slot.insert("slot".to_string(), Nbt::String("helmet".to_string()));
    assert_eq!(
        AttributeList::from_component_nbt(&Nbt::List(Box::new([Nbt::Compound(invalid_slot)]))),
        None
    );

    let mut display = NbtCompound::new();
    display.insert("type".to_string(), Nbt::String("glow".to_string()));
    let mut invalid_display = valid_attribute_modifier_nbt();
    invalid_display.insert("display".to_string(), Nbt::Compound(display));
    assert_eq!(
        AttributeList::from_component_nbt(&Nbt::List(Box::new([Nbt::Compound(invalid_display)]))),
        None
    );
}

#[test]
fn equipment_slot_group_names_ids_and_filters_match_minestom_attribute_list_slot_groups() {
    assert_eq!(EquipmentSlotGroup::Any.protocol_id(), 0);
    assert_eq!(EquipmentSlotGroup::MainHand.protocol_id(), 1);
    assert_eq!(EquipmentSlotGroup::OffHand.protocol_id(), 2);
    assert_eq!(EquipmentSlotGroup::Hand.protocol_id(), 3);
    assert_eq!(EquipmentSlotGroup::Feet.protocol_id(), 4);
    assert_eq!(EquipmentSlotGroup::Legs.protocol_id(), 5);
    assert_eq!(EquipmentSlotGroup::Chest.protocol_id(), 6);
    assert_eq!(EquipmentSlotGroup::Head.protocol_id(), 7);
    assert_eq!(EquipmentSlotGroup::Armor.protocol_id(), 8);
    assert_eq!(EquipmentSlotGroup::Body.protocol_id(), 9);

    assert_eq!(
        EquipmentSlotGroup::from_nbt_name("any"),
        Some(EquipmentSlotGroup::Any)
    );
    assert_eq!(
        EquipmentSlotGroup::from_nbt_name("mainhand"),
        Some(EquipmentSlotGroup::MainHand)
    );
    assert_eq!(
        EquipmentSlotGroup::from_nbt_name("offhand"),
        Some(EquipmentSlotGroup::OffHand)
    );
    assert_eq!(
        EquipmentSlotGroup::from_nbt_name("hand"),
        Some(EquipmentSlotGroup::Hand)
    );
    assert_eq!(
        EquipmentSlotGroup::from_nbt_name("feet"),
        Some(EquipmentSlotGroup::Feet)
    );
    assert_eq!(
        EquipmentSlotGroup::from_nbt_name("legs"),
        Some(EquipmentSlotGroup::Legs)
    );
    assert_eq!(
        EquipmentSlotGroup::from_nbt_name("chest"),
        Some(EquipmentSlotGroup::Chest)
    );
    assert_eq!(
        EquipmentSlotGroup::from_nbt_name("head"),
        Some(EquipmentSlotGroup::Head)
    );
    assert_eq!(
        EquipmentSlotGroup::from_nbt_name("armor"),
        Some(EquipmentSlotGroup::Armor)
    );
    assert_eq!(
        EquipmentSlotGroup::from_nbt_name("body"),
        Some(EquipmentSlotGroup::Body)
    );
    assert_eq!(EquipmentSlotGroup::from_nbt_name("helmet"), None);

    assert!(EquipmentSlotGroup::Any.contains_slot_name("mainhand"));
    assert!(EquipmentSlotGroup::Hand.contains_slot_name("mainhand"));
    assert!(EquipmentSlotGroup::Hand.contains_slot_name("offhand"));
    assert!(!EquipmentSlotGroup::Hand.contains_slot_name("head"));
    assert!(EquipmentSlotGroup::Armor.contains_slot_name("feet"));
    assert!(EquipmentSlotGroup::Armor.contains_slot_name("legs"));
    assert!(EquipmentSlotGroup::Armor.contains_slot_name("chest"));
    assert!(EquipmentSlotGroup::Armor.contains_slot_name("head"));
    assert!(!EquipmentSlotGroup::Armor.contains_slot_name("body"));
    assert!(EquipmentSlotGroup::Body.contains_slot_name("body"));
    assert!(!EquipmentSlotGroup::Body.contains_slot_name("chest"));
}

fn valid_attribute_modifier_nbt() -> NbtCompound {
    let mut compound = NbtCompound::new();
    compound.insert(
        "type".to_string(),
        Nbt::String("minecraft:attack_speed".to_string()),
    );
    compound.insert(
        "id".to_string(),
        Nbt::String("minecraft:test_speed".to_string()),
    );
    compound.insert("amount".to_string(), Nbt::Double(1.0));
    compound.insert(
        "operation".to_string(),
        Nbt::String("add_value".to_string()),
    );
    compound
}
