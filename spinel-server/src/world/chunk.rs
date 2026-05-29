use crate::entity::Player;
use crate::world::chunk_heightmaps::ChunkHeightmaps;
use crate::world::chunk_lighting::ChunkLighting;
use crate::world::section_palette::SectionPaletteError;
use crate::world::{
    Biome, Block, BlockEntity, BlockHandlerDestroy, BlockHandlerPlacement, BlockHandlerRegistry,
    BlockHandlerTick, BlockPosition, ChunkPosition, ChunkSection,
};
use spinel_core::network::clientbound::play::chunk_data::ChunkDataAndUpdateLightPacket;
use spinel_nbt::{Nbt, NbtCompound, TagHandler, Taggable};
use spinel_network::types::chunk::{ChunkData, HeightmapEntry};
use spinel_network::types::light::LightData;
use spinel_registry::{Registries, RegistryKey};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use uuid::Uuid;

pub const CHUNK_SIZE_X: i32 = 16;
pub const CHUNK_SIZE_Z: i32 = 16;
pub const CHUNK_SECTION_SIZE: i32 = 16;

const WORLD_MIN_SECTION: i32 = -4;
const WORLD_SECTION_COUNT: i32 = 24;

pub struct Chunk {
    identifier: Uuid,
    position: ChunkPosition,
    sections: Vec<ChunkSection>,
    block_entities: Vec<BlockEntity>,
    motion_blocking_heightmap: ChunkHeightmaps,
    world_surface_heightmap: ChunkHeightmaps,
    cached_chunk_data: RefCell<Option<ChunkData>>,
    tag_handler: TagHandler,
    world: Option<Uuid>,
    viewers: HashSet<i32>,
    tickable_blocks: HashMap<BlockPosition, Block>,
    has_generated: bool,
    loaded: bool,
    read_only: bool,
    invalidated: bool,
}

impl Chunk {
    pub fn new(position: ChunkPosition) -> Self {
        Self {
            identifier: Uuid::new_v4(),
            position,
            sections: (0..WORLD_SECTION_COUNT)
                .map(|section_offset| ChunkSection::new(WORLD_MIN_SECTION + section_offset))
                .collect(),
            block_entities: Vec::new(),
            motion_blocking_heightmap: ChunkHeightmaps::from_sections(&[]),
            world_surface_heightmap: ChunkHeightmaps::from_sections(&[]),
            cached_chunk_data: RefCell::new(None),
            tag_handler: TagHandler::new_handler(),
            world: None,
            viewers: HashSet::new(),
            tickable_blocks: HashMap::new(),
            has_generated: false,
            loaded: true,
            read_only: false,
            invalidated: false,
        }
        .with_refreshed_heightmaps()
    }

    fn with_refreshed_heightmaps(mut self) -> Self {
        self.refresh_heightmaps_from_sections();
        self
    }

