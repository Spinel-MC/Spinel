use crate::entity::{EntityId, TimedPotionEffect};
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct EntityPotionRemoveEvent {
    entity_id: EntityId,
    effect: TimedPotionEffect,
}

impl EntityPotionRemoveEvent {
    pub fn new(entity_id: EntityId, effect: TimedPotionEffect) -> Self {
        Self { entity_id, effect }
    }

    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub fn effect(&self) -> &TimedPotionEffect {
        &self.effect
    }
}
