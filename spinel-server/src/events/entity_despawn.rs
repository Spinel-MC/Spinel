use crate::entity::{Entity, EntityId};
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct EntityDespawnEvent {
    entity: *mut Entity,
}

impl EntityDespawnEvent {
    pub fn new(entity: *mut Entity) -> Self {
        Self { entity }
    }

    pub fn entity(&mut self) -> &mut Entity {
        unsafe { &mut *self.entity }
    }

    pub fn entity_id(&self) -> EntityId {
        unsafe { (&*self.entity).entity_id() }
    }
}
