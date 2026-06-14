use crate::entity::{Entity, EntityId, TimedPotionEffect};
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct EntityPotionRemoveEvent {
    entity: *mut Entity,
    effect: TimedPotionEffect,
}

impl EntityPotionRemoveEvent {
    pub fn new(entity: *mut Entity, effect: TimedPotionEffect) -> Self {
        Self { entity, effect }
    }

    pub fn entity(&mut self) -> &mut Entity {
        unsafe { &mut *self.entity }
    }

    pub fn entity_id(&self) -> EntityId {
        unsafe { (&*self.entity).entity_id() }
    }

    pub fn effect(&self) -> &TimedPotionEffect {
        &self.effect
    }
}
