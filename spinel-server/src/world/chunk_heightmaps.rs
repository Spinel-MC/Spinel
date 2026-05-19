use crate::world::{Block, ChunkSection};
use spinel_network::types::chunk::HeightmapEntry;

const CHUNK_COLUMN_COUNT: usize = 256;
const HEIGHTMAP_BITS_PER_ENTRY: usize = 9;
const HEIGHTMAP_LONG_COUNT: usize = 37;
const WORLD_MIN_BLOCK_Y: i32 = -64;

pub(crate) struct ChunkHeightmaps {
    heights: [i32; CHUNK_COLUMN_COUNT],
}

impl ChunkHeightmaps {
    pub(crate) fn from_sections(sections: &[ChunkSection]) -> Self {
        let heights = std::array::from_fn(|column_index| {
            let local_x = (column_index & 15) as i32;
            let local_z = ((column_index >> 4) & 15) as i32;
            Self::column_height(sections, local_x, local_z)
        });
        Self { heights }
    }

    pub(crate) fn entries(&self) -> Vec<HeightmapEntry> {
        let packed_heights = self.packed_heights();
        [1, 4, 5]
            .into_iter()
            .map(|kind| HeightmapEntry {
                kind,
                data: packed_heights.clone(),
            })
            .collect()
    }

    fn column_height(sections: &[ChunkSection], local_x: i32, local_z: i32) -> i32 {
        sections
            .iter()
            .rev()
            .find_map(|section| Self::section_column_height(section, local_x, local_z))
            .unwrap_or(0)
    }

    fn section_column_height(section: &ChunkSection, local_x: i32, local_z: i32) -> Option<i32> {
        (0..16).rev().find_map(|local_y| {
            let block_is_air = section
                .block(local_x, local_y, local_z)
                .map(|block| block.state_id() == Block::AIR.state_id())
                .unwrap_or(true);
            if block_is_air {
                return None;
            }
            Some(section.y * 16 + local_y + 1 - WORLD_MIN_BLOCK_Y)
        })
    }

    fn packed_heights(&self) -> Vec<i64> {
        let mut packed_heights = vec![0u64; HEIGHTMAP_LONG_COUNT];
        self.heights
            .iter()
            .enumerate()
            .for_each(|(height_index, height)| {
                Self::pack_height(&mut packed_heights, height_index, *height as u64);
            });
        packed_heights
            .into_iter()
            .map(|height| height as i64)
            .collect()
    }

    fn pack_height(packed_heights: &mut [u64], height_index: usize, height: u64) {
        let bit_index = height_index * HEIGHTMAP_BITS_PER_ENTRY;
        let packed_index = bit_index / 64;
        let bit_offset = bit_index % 64;
        packed_heights[packed_index] |= height << bit_offset;
        if bit_offset <= 64 - HEIGHTMAP_BITS_PER_ENTRY {
            return;
        }
        packed_heights[packed_index + 1] |= height >> (64 - bit_offset);
    }
}
