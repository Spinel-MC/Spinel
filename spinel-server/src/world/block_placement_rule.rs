use crate::entity::{EntityId, EntityPosition, PlayerHand};
use crate::events::player_block_interact::BlockFace;
use crate::world::{Block, BlockPosition};
use std::collections::HashMap;
use std::sync::Arc;

pub const DEFAULT_BLOCK_UPDATE_RANGE: i32 = 10;

#[derive(Clone, Copy, Debug)]
pub struct BlockPlacementState {
    block: Block,
    block_face: Option<BlockFace>,
    block_position: BlockPosition,
    cursor_position: Option<(f32, f32, f32)>,
    player_position: Option<EntityPosition>,
    player: Option<EntityId>,
    hand: Option<PlayerHand>,
    player_is_sneaking: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct BlockUpdateState {
    block_position: BlockPosition,
    current_block: Block,
    from_face: BlockFace,
}

pub trait BlockPlacementRule: Send + Sync {
    fn block(&self) -> Block;

    fn block_place(&self, placement: BlockPlacementState) -> Option<Block>;

    fn block_update(&self, update: BlockUpdateState) -> Block {
        update.current_block()
    }

    fn max_update_distance(&self) -> i32 {
        DEFAULT_BLOCK_UPDATE_RANGE
    }
}

#[derive(Default)]
pub struct BlockPlacementRuleRegistry {
    rules: HashMap<Block, Arc<dyn BlockPlacementRule>>,
}

impl BlockPlacementRuleRegistry {
    pub fn register(&mut self, rule: impl BlockPlacementRule + 'static) {
        self.rules.insert(rule.block(), Arc::new(rule));
    }

    pub fn rule(&self, block: Block) -> Option<Arc<dyn BlockPlacementRule>> {
        self.rules.get(&block).cloned()
    }
}

impl BlockPlacementState {
    pub const fn new(
        block: Block,
        block_face: Option<BlockFace>,
        block_position: BlockPosition,
        cursor_position: Option<(f32, f32, f32)>,
        player_position: Option<EntityPosition>,
        player: Option<EntityId>,
        hand: Option<PlayerHand>,
        player_is_sneaking: bool,
    ) -> Self {
        Self {
            block,
            block_face,
            block_position,
            cursor_position,
            player_position,
            player,
            hand,
            player_is_sneaking,
        }
    }

    pub const fn block(&self) -> Block {
        self.block
    }

    pub const fn block_face(&self) -> Option<BlockFace> {
        self.block_face
    }

    pub const fn block_position(&self) -> BlockPosition {
        self.block_position
    }

    pub const fn cursor_position(&self) -> Option<(f32, f32, f32)> {
        self.cursor_position
    }

    pub const fn player_position(&self) -> Option<EntityPosition> {
        self.player_position
    }

    pub const fn player(&self) -> Option<EntityId> {
        self.player
    }

    pub const fn hand(&self) -> Option<PlayerHand> {
        self.hand
    }

    pub const fn player_is_sneaking(&self) -> bool {
        self.player_is_sneaking
    }
}

impl BlockUpdateState {
    pub const fn new(
        block_position: BlockPosition,
        current_block: Block,
        from_face: BlockFace,
    ) -> Self {
        Self {
            block_position,
            current_block,
            from_face,
        }
    }

    pub const fn block_position(&self) -> BlockPosition {
        self.block_position
    }

    pub const fn current_block(&self) -> Block {
        self.current_block
    }

    pub const fn from_face(&self) -> BlockFace {
        self.from_face
    }
}
