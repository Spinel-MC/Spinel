use crate::entity::{EquipmentSlot, Player, PlayerHand};
use crate::network::client::instance::Client;
use spinel_core::network::clientbound::play::update_attributes::{
    EntityAttribute, EntityAttributeModifier, UpdateAttributesPacket,
};
use spinel_registry::AttributeOperation;
use spinel_registry::data_components::vanilla_components::ATTRIBUTE_MODIFIERS;
use spinel_registry::{Identifier, ItemStack};
use std::io;

const BASE_ATTACK_SPEED: f64 = 4.0;

impl Player {
    pub(crate) fn sync_main_hand_attributes(&self, client: &mut Client) -> io::Result<()> {
        UpdateAttributesPacket {
            entity_id: self.entity_id().value(),
            attributes: vec![EntityAttribute::attack_speed(
                BASE_ATTACK_SPEED,
                self.main_hand_attack_speed_modifiers(),
            )],
        }
        .dispatch(client)
    }

    pub(crate) fn set_held_slot_with_client(
        &mut self,
        held_slot: i32,
        client: &mut Client,
    ) -> bool {
        if !self.set_held_slot(held_slot) {
            return false;
        }
        self.sync_main_hand_attributes(client).is_ok()
    }

    pub(crate) fn slot_is_held_main_hand(&self, slot: i32) -> bool {
        slot - self.open_inventory_size() == self.held_slot()
    }

    fn main_hand_attack_speed_modifiers(&self) -> Vec<EntityAttributeModifier> {
        attack_speed_modifiers(
            &self.item_in_hand(PlayerHand::Main),
            EquipmentSlot::MainHand,
        )
        .into_iter()
        .map(|modifier| EntityAttributeModifier::attack_speed(modifier.id, modifier.amount))
        .collect()
    }
}

struct AttackSpeedModifier {
    id: Identifier,
    amount: f64,
}

fn attack_speed_modifiers(
    item_stack: &ItemStack,
    equipment_slot: EquipmentSlot,
) -> Vec<AttackSpeedModifier> {
    item_stack
        .get(ATTRIBUTE_MODIFIERS)
        .map(|attribute_list| {
            attribute_list
                .modifiers()
                .iter()
                .filter(|modifier| {
                    modifier.attribute_type().to_string() == "minecraft:attack_speed"
                })
                .filter(|modifier| modifier.operation() == AttributeOperation::AddValue)
                .filter(|modifier| {
                    modifier
                        .slot()
                        .contains_slot_name(equipment_slot.nbt_name())
                })
                .map(|modifier| AttackSpeedModifier {
                    id: modifier.id().clone(),
                    amount: modifier.amount(),
                })
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::{EquipmentSlot, attack_speed_modifiers};
    use spinel_registry::AttributeList;
    use spinel_registry::data_components::vanilla_components::ATTRIBUTE_MODIFIERS;
    use spinel_registry::data_components::{
        AttributeModifierDisplay, AttributeModifierEntry, AttributeOperation, EquipmentSlotGroup,
    };
    use spinel_registry::{ItemStack, Material};

    #[test]
    fn main_hand_attack_speed_reads_default_pickaxe_attribute_modifier() {
        let item_stack = ItemStack::of(Material::DIAMOND_PICKAXE);

        let modifiers = attack_speed_modifiers(&item_stack, EquipmentSlot::MainHand);

        assert!(modifiers.iter().any(|modifier| {
            modifier.id.to_string() == "minecraft:base_attack_speed"
                && modifier.amount == -2.799999952316284
        }));
    }

    #[test]
    fn attack_speed_ignores_modifiers_for_other_slots() {
        let item_stack = ItemStack::of(Material::DIAMOND_PICKAXE).with(
            ATTRIBUTE_MODIFIERS,
            AttributeList::new(vec![AttributeModifierEntry::new(
                "minecraft:attack_speed".parse().unwrap(),
                "minecraft:base_attack_speed".parse().unwrap(),
                -2.8,
                AttributeOperation::AddValue,
                EquipmentSlotGroup::OffHand,
                AttributeModifierDisplay::Default,
            )]),
        );

        let modifiers = attack_speed_modifiers(&item_stack, EquipmentSlot::MainHand);

        assert!(modifiers.is_empty());
    }
}
