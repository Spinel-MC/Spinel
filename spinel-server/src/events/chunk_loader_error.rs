use crate::world::{ChunkLoaderOperation, ChunkPosition, World};
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct ChunkLoaderErrorEvent {
    world: *mut World,
    pub operation: ChunkLoaderOperation,
    pub chunk_position: Option<ChunkPosition>,
    pub message: String,
}

impl ChunkLoaderErrorEvent {
    pub fn new(
        world: *mut World,
        operation: ChunkLoaderOperation,
        chunk_position: Option<ChunkPosition>,
        message: String,
    ) -> Self {
        Self {
            world,
            operation,
            chunk_position,
            message,
        }
    }

    pub fn world(&mut self) -> &mut World {
        unsafe { &mut *self.world }
    }
}
