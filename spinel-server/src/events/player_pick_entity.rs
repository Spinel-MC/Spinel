use crate::entity::{EntityId, Player};
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerPickEntityEvent {
    player: *mut Player,
    target: EntityId,
    include_data: bool,
    cancelled: bool,
}

impl PlayerPickEntityEvent {
    pub fn new(player: *mut Player, target: EntityId, include_data: bool) -> Self {
        Self {
            player,
            target,
            include_data,
            cancelled: false,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub const fn target(&self) -> EntityId {
        self.target
    }

    pub const fn include_data(&self) -> bool {
        self.include_data
    }

    pub const fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
