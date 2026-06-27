use crate::entity::{EquipmentSlot, GenericEntity, LivingAttributes, Player};
use crate::network::client::instance::Client;
use spinel_core::network::clientbound::play::set_player_inventory::SetPlayerInventoryPacket;
use spinel_core::network::clientbound::play::update_attributes::{
    EntityAttributeModifier, UpdateAttributesPacket,
};
use spinel_network::ConnectionState;
use spinel_registry::data_components::vanilla_components::ATTRIBUTE_MODIFIERS;
use spinel_registry::data_components::{
    AttributeModifier, AttributeModifierDisplay, AttributeModifierEntry, AttributeOperation,
    EquipmentSlotGroup,
};
use spinel_registry::{Attribute, AttributeList, EntityType, Identifier, ItemStack, Material};
use std::net::TcpListener;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use uuid::Uuid;

fn player() -> Player {
    Player::new(
        Uuid::new_v4(),
        "living_attributes".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    )
}

#[test]
fn equipment_attributes_apply_vanilla_main_hand_modifier() {
    let mut player = player();

    assert!(player.set_equipment(
        EquipmentSlot::MainHand,
        ItemStack::of(Material::DIAMOND_PICKAXE),
    ));

    assert_eq!(
        player.get_attribute_value(Attribute::ATTACK_SPEED),
        1.2000000476837158
    );
}

#[test]
fn equipment_attributes_replace_old_modifier_for_changed_slot() {
    let mut player = player();
    let main_hand = |amount| {
        ItemStack::of(Material::STONE).with(
            ATTRIBUTE_MODIFIERS,
            AttributeList::new(vec![AttributeModifierEntry::new(
                Attribute::ATTACK_SPEED.identifier(),
                AttributeModifier::new(
                    "minecraft:test_speed".parse().unwrap(),
                    amount,
                    AttributeOperation::AddValue,
                ),
                EquipmentSlotGroup::MainHand,
                AttributeModifierDisplay::Default,
            )]),
        )
    };

    assert!(player.set_equipment(EquipmentSlot::MainHand, main_hand(-1.0)));
    assert_eq!(player.get_attribute_value(Attribute::ATTACK_SPEED), 3.0);

    assert!(player.set_equipment(EquipmentSlot::MainHand, main_hand(-2.0)));
    assert_eq!(player.get_attribute_value(Attribute::ATTACK_SPEED), 2.0);
}

#[test]
fn login_attribute_sync_applies_preconfigured_selected_slot_item() {
    let mut player = player();
    let mut client = queued_client();
    assert!(
        player
            .get_inventory()
            .set_item_stack(0, ItemStack::of(Material::DIAMOND_PICKAXE))
    );

    player.sync_main_hand_attributes(&mut client).unwrap();

    assert_eq!(
        player.get_attribute_value(Attribute::ATTACK_SPEED),
        1.2000000476837158
    );
    assert_eq!(
        client.queued_outbound_packet_ids(),
        vec![UpdateAttributesPacket::get_id()]
    );
}

#[test]
fn active_equipment_change_syncs_inventory_slot_and_attributes() {
    let mut player = player();
    let mut client = queued_client();
    player.set_client(&mut client);

    assert!(player.set_equipment(
        EquipmentSlot::MainHand,
        ItemStack::of(Material::DIAMOND_PICKAXE),
    ));

    assert_eq!(
        player.get_attribute_value(Attribute::ATTACK_SPEED),
        1.2000000476837158
    );
    assert_eq!(
        client.queued_outbound_packet_ids(),
        vec![
            SetPlayerInventoryPacket::get_id(),
            UpdateAttributesPacket::get_id(),
        ]
    );
}

fn queued_client() -> Client {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let stream = std::net::TcpStream::connect(addr).unwrap();
    let _ = listener.accept().unwrap();
    let mut client = Client::new(stream, addr);
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    client
}

