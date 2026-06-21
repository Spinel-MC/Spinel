use crate::entity::{Player, PlayerHand};
use spinel_macros::event_dispatcher;
use spinel_registry::ItemStack;

#[event_dispatcher(with_client: true)]
pub struct PlayerUseItemEvent {
    player: *mut Player,
    hand: PlayerHand,
    item_stack: ItemStack,
    item_use_time: u64,
    cancelled: bool,
}

impl PlayerUseItemEvent {
    pub fn new(
        player: *mut Player,
        hand: PlayerHand,
        item_stack: ItemStack,
        item_use_time: u64,
    ) -> Self {
        Self {
            player,
            hand,
            item_stack,
            item_use_time,
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

    pub fn get_item_stack(&self) -> &ItemStack {
        &self.item_stack
    }

    pub fn item_use_time(&self) -> u64 {
        self.item_use_time
    }

    pub fn set_item_use_time(&mut self, item_use_time: u64) {
        self.item_use_time = item_use_time;
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
