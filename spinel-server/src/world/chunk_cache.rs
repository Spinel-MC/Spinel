use crate::world::{Block, BlockPosition, BlockState, Chunk, ChunkPosition, World};

pub struct ChunkCache<'world> {
    world: &'world World,
    chunk: Option<&'world Chunk>,
    default_block_state: BlockState,
}

impl<'world> ChunkCache<'world> {
    pub const fn new(world: &'world World, chunk: Option<&'world Chunk>) -> Self {
        Self::with_default_block(world, chunk, Block::AIR)
    }

    pub const fn with_default_block(
        world: &'world World,
        chunk: Option<&'world Chunk>,
        default_block: Block,
    ) -> Self {
        Self {
            world,
            chunk,
            default_block_state: default_block.default_state(),
        }
    }

    pub fn block(&mut self, position: BlockPosition) -> Block {
        self.block_state(position).block()
    }

    pub fn block_state(&mut self, position: BlockPosition) -> BlockState {
        let position_chunk = ChunkPosition::from(position);
        let cached_chunk_matches = self.chunk.is_some_and(|chunk| {
            chunk.is_loaded() && chunk.x() == position_chunk.x && chunk.z() == position_chunk.z
        });
        if !cached_chunk_matches {
            self.chunk = self.world.chunk(position_chunk);
        }
        self.chunk
            .filter(|chunk| chunk.is_loaded())
            .map(|chunk| chunk.block_state(position))
            .unwrap_or(self.default_block_state)
    }

    pub fn chunk(&self) -> Option<&'world Chunk> {
        self.chunk
    }
}
