use crate::entity::player::position::PlayerPosition;
use spinel_network::types::ChunkPos;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

    pub(crate) fn surrounding(self, view_distance: i32) -> Vec<Self> {
        (-view_distance..=view_distance)
            .flat_map(|chunk_x_offset| {
                (-view_distance..=view_distance).map(move |chunk_z_offset| {
                    Self::new(self.x + chunk_x_offset, self.z + chunk_z_offset)
                })
            })
            .collect()
    }

    pub(crate) fn arriving_chunks(self, previous_chunk: Self, view_distance: i32) -> Vec<Self> {
        let previous_chunks = previous_chunk.chunk_set(view_distance);
        self.surrounding(view_distance)
            .into_iter()
            .filter(|chunk| previous_chunks.get(chunk).is_none())
            .collect()
    }

    pub(crate) fn departing_chunks(self, next_chunk: Self, view_distance: i32) -> Vec<Self> {
        let next_chunks = next_chunk.chunk_set(view_distance);
        self.surrounding(view_distance)
            .into_iter()
            .filter(|chunk| next_chunks.get(chunk).is_none())
            .collect()
    }

    pub(crate) fn transition_to(
        self,
        next: Self,
        view_distance: i32,
    ) -> Option<PlayerChunkTransition> {
        if self == next {
            return None;
        }
        Some(PlayerChunkTransition {
            next,
            arriving: next.arriving_chunks(self, view_distance),
            departing: self.departing_chunks(next, view_distance),
        })
    }

    pub(crate) fn distance_to(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.z - other.z).abs()
    }

    pub(crate) fn is_within_view_distance(self, center: Self, view_distance: i32) -> bool {
        (self.x - center.x).abs() <= view_distance && (self.z - center.z).abs() <= view_distance
    }

    fn chunk_set(self, view_distance: i32) -> HashSet<Self> {
        self.surrounding(view_distance).into_iter().collect()
    }
}
