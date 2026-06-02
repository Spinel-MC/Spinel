use crate::entity::{EntityId, EquipmentSlot};
use spinel_macros::event_dispatcher;
use spinel_registry::ItemStack;

#[event_dispatcher]
pub struct EntityEquipEvent {
    entity_id: EntityId,
    equipped_item: ItemStack,
    slot: EquipmentSlot,
}

impl EntityEquipEvent {
    pub fn new(entity_id: EntityId, equipped_item: ItemStack, slot: EquipmentSlot) -> Self {
        Self {
            entity_id,
            equipped_item,
            slot,
        }
    }

    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub fn equipped_item(&self) -> &ItemStack {
        &self.equipped_item
    }

    pub fn set_equipped_item(&mut self, equipped_item: ItemStack) {
        self.equipped_item = equipped_item;
    }

    pub fn item_stack(&self) -> &ItemStack {
        &self.equipped_item
    }

    pub fn slot(&self) -> EquipmentSlot {
        self.slot
    }
}
