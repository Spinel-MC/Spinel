use crate::entity::{Player, PlayerHand};
use spinel_macros::event_dispatcher;
use spinel_registry::ItemStack;

#[event_dispatcher(with_client: true)]
pub struct PlayerFinishItemUseEvent {
    player: *mut Player,
    hand: PlayerHand,
    item_stack: ItemStack,
    item_use_duration: u64,
}

impl PlayerFinishItemUseEvent {
    pub fn new(
        player: *mut Player,
        hand: PlayerHand,
        item_stack: ItemStack,
        item_use_duration: u64,
    ) -> Self {
        Self {
            player,
            hand,
            item_stack,
            item_use_duration,
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

    pub fn item_use_duration(&self) -> u64 {
        self.item_use_duration
    }
}
