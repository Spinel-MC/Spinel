use crate::entity::{Player, PlayerHand};
use spinel_macros::event_dispatcher;
use spinel_registry::{ItemAnimation, ItemStack};

#[event_dispatcher(with_client: true)]
pub struct PlayerBeginItemUseEvent {
    player: *mut Player,
    hand: PlayerHand,
    item_stack: ItemStack,
    animation: ItemAnimation,
    item_use_duration: u64,
    cancelled: bool,
}

impl PlayerBeginItemUseEvent {
    pub fn new(
        player: *mut Player,
        hand: PlayerHand,
        item_stack: ItemStack,
        animation: ItemAnimation,
        item_use_duration: u64,
    ) -> Self {
        Self {
            player,
            hand,
            item_stack,
            animation,
            item_use_duration,
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

    pub fn item_stack(&self) -> &ItemStack {
        &self.item_stack
    }

    pub fn animation(&self) -> ItemAnimation {
        self.animation
    }

    pub fn item_use_duration(&self) -> u64 {
        self.item_use_duration
    }

    pub fn set_item_use_duration(&mut self, item_use_duration: u64) {
        self.item_use_duration = item_use_duration;
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
