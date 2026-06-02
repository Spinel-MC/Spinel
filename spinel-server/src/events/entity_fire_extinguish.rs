use crate::entity::EntityId;
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct EntityFireExtinguishEvent {
    entity_id: EntityId,
    natural: bool,
    cancelled: bool,
}

impl EntityFireExtinguishEvent {
    pub fn new(entity_id: EntityId, natural: bool) -> Self {
        Self {
            entity_id,
            natural,
            cancelled: false,
        }
    }

    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub fn is_natural(&self) -> bool {
        self.natural
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
