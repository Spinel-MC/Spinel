use crate::entity::player::PlayerChunk;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChunkPosition {
    pub x: i32,
    pub z: i32,
}

impl ChunkPosition {
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }
}

impl From<PlayerChunk> for ChunkPosition {
    fn from(chunk: PlayerChunk) -> Self {
        Self {
            x: chunk.x,
            z: chunk.z,
        }
    }
}
