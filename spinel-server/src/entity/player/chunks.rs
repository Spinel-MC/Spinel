use crate::entity::player::position::PlayerPosition;
use spinel_network::types::ChunkPos;
use std::collections::HashSet;

const PLAYER_CHUNK_VIEW_DISTANCE: i32 = 8;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct PlayerChunk {
    pub x: i32,
    pub z: i32,
}

#[derive(Clone)]
pub(crate) struct PlayerChunkTransition {
    pub next: PlayerChunk,
    pub arriving: Vec<PlayerChunk>,
    pub departing: Vec<PlayerChunk>,
}

impl PlayerChunk {
    pub(crate) const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    pub(crate) fn from_position(position: PlayerPosition) -> Self {
        Self {
            x: position.x.floor() as i32 >> 4,
            z: position.z.floor() as i32 >> 4,
        }
    }

    pub(crate) fn packet_position(self) -> ChunkPos {
        ChunkPos {
            x: self.x,
            z: self.z,
        }
    }

    pub(crate) fn surrounding(self) -> Vec<Self> {
        (-PLAYER_CHUNK_VIEW_DISTANCE..=PLAYER_CHUNK_VIEW_DISTANCE)
            .flat_map(|chunk_x_offset| {
                (-PLAYER_CHUNK_VIEW_DISTANCE..=PLAYER_CHUNK_VIEW_DISTANCE).map(
                    move |chunk_z_offset| {
                        Self::new(self.x + chunk_x_offset, self.z + chunk_z_offset)
                    },
                )
            })
            .collect()
    }

    pub(crate) fn arriving_chunks(self, previous_chunk: Self) -> Vec<Self> {
        let previous_chunks = previous_chunk.chunk_set();
        self.surrounding()
            .into_iter()
            .filter(|chunk| previous_chunks.get(chunk).is_none())
            .collect()
    }

    pub(crate) fn departing_chunks(self, next_chunk: Self) -> Vec<Self> {
        let next_chunks = next_chunk.chunk_set();
        self.surrounding()
            .into_iter()
            .filter(|chunk| next_chunks.get(chunk).is_none())
            .collect()
    }

    pub(crate) fn transition_to(self, next: Self) -> Option<PlayerChunkTransition> {
        if self == next {
            return None;
        }
        Some(PlayerChunkTransition {
            next,
            arriving: next.arriving_chunks(self),
            departing: self.departing_chunks(next),
        })
    }

    fn chunk_set(self) -> HashSet<Self> {
        self.surrounding().into_iter().collect()
    }
}
