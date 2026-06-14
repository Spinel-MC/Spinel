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

    pub fn entity(&mut self) -> &mut Entity {
        unsafe { &mut *self.entity }
    }

    pub fn entity_id(&self) -> EntityId {
        unsafe { (&*self.entity).entity_id() }
    }

    pub fn merged_entity(&mut self) -> &mut Entity {
        unsafe { &mut *self.merged_entity }
    }

    pub fn merged_entity_id(&self) -> EntityId {
        unsafe { (&*self.merged_entity).entity_id() }
    }

    pub fn result(&self) -> &ItemStack {
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
