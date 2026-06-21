use crate::entity::{Entity, EntityId, TimedPotionEffect};
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct EntityPotionAddEvent {
    entity: *mut Entity,
    effect: TimedPotionEffect,
    cancelled: bool,
}

impl EntityPotionAddEvent {
    pub fn new(entity: *mut Entity, effect: TimedPotionEffect) -> Self {
        Self {
            entity,
            effect,
            cancelled: false,
        }
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

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