#[test]
fn runtime_attribute_modifiers_follow_minestom_operation_order() {
    let mut player = player();
    let attribute = player.get_attribute(Attribute::ATTACK_DAMAGE);
    attribute.set_base_value(10.0);
    attribute.add_modifier(EntityAttributeModifier {
        modifier_id: Identifier::minecraft("add_value"),
        amount: 2.0,
        operation: AttributeOperation::AddValue.protocol_id(),
    });
    attribute.add_modifier(EntityAttributeModifier {
        modifier_id: Identifier::minecraft("add_multiplied_base"),
        amount: 0.5,
        operation: AttributeOperation::AddMultipliedBase.protocol_id(),
    });
    attribute.add_modifier(EntityAttributeModifier {
        modifier_id: Identifier::minecraft("add_multiplied_total"),
        amount: 0.5,
        operation: AttributeOperation::AddMultipliedTotal.protocol_id(),
    });

    assert_eq!(player.get_attribute_value(Attribute::ATTACK_DAMAGE), 27.0);
}

#[test]
fn runtime_attribute_modifier_duplicate_id_replaces_same_attribute_entry() {
    let mut player = player();
    let attribute = player.get_attribute(Attribute::ATTACK_SPEED);
    let modifier_id = Identifier::minecraft("replace_speed");
    assert_eq!(
        attribute.add_modifier(EntityAttributeModifier {
            modifier_id: modifier_id.clone(),
            amount: -1.0,
            operation: AttributeOperation::AddValue.protocol_id(),
        }),
        None
    );
    assert_eq!(
        attribute
            .add_modifier(EntityAttributeModifier {
                modifier_id: modifier_id.clone(),
                amount: -2.0,
                operation: AttributeOperation::AddValue.protocol_id(),
            })
            .unwrap()
            .amount,
        -1.0
    );

    assert_eq!(player.get_attribute_value(Attribute::ATTACK_SPEED), 2.0);
}

#[test]
fn equipment_attribute_slot_groups_match_target_slot_names() {
    let mut main_hand_player = player();
    assert!(main_hand_player.set_equipment(
        EquipmentSlot::MainHand,
        item_with_modifier(EquipmentSlotGroup::Hand, "hand_speed", 1.0),
    ));
    assert_eq!(
        main_hand_player.get_attribute_value(Attribute::ATTACK_SPEED),
        5.0
    );

    let mut chest_player = player();
    assert!(chest_player.set_equipment(
        EquipmentSlot::Chestplate,
        item_with_modifier(EquipmentSlotGroup::Armor, "armor_speed", 1.0),
    ));
    assert_eq!(
        chest_player.get_attribute_value(Attribute::ATTACK_SPEED),
        5.0
    );

    let mut body_entity = GenericEntity::new(EntityType::ZOMBIE);
    body_entity.set_equipment(
        EquipmentSlot::Body,
        item_with_modifier(EquipmentSlotGroup::Body, "body_speed", 1.0),
    );
    assert_eq!(
        body_entity.get_attribute_value(Attribute::ATTACK_SPEED.protocol_id(), 4.0),
        5.0
    );
}

#[test]
fn equipment_attribute_transition_removes_previous_item_modifiers() {
    let mut player = player();
    assert!(player.set_equipment(
        EquipmentSlot::MainHand,
        item_with_modifier(EquipmentSlotGroup::MainHand, "temporary_speed", -1.0),
    ));
    assert_eq!(player.get_attribute_value(Attribute::ATTACK_SPEED), 3.0);

    assert!(player.set_equipment(EquipmentSlot::MainHand, ItemStack::of(Material::AIR)));

    assert_eq!(player.get_attribute_value(Attribute::ATTACK_SPEED), 4.0);
}

