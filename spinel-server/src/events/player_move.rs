use crate::entity::{EntityPosition, Player};
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerMoveEvent {
    player: *mut Player,
    new_position: EntityPosition,
    on_ground: bool,
    cancelled: bool,
}

impl PlayerMoveEvent {
    pub fn new(player: *mut Player, new_position: EntityPosition, on_ground: bool) -> Self {
        Self {
            player,
            new_position,
            on_ground,
            cancelled: false,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn new_position(&self) -> EntityPosition {
        self.new_position
    }

    pub fn set_new_position(&mut self, new_position: EntityPosition) {
        self.new_position = new_position;
    }

    pub fn is_on_ground(&self) -> bool {
        self.on_ground
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
