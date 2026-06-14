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

    pub fn entity(&mut self) -> &mut Entity {
        unsafe { &mut *self.entity }
    }

    pub fn entity_id(&self) -> EntityId {
        unsafe { (&*self.entity).entity_id() }
    }

    pub fn effect(&self) -> &TimedPotionEffect {
        &self.effect
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
