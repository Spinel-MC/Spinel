use crate::world::section_palette::{SectionPalette, SectionPaletteError};
use crate::world::{Biome, Block, BlockState};
use spinel_network::types::chunk::ChunkSection as NetworkChunkSection;
use spinel_registry::{Registries, RegistryKey};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

const CHUNK_SECTION_BLOCK_COUNT: usize = 4096;
const CHUNK_SECTION_BIOME_COUNT: usize = 64;
const CHUNK_SECTION_LIGHT_BYTES: usize = 2048;
const EMPTY_SECTION_LIGHT: [u8; CHUNK_SECTION_LIGHT_BYTES] = [0; CHUNK_SECTION_LIGHT_BYTES];
const FULL_SECTION_LIGHT: [u8; CHUNK_SECTION_LIGHT_BYTES] = [255; CHUNK_SECTION_LIGHT_BYTES];

pub type ChunkSectionBlockPalette = SectionPalette<BlockState, CHUNK_SECTION_BLOCK_COUNT, 4>;
pub type ChunkSectionBiomePalette =
    SectionPalette<RegistryKey<Biome>, CHUNK_SECTION_BIOME_COUNT, 1>;

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
    blocks: ChunkSectionBlockPalette,
    biomes: ChunkSectionBiomePalette,
    block_writes: SectionBlockWrites,
    sky_light: Option<Arc<[u8; CHUNK_SECTION_LIGHT_BYTES]>>,
    block_light: Option<Arc<[u8; CHUNK_SECTION_LIGHT_BYTES]>>,
    sky_light_invalidated: bool,
    block_light_invalidated: bool,
    sky_light_needs_send: bool,
    block_light_needs_send: bool,
    special_blocks: HashMap<usize, Block>,
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
            block_count: self.non_air_block_count(),
            block_states: self.blocks.to_network(4, 15),
            biomes: self
                .biomes
                .try_to_network(1, 8, |biome| registries.biome().get_id(biome))?,
        })
    }

    pub(crate) fn block_state(&self, x: i32, y: i32, z: i32) -> Option<BlockState> {
        Self::block_index(x, y, z).and_then(|block_index| self.blocks.get(block_index))
    }

    pub(crate) fn biome(&self, x: i32, y: i32, z: i32) -> Option<RegistryKey<Biome>> {
        Self::biome_index(x, y, z).and_then(|biome_index| self.biomes.get(biome_index))
    }

    pub(crate) fn set_block(&mut self, x: i32, y: i32, z: i32, block: Block) -> bool {
        self.set_block_state(x, y, z, block.default_state())
    }

    pub(crate) fn set_block_state(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        block_state: BlockState,
    ) -> bool {
        let Some(block_index) = Self::block_index(x, y, z) else {
            return false;
        };
        let previous_state = self
            .blocks
            .get(block_index)
            .unwrap_or_else(|| Block::AIR.default_state());
        self.block_writes.record(block_index);
        if previous_state == block_state {
            return true;
        }
        if !self.special_blocks.is_empty() {
            self.special_blocks.remove(&block_index);
        }
        let block = block_state.block();
        let block_was_set = self.blocks.set(block_index, block_state);
        if block.block_entity_type().is_some() {
            self.cache_special_block(x, y, z, block);
        }
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
        self.fill_block_state(block.default_state());
    }

    pub(crate) fn fill_block_state(&mut self, block_state: BlockState) {
        let block = block_state.block();
        self.blocks.fill(block_state);
        self.block_writes.record_all();
        self.special_blocks.clear();
        if block.block_entity_type().is_some() {
            (0..CHUNK_SECTION_BLOCK_COUNT).for_each(|block_index| {
                self.special_blocks.insert(block_index, block);
            });
        }
    }

    pub(crate) fn fill_block_layers(
        &mut self,
        minimum_y: i32,
        maximum_y: i32,
        block: Block,
    ) -> bool {
        if minimum_y < 0 || maximum_y > 16 || minimum_y > maximum_y {
            return false;
        }
        let start = minimum_y as usize * 256;
        let end = maximum_y as usize * 256;
        let block_state = block.default_state();
        if !self.blocks.fill_range(start, end, block_state) {
            return false;
        }
        self.block_writes.record_range(start, end);
        self.special_blocks
            .retain(|block_index, _| *block_index < start || *block_index >= end);
        if block.block_entity_type().is_some() {
            (start..end).for_each(|block_index| {
                self.special_blocks.insert(block_index, block);
            });
        }
        true
    }

    pub(crate) fn fill_block_area(
        &mut self,
        minimum_x: i32,
        minimum_y: i32,
        minimum_z: i32,
        maximum_x: i32,
        maximum_y: i32,
        maximum_z: i32,
        block: Block,
    ) -> bool {
        let coordinates_are_valid = minimum_x >= 0
            && minimum_y >= 0
            && minimum_z >= 0
            && maximum_x <= 16
            && maximum_y <= 16
            && maximum_z <= 16
            && minimum_x <= maximum_x
            && minimum_y <= maximum_y
            && minimum_z <= maximum_z;
        if !coordinates_are_valid {
            return false;
        }
        let block_state = block.default_state();
        for y in minimum_y as usize..maximum_y as usize {
            for z in minimum_z as usize..maximum_z as usize {
                let row_start = (y << 8) | (z << 4);
                let start = row_start + minimum_x as usize;
                let end = row_start + maximum_x as usize;
                self.blocks.fill_range(start, end, block_state);
                self.block_writes.record_range(start, end);
            }
        }
        self.special_blocks.retain(|block_index, _| {
            let (x, y, z) = Self::block_coordinates(*block_index);
            x < minimum_x
                || x >= maximum_x
                || y < minimum_y
                || y >= maximum_y
                || z < minimum_z
                || z >= maximum_z
        });
        if block.block_entity_type().is_some() {
            for y in minimum_y..maximum_y {
                for z in minimum_z..maximum_z {
                    for x in minimum_x..maximum_x {
                        self.cache_special_block(x, y, z, block);
                    }
                }
            }
        }
        true
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

    pub fn block_palette(&self) -> &ChunkSectionBlockPalette {
        &self.blocks
    }

    pub fn block_palette_mut(&mut self) -> &mut ChunkSectionBlockPalette {
        &mut self.blocks
    }

    pub fn biome_palette(&self) -> &ChunkSectionBiomePalette {
        &self.biomes
    }

    pub fn biome_palette_mut(&mut self) -> &mut ChunkSectionBiomePalette {
        &mut self.biomes
    }

    pub fn clear(&mut self) {
        self.blocks.fill(Block::AIR.default_state());
        self.biomes.fill(Biome::PLAINS);
        self.block_writes.record_all();
        self.special_blocks.clear();
    }

    pub fn set_sky_light(&mut self, light_levels: &[u8]) -> Result<(), SetChunkSectionLightError> {
        Self::validate_light_array_length(light_levels)?;
        self.sky_light = Self::stored_light(light_levels);
        self.sky_light_invalidated = false;
        self.sky_light_needs_send = true;
        Ok(())
    }

    pub fn set_block_light(
        &mut self,
        light_levels: &[u8],
    ) -> Result<(), SetChunkSectionLightError> {
        Self::validate_light_array_length(light_levels)?;
        self.block_light = Self::stored_light(light_levels);
        self.block_light_invalidated = false;
        self.block_light_needs_send = true;
        Ok(())
    }

    pub fn sky_light(&self) -> &[u8] {
        Self::light_values(&self.sky_light)
    }

    pub fn block_light(&self) -> &[u8] {
        Self::light_values(&self.block_light)
    }

    pub(crate) fn sky_light_level(&self, x: i32, y: i32, z: i32) -> u8 {
        Self::light_level(self.sky_light(), x, y, z)
    }

    pub(crate) fn block_light_level(&self, x: i32, y: i32, z: i32) -> u8 {
        Self::light_level(self.block_light(), x, y, z)
    }

    pub(crate) fn clear_light(&mut self) {
        self.sky_light = None;
        self.block_light = None;
    }

    pub(crate) fn set_sky_light_level(&mut self, x: i32, y: i32, z: i32, level: u8) {
        let Some(block_index) = Self::block_index(x, y, z) else {
            return;
        };
        Self::set_stored_light_level(&mut self.sky_light, block_index, level);
    }

    pub(crate) fn set_block_light_level(&mut self, x: i32, y: i32, z: i32, level: u8) {
        let Some(block_index) = Self::block_index(x, y, z) else {
            return;
        };
        Self::set_stored_light_level(&mut self.block_light, block_index, level);
    }

    pub(crate) fn validate_light(&mut self) {
        Self::normalize_stored_light(&mut self.sky_light);
        Self::normalize_stored_light(&mut self.block_light);
        self.sky_light_invalidated = false;
        self.block_light_invalidated = false;
    }

    pub(crate) fn invalidate_sky_light(&mut self) {
        self.sky_light_invalidated = true;
        self.sky_light_needs_send = true;
    }

    pub(crate) fn invalidate_block_light(&mut self) {
        self.block_light_invalidated = true;
        self.block_light_needs_send = true;
    }

    pub(crate) fn relight_block_light(&mut self) -> bool {
        if !self.block_light_invalidated {
            return false;
        }
        let mut light = [0; CHUNK_SECTION_LIGHT_BYTES];
        (0..CHUNK_SECTION_BLOCK_COUNT).for_each(|block_index| {
            let block_state = self
                .blocks
                .get(block_index)
                .unwrap_or_else(|| Block::AIR.default_state());
            Self::set_light_level(&mut light, block_index, block_state.light_emission());
        });
        self.block_light = Self::stored_light(&light);
        self.block_light_invalidated = false;
        true
    }

    pub(crate) fn relight_sky_light(&mut self) -> bool {
        if !self.sky_light_invalidated {
            return false;
        }
        let mut light = [0; CHUNK_SECTION_LIGHT_BYTES];
        (0..CHUNK_SECTION_BLOCK_COUNT).for_each(|block_index| {
            let block_state = self
                .blocks
                .get(block_index)
                .unwrap_or_else(|| Block::AIR.default_state());
            let light_level = if block_state.propagates_skylight_down() {
                15
            } else {
                15u8.saturating_sub(block_state.light_block())
            };
            Self::set_light_level(&mut light, block_index, light_level);
        });
        self.sky_light = Self::stored_light(&light);
        self.sky_light_invalidated = false;
        true
    }

    pub fn sky_light_is_invalidated(&self) -> bool {
        self.sky_light_invalidated
    }

    pub fn block_light_is_invalidated(&self) -> bool {
        self.block_light_invalidated
    }

    pub(crate) fn sky_light_needs_send(&self) -> bool {
        self.sky_light_needs_send
    }

    pub(crate) fn block_light_needs_send(&self) -> bool {
        self.block_light_needs_send
    }

    pub(crate) fn consume_sky_light_send(&mut self) {
        self.sky_light_needs_send = false;
    }

    pub(crate) fn consume_block_light_send(&mut self) {
        self.block_light_needs_send = false;
    }

    fn with_block_writes(y: i32, block_writes: SectionBlockWrites) -> Self {
        Self {
            y,
            blocks: ChunkSectionBlockPalette::new(Block::AIR.default_state()),
            biomes: ChunkSectionBiomePalette::new(Biome::PLAINS),
            block_writes,
            sky_light: None,
            block_light: None,
            sky_light_invalidated: true,
            block_light_invalidated: true,
            sky_light_needs_send: false,
            block_light_needs_send: false,
            special_blocks: HashMap::new(),
        }
    }

    fn non_air_block_count(&self) -> i16 {
        let air_state_id = Block::AIR.state_id();
        self.blocks
            .count_matching(|block_state| block_state.state_id() != air_state_id) as i16
    }

    pub(crate) fn has_non_air_blocks(&self) -> bool {
        self.non_air_block_count() > 0
    }

    fn validate_light_array_length(light_levels: &[u8]) -> Result<(), SetChunkSectionLightError> {
        if light_levels.len() == CHUNK_SECTION_LIGHT_BYTES {
            return Ok(());
        }
        Err(SetChunkSectionLightError {
            actual_length: light_levels.len(),
        })
    }

    fn stored_light(light_levels: &[u8]) -> Option<Arc<[u8; CHUNK_SECTION_LIGHT_BYTES]>> {
        if light_levels.iter().all(|light_level| *light_level == 0) {
            return None;
        }
        if light_levels.iter().all(|light_level| *light_level == 255) {
            return Some(Arc::clone(Self::fully_lit_storage()));
        }
        let mut stored_light = Box::new(EMPTY_SECTION_LIGHT);
        stored_light.copy_from_slice(light_levels);
        Some(Arc::from(stored_light))
    }

    fn light_values(light: &Option<Arc<[u8; CHUNK_SECTION_LIGHT_BYTES]>>) -> &[u8] {
        light.as_deref().unwrap_or(&EMPTY_SECTION_LIGHT)
    }

    fn normalize_stored_light(light: &mut Option<Arc<[u8; CHUNK_SECTION_LIGHT_BYTES]>>) {
        let Some(light_levels) = light else {
            return;
        };
        if light_levels.iter().all(|light_level| *light_level == 0) {
            *light = None;
            return;
        }
        if light_levels.iter().all(|light_level| *light_level == 255) {
            *light = Some(Arc::clone(Self::fully_lit_storage()));
        }
    }

    fn fully_lit_storage() -> &'static Arc<[u8; CHUNK_SECTION_LIGHT_BYTES]> {
        static FULLY_LIT_STORAGE: OnceLock<Arc<[u8; CHUNK_SECTION_LIGHT_BYTES]>> = OnceLock::new();
        FULLY_LIT_STORAGE.get_or_init(|| Arc::new(FULL_SECTION_LIGHT))
    }

    #[cfg(test)]
    fn light_storage_is_allocated(light: &Option<Arc<[u8; CHUNK_SECTION_LIGHT_BYTES]>>) -> bool {
        light
            .as_ref()
            .is_some_and(|light_levels| !Arc::ptr_eq(light_levels, Self::fully_lit_storage()))
    }

    fn merge_fork_block(&mut self, fork_section: &ChunkSection, block_index: usize) {
        let Some(block_state) = fork_section.blocks.get(block_index) else {
            return;
        };
        let (x, y, z) = Self::block_coordinates(block_index);
        self.set_block_state(x, y, z, block_state);
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

    fn set_light_level(light: &mut [u8], block_index: usize, level: u8) {
        let Some(light_byte) = light.get_mut(block_index / 2) else {
            return;
        };
        let level = level.min(15);
        if block_index % 2 == 0 {
            *light_byte = (*light_byte & 240) | level;
            return;
        }
        *light_byte = (*light_byte & 15) | (level << 4);
    }

    fn set_stored_light_level(
        light: &mut Option<Arc<[u8; CHUNK_SECTION_LIGHT_BYTES]>>,
        block_index: usize,
        level: u8,
    ) {
        if light.is_none() && level == 0 {
            return;
        }
        let light_is_fully_lit = light
            .as_ref()
            .is_some_and(|light_levels| Arc::ptr_eq(light_levels, Self::fully_lit_storage()));
        if light_is_fully_lit && level == 15 {
            return;
        }
        let stored_light = light.get_or_insert_with(|| Arc::new(EMPTY_SECTION_LIGHT));
        let light_levels = Arc::make_mut(stored_light);
        Self::set_light_level(light_levels.as_mut_slice(), block_index, level);
        if level == 0 && light_levels.iter().all(|light_level| *light_level == 0) {
            *light = None;
        }
    }

    #[cfg(test)]
    pub(crate) fn has_allocated_sky_light(&self) -> bool {
        Self::light_storage_is_allocated(&self.sky_light)
    }

    #[cfg(test)]
    pub(crate) fn has_allocated_block_light(&self) -> bool {
        Self::light_storage_is_allocated(&self.block_light)
    }

    #[cfg(test)]
    pub(crate) fn allocated_sky_light_bytes(&self) -> usize {
        usize::from(Self::light_storage_is_allocated(&self.sky_light)) * CHUNK_SECTION_LIGHT_BYTES
    }

    fn biome_index(x: i32, y: i32, z: i32) -> Option<usize> {
        let local_coordinates_are_valid =
            (0..4).contains(&x) && (0..4).contains(&y) && (0..4).contains(&z);
        local_coordinates_are_valid
            .then_some(((y as usize) << 4) | ((z as usize) << 2) | x as usize)
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error(
    "chunk section light array must contain {CHUNK_SECTION_LIGHT_BYTES} bytes, received {actual_length}"
)]
pub struct SetChunkSectionLightError {
    actual_length: usize,
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

    fn record_range(&mut self, start: usize, end: usize) {
        let Self::Sparse(block_writes) = self else {
            return;
        };
        block_writes[start..end]
            .iter_mut()
            .for_each(|block_was_written| *block_was_written = SectionBlockWrite::Written);
    }
}