    pub const fn identifier(&self) -> Uuid {
        self.identifier
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

    pub fn section(&self, section_y: i32) -> Option<&ChunkSection> {
        self.sections.iter().find(|section| section.y == section_y)
    }

    pub fn section_mut(&mut self, section_y: i32) -> Option<&mut ChunkSection> {
        if self.read_only {
            return None;
        }
        self.sections
            .iter_mut()
            .find(|section| section.y == section_y)
    }

    pub fn section_at_block_y(&self, block_y: i32) -> Option<&ChunkSection> {
        self.section(block_y.div_euclid(CHUNK_SECTION_SIZE))
    }

    pub fn section_at_block_y_mut(&mut self, block_y: i32) -> Option<&mut ChunkSection> {
        self.section_mut(block_y.div_euclid(CHUNK_SECTION_SIZE))
    }

    pub const fn min_section(&self) -> i32 {
        WORLD_MIN_SECTION
    }

    pub const fn max_section(&self) -> i32 {
        WORLD_MIN_SECTION + WORLD_SECTION_COUNT
    }

    pub const fn world_position(&self) -> BlockPosition {
        BlockPosition::new(
            self.position.x * CHUNK_SIZE_X,
            0,
            self.position.z * CHUNK_SIZE_Z,
        )
    }

    pub fn copy_for_position(&self, position: ChunkPosition) -> Self {
        Self {
            identifier: Uuid::new_v4(),
            position,
            sections: self.sections.clone(),
            block_entities: self.block_entities.clone(),
            motion_blocking_heightmap: self.motion_blocking_heightmap.clone(),
            world_surface_heightmap: self.world_surface_heightmap.clone(),
            cached_chunk_data: RefCell::new(self.cached_chunk_data.borrow().clone()),
            tag_handler: self.tag_handler.readable_copy(),
            world: self.world,
            viewers: self.viewers.clone(),
            tickable_blocks: self.tickable_blocks.clone(),
            has_generated: self.has_generated,
            loaded: self.loaded,
            read_only: self.read_only,
            invalidated: self.invalidated,
        }
    }

    pub fn reset(&mut self) {
        if self.read_only {
            return;
        }
        self.sections = (0..WORLD_SECTION_COUNT)
            .map(|section_offset| ChunkSection::new(WORLD_MIN_SECTION + section_offset))
            .collect();
        self.block_entities.clear();
        self.tickable_blocks.clear();
        self.has_generated = false;
        self.refresh_heightmaps_from_sections();
        self.loaded = true;
        self.invalidate();
    }

    pub fn block(&self, position: BlockPosition) -> Block {
        self.section(position.y.div_euclid(CHUNK_SECTION_SIZE))
            .and_then(|section| {
                section.block(
                    position.x.rem_euclid(CHUNK_SIZE_X),
                    position.y.rem_euclid(CHUNK_SECTION_SIZE),
                    position.z.rem_euclid(CHUNK_SIZE_Z),
                )
            })
            .unwrap_or(Block::AIR)
    }

    pub fn biome(&self, position: BlockPosition) -> RegistryKey<Biome> {
        self.section(position.y.div_euclid(CHUNK_SECTION_SIZE))
            .and_then(|section| {
                section.biome(
                    position.x.rem_euclid(CHUNK_SIZE_X) >> 2,
                    position.y.rem_euclid(CHUNK_SECTION_SIZE) >> 2,
                    position.z.rem_euclid(CHUNK_SIZE_Z) >> 2,
                )
            })
            .unwrap_or(Biome::PLAINS)
    }

    pub fn set_block(&mut self, position: BlockPosition, block: Block) -> bool {
        self.try_set_block(position, block).block_was_set()
    }

    pub fn try_set_block(&mut self, position: BlockPosition, block: Block) -> SetChunkBlockResult {
        self.try_set_block_with_handler(position, block, None, None, None)
    }

    pub(crate) fn try_set_block_with_handler(
        &mut self,
        position: BlockPosition,
        block: Block,
        handlers: Option<&BlockHandlerRegistry>,
        placement: Option<BlockHandlerPlacement>,
        destroy: Option<BlockHandlerDestroy>,
    ) -> SetChunkBlockResult {
        if self.read_only {
            return SetChunkBlockResult::ReadOnly;
        }
        let previous_block = self.block(position);
        let Some(section) = self.section_mut(position.y.div_euclid(CHUNK_SECTION_SIZE)) else {
            return SetChunkBlockResult::OutsideHeight;
        };
        let block_was_set = section.set_block(
            position.x.rem_euclid(CHUNK_SIZE_X),
            position.y.rem_euclid(CHUNK_SECTION_SIZE),
            position.z.rem_euclid(CHUNK_SIZE_Z),
            block,
        );
        if block_was_set {
            self.update_tickable_block(position, block, handlers);
            if !ChunkSection::block_can_own_block_entity(block) {
                self.remove_block_entity(position);
            }
            self.refresh_heightmaps_after_block_change(position, block);
            self.invalidate();
            self.dispatch_block_handler_destroy(previous_block, block, position, handlers, destroy);
            self.dispatch_block_handler_place(previous_block, block, position, handlers, placement);
            return SetChunkBlockResult::Changed;
        }
        SetChunkBlockResult::Unchanged
    }

    pub fn set_biome(&mut self, position: BlockPosition, biome: RegistryKey<Biome>) -> bool {
        if self.read_only {
            return false;
        }
        let Some(section) = self.section_mut(position.y.div_euclid(CHUNK_SECTION_SIZE)) else {
            return false;
        };
        let biome_was_set = section.set_biome(
            position.x.rem_euclid(CHUNK_SIZE_X) >> 2,
            position.y.rem_euclid(CHUNK_SECTION_SIZE) >> 2,
            position.z.rem_euclid(CHUNK_SIZE_Z) >> 2,
            biome,
        );
        if biome_was_set {
            self.invalidate();
        }
        biome_was_set
    }

    pub fn fill_biome(&mut self, biome: RegistryKey<Biome>) {
        if self.read_only {
            return;
        }
        self.sections
            .iter_mut()
            .for_each(|section| section.fill_biome(biome.clone()));
        self.invalidate();
    }

    pub fn data(&self, registries: &Registries) -> Result<ChunkData, ChunkDataError> {
        if let Some(chunk_data) = self.cached_chunk_data.borrow().clone() {
            return Ok(chunk_data);
        }
        let chunk_data = ChunkData {
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
        };
        *self.cached_chunk_data.borrow_mut() = Some(chunk_data.clone());
        Ok(chunk_data)
    }

    pub fn light_data(&self) -> LightData {
        ChunkLighting::from_sections(&self.sections).data()
    }

    pub fn block_light(&self, position: BlockPosition) -> u8 {
        self.section_at_block_y(position.y)
            .map(|section| {
                section.block_light_level(
                    position.x.rem_euclid(CHUNK_SIZE_X),
                    position.y.rem_euclid(CHUNK_SECTION_SIZE),
                    position.z.rem_euclid(CHUNK_SIZE_Z),
                )
            })
            .unwrap_or_default()
    }

    pub fn sky_light(&self, position: BlockPosition) -> u8 {
        self.section_at_block_y(position.y)
            .map(|section| {
                section.sky_light_level(
                    position.x.rem_euclid(CHUNK_SIZE_X),
                    position.y.rem_euclid(CHUNK_SECTION_SIZE),
                    position.z.rem_euclid(CHUNK_SIZE_Z),
                )
            })
            .unwrap_or_default()
    }

    pub fn invalidate_section(&mut self, section_y: i32) -> bool {
        let Some(section) = self
            .sections
            .iter_mut()
            .find(|section| section.y == section_y)
        else {
            return false;
        };
        section.invalidate_sky_light();
        section.invalidate_block_light();
        self.invalidate();
        true
    }

    pub fn full_data_packet(
        &self,
        registries: &Registries,
    ) -> Result<ChunkDataAndUpdateLightPacket, ChunkDataError> {
        Ok(ChunkDataAndUpdateLightPacket::with_light_data(
            self.x(),
            self.z(),
            self.data(registries)?,
            self.light_data(),
        ))
    }

    pub(crate) fn replace_sections(&mut self, sections: Vec<ChunkSection>) {
        self.sections = sections;
        self.refresh_heightmaps_from_sections();
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
        self.refresh_heightmaps_from_sections();
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

    pub fn block_entity(&self, position: BlockPosition) -> Option<&BlockEntity> {
        self.block_entities
            .iter()
            .find(|block_entity| block_entity.position() == position)
    }

    pub fn block_entities(&self) -> &[BlockEntity] {
        &self.block_entities
    }

    pub fn network_block_entities(&self) -> Vec<spinel_network::types::chunk::BlockEntity> {
        self.block_entities
            .iter()
            .map(BlockEntity::to_network)
            .collect()
    }

    pub fn tick(&self, world: Uuid, handlers: &BlockHandlerRegistry, _time: u64) -> usize {
        self.tickable_blocks
            .iter()
            .filter_map(|(position, block)| {
                handlers
                    .handler(*block)
                    .map(|handler| (handler, *block, *position))
            })
            .map(|(handler, block, position)| {
                handler.tick(BlockHandlerTick::new(block, world, position));
                1
            })
            .sum()
    }

    pub fn tickable_block_count(&self) -> usize {
        self.tickable_blocks.len()
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
        *self.cached_chunk_data.borrow_mut() = None;
    }

    pub fn is_invalidated(&self) -> bool {
        self.invalidated
    }

    pub(crate) fn clear_invalidated(&mut self) {
        self.invalidated = false;
    }

    pub fn should_generate(&self) -> bool {
        !self.has_generated
    }

    pub fn on_generate(&mut self) {
        self.mark_generated();
    }

    pub fn motion_blocking_heightmap(&self) -> Vec<i64> {
        self.motion_blocking_heightmap.packed_heights()
    }

    pub fn world_surface_heightmap(&self) -> Vec<i64> {
        self.world_surface_heightmap.packed_heights()
    }

    pub fn load_heightmaps_from_nbt(&mut self, heightmaps: &NbtCompound) {
        if let Some(Nbt::LongArray(heights)) = heightmaps.get("MOTION_BLOCKING") {
            self.motion_blocking_heightmap
                .load_motion_blocking_from_longs(heights);
        }
        if let Some(Nbt::LongArray(heights)) = heightmaps.get("WORLD_SURFACE") {
            self.world_surface_heightmap
                .load_world_surface_from_longs(heights);
        }
        self.invalidate();
    }

    pub fn add_viewer(&mut self, entity_id: crate::entity::EntityId) -> bool {
        self.viewers.insert(entity_id.value())
    }

    pub fn remove_viewer(&mut self, entity_id: crate::entity::EntityId) -> bool {
        self.viewers.remove(&entity_id.value())
    }

    pub fn viewers(&self) -> impl Iterator<Item = i32> + '_ {
        self.viewers.iter().copied()
    }

    pub fn world(&self) -> Option<Uuid> {
        self.world
    }

    pub(crate) fn set_world(&mut self, world: Uuid) {
        self.world = Some(world);
    }

    pub fn send_chunk_to_player(&self, player: &mut Player) -> bool {
        player.send_loaded_chunk(self)
    }

    pub(crate) fn mark_generated(&mut self) {
        self.has_generated = true;
        self.clear_invalidated();
    }

    pub(crate) fn mark_not_generated(&mut self) {
        self.has_generated = false;
    }

    fn heightmaps(&self) -> Vec<HeightmapEntry> {
        ChunkHeightmaps::entries(
            &self.motion_blocking_heightmap,
            &self.world_surface_heightmap,
        )
    }

    fn refresh_heightmaps_from_sections(&mut self) {
        self.motion_blocking_heightmap
            .refresh_from_sections(&self.sections);
        self.world_surface_heightmap
            .refresh_from_sections(&self.sections);
    }

    fn refresh_heightmaps_after_block_change(&mut self, position: BlockPosition, block: Block) {
        self.motion_blocking_heightmap
            .refresh_block(&self.sections, position, block);
        self.world_surface_heightmap
            .refresh_block(&self.sections, position, block);
    }

    fn update_tickable_block(
        &mut self,
        position: BlockPosition,
        block: Block,
        handlers: Option<&BlockHandlerRegistry>,
    ) {
        if handlers.is_some_and(|handlers| handlers.has_tickable_handler(block)) {
            self.tickable_blocks.insert(position, block);
            return;
        }
        self.tickable_blocks.remove(&position);
    }

    fn dispatch_block_handler_destroy(
        &self,
        previous_block: Block,
        new_block: Block,
        position: BlockPosition,
        handlers: Option<&BlockHandlerRegistry>,
        destroy: Option<BlockHandlerDestroy>,
    ) {
        let Some(handler) = handlers.and_then(|handlers| handlers.handler(previous_block)) else {
            return;
        };
        let world = self.world.unwrap_or_default();
        handler.on_destroy(destroy.unwrap_or_else(|| {
            BlockHandlerDestroy::new(previous_block, new_block, world, position, None)
        }));
    }

    fn dispatch_block_handler_place(
        &self,
        previous_block: Block,
        block: Block,
        position: BlockPosition,
        handlers: Option<&BlockHandlerRegistry>,
        placement: Option<BlockHandlerPlacement>,
    ) {
        let Some(handler) = handlers.and_then(|handlers| handlers.handler(block)) else {
            return;
        };
        let world = self.world.unwrap_or_default();
        handler.on_place(
            placement.unwrap_or_else(|| {
                BlockHandlerPlacement::new(block, previous_block, world, position)
            }),
        );
    }
}

impl Taggable for Chunk {
    fn tag_handler(&self) -> &TagHandler {
        &self.tag_handler
    }

    fn tag_handler_mut(&mut self) -> &mut TagHandler {
        &mut self.tag_handler
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkDataError {
    MissingRegistryEntry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetChunkBlockResult {
    Changed,
    Unchanged,
    ReadOnly,
    OutsideHeight,
}

impl SetChunkBlockResult {
    pub const fn block_was_set(self) -> bool {
        matches!(self, Self::Changed)
    }
}

impl From<SectionPaletteError> for ChunkDataError {
    fn from(_error: SectionPaletteError) -> Self {
        Self::MissingRegistryEntry
    }
}
