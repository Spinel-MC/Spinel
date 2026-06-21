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

    pub fn get_entity(&mut self) -> &mut Entity {
        unsafe { &mut *self.entity }
    }

    pub fn get_entity_id(&self) -> EntityId {
        unsafe { (&*self.entity).get_entity_id() }
    }

    pub fn get_effect(&self) -> &TimedPotionEffect {
        &self.effect
    }
}
