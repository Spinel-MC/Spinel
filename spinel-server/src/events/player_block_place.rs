use crate::entity::{Player, PlayerHand};
use crate::events::player_block_interact::BlockFace;
use crate::world::{Block, BlockPosition};
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerBlockPlaceEvent {
    player: *mut Player,
    block: Block,
    block_face: BlockFace,
    block_position: BlockPosition,
    cursor_position: (f32, f32, f32),
    hand: PlayerHand,
    consume_block: bool,
    do_block_updates: bool,
    cancelled: bool,
}

impl PlayerBlockPlaceEvent {
    pub fn new(
        player: *mut Player,
        block: Block,
        block_face: BlockFace,
        block_position: BlockPosition,
        cursor_position: (f32, f32, f32),
        hand: PlayerHand,
    ) -> Self {
        Self {
            player,
            block,
            block_face,
            block_position,
            cursor_position,
            hand,
            consume_block: true,
            do_block_updates: true,
            cancelled: false,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn block(&self) -> Block {
        self.block
    }

    pub fn set_block(&mut self, block: Block) {
        self.block = block;
    }

    pub fn block_face(&self) -> BlockFace {
        self.block_face
    }

    pub fn block_position(&self) -> BlockPosition {
        self.block_position
    }

    pub fn cursor_position(&self) -> (f32, f32, f32) {
        self.cursor_position
    }

    pub fn hand(&self) -> PlayerHand {
        self.hand
    }

    pub fn does_consume_block(&self) -> bool {
        self.consume_block
    }

    pub fn consume_block(&mut self, consume_block: bool) {
        self.consume_block = consume_block;
    }

    pub fn should_do_block_updates(&self) -> bool {
        self.do_block_updates
    }

    pub fn set_do_block_updates(&mut self, do_block_updates: bool) {
        self.do_block_updates = do_block_updates;
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
