use crate::entity::{Entity, EntityId};
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct EntitySetFireEvent {
    entity: *mut Entity,
    fire_ticks: i32,
    cancelled: bool,
}

impl EntitySetFireEvent {
    pub fn new(entity: *mut Entity, fire_ticks: i32) -> Self {
        Self {
            entity,
            fire_ticks,
            cancelled: false,
        }
    }

    pub fn entity(&mut self) -> &mut Entity {
        unsafe { &mut *self.entity }
    }

    pub fn entity_id(&self) -> EntityId {
        unsafe { (&*self.entity).entity_id() }
    }

    pub fn fire_ticks(&self) -> i32 {
        self.fire_ticks
    }

    pub fn set_fire_ticks(&mut self, fire_ticks: i32) {
        self.fire_ticks = fire_ticks.max(0);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
