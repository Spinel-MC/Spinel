use crate::world::{Block, BlockPosition, World};
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct InstanceBlockUpdateEvent {
    world: *mut World,
    block_position: BlockPosition,
    block: Block,
}

impl InstanceBlockUpdateEvent {
    pub fn new(world: *mut World, block_position: BlockPosition, block: Block) -> Self {
        Self {
            world,
            block_position,
            block,
        }
    }

    pub fn world(&mut self) -> &mut World {
        unsafe { &mut *self.world }
    }

    pub const fn block_position(&self) -> BlockPosition {
        self.block_position
    }

    pub const fn block(&self) -> Block {
        self.block
    }
}
