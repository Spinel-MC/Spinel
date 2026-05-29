use crate::world::section_palette::{SectionPalette, SectionPaletteError};
use crate::world::{Biome, Block};
use spinel_network::types::chunk::ChunkSection as NetworkChunkSection;
use spinel_registry::{Registries, RegistryKey};
use std::collections::HashMap;

const CHUNK_SECTION_BLOCK_COUNT: usize = 4096;
const CHUNK_SECTION_BIOME_COUNT: usize = 64;
const CHUNK_SECTION_LIGHT_BYTES: usize = 2048;

type BlockPalette = SectionPalette<Block, CHUNK_SECTION_BLOCK_COUNT>;
type BiomePalette = SectionPalette<RegistryKey<Biome>, CHUNK_SECTION_BIOME_COUNT>;

#[derive(Clone)]
enum SectionBlockWrites {
    Complete,
    Sparse(Vec<SectionBlockWrite>),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SectionBlockWrite {
    Untouched,
    Written,
}

#[derive(Clone)]
pub struct ChunkSection {
    pub y: i32,
    blocks: BlockPalette,
    biomes: BiomePalette,
    block_writes: SectionBlockWrites,
    sky_light: Vec<u8>,
    block_light: Vec<u8>,
    sky_light_invalidated: bool,
    block_light_invalidated: bool,
    special_blocks: HashMap<usize, Block>,
    non_air_block_count: i16,
}

impl ChunkSection {
    pub fn new(y: i32) -> Self {
        Self::with_block_writes(y, SectionBlockWrites::Complete)
    }

    pub(crate) fn new_sparse(y: i32) -> Self {
        Self::with_block_writes(
            y,
            SectionBlockWrites::Sparse(vec![
                SectionBlockWrite::Untouched;
                CHUNK_SECTION_BLOCK_COUNT
            ]),
        )
    }

    pub fn to_network_section(
        &self,
        registries: &Registries,
    ) -> Result<NetworkChunkSection, SectionPaletteError> {
        Ok(NetworkChunkSection {
            block_count: self.non_air_block_count,
            block_states: self.blocks.to_network(4, 15),
            biomes: self
                .biomes
                .try_to_network(1, 8, |biome| registries.biome().get_id(biome))?,
        })
    }

    pub(crate) fn block(&self, x: i32, y: i32, z: i32) -> Option<Block> {
        Self::block_index(x, y, z).and_then(|block_index| self.blocks.get(block_index))
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.non_air_block_count == 0
    }

    pub(crate) fn is_filled_with_blocks(&self) -> bool {
        self.non_air_block_count == CHUNK_SECTION_BLOCK_COUNT as i16
    }

    pub(crate) fn biome(&self, x: i32, y: i32, z: i32) -> Option<RegistryKey<Biome>> {
        Self::biome_index(x, y, z).and_then(|biome_index| self.biomes.get(biome_index))
    }

    pub(crate) fn set_block(&mut self, x: i32, y: i32, z: i32, block: Block) -> bool {
        let Some(block_index) = Self::block_index(x, y, z) else {
            return false;
        };
        let previous_block = self.blocks.get(block_index).unwrap_or(Block::AIR);
        self.block_writes.record(block_index);
        if previous_block == block {
            return true;
        }
        self.special_blocks.remove(&block_index);
        let block_was_set = self.blocks.set(block_index, block);
        if Self::block_can_own_block_entity(block) {
            self.cache_special_block(x, y, z, block);
        }
        self.update_non_air_block_count(previous_block, block);
        block_was_set
    }

    pub(crate) fn set_biome(&mut self, x: i32, y: i32, z: i32, biome: RegistryKey<Biome>) -> bool {
        let Some(biome_index) = Self::biome_index(x, y, z) else {
            return false;
        };
        self.biomes.set(biome_index, biome)
    }

    pub(crate) fn fill_biome(&mut self, biome: RegistryKey<Biome>) {
        self.biomes.fill(biome);
    }