#[test]
fn runtime_attribute_base_value_removal_and_clamp_match_minestom_attribute_instance() {
    let mut player = player();
    let attribute = player.get_attribute(Attribute::ATTACK_SPEED);
    let modifier = EntityAttributeModifier {
        modifier_id: Identifier::minecraft("temporary_speed"),
        amount: 5.0,
        operation: AttributeOperation::AddValue.protocol_id(),
    };

    assert_eq!(attribute.get_base_value(), 4.0);
    attribute.set_base_value(10.0);
    assert_eq!(attribute.get_base_value(), 10.0);
    assert_eq!(attribute.get_value(), 10.0);
    assert_eq!(attribute.add_modifier(modifier.clone()), None);
    assert_eq!(attribute.get_value(), 15.0);
    assert_eq!(
        attribute.remove_modifier(&modifier.modifier_id),
        Some(modifier)
    );
    assert_eq!(attribute.get_value(), 10.0);

    attribute.set_base_value(2000.0);
    assert_eq!(attribute.get_value(), 1024.0);
    attribute.set_base_value(-1.0);
    assert_eq!(attribute.get_value(), 0.0);
}
#[test]
fn living_attributes_default_lookup_does_not_create_lazy_attribute_instance() {
    let mut attributes = LivingAttributes::default();

    assert!(attributes.is_empty());
    assert_eq!(attributes.get_attribute_value(Attribute::ATTACK_SPEED), 4.0);
    assert!(attributes.is_empty());

    let attack_speed = attributes.get_attribute(Attribute::ATTACK_SPEED);
    assert_eq!(attack_speed.get_attribute(), Attribute::ATTACK_SPEED);
    assert_eq!(attack_speed.get_base_value(), 4.0);
    assert_eq!(attack_speed.get_value(), 4.0);
    assert_eq!(attributes.get_attributes().len(), 1);
}

#[test]
fn runtime_attribute_absent_modifier_removal_returns_none_and_keeps_value() {
    let mut player = player();
    let attribute = player.get_attribute(Attribute::ATTACK_SPEED);

    assert_eq!(
        attribute.remove_modifier(&Identifier::minecraft("missing_speed")),
        None
    );
    assert_eq!(attribute.get_value(), 4.0);
}

#[test]
fn runtime_attribute_duplicate_modifier_id_is_scoped_to_attribute_instance() {
    let mut player = player();
    let modifier_id = Identifier::minecraft("shared_modifier");
    let attack_speed = player.get_attribute(Attribute::ATTACK_SPEED);
    attack_speed.add_modifier(EntityAttributeModifier {
        modifier_id: modifier_id.clone(),
        amount: -1.0,
        operation: AttributeOperation::AddValue.protocol_id(),
    });
    let attack_damage = player.get_attribute(Attribute::ATTACK_DAMAGE);
    attack_damage.add_modifier(EntityAttributeModifier {
        modifier_id,
        amount: 2.0,
        operation: AttributeOperation::AddValue.protocol_id(),
    });

    assert_eq!(player.get_attribute_value(Attribute::ATTACK_SPEED), 3.0);
    assert_eq!(player.get_attribute_value(Attribute::ATTACK_DAMAGE), 3.0);
}

