use crate::world::{Block, BlockPosition, ChunkSection};
use spinel_network::types::chunk::HeightmapEntry;

const CHUNK_COLUMN_COUNT: usize = 256;
const HEIGHTMAP_BITS_PER_ENTRY: usize = 9;
const HEIGHTMAP_LONG_COUNT: usize = 37;
const WORLD_MIN_HEIGHTMAP_Y: i32 = -65;

#[derive(Clone)]
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

    pub(crate) fn refresh_from_sections(&mut self, sections: &[ChunkSection]) {
        *self = Self::from_sections(sections);
    }

    pub(crate) fn refresh_block(
        &mut self,
        sections: &[ChunkSection],
        position: BlockPosition,
        block: Block,
    ) {
        let local_x = position.x.rem_euclid(16);
        let local_z = position.z.rem_euclid(16);
        let current_height = self.height_at(local_x, local_z);
        if !Self::block_is_air(block) && current_height < position.y {
            self.set_height(local_x, local_z, position.y);
            return;
        }
        if Self::block_is_air(block) && current_height == position.y {
            let new_height = Self::column_height(sections, local_x, local_z);
            self.set_packed_height(local_x, local_z, new_height);
        }
    }

    pub(crate) fn load_motion_blocking_from_longs(&mut self, heights: &[i64]) {
        self.load_from_longs(heights);
    }

    pub(crate) fn load_world_surface_from_longs(&mut self, heights: &[i64]) {
        self.load_from_longs(heights);
    }

    pub(crate) fn entries(
        motion_blocking_heightmap: &Self,
        world_surface_heightmap: &Self,
    ) -> Vec<HeightmapEntry> {
        vec![
            HeightmapEntry {
                kind: 4,
                data: motion_blocking_heightmap.packed_heights(),
            },
            HeightmapEntry {
                kind: 5,
                data: world_surface_heightmap.packed_heights(),
            },
        ]
    }

    pub(crate) fn packed_heights(&self) -> Vec<i64> {
        self.encode_heights()
    }

    fn load_from_longs(&mut self, heights: &[i64]) {
        self.heights =
            std::array::from_fn(|height_index| Self::unpack_height(heights, height_index));
    }

    fn height_at(&self, local_x: i32, local_z: i32) -> i32 {
        self.heights[Self::column_index(local_x, local_z)] + WORLD_MIN_HEIGHTMAP_Y
    }

    fn set_height(&mut self, local_x: i32, local_z: i32, height: i32) {
        self.set_packed_height(local_x, local_z, height - WORLD_MIN_HEIGHTMAP_Y);
    }

    fn set_packed_height(&mut self, local_x: i32, local_z: i32, packed_height: i32) {
        self.heights[Self::column_index(local_x, local_z)] = packed_height;
    }

    fn column_index(local_x: i32, local_z: i32) -> usize {
        ((local_z as usize) << 4) | local_x as usize
    }

    fn column_height(sections: &[ChunkSection], local_x: i32, local_z: i32) -> i32 {
        sections
            .iter()
            .rev()
            .find_map(|section| Self::section_column_height(section, local_x, local_z))
            .unwrap_or(0)
    }

    fn section_column_height(section: &ChunkSection, local_x: i32, local_z: i32) -> Option<i32> {
        if section.is_empty() {
            return None;
        }
        if section.is_filled_with_blocks() {
            return Some(section.y * 16 + 15 - WORLD_MIN_HEIGHTMAP_Y);
        }
        (0..16).rev().find_map(|local_y| {
            let block = section
                .block(local_x, local_y, local_z)
                .unwrap_or(Block::AIR);
            if Self::block_is_air(block) {
                return None;
            }
            Some(section.y * 16 + local_y - WORLD_MIN_HEIGHTMAP_Y)
        })
    }

    fn encode_heights(&self) -> Vec<i64> {
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

    fn unpack_height(packed_heights: &[i64], height_index: usize) -> i32 {
        let bit_index = height_index * HEIGHTMAP_BITS_PER_ENTRY;
        let packed_index = bit_index / 64;
        let bit_offset = bit_index % 64;
        let Some(height_bits) = packed_heights.get(packed_index).copied() else {
            return 0;
        };
        let mut height = (height_bits as u64) >> bit_offset;
        if bit_offset > 64 - HEIGHTMAP_BITS_PER_ENTRY {
            let next_height_bits = packed_heights.get(packed_index + 1).copied().unwrap_or(0);
            height |= (next_height_bits as u64) << (64 - bit_offset);
        }
        (height & ((1 << HEIGHTMAP_BITS_PER_ENTRY) - 1)) as i32
    }

    fn block_is_air(block: Block) -> bool {
        block.state_id() == Block::AIR.state_id()
    }
}
