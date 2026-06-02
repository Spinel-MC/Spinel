use crate::entity::{EntityId, Player};
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerSpectateEvent {
    player: *mut Player,
    target_id: EntityId,
    cancelled: bool,
}

impl PlayerSpectateEvent {
    pub fn new(player: *mut Player, target_id: EntityId) -> Self {
        Self {
            player,
            target_id,
            cancelled: false,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn target_id(&self) -> EntityId {
        self.target_id
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
