use crate::entity::Player;
use crate::world::BlockPosition;
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerPickBlockEvent {
    player: *mut Player,
    position: BlockPosition,
    include_data: bool,
    cancelled: bool,
}

impl PlayerPickBlockEvent {
    pub fn new(player: *mut Player, position: BlockPosition, include_data: bool) -> Self {
        Self {
            player,
            position,
            include_data,
            cancelled: false,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub const fn position(&self) -> BlockPosition {
        self.position
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
