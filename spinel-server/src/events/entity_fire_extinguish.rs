use crate::entity::{Entity, EntityId};
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct EntityFireExtinguishEvent {
    entity: *mut Entity,
    natural: bool,
    cancelled: bool,
}

impl EntityFireExtinguishEvent {
    pub fn new(entity: *mut Entity, natural: bool) -> Self {
        Self {
            entity,
            natural,
            cancelled: false,
        }
    }

    pub fn get_entity(&mut self) -> &mut Entity {
        unsafe { &mut *self.entity }
    }

    pub fn get_entity_id(&self) -> EntityId {
        unsafe { (&*self.entity).get_entity_id() }
    }

    pub fn is_natural(&self) -> bool {
        self.natural
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
