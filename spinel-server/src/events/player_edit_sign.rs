use crate::entity::Player;
use crate::world::{Block, BlockPosition};
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerEditSignEvent {
    player: *mut Player,
    block: Block,
    block_position: BlockPosition,
    lines: [String; 4],
    is_front_text: bool,
}

impl PlayerEditSignEvent {
    pub fn new(
        player: *mut Player,
        block: Block,
        block_position: BlockPosition,
        lines: [String; 4],
        is_front_text: bool,
    ) -> Self {
        Self {
            player,
            block,
            block_position,
            lines,
            is_front_text,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub const fn block(&self) -> Block {
        self.block
    }

    pub const fn block_position(&self) -> BlockPosition {
        self.block_position
    }

    pub fn lines(&self) -> &[String; 4] {
        &self.lines
    }

    pub const fn is_front_text(&self) -> bool {
        self.is_front_text
    }
}
