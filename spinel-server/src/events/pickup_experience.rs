use crate::entity::EntityId;
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct PickupExperienceEvent {
    player_id: EntityId,
    experience_orb_id: EntityId,
    experience_count: i16,
    cancelled: bool,
}

impl PickupExperienceEvent {
    pub fn new(player_id: EntityId, experience_orb_id: EntityId, experience_count: i16) -> Self {
        Self {
            player_id,
            experience_orb_id,
            experience_count,
            cancelled: false,
        }
    }

    pub fn player_id(&self) -> EntityId {
        self.player_id
    }

    pub fn experience_orb_id(&self) -> EntityId {
        self.experience_orb_id
    }

    pub fn experience_count(&self) -> i16 {
        self.experience_count
    }

    pub fn set_experience_count(&mut self, experience_count: i16) {
        self.experience_count = experience_count;
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
