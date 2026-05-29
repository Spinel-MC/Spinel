use crate::world::{Chunk, ChunkPosition};
use std::io;

pub trait ChunkLoader: Send + Sync {
    fn load_chunk(&self, position: ChunkPosition) -> io::Result<Option<Chunk>>;
    fn save_chunk(&self, chunk: &Chunk) -> io::Result<()>;
    fn save_chunks(&self, chunks: &[&Chunk]) -> io::Result<()> {
        chunks.iter().try_for_each(|chunk| self.save_chunk(chunk))
    }
    fn load_instance(&self) -> io::Result<()> {
        Ok(())
    }
    fn save_instance(&self) -> io::Result<()> {
        Ok(())
    }
    fn unload_chunk(&self, chunk: &mut Chunk) -> io::Result<()>;
    fn supports_parallel_loading(&self) -> bool {
        false
    }
    fn supports_parallel_saving(&self) -> bool {
        false
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