    pub(crate) fn fill_block(&mut self, block: Block) {
        self.blocks.fill(block);
        self.block_writes.record_all();
        self.special_blocks.clear();
        if Self::block_can_own_block_entity(block) {
            (0..CHUNK_SECTION_BLOCK_COUNT).for_each(|block_index| {
                self.special_blocks.insert(block_index, block);
            });
        }
        self.non_air_block_count = if Self::block_is_air(block) {
            0
        } else {
            CHUNK_SECTION_BLOCK_COUNT as i16
        };
    }

    pub(crate) fn merge_from_fork(&mut self, fork_section: &ChunkSection) {
        fork_section
            .written_block_indices()
            .into_iter()
            .for_each(|block_index| self.merge_fork_block(fork_section, block_index));
    }

    pub(crate) fn cache_special_block(&mut self, x: i32, y: i32, z: i32, block: Block) -> bool {
        let Some(block_index) = Self::block_index(x, y, z) else {
            return false;
        };
        self.special_blocks.insert(block_index, block);
        true
    }

    pub fn special_blocks(&self) -> &HashMap<usize, Block> {
        &self.special_blocks
    }

    pub(crate) fn sky_light(&self) -> &[u8] {
        &self.sky_light
    }

    pub(crate) fn block_light(&self) -> &[u8] {
        &self.block_light
    }

    pub(crate) fn sky_light_level(&self, x: i32, y: i32, z: i32) -> u8 {
        Self::light_level(&self.sky_light, x, y, z)
    }

    pub(crate) fn block_light_level(&self, x: i32, y: i32, z: i32) -> u8 {
        Self::light_level(&self.block_light, x, y, z)
    }

    pub(crate) fn invalidate_sky_light(&mut self) {
        self.sky_light_invalidated = true;
    }

    pub(crate) fn invalidate_block_light(&mut self) {
        self.block_light_invalidated = true;
    }

    pub fn sky_light_is_invalidated(&self) -> bool {
        self.sky_light_invalidated
    }

    pub fn block_light_is_invalidated(&self) -> bool {
        self.block_light_invalidated
    }

    fn with_block_writes(y: i32, block_writes: SectionBlockWrites) -> Self {
        Self {
            y,
            blocks: BlockPalette::new(Block::AIR),
            biomes: BiomePalette::new(Biome::PLAINS),
            block_writes,
            sky_light: vec![255; CHUNK_SECTION_LIGHT_BYTES],
            block_light: vec![0; CHUNK_SECTION_LIGHT_BYTES],
            sky_light_invalidated: false,
            block_light_invalidated: false,
            special_blocks: HashMap::new(),
            non_air_block_count: 0,
        }
    }

    fn update_non_air_block_count(&mut self, previous_block: Block, next_block: Block) {
        let previous_block_is_air = Self::block_is_air(previous_block);
        let next_block_is_air = Self::block_is_air(next_block);
        if previous_block_is_air && !next_block_is_air {
            self.non_air_block_count += 1;
            return;
        }
        if !previous_block_is_air && next_block_is_air {
            self.non_air_block_count -= 1;
        }
    }

    fn merge_fork_block(&mut self, fork_section: &ChunkSection, block_index: usize) {
        let Some(block) = fork_section.blocks.get(block_index) else {
            return;
        };
        let (x, y, z) = Self::block_coordinates(block_index);
        self.set_block(x, y, z, block);
        if let Some(special_block) = fork_section.special_blocks.get(&block_index).copied() {
            self.special_blocks.insert(block_index, special_block);
        }
    }

    fn written_block_indices(&self) -> Vec<usize> {
        match &self.block_writes {
            SectionBlockWrites::Complete => (0..CHUNK_SECTION_BLOCK_COUNT).collect(),
            SectionBlockWrites::Sparse(block_writes) => block_writes
                .iter()
                .enumerate()
                .filter_map(|(block_index, block_was_written)| {
                    (*block_was_written == SectionBlockWrite::Written).then_some(block_index)
                })
                .collect(),
        }
    }

