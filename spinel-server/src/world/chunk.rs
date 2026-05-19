use crate::world::chunk_heightmaps::ChunkHeightmaps;
use crate::world::chunk_lighting::ChunkLighting;
use crate::world::section_palette::SectionPaletteError;
use crate::world::{BlockEntity, BlockPosition, ChunkPosition, ChunkSection};
use spinel_network::types::chunk::{ChunkData, HeightmapEntry};
use spinel_network::types::light::LightData;
use spinel_registry::Registries;

const WORLD_MIN_SECTION: i32 = -4;
const WORLD_SECTION_COUNT: i32 = 24;

pub struct Chunk {
    position: ChunkPosition,
    sections: Vec<ChunkSection>,
    block_entities: Vec<BlockEntity>,
    has_generated: bool,
    loaded: bool,
    read_only: bool,
    invalidated: bool,
}

impl Chunk {
    pub fn new(position: ChunkPosition) -> Self {
        Self {
            position,
            sections: (0..WORLD_SECTION_COUNT)
                .map(|section_offset| ChunkSection::new(WORLD_MIN_SECTION + section_offset))
                .collect(),
            block_entities: Vec::new(),
            has_generated: false,
            loaded: true,
            read_only: false,
            invalidated: false,
        }
    }

    pub fn x(&self) -> i32 {
        self.position.x
    }

    pub fn z(&self) -> i32 {
        self.position.z
    }

    pub fn sections(&self) -> &[ChunkSection] {
        &self.sections
    }

    pub fn data(&self, registries: &Registries) -> Result<ChunkData, ChunkDataError> {
        Ok(ChunkData {
            heightmaps: self.heightmaps(),
            sections: self
                .sections
                .iter()
                .map(|section| section.to_network_section(registries))
                .collect::<Result<Vec<_>, _>>()?,
            block_entities: self
                .block_entities
                .iter()
                .map(BlockEntity::to_network)
                .collect(),
        })
    }

    pub(crate) fn light_data(&self) -> LightData {
        ChunkLighting::from_sections(&self.sections).data()
    }

    pub(crate) fn replace_sections(&mut self, sections: Vec<ChunkSection>) {
        self.sections = sections;
        self.invalidate();
    }

    pub(crate) fn merge_section_from_fork(&mut self, section_y: i32, fork_section: &ChunkSection) {
        let Some(chunk_section) = self
            .sections
            .iter_mut()
            .find(|chunk_section| chunk_section.y == section_y)
        else {
            return;
        };
        chunk_section.merge_from_fork(fork_section);
        self.invalidate();
    }

    pub fn set_block_entity(&mut self, block_entity: BlockEntity) {
        if self.read_only {
            return;
        }
        let position = block_entity.position();
        self.block_entities
            .retain(|stored_entity| stored_entity.position() != position);
        self.block_entities.push(block_entity);
        self.invalidate();
    }

    pub fn remove_block_entity(&mut self, position: BlockPosition) -> Option<BlockEntity> {
        let block_entity_index = self
            .block_entities
            .iter()
            .position(|block_entity| block_entity.position() == position)?;
        self.invalidate();
        Some(self.block_entities.remove(block_entity_index))
    }

    pub fn block_entities(&self) -> &[BlockEntity] {
        &self.block_entities
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn unload(&mut self) {
        self.loaded = false;
    }

    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }

    pub fn invalidate(&mut self) {
        self.invalidated = true;
    }

    pub fn is_invalidated(&self) -> bool {
        self.invalidated
    }

    pub(crate) fn clear_invalidated(&mut self) {
        self.invalidated = false;
    }

    pub(crate) fn should_generate(&self) -> bool {
        !self.has_generated
    }

    pub(crate) fn mark_generated(&mut self) {
        self.has_generated = true;
        self.clear_invalidated();
    }

    pub(crate) fn mark_not_generated(&mut self) {
        self.has_generated = false;
    }

    fn heightmaps(&self) -> Vec<HeightmapEntry> {
        ChunkHeightmaps::from_sections(&self.sections).entries()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkDataError {
    MissingRegistryEntry,
}

impl From<SectionPaletteError> for ChunkDataError {
    fn from(_error: SectionPaletteError) -> Self {
        Self::MissingRegistryEntry
    }
}
