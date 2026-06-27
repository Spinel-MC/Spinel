use crate::entity::{EquipmentSlot, Player};
use crate::network::client::instance::Client;
use spinel_core::network::clientbound::play::update_attributes::UpdateAttributesPacket;
use spinel_registry::{Attribute, ItemStack};
use std::io;

impl Player {
    pub(crate) fn sync_main_hand_attributes(&mut self, client: &mut Client) -> io::Result<()> {
        self.refresh_current_equipment_attributes();
        self.update_attributes_packet().dispatch(client)
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

    pub(crate) fn get_slot_is_held_main_hand(&self, slot: i32) -> bool {
        slot - self.open_inventory_size() == self.get_held_slot()
    }

    pub(crate) fn update_inventory_slot_attributes(
        &mut self,
        slot: i32,
        previous_item_stack: &ItemStack,
        current_item_stack: &ItemStack,
    ) -> bool {
        let Some(equipment_slot) = self
            .get_inventory_ref()
            .equipment_slot_for_slot(slot, self.get_held_slot())
        else {
            return false;
        };
        self.living
            .get_attributes_mut()
            .update_equipment_attributes(previous_item_stack, current_item_stack, equipment_slot);
        true
    }

    pub fn get_attribute_value(&self, attribute: Attribute) -> f64 {
        self.living.get_attribute_value(attribute)
    }

    pub fn update_attributes_packet(&self) -> UpdateAttributesPacket {
        self.living.update_attributes_packet(self.get_entity_id())
    }

    pub(crate) fn refresh_current_equipment_attributes(&mut self) {
        [
            EquipmentSlot::MainHand,
            EquipmentSlot::OffHand,
            EquipmentSlot::Boots,
            EquipmentSlot::Leggings,
            EquipmentSlot::Chestplate,
            EquipmentSlot::Helmet,
        ]
        .into_iter()
        .for_each(|equipment_slot| {
            let current_item_stack = self
                .get_inventory_ref()
                .get_equipment(equipment_slot, self.get_held_slot());
            let previous_item_stack = self
                .attribute_equipment
                .insert(equipment_slot, current_item_stack.clone())
                .unwrap_or_else(ItemStack::air);
            self.living
                .get_attributes_mut()
                .update_equipment_attributes(
                    &previous_item_stack,
                    &current_item_stack,
                    equipment_slot,
                );
        });
    }
}
