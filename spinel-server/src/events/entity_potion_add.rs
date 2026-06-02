use crate::entity::{EntityId, TimedPotionEffect};
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct EntityPotionAddEvent {
    entity_id: EntityId,
    effect: TimedPotionEffect,
    cancelled: bool,
}

impl EntityPotionAddEvent {
    pub fn new(entity_id: EntityId, effect: TimedPotionEffect) -> Self {
        Self {
            entity_id,
            effect,
            cancelled: false,
        }
    }

    pub fn entity_id(&self) -> EntityId {
        self.entity_id
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
