use crate::entity::{EntityId, EntityPosition, PlayerHand};
use crate::events::player_block_interact::BlockFace;
use crate::world::{Block, BlockPosition};
use spinel_nbt::{Nbt, Tag};
use spinel_network::types::Identifier;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct BlockHandlerPlacement {
    block: Block,
    previous_block: Block,
    world: uuid::Uuid,
    block_position: BlockPosition,
    player: Option<EntityId>,
    hand: Option<PlayerHand>,
    block_face: Option<BlockFace>,
    cursor_position: Option<(f32, f32, f32)>,
}

#[derive(Clone, Copy, Debug)]
pub struct BlockHandlerDestroy {
    block: Block,
    new_block: Block,
    world: uuid::Uuid,
    block_position: BlockPosition,
    player: Option<EntityId>,
}

#[derive(Clone, Copy, Debug)]
pub struct BlockHandlerTick {
    block: Block,
    world: uuid::Uuid,
    block_position: BlockPosition,
}

#[derive(Clone, Copy, Debug)]
pub struct BlockHandlerTouch {
    block: Block,
    world: uuid::Uuid,
    block_position: BlockPosition,
    entity: EntityId,
}

#[derive(Clone, Copy, Debug)]
pub struct BlockHandlerInteraction {
    block: Block,
    world: uuid::Uuid,
    block_face: BlockFace,
    block_position: BlockPosition,
    cursor_position: EntityPosition,
    player: EntityId,
    hand: PlayerHand,
}

pub trait BlockHandler: Send + Sync {
    fn key(&self) -> &Identifier;

    fn on_place(&self, _placement: BlockHandlerPlacement) {}

    fn on_destroy(&self, _destroy: BlockHandlerDestroy) {}

    fn on_interact(&self, _interaction: BlockHandlerInteraction) -> bool {
        true
    }

    fn on_touch(&self, _touch: BlockHandlerTouch) {}

    fn tick(&self, _tick: BlockHandlerTick) {}

    fn is_tickable(&self) -> bool {
        false
    }

    fn block_entity_tags(&self) -> Vec<Tag<Nbt>> {
        Vec::new()
    }

    fn block_entity_action(&self) -> i8 {
        -1
    }
}

#[derive(Default)]
pub struct BlockHandlerRegistry {
    handlers: HashMap<Block, Arc<dyn BlockHandler>>,
}

impl BlockHandlerRegistry {
    pub fn register(&mut self, block: Block, handler: impl BlockHandler + 'static) {
        self.handlers.insert(block, Arc::new(handler));
    }

    pub fn handler(&self, block: Block) -> Option<Arc<dyn BlockHandler>> {
        self.handlers.get(&block).cloned()
    }

    pub fn has_tickable_handler(&self, block: Block) -> bool {
        self.handlers
            .get(&block)
            .is_some_and(|handler| handler.is_tickable())
    }
}

impl BlockHandlerPlacement {
    pub fn new(
        block: Block,
        previous_block: Block,
        world: uuid::Uuid,
        block_position: BlockPosition,
    ) -> Self {
        Self {
            block,
            previous_block,
            world,
            block_position,
            player: None,
            hand: None,
            block_face: None,
            cursor_position: None,
        }
    }

    pub fn player_placement(
        mut self,
        player: EntityId,
        hand: PlayerHand,
        block_face: BlockFace,
        cursor_position: (f32, f32, f32),
    ) -> Self {
        self.player = Some(player);
        self.hand = Some(hand);
        self.block_face = Some(block_face);
        self.cursor_position = Some(cursor_position);
        self
    }

    pub const fn block(&self) -> Block {
        self.block
    }

    pub const fn previous_block(&self) -> Block {
        self.previous_block
    }

    pub const fn world(&self) -> uuid::Uuid {
        self.world
    }

    pub const fn block_position(&self) -> BlockPosition {
        self.block_position
    }

    pub const fn player(&self) -> Option<EntityId> {
        self.player
    }

    pub const fn hand(&self) -> Option<PlayerHand> {
        self.hand
    }

    pub const fn block_face(&self) -> Option<BlockFace> {
        self.block_face
    }

    pub const fn cursor_position(&self) -> Option<(f32, f32, f32)> {
        self.cursor_position
    }
}

impl BlockHandlerDestroy {
    pub const fn new(
        block: Block,
        new_block: Block,
        world: uuid::Uuid,
        block_position: BlockPosition,
        player: Option<EntityId>,
    ) -> Self {
        Self {
            block,
            new_block,
            world,
            block_position,
            player,
        }
    }

    pub const fn block(&self) -> Block {
        self.block
    }

    pub const fn new_block(&self) -> Block {
        self.new_block
    }

    pub const fn world(&self) -> uuid::Uuid {
        self.world
    }

    pub const fn block_position(&self) -> BlockPosition {
        self.block_position
    }

    pub const fn player(&self) -> Option<EntityId> {
        self.player
    }
}

impl BlockHandlerTick {
    pub const fn new(block: Block, world: uuid::Uuid, block_position: BlockPosition) -> Self {
        Self {
            block,
            world,
            block_position,
        }
    }

    pub const fn block(&self) -> Block {
        self.block
    }

    pub const fn world(&self) -> uuid::Uuid {
        self.world
    }

    pub const fn block_position(&self) -> BlockPosition {
        self.block_position
    }
}

impl BlockHandlerTouch {
    pub const fn new(
        block: Block,
        world: uuid::Uuid,
        block_position: BlockPosition,
        entity: EntityId,
    ) -> Self {
        Self {
            block,
            world,
            block_position,
            entity,
        }
    }

    pub const fn block(&self) -> Block {
        self.block
    }

    pub const fn world(&self) -> uuid::Uuid {
        self.world
    }

    pub const fn block_position(&self) -> BlockPosition {
        self.block_position
    }

    pub const fn entity(&self) -> EntityId {
        self.entity
    }
}

impl BlockHandlerInteraction {
    pub const fn new(
        block: Block,
        world: uuid::Uuid,
        block_face: BlockFace,
        block_position: BlockPosition,
        cursor_position: EntityPosition,
        player: EntityId,
        hand: PlayerHand,
    ) -> Self {
        Self {
            block,
            world,
            block_face,
            block_position,
            cursor_position,
            player,
            hand,
        }
    }

    pub const fn block(&self) -> Block {
        self.block
    }

    pub const fn world(&self) -> uuid::Uuid {
        self.world
    }

    pub const fn block_face(&self) -> BlockFace {
        self.block_face
    }

    pub const fn block_position(&self) -> BlockPosition {
        self.block_position
    }

    pub const fn cursor_position(&self) -> EntityPosition {
        self.cursor_position
    }

    pub const fn player(&self) -> EntityId {
        self.player
    }

    pub const fn hand(&self) -> PlayerHand {
        self.hand
    }
}
