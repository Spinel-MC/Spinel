use crate::entity::EntityId;
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct EntityAttackEvent {
    entity_id: EntityId,
    target_id: EntityId,
}

impl EntityAttackEvent {
    pub fn new(entity_id: EntityId, target_id: EntityId) -> Self {
        Self {
            entity_id,
            target_id,
        }
    }

    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub fn target_id(&self) -> EntityId {
        self.target_id
    }
}