#[test]
fn equipment_attribute_every_slot_group_applies_only_to_matching_equipment_slots() {
    let matching_cases = [
        (EquipmentSlotGroup::Any, EquipmentSlot::MainHand),
        (EquipmentSlotGroup::Any, EquipmentSlot::OffHand),
        (EquipmentSlotGroup::Any, EquipmentSlot::Boots),
        (EquipmentSlotGroup::Any, EquipmentSlot::Leggings),
        (EquipmentSlotGroup::Any, EquipmentSlot::Chestplate),
        (EquipmentSlotGroup::Any, EquipmentSlot::Helmet),
        (EquipmentSlotGroup::Any, EquipmentSlot::Body),
        (EquipmentSlotGroup::MainHand, EquipmentSlot::MainHand),
        (EquipmentSlotGroup::OffHand, EquipmentSlot::OffHand),
        (EquipmentSlotGroup::Hand, EquipmentSlot::MainHand),
        (EquipmentSlotGroup::Hand, EquipmentSlot::OffHand),
        (EquipmentSlotGroup::Feet, EquipmentSlot::Boots),
        (EquipmentSlotGroup::Legs, EquipmentSlot::Leggings),
        (EquipmentSlotGroup::Chest, EquipmentSlot::Chestplate),
        (EquipmentSlotGroup::Head, EquipmentSlot::Helmet),
        (EquipmentSlotGroup::Armor, EquipmentSlot::Boots),
        (EquipmentSlotGroup::Armor, EquipmentSlot::Leggings),
        (EquipmentSlotGroup::Armor, EquipmentSlot::Chestplate),
        (EquipmentSlotGroup::Armor, EquipmentSlot::Helmet),
        (EquipmentSlotGroup::Body, EquipmentSlot::Body),
    ];
    for (case_index, (slot_group, equipment_slot)) in matching_cases.into_iter().enumerate() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);
        entity.set_equipment(
            equipment_slot,
            item_with_modifier(slot_group, &format!("matched_speed_{case_index}"), 1.0),
        );
        assert_eq!(
            entity.get_attribute_value(Attribute::ATTACK_SPEED.protocol_id(), 4.0),
            5.0
        );
    }

    let non_matching_cases = [
        (EquipmentSlotGroup::MainHand, EquipmentSlot::OffHand),
        (EquipmentSlotGroup::OffHand, EquipmentSlot::MainHand),
        (EquipmentSlotGroup::Hand, EquipmentSlot::Helmet),
        (EquipmentSlotGroup::Feet, EquipmentSlot::Leggings),
        (EquipmentSlotGroup::Legs, EquipmentSlot::Boots),
        (EquipmentSlotGroup::Chest, EquipmentSlot::Helmet),
        (EquipmentSlotGroup::Head, EquipmentSlot::Chestplate),
        (EquipmentSlotGroup::Armor, EquipmentSlot::Body),
        (EquipmentSlotGroup::Body, EquipmentSlot::Chestplate),
    ];
    for (case_index, (slot_group, equipment_slot)) in non_matching_cases.into_iter().enumerate() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);
        entity.set_equipment(
            equipment_slot,
            item_with_modifier(slot_group, &format!("unmatched_speed_{case_index}"), 1.0),
        );
        assert_eq!(
            entity.get_attribute_value(Attribute::ATTACK_SPEED.protocol_id(), 4.0),
            4.0
        );
    }
}

#[test]
fn equipment_attribute_unknown_attribute_identifier_is_ignored() {
    let mut player = player();
    let item_stack = ItemStack::of(Material::STONE).with(
        ATTRIBUTE_MODIFIERS,
        AttributeList::from_modifier(AttributeModifierEntry::new(
            "minecraft:not_a_real_attribute".parse().unwrap(),
            AttributeModifier::from(
                "minecraft:unknown_attribute",
                1.0,
                AttributeOperation::AddValue,
            )
            .unwrap(),
            EquipmentSlotGroup::MainHand,
            AttributeModifierDisplay::Default,
        )),
    );

    assert!(player.set_equipment(EquipmentSlot::MainHand, item_stack));
    assert_eq!(player.get_attribute_value(Attribute::ATTACK_SPEED), 4.0);
}

#[test]
fn equipment_attribute_unequip_and_reequip_round_trip_restores_modifier() {
    let mut player = player();
    let item_stack = item_with_modifier(EquipmentSlotGroup::MainHand, "round_trip_speed", -1.0);

    assert!(player.set_equipment(EquipmentSlot::MainHand, item_stack.clone()));
    assert_eq!(player.get_attribute_value(Attribute::ATTACK_SPEED), 3.0);

    assert!(player.set_equipment(EquipmentSlot::MainHand, ItemStack::of(Material::AIR)));
    assert_eq!(player.get_attribute_value(Attribute::ATTACK_SPEED), 4.0);

    assert!(player.set_equipment(EquipmentSlot::MainHand, item_stack));
    assert_eq!(player.get_attribute_value(Attribute::ATTACK_SPEED), 3.0);
}
#[test]
fn runtime_attribute_remove_modifier_by_value_uses_modifier_id_and_returns_stored_modifier() {
    let mut player = player();
    let attribute = player.get_attribute(Attribute::ATTACK_SPEED);
    let modifier_id = Identifier::minecraft("remove_by_value_speed");
    let stored_modifier = EntityAttributeModifier {
        modifier_id: modifier_id.clone(),
        amount: -1.0,
        operation: AttributeOperation::AddValue.protocol_id(),
    };
    let lookup_modifier = EntityAttributeModifier {
        modifier_id,
        amount: -9.0,
        operation: AttributeOperation::AddMultipliedTotal.protocol_id(),
    };

    assert_eq!(attribute.add_modifier(stored_modifier.clone()), None);
    assert_eq!(
        attribute.remove_modifier_by_value(&lookup_modifier),
        Some(stored_modifier)
    );
    assert_eq!(attribute.get_value(), 4.0);
}

