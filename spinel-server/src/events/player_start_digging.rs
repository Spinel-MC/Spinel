use crate::entity::Player;
use crate::events::player_block_interact::BlockFace;
use crate::world::{Block, BlockPosition};
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerStartDiggingEvent {
    player: *mut Player,
    block: Block,
    block_position: BlockPosition,
    block_face: BlockFace,
    cancelled: bool,
}

impl PlayerStartDiggingEvent {
    pub fn new(
        player: *mut Player,
        block: Block,
        block_position: BlockPosition,
        block_face: BlockFace,
    ) -> Self {
        Self {
            player,
            block,
            block_position,
            block_face,
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

    pub fn block_position(&self) -> BlockPosition {
        self.block_position
    }

    pub fn block_face(&self) -> BlockFace {
        self.block_face
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
