use crate::entity::{Player, PlayerHand};
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerHandAnimationEvent {
    player: *mut Player,
    hand: PlayerHand,
    cancelled: bool,
}

impl PlayerHandAnimationEvent {
    pub fn new(player: *mut Player, hand: PlayerHand) -> Self {
        Self {
            player,
            hand,
            cancelled: false,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn hand(&self) -> PlayerHand {
        self.hand
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
