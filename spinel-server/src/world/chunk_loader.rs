use crate::world::{Chunk, ChunkPosition, World};
use spinel_nbt::{NbtCompound, Taggable};
use std::io;

pub trait ChunkLoader: Send + Sync {
    fn load_chunk(&self, position: ChunkPosition) -> io::Result<Option<Chunk>>;
    fn save_chunk(&self, chunk: &Chunk) -> io::Result<()>;
    fn save_chunks(&self, chunks: &[&Chunk]) -> io::Result<()> {
        chunks.iter().try_for_each(|chunk| self.save_chunk(chunk))
    }
    fn load_world(&self, _world: &mut World) -> io::Result<()> {
        Ok(())
    }
    fn save_world(&self, world: &World) -> io::Result<()> {
        self.save_world_tags(WorldPersistentTags::from_world(world))
    }
    fn save_world_tags(&self, _world_tags: WorldPersistentTags) -> io::Result<()> {
        Ok(())
    }
    fn unload_chunk(&self, chunk: &mut Chunk) -> io::Result<()>;
    fn supports_parallel_loading(&self) -> bool {
        false
    }
    fn supports_parallel_saving(&self) -> bool {
        false
    }
    fn drain_failures(&self) -> Vec<ChunkLoaderFailure> {
        Vec::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkLoaderOperation {
    LoadWorld,
    LoadChunk,
    GenerateChunk,
    SaveWorld,
    SaveChunk,
    UnloadChunk,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkLoaderFailure {
    pub operation: ChunkLoaderOperation,
    pub chunk_position: Option<ChunkPosition>,
    pub message: String,
}

impl ChunkLoaderFailure {
    pub fn new(
        operation: ChunkLoaderOperation,
        chunk_position: Option<ChunkPosition>,
        message: String,
    ) -> Self {
        Self {
            operation,
            chunk_position,
            message,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldPersistentTags {
    compound: NbtCompound,
}

impl WorldPersistentTags {
    pub fn from_world(world: &World) -> Self {
        Self {
            compound: world.tag_handler().as_compound(),
        }
    }

    pub fn from_compound(compound: NbtCompound) -> Self {
        Self { compound }
    }

    pub fn is_empty(&self) -> bool {
        self.compound.is_empty()
    }

    pub fn into_compound(self) -> NbtCompound {
        self.compound
    }

    pub fn get_compound(&self) -> &NbtCompound {
        &self.compound
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NoopChunkLoader;

impl ChunkLoader for NoopChunkLoader {
    fn load_chunk(&self, _position: ChunkPosition) -> io::Result<Option<Chunk>> {
        Ok(None)
    }

    fn save_chunk(&self, _chunk: &Chunk) -> io::Result<()> {
        Ok(())
    }

    fn unload_chunk(&self, chunk: &mut Chunk) -> io::Result<()> {
        chunk.unload();
        Ok(())
    }
}
