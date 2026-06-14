use crate::entity::{Player, PlayerHand};
use spinel_macros::event_dispatcher;
use spinel_registry::ItemStack;

#[event_dispatcher(with_client: true)]
pub struct PlayerStabEvent {
    player: *mut Player,
}

impl PlayerStabEvent {
    pub fn new(player: *mut Player) -> Self {
        Self {
            player,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn item_stack(&self) -> ItemStack {
        unsafe { &*self.player }.item_in_hand(PlayerHand::Main)
    }
}
