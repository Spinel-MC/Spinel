use crate::entity::{Player, PlayerHand};
use crate::world::{Block, BlockPosition};
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerBlockInteractEvent {
    player: *mut Player,
    hand: PlayerHand,
    block: Block,
    block_position: BlockPosition,
    cursor_position: (f32, f32, f32),
    block_face: BlockFace,
    blocking_item_use: bool,
    cancelled: bool,
}

impl PlayerBlockInteractEvent {
    pub fn new(
        player: *mut Player,
        hand: PlayerHand,
        block: Block,
        block_position: BlockPosition,
        cursor_position: (f32, f32, f32),
        block_face: BlockFace,
    ) -> Self {
        Self {
            player,
            hand,
            block,
            block_position,
            cursor_position,
            block_face,
            blocking_item_use: false,
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

    pub fn block(&self) -> Block {
        self.block
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

    pub fn is_blocking_item_use(&self) -> bool {
        self.blocking_item_use
    }

    pub fn set_blocking_item_use(&mut self, blocking_item_use: bool) {
        self.blocking_item_use = blocking_item_use;
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockFace {
    Bottom,
    Top,
    North,
    South,
    West,
    East,
}

impl BlockFace {
    pub fn from_network_direction(direction: i32) -> Option<Self> {
        match direction {
            0 => Some(Self::Bottom),
            1 => Some(Self::Top),
            2 => Some(Self::North),
            3 => Some(Self::South),
            4 => Some(Self::West),
            5 => Some(Self::East),
            _ => None,
        }
    }

    pub const fn normal(self) -> (i32, i32, i32) {
        match self {
            Self::Bottom => (0, -1, 0),
            Self::Top => (0, 1, 0),
            Self::North => (0, 0, -1),
            Self::South => (0, 0, 1),
            Self::West => (-1, 0, 0),
            Self::East => (1, 0, 0),
        }
    }

    pub const fn opposite(self) -> Self {
        match self {
            Self::Bottom => Self::Top,
            Self::Top => Self::Bottom,
            Self::North => Self::South,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::East => Self::West,
        }
    }

    pub const fn update_faces() -> [Self; 6] {
        [
            Self::Bottom,
            Self::Top,
            Self::North,
            Self::South,
            Self::West,
            Self::East,
        ]
    }
}
