use crate::entity::{Player, PlayerHand};
use crate::events::player_block_interact::BlockFace;
use crate::world::BlockPosition;
use spinel_macros::event_dispatcher;
use spinel_registry::ItemStack;

#[event_dispatcher(with_client: true)]
pub struct PlayerUseItemOnBlockEvent {
    player: *mut Player,
    hand: PlayerHand,
    item_stack: ItemStack,
    block_position: BlockPosition,
    cursor_position: (f32, f32, f32),
    block_face: BlockFace,
}

impl PlayerUseItemOnBlockEvent {
    pub fn new(
        player: *mut Player,
        hand: PlayerHand,
        item_stack: ItemStack,
        block_position: BlockPosition,
        cursor_position: (f32, f32, f32),
        block_face: BlockFace,
    ) -> Self {
        Self {
            player,
            hand,
            item_stack,
            block_position,
            cursor_position,
            block_face,
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

    pub fn block_position(&self) -> BlockPosition {
        self.block_position
    }

    pub fn cursor_position(&self) -> (f32, f32, f32) {
        self.cursor_position
    }

    pub fn block_face(&self) -> BlockFace {
        self.block_face
    }
}
