use crate::entity::EntityId;
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct PickupItemEvent {
    living_entity_id: EntityId,
    item_entity_id: EntityId,
    cancelled: bool,
}

impl PickupItemEvent {
    pub fn new(living_entity_id: EntityId, item_entity_id: EntityId) -> Self {
        Self {
            living_entity_id,
            item_entity_id,
            cancelled: false,
        }
    }

    pub fn living_entity_id(&self) -> EntityId {
        self.living_entity_id
    }

    pub fn item_entity_id(&self) -> EntityId {
        self.item_entity_id
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
