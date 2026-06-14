use crate::entity::{EntityPosition, player::PlayerChunk};
use crate::world::{BlockPosition, Chunk};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChunkPosition {
    pub x: i32,
    pub z: i32,
}

impl ChunkPosition {
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    pub const fn from_index(index: i64) -> Self {
        Self {
            x: index as i32,
            z: (index >> 32) as i32,
        }
    }

    pub const fn index(self) -> i64 {
        (self.x as i64 & 0xffff_ffff) | ((self.z as i64 & 0xffff_ffff) << 32)
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

impl From<BlockPosition> for ChunkPosition {
    fn from(position: BlockPosition) -> Self {
        Self {
            x: position.x.div_euclid(16),
            z: position.z.div_euclid(16),
        }
    }
}

impl From<EntityPosition> for ChunkPosition {
    fn from(position: EntityPosition) -> Self {
        Self {
            x: (position.x().floor() as i32).div_euclid(16),
            z: (position.z().floor() as i32).div_euclid(16),
        }
    }
}

impl From<&Chunk> for ChunkPosition {
    fn from(chunk: &Chunk) -> Self {
        Self {
            x: chunk.x(),
            z: chunk.z(),
        }
    }
}
