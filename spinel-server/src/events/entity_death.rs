use crate::entity::EntityId;
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct EntityDeathEvent {
    entity_id: EntityId,
}

impl EntityDeathEvent {
    pub fn new(entity_id: EntityId) -> Self {
        Self { entity_id }
    }

    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}