    fn block_is_air(block: Block) -> bool {
        block.state_id() == Block::AIR.state_id()
    }

    pub(crate) fn block_can_own_block_entity(block: Block) -> bool {
        let block_is_common_non_special = matches!(
            block,
            Block::AIR
                | Block::CAVE_AIR
                | Block::VOID_AIR
                | Block::STONE
                | Block::BEDROCK
                | Block::GRASS_BLOCK
                | Block::DIRT
                | Block::DEEPSLATE
                | Block::DARK_PRISMARINE
                | Block::WATER
                | Block::LAVA
        );
        if block_is_common_non_special {
            return false;
        }
        let name = format!("{block:?}").to_ascii_lowercase();
        let name = name.as_str();
        matches!(
            name,
            "barrel"
                | "beacon"
                | "beehive"
                | "bee_nest"
                | "bell"
                | "blast_furnace"
                | "brewing_stand"
                | "calibrated_sculk_sensor"
                | "campfire"
                | "chest"
                | "chiseled_bookshelf"
                | "comparator"
                | "conduit"
                | "crafter"
                | "decorated_pot"
                | "dispenser"
                | "dropper"
                | "enchanting_table"
                | "ender_chest"
                | "end_gateway"
                | "end_portal"
                | "furnace"
                | "hopper"
                | "jigsaw"
                | "jukebox"
                | "lectern"
                | "mob_spawner"
                | "piston"
                | "sculk_catalyst"
                | "sculk_sensor"
                | "sculk_shrieker"
                | "smoker"
                | "structure_block"
                | "trapped_chest"
        ) || name.ends_with("_bed")
            || name.ends_with("_banner")
            || name.ends_with("_wall_banner")
            || name.ends_with("_sign")
            || name.ends_with("_wall_sign")
            || name.ends_with("_hanging_sign")
            || name.ends_with("_wall_hanging_sign")
            || name.ends_with("_shulker_box")
            || name.ends_with("_head")
            || name.ends_with("_wall_head")
            || name.ends_with("_skull")
            || name.ends_with("_wall_skull")
    }

    fn block_index(x: i32, y: i32, z: i32) -> Option<usize> {
        let local_coordinates_are_valid =
            (0..16).contains(&x) && (0..16).contains(&y) && (0..16).contains(&z);
        local_coordinates_are_valid
            .then_some(((y as usize) << 8) | ((z as usize) << 4) | x as usize)
    }

    fn block_coordinates(block_index: usize) -> (i32, i32, i32) {
        let x = (block_index & 15) as i32;
        let z = ((block_index >> 4) & 15) as i32;
        let y = ((block_index >> 8) & 15) as i32;
        (x, y, z)
    }

    fn light_level(light: &[u8], x: i32, y: i32, z: i32) -> u8 {
        let Some(block_index) = Self::block_index(x, y, z) else {
            return 0;
        };
        let light_byte = light.get(block_index / 2).copied().unwrap_or_default();
        if block_index % 2 == 0 {
            return light_byte & 15;
        }
        light_byte >> 4
    }

    fn biome_index(x: i32, y: i32, z: i32) -> Option<usize> {
        let local_coordinates_are_valid =
            (0..4).contains(&x) && (0..4).contains(&y) && (0..4).contains(&z);
        local_coordinates_are_valid
            .then_some(((y as usize) << 4) | ((z as usize) << 2) | x as usize)
    }
}

impl SectionBlockWrites {
    fn record(&mut self, block_index: usize) {
        let Self::Sparse(block_writes) = self else {
            return;
        };
        if let Some(block_was_written) = block_writes.get_mut(block_index) {
            *block_was_written = SectionBlockWrite::Written;
        }
    }

    fn record_all(&mut self) {
        let Self::Sparse(block_writes) = self else {
            return;
        };
        block_writes
            .iter_mut()
            .for_each(|block_was_written| *block_was_written = SectionBlockWrite::Written);
    }
}
