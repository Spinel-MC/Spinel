use crate::world::{Biome, Block};
use spinel_network::types::chunk::{ChunkSection as NetworkChunkSection, PalettedContainer};

const CHUNK_SECTION_BLOCK_COUNT: usize = 4096;
const CHUNK_SECTION_BIOME_COUNT: usize = 64;

#[derive(Clone)]
pub struct ChunkSection {
    pub y: i32,
    blocks: Vec<Block>,
    biomes: Vec<Biome>,
}

impl ChunkSection {
    pub fn new(y: i32) -> Self {
        Self {
            y,
            blocks: vec![Block::AIR; CHUNK_SECTION_BLOCK_COUNT],
            biomes: vec![Biome::plains(); CHUNK_SECTION_BIOME_COUNT],
        }
    }

    pub fn to_network_section(&self) -> NetworkChunkSection {
        NetworkChunkSection {
            block_count: self.block_count(),
            block_states: self.block_palette(),
            biomes: self.biome_palette(),
        }
    }

    pub(crate) fn set_block(&mut self, x: i32, y: i32, z: i32, block: Block) {
        if let Some(block_slot) = self.blocks.get_mut(Self::block_index(x, y, z)) {
            *block_slot = block;
        }
    }

    pub(crate) fn set_biome(&mut self, biome: Biome) {
        self.biomes.fill(biome);
    }

    fn block_count(&self) -> i16 {
        self.blocks
            .iter()
            .filter(|block| block.state_id() != Block::AIR.state_id())
            .count() as i16
    }

    fn block_palette(&self) -> PalettedContainer {
        let palette = self.block_state_palette();
        if palette.len() == 1 {
            return Self::single_palette(palette[0]);
        }
        let bits_per_entry = Self::bits_per_entry(palette.len());
        PalettedContainer {
            bits_per_entry,
            palette: Some(palette.clone()),
            data: Self::pack_block_states(&self.blocks, &palette, bits_per_entry),
        }
    }

    fn biome_palette(&self) -> PalettedContainer {
        let biome = match self.biomes.first().copied() {
            Some(biome) => biome,
            None => Biome::plains(),
        };
        Self::single_palette(biome.registry_id)
    }

    fn single_palette(palette_id: i32) -> PalettedContainer {
        PalettedContainer {
            bits_per_entry: 0,
            palette: Some(vec![palette_id]),
            data: Vec::new(),
        }
    }

    fn block_state_palette(&self) -> Vec<i32> {
        self.blocks.iter().fold(Vec::new(), |mut palette, block| {
            if !palette.contains(&block.state_id()) {
                palette.push(block.state_id());
            }
            palette
        })
    }

    fn bits_per_entry(palette_len: usize) -> u8 {
        let palette_index_limit = palette_len.saturating_sub(1);
        let required_bits = usize::BITS - palette_index_limit.leading_zeros();
        required_bits.max(4) as u8
    }

    fn pack_block_states(blocks: &[Block], palette: &[i32], bits_per_entry: u8) -> Vec<u64> {
        let entries_per_long = 64 / bits_per_entry as usize;
        let storage_len = blocks.len().div_ceil(entries_per_long);
        let mut data = vec![0; storage_len];
        blocks.iter().enumerate().for_each(|(block_index, block)| {
            let Some(palette_index) = palette
                .iter()
                .position(|state_id| *state_id == block.state_id())
            else {
                return;
            };
            Self::pack_palette_index(
                &mut data,
                block_index,
                palette_index as u64,
                bits_per_entry,
                entries_per_long,
            );
        });
        data
    }

    fn pack_palette_index(
        data: &mut [u64],
        block_index: usize,
        palette_index: u64,
        bits_per_entry: u8,
        entries_per_long: usize,
    ) {
        let data_index = block_index / entries_per_long;
        let bit_offset = (block_index % entries_per_long) * bits_per_entry as usize;
        data[data_index] |= palette_index << bit_offset;
    }

    fn block_index(x: i32, y: i32, z: i32) -> usize {
        ((y as usize) << 8) | ((z as usize) << 4) | x as usize
    }
}