#[test]
fn runtime_attribute_apply_modifiers_uses_supplied_base_without_mutating_base_value() {
    let mut player = player();
    let attribute = player.get_attribute(Attribute::ATTACK_DAMAGE);
    attribute.set_base_value(10.0);
    attribute.add_modifier(EntityAttributeModifier {
        modifier_id: Identifier::minecraft("add_value_for_supplied_base"),
        amount: 2.0,
        operation: AttributeOperation::AddValue.protocol_id(),
    });
    attribute.add_modifier(EntityAttributeModifier {
        modifier_id: Identifier::minecraft("multiply_supplied_base"),
        amount: 0.5,
        operation: AttributeOperation::AddMultipliedBase.protocol_id(),
    });
    attribute.add_modifier(EntityAttributeModifier {
        modifier_id: Identifier::minecraft("multiply_supplied_total"),
        amount: 0.5,
        operation: AttributeOperation::AddMultipliedTotal.protocol_id(),
    });

    assert_eq!(attribute.apply_modifiers(20.0), 49.5);
    assert_eq!(attribute.get_base_value(), 10.0);
    assert_eq!(attribute.get_value(), 27.0);
}

#[test]
fn runtime_attribute_clear_modifiers_preserves_protected_sprinting_modifier_only() {
    let mut player = player();
    let attribute = player.get_attribute(Attribute::ATTACK_SPEED);
    let protected_modifier = EntityAttributeModifier {
        modifier_id: Identifier::minecraft("sprinting"),
        amount: 0.3,
        operation: AttributeOperation::AddMultipliedTotal.protocol_id(),
    };
    let ordinary_modifier = EntityAttributeModifier {
        modifier_id: Identifier::minecraft("temporary_speed_boost"),
        amount: 2.0,
        operation: AttributeOperation::AddValue.protocol_id(),
    };

    attribute.add_modifier(protected_modifier.clone());
    attribute.add_modifier(ordinary_modifier);
    assert_eq!(attribute.get_modifiers().len(), 2);

    attribute.clear_modifiers();

    assert_eq!(attribute.get_modifiers(), vec![&protected_modifier]);
    assert_eq!(attribute.get_value(), 5.2);
}

#[test]
fn runtime_attribute_clear_modifiers_on_empty_instance_is_noop() {
    let mut player = player();
    let attribute = player.get_attribute(Attribute::ATTACK_SPEED);

    attribute.clear_modifiers();

    assert!(attribute.get_modifiers().is_empty());
    assert_eq!(attribute.get_value(), 4.0);
}

#[test]
fn runtime_attribute_modifier_view_is_snapshot_without_mutable_map_access() {
    let mut player = player();
    let attribute = player.get_attribute(Attribute::ATTACK_SPEED);
    let modifier = EntityAttributeModifier {
        modifier_id: Identifier::minecraft("snapshot_speed"),
        amount: 1.0,
        operation: AttributeOperation::AddValue.protocol_id(),
    };
    attribute.add_modifier(modifier.clone());

    let modifiers = attribute.get_modifiers();

    assert_eq!(modifiers, vec![&modifier]);
    assert_eq!(attribute.get_modifiers().len(), 1);
}
fn item_with_modifier(slot_group: EquipmentSlotGroup, modifier_id: &str, amount: f64) -> ItemStack {
    ItemStack::of(Material::STONE).with(
        ATTRIBUTE_MODIFIERS,
        AttributeList::from_modifier(attribute_modifier_entry(slot_group, modifier_id, amount)),
    )
}

fn attribute_modifier_entry(
    slot_group: EquipmentSlotGroup,
    modifier_id: &str,
    amount: f64,
) -> AttributeModifierEntry {
    AttributeModifierEntry::new(
        Attribute::ATTACK_SPEED.identifier(),
        AttributeModifier::from(modifier_id, amount, AttributeOperation::AddValue).unwrap(),
        slot_group,
        AttributeModifierDisplay::Default,
    )
}
