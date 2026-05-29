use crate::entity::Player;
use crate::world::{Block, BlockPosition};
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerCancelDiggingEvent {
    player: *mut Player,
    block: Block,
    block_position: BlockPosition,
}

impl PlayerCancelDiggingEvent {
    pub fn new(player: *mut Player, block: Block, block_position: BlockPosition) -> Self {
        Self {
            player,
            block,
            block_position,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn block(&self) -> Block {
        self.block
    }

    pub fn block_position(&self) -> BlockPosition {
        self.block_position
    }
}
