use crate::entity::{Entity, EntityId};
use spinel_macros::event_dispatcher;
use spinel_registry::ItemStack;

#[event_dispatcher]
pub struct EntityItemMergeEvent {
    entity: *mut Entity,
    merged_entity: *mut Entity,
    result: ItemStack,
    cancelled: bool,
}

impl EntityItemMergeEvent {
    pub fn new(entity: *mut Entity, merged_entity: *mut Entity, result: ItemStack) -> Self {
        Self {
            entity,
            merged_entity,
            result,
            cancelled: false,
        }
    }

    pub fn get_entity(&mut self) -> &mut Entity {
        unsafe { &mut *self.entity }
    }

    pub fn get_entity_id(&self) -> EntityId {
        unsafe { (&*self.entity).get_entity_id() }
    }

    pub fn get_merged_entity(&mut self) -> &mut Entity {
        unsafe { &mut *self.merged_entity }
    }

    pub fn get_merged_entity_id(&self) -> EntityId {
        unsafe { (&*self.merged_entity).get_entity_id() }
    }

    pub fn get_result(&self) -> &ItemStack {
        &self.result
    }

    pub fn set_result(&mut self, result: ItemStack) {
        self.result = result;
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
