use crate::world::{ChunkPosition, World};
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct WorldChunkLoadEvent {
    world: *mut World,
    chunk_position: ChunkPosition,
}

impl WorldChunkLoadEvent {
    pub fn new(world: *mut World, chunk_position: ChunkPosition) -> Self {
        Self {
            world,
            chunk_position,
        }
    }

    pub fn world(&mut self) -> &mut World {
        unsafe { &mut *self.world }
    }

    pub const fn chunk_position(&self) -> ChunkPosition {
        self.chunk_position
    }

    pub const fn chunk_x(&self) -> i32 {
        self.chunk_position.x
    }

    pub const fn chunk_z(&self) -> i32 {
        self.chunk_position.z
    }
}
