use crate::entity::EquipmentSlot;
use spinel_core::network::clientbound::play::set_equipment::EntityEquipmentEntry;
use spinel_network::types::Slot;
use spinel_registry::ItemStack;

#[derive(Clone, Debug, PartialEq)]
pub struct LivingEquipment {
    main_hand: ItemStack,
    off_hand: ItemStack,
    boots: ItemStack,
    leggings: ItemStack,
    chestplate: ItemStack,
    helmet: ItemStack,
    body: ItemStack,
    saddle: ItemStack,
}

impl LivingEquipment {
    pub fn item_stack(&self, equipment_slot: EquipmentSlot) -> &ItemStack {
        match equipment_slot {
            EquipmentSlot::MainHand => &self.main_hand,
            EquipmentSlot::OffHand => &self.off_hand,
            EquipmentSlot::Boots => &self.boots,
            EquipmentSlot::Leggings => &self.leggings,
            EquipmentSlot::Chestplate => &self.chestplate,
            EquipmentSlot::Helmet => &self.helmet,
            EquipmentSlot::Body => &self.body,
            EquipmentSlot::Saddle => &self.saddle,
        }
    }

    pub fn set_item_stack(&mut self, equipment_slot: EquipmentSlot, item_stack: ItemStack) {
        match equipment_slot {
            EquipmentSlot::MainHand => self.main_hand = item_stack,
            EquipmentSlot::OffHand => self.off_hand = item_stack,
            EquipmentSlot::Boots => self.boots = item_stack,
            EquipmentSlot::Leggings => self.leggings = item_stack,
            EquipmentSlot::Chestplate => self.chestplate = item_stack,
            EquipmentSlot::Helmet => self.helmet = item_stack,
            EquipmentSlot::Body => self.body = item_stack,
            EquipmentSlot::Saddle => self.saddle = item_stack,
        }
    }

    pub fn visible_entries(&self) -> Vec<EntityEquipmentEntry> {
        [
            EquipmentSlot::MainHand,
            EquipmentSlot::OffHand,
            EquipmentSlot::Boots,
            EquipmentSlot::Leggings,
            EquipmentSlot::Chestplate,
            EquipmentSlot::Helmet,
            EquipmentSlot::Body,
        ]
        .into_iter()
        .map(|equipment_slot| EntityEquipmentEntry {
            slot: equipment_slot.entity_equipment_slot(),
            item: Slot::from_item_stack(self.item_stack(equipment_slot)),
        })
        .collect()
    }
}

impl Default for LivingEquipment {
    fn default() -> Self {
        Self {
            main_hand: ItemStack::air(),
            off_hand: ItemStack::air(),
            boots: ItemStack::air(),
            leggings: ItemStack::air(),
            chestplate: ItemStack::air(),
            helmet: ItemStack::air(),
            body: ItemStack::air(),
            saddle: ItemStack::air(),
        }
    }
}
