use crate::entity::Player;
use crate::world::chunk_heightmaps::ChunkHeightmaps;
use crate::world::chunk_lighting::ChunkLighting;
use crate::world::section_palette::SectionPaletteError;
use crate::world::{
    Biome, Block, BlockEntity, BlockHandlerDestroy, BlockHandlerPlacement, BlockHandlerRegistry,
    BlockHandlerTick, BlockInstance, BlockLookupCondition, BlockPosition, BlockState,
    ChunkPosition, ChunkSection,
};
use spinel_core::network::clientbound::play::chunk_data::ChunkDataAndUpdateLightPacket;
use spinel_nbt::{Nbt, NbtCompound, TagHandler, Taggable};
use spinel_network::types::chunk::{ChunkData, HeightmapEntry};
use spinel_network::types::light::LightData;
use spinel_registry::{Registries, RegistryKey};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::{Arc, Weak};
use uuid::Uuid;

pub const CHUNK_SIZE_X: i32 = 16;
pub const CHUNK_SIZE_Z: i32 = 16;
pub const CHUNK_SECTION_SIZE: i32 = 16;

const WORLD_MIN_SECTION: i32 = -4;
const WORLD_SECTION_COUNT: i32 = 24;
const LIGHTING_RESEND_DELAY_TICKS: u16 = 100;
const GENERATED_LIGHTING_RESEND_DELAY_TICKS: u16 = 20;

type GenerationCallback = Arc<dyn Fn(&mut Chunk) + Send + Sync>;
type LoadCallback = Arc<dyn Fn(&mut Chunk) + Send + Sync>;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ChunkType {
    Dynamic,
    Lighting,
}

pub struct Chunk {
    identifier: Uuid,
    position: ChunkPosition,
    chunk_type: ChunkType,
    sections: Arc<Vec<ChunkSection>>,
    block_instances: HashMap<BlockPosition, BlockInstance>,
    block_entities: Vec<BlockEntity>,
    motion_blocking_heightmap: RefCell<ChunkHeightmaps>,
    world_surface_heightmap: RefCell<ChunkHeightmaps>,
    heightmaps_need_complete_refresh: Cell<bool>,
    cached_chunk_data: RefCell<Weak<ChunkData>>,
    #[cfg(test)]
    chunk_data_build_count: Cell<usize>,
    tag_handler: TagHandler,
    world: Option<Uuid>,
    viewers: HashSet<i32>,
    tickable_blocks: HashMap<BlockPosition, BlockInstance>,
    load_callback: Option<LoadCallback>,
    generation_callback: Option<GenerationCallback>,
    lighting_invalidation_frozen: bool,
    lighting_resend_ticks: Option<u16>,
    sky_occlusion_map: Option<Box<[i32; 256]>>,
    highest_block: i32,
    should_generate: bool,
    has_generated: bool,
    loaded: bool,
    read_only: bool,
    invalidated: Cell<bool>,
}

impl Chunk {
    pub fn new(position: ChunkPosition) -> Self {
        Self::new_with_type(position, true, ChunkType::Dynamic)
    }

    pub fn new_lighting(position: ChunkPosition) -> Self {
        Self::new_with_type(position, true, ChunkType::Lighting)
    }

    pub fn new_with_generation(position: ChunkPosition, should_generate: bool) -> Self {
        Self::new_with_type(position, should_generate, ChunkType::Dynamic)
    }

    pub fn new_lighting_with_generation(position: ChunkPosition, should_generate: bool) -> Self {
        Self::new_with_type(position, should_generate, ChunkType::Lighting)
    }

    fn new_with_type(
        position: ChunkPosition,
        should_generate: bool,
        chunk_type: ChunkType,
    ) -> Self {
        Self {
            identifier: Uuid::new_v4(),
            position,
            chunk_type,
            sections: Arc::new(
                (0..WORLD_SECTION_COUNT)
                    .map(|section_offset| ChunkSection::new(WORLD_MIN_SECTION + section_offset))
                    .collect(),
            ),
            block_instances: HashMap::new(),
            block_entities: Vec::new(),
            motion_blocking_heightmap: RefCell::new(ChunkHeightmaps::from_sections(&[])),
            world_surface_heightmap: RefCell::new(ChunkHeightmaps::from_sections(&[])),
            heightmaps_need_complete_refresh: Cell::new(false),
            cached_chunk_data: RefCell::new(Weak::new()),
            #[cfg(test)]
            chunk_data_build_count: Cell::new(0),
            tag_handler: TagHandler::new_handler(),
            world: None,
            viewers: HashSet::new(),
            tickable_blocks: HashMap::new(),
            load_callback: None,
            generation_callback: None,
            lighting_invalidation_frozen: false,
            lighting_resend_ticks: None,
            sky_occlusion_map: None,
            highest_block: WORLD_MIN_SECTION * CHUNK_SECTION_SIZE - 1,
            should_generate,
            has_generated: false,
            loaded: true,
            read_only: false,
            invalidated: Cell::new(false),
        }
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
        self.sections.as_slice()
    }

    pub(crate) fn snapshot_sections(&self) -> Arc<Vec<ChunkSection>> {
        Arc::clone(&self.sections)
    }

    #[cfg(test)]
    pub(crate) fn section_storage_address(&self) -> usize {
        Arc::as_ptr(&self.sections) as usize
    }

    pub fn section(&self, section_y: i32) -> Option<&ChunkSection> {
        self.sections.iter().find(|section| section.y == section_y)
    }

    pub fn section_mut(&mut self, section_y: i32) -> Option<&mut ChunkSection> {
        if self.read_only {
            return None;
        }
        self.invalidate();
        Arc::make_mut(&mut self.sections)
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
            chunk_type: self.chunk_type,
            sections: Arc::new(self.sections.as_ref().clone()),
            block_instances: self.block_instances.clone(),
            block_entities: self.block_entities.clone(),
            motion_blocking_heightmap: RefCell::new(
                self.motion_blocking_heightmap.borrow().clone(),
            ),
            world_surface_heightmap: RefCell::new(self.world_surface_heightmap.borrow().clone()),
            heightmaps_need_complete_refresh: Cell::new(
                self.heightmaps_need_complete_refresh.get(),
            ),
            cached_chunk_data: RefCell::new(self.cached_chunk_data.borrow().clone()),
            #[cfg(test)]
            chunk_data_build_count: Cell::new(self.chunk_data_build_count.get()),
            tag_handler: self.tag_handler.readable_copy(),
            world: self.world,
            viewers: self.viewers.clone(),
            tickable_blocks: self.tickable_blocks.clone(),
            load_callback: self.load_callback.clone(),
            generation_callback: self.generation_callback.clone(),
            lighting_invalidation_frozen: self.lighting_invalidation_frozen,
            lighting_resend_ticks: self.lighting_resend_ticks,
            sky_occlusion_map: self.sky_occlusion_map.clone(),
            highest_block: self.highest_block,
            should_generate: self.should_generate,
            has_generated: self.has_generated,
            loaded: self.loaded,
            read_only: self.read_only,
            invalidated: Cell::new(self.invalidated.get()),
        }
    }

    pub fn reset(&mut self) {
        if self.read_only {
            return;
        }
        self.sections = Arc::new(
            (0..WORLD_SECTION_COUNT)
                .map(|section_offset| ChunkSection::new(WORLD_MIN_SECTION + section_offset))
                .collect(),
        );
        self.block_instances.clear();
        self.block_entities.clear();
        self.tickable_blocks.clear();
        self.has_generated = false;
        self.sky_occlusion_map = None;
        self.highest_block = WORLD_MIN_SECTION * CHUNK_SECTION_SIZE - 1;
        self.reset_heightmaps();
        self.loaded = true;
        self.invalidate();
    }

    pub fn block(&self, position: BlockPosition) -> Block {
        self.block_state(position).block()
    }

    pub fn block_with_condition(
        &self,
        position: BlockPosition,
        condition: BlockLookupCondition,
    ) -> Option<Block> {
        self.block_instance_with_condition(position, condition)
            .map(|block_instance| block_instance.block())
    }

    pub fn block_instance_with_condition(
        &self,
        position: BlockPosition,
        condition: BlockLookupCondition,
    ) -> Option<BlockInstance> {
        let cached_block = self.block_instances.get(&position).cloned();
        match condition {
            BlockLookupCondition::None => {
                cached_block.or_else(|| Some(self.block(position).into()))
            }
            BlockLookupCondition::Cached => cached_block,
            BlockLookupCondition::Type => Some(self.block(position).into()),
        }
    }

    pub fn block_state(&self, position: BlockPosition) -> BlockState {
        self.section(position.y.div_euclid(CHUNK_SECTION_SIZE))
            .and_then(|section| {
                section.block_state(
                    position.x.rem_euclid(CHUNK_SIZE_X),
                    position.y.rem_euclid(CHUNK_SECTION_SIZE),
                    position.z.rem_euclid(CHUNK_SIZE_Z),
                )
            })
            .unwrap_or_else(|| Block::AIR.default_state())
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

    pub fn set_block_instance(
        &mut self,
        position: BlockPosition,
        block_instance: BlockInstance,
    ) -> bool {
        self.try_set_block_instance(position, block_instance)
            .block_was_set()
    }

    pub fn set_block_state(&mut self, position: BlockPosition, block_state: BlockState) -> bool {
        self.try_set_block_state(position, block_state)
            .block_was_set()
    }

    pub fn try_set_block_state(
        &mut self,
        position: BlockPosition,
        block_state: BlockState,
    ) -> SetChunkBlockResult {
        self.try_set_block_instance(position, block_state.into())
    }

    pub fn try_set_block(&mut self, position: BlockPosition, block: Block) -> SetChunkBlockResult {
        self.try_set_block_instance(position, block.into())
    }

    pub fn try_set_block_instance(
        &mut self,
        position: BlockPosition,
        block_instance: BlockInstance,
    ) -> SetChunkBlockResult {
        self.try_set_block_instance_with_handler(position, block_instance, None, None, None)
    }

    pub(crate) fn try_set_block_instance_with_handler(
        &mut self,
        position: BlockPosition,
        mut block_instance: BlockInstance,
        handlers: Option<&BlockHandlerRegistry>,
        placement: Option<BlockHandlerPlacement>,
        destroy: Option<BlockHandlerDestroy>,
    ) -> SetChunkBlockResult {
        if self.read_only {
            return SetChunkBlockResult::ReadOnly;
        }
        if block_instance.handler().is_none() {
            if let Some(handler) =
                handlers.and_then(|handlers| handlers.handler_for_block(block_instance.block()))
            {
                block_instance = block_instance.with_handler(Some(handler));
            }
        }
        let previous_block_instance = self
            .block_instance_with_condition(position, BlockLookupCondition::None)
            .unwrap_or_else(|| self.block(position).into());
        let block_state = block_instance.block_state();
        let block = block_instance.block();
        self.sky_occlusion_map = None;
        let Some(section) = self.section_mut(position.y.div_euclid(CHUNK_SECTION_SIZE)) else {
            return SetChunkBlockResult::OutsideHeight;
        };
        let block_was_set = section.set_block_state(
            position.x.rem_euclid(CHUNK_SIZE_X),
            position.y.rem_euclid(CHUNK_SECTION_SIZE),
            position.z.rem_euclid(CHUNK_SIZE_Z),
            block_state,
        );
        if block_was_set {
            section.invalidate_sky_light();
            section.invalidate_block_light();
            self.store_block_instance(position, block_instance.clone());
            self.synchronize_block_entity(position, &block_instance);
            self.update_tickable_block(position, &block_instance);
            self.refresh_heightmaps_after_block_change(position, block);
            self.invalidate();
            self.dispatch_block_handler_destroy(
                &previous_block_instance,
                &block_instance,
                position,
                destroy,
            );
            self.dispatch_block_handler_place(
                &previous_block_instance,
                &block_instance,
                position,
                placement,
            );
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
        Arc::make_mut(&mut self.sections)
            .iter_mut()
            .for_each(|section| section.fill_biome(biome.clone()));
        self.invalidate();
    }

    pub fn data(&self, registries: &Registries) -> Result<Arc<ChunkData>, ChunkDataError> {
        if let Some(chunk_data) = self.cached_chunk_data.borrow().upgrade() {
            return Ok(chunk_data);
        }
        let chunk_data = Arc::new(ChunkData {
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
        });
        #[cfg(test)]
        self.chunk_data_build_count
            .set(self.chunk_data_build_count.get() + 1);
        *self.cached_chunk_data.borrow_mut() = Arc::downgrade(&chunk_data);
        self.invalidated.set(false);
        Ok(chunk_data)
    }

    #[cfg(test)]
    pub(crate) fn chunk_data_build_count(&self) -> usize {
        self.chunk_data_build_count.get()
    }

    pub fn light_data(&self) -> LightData {
        if !self.is_lighting_chunk() {
            return ChunkLighting::empty_data(self.sections.len());
        }
        ChunkLighting::from_sections(&self.sections).data()
    }

    pub fn partial_light_data(&mut self) -> Option<LightData> {
        ChunkLighting::partial_data(Arc::make_mut(&mut self.sections).as_mut_slice())
    }

    pub(crate) fn schedule_lighting_update(&mut self) {
        if !self.is_lighting_chunk() {
            return;
        }
        self.schedule_lighting_update_after(LIGHTING_RESEND_DELAY_TICKS);
    }

    pub(crate) fn schedule_generated_lighting_update(&mut self) {
        if !self.is_lighting_chunk() {
            return;
        }
        self.schedule_lighting_update_after(GENERATED_LIGHTING_RESEND_DELAY_TICKS);
    }

    pub fn set_freeze_lighting_invalidation(&mut self, should_freeze: bool) {
        if !self.is_lighting_chunk() {
            return;
        }
        self.lighting_invalidation_frozen = should_freeze;
    }

    pub const fn is_lighting_invalidation_frozen(&self) -> bool {
        self.lighting_invalidation_frozen
    }

    pub(crate) fn tick_lighting(&mut self) -> Option<LightData> {
        let remaining_ticks = self.lighting_resend_ticks?;
        if remaining_ticks > 1 {
            self.lighting_resend_ticks = Some(remaining_ticks - 1);
            return None;
        }
        self.lighting_resend_ticks = None;
        self.partial_light_data()
    }

    pub(crate) fn lighting_update_is_due(&self) -> bool {
        self.is_lighting_chunk() && self.lighting_resend_ticks == Some(1)
    }

    pub(crate) fn lighting_is_invalidated(&self) -> bool {
        self.sections.iter().any(|section| {
            section.sky_light_is_invalidated() || section.block_light_is_invalidated()
        })
    }

    pub(crate) fn clear_light(&mut self) {
        Arc::make_mut(&mut self.sections)
            .iter_mut()
            .for_each(ChunkSection::clear_light);
    }

    pub(crate) fn set_block_light(&mut self, position: BlockPosition, level: u8) {
        let Some(section) = self.section_at_block_y_mut(position.y) else {
            return;
        };
        section.set_block_light_level(
            position.x.rem_euclid(CHUNK_SIZE_X),
            position.y.rem_euclid(CHUNK_SECTION_SIZE),
            position.z.rem_euclid(CHUNK_SIZE_Z),
            level,
        );
    }

    pub(crate) fn set_sky_light(&mut self, position: BlockPosition, level: u8) {
        let Some(section) = self.section_at_block_y_mut(position.y) else {
            return;
        };
        section.set_sky_light_level(
            position.x.rem_euclid(CHUNK_SIZE_X),
            position.y.rem_euclid(CHUNK_SECTION_SIZE),
            position.z.rem_euclid(CHUNK_SIZE_Z),
            level,
        );
    }

    pub(crate) fn validate_light(&mut self) {
        Arc::make_mut(&mut self.sections)
            .iter_mut()
            .for_each(ChunkSection::validate_light);
    }

    fn schedule_lighting_update_after(&mut self, delay_ticks: u16) {
        if self.lighting_invalidation_frozen {
            return;
        }
        self.lighting_resend_ticks = Some(delay_ticks);
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

    pub fn sky_occlusion_map(&mut self) -> &[i32; 256] {
        if self.sky_occlusion_map.is_none() {
            let minimum_y = self.min_section() * CHUNK_SECTION_SIZE;
            let maximum_y = self.max_section() * CHUNK_SECTION_SIZE - 1;
            let mut occlusion_map = [minimum_y; 256];
            let mut highest_block = minimum_y - 1;
            (0..CHUNK_SIZE_X).for_each(|local_x| {
                (0..CHUNK_SIZE_Z).for_each(|local_z| {
                    let global_x = self.x() * CHUNK_SIZE_X + local_x;
                    let global_z = self.z() * CHUNK_SIZE_Z + local_z;
                    let mut occlusion_height = minimum_y;
                    for y in (minimum_y..=maximum_y).rev() {
                        let state = self.block_state(BlockPosition::new(global_x, y, global_z));
                        if !state.block().is_air() {
                            highest_block = highest_block.max(y);
                        }
                        if state.propagates_skylight_down() {
                            continue;
                        }
                        occlusion_height = y + 1;
                        break;
                    }
                    occlusion_map[(local_z * CHUNK_SIZE_X + local_x) as usize] = occlusion_height;
                });
            });
            self.sky_occlusion_map = Some(Box::new(occlusion_map));
            self.highest_block = highest_block;
        }
        self.sky_occlusion_map
            .as_ref()
            .unwrap_or_else(|| unreachable!())
    }

    pub const fn highest_block(&self) -> i32 {
        self.highest_block
    }

    pub fn relight_block_light_at(&mut self, block_y: i32) -> bool {
        let Some(section) = self.section_at_block_y_mut(block_y) else {
            return false;
        };
        let light_was_recalculated = section.relight_block_light();
        if light_was_recalculated {
            self.invalidate();
        }
        light_was_recalculated
    }

    pub fn relight_sky_light_at(&mut self, block_y: i32) -> bool {
        let Some(section) = self.section_at_block_y_mut(block_y) else {
            return false;
        };
        let light_was_recalculated = section.relight_sky_light();
        if light_was_recalculated {
            self.invalidate();
        }
        light_was_recalculated
    }

    pub fn invalidate_section(&mut self, section_y: i32) -> bool {
        let Some(section) = Arc::make_mut(&mut self.sections)
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

    pub fn is_lighting_chunk(&self) -> bool {
        self.chunk_type == ChunkType::Lighting
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
        self.sections = Arc::new(sections);
        self.heightmaps_need_complete_refresh.set(true);
        self.invalidate();
    }

    pub(crate) fn merge_section_from_fork(&mut self, section_y: i32, fork_section: &ChunkSection) {
        let Some(chunk_section) = Arc::make_mut(&mut self.sections)
            .iter_mut()
            .find(|chunk_section| chunk_section.y == section_y)
        else {
            return;
        };
        chunk_section.merge_from_fork(fork_section);
        self.heightmaps_need_complete_refresh.set(true);
        self.invalidate();
    }

    pub fn set_block_entity(&mut self, block_entity: BlockEntity) {
        if self.read_only {
            return;
        }
        let position = block_entity.position();
        let Some(block_instance) =
            self.block_instance_with_condition(position, BlockLookupCondition::None)
        else {
            return;
        };
        if block_instance.block().block_entity_type() != Some(block_entity.block_entity_type()) {
            return;
        }
        self.block_instances.insert(
            position,
            block_instance.with_nbt(Some(block_entity.nbt().clone())),
        );
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
        if let Some(block_instance) = self.block_instances.get(&position).cloned() {
            self.block_instances
                .insert(position, block_instance.with_nbt(None));
        }
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

    pub fn tick(&self, world: Uuid, _handlers: &BlockHandlerRegistry, _time: u64) -> usize {
        self.tickable_blocks
            .iter()
            .filter_map(|(position, block_instance)| {
                block_instance
                    .handler()
                    .cloned()
                    .map(|handler| (handler, block_instance.block(), *position))
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
        self.invalidated.set(true);
        *self.cached_chunk_data.borrow_mut() = Weak::new();
    }

    pub fn is_invalidated(&self) -> bool {
        self.invalidated.get()
    }

    pub(crate) fn clear_invalidated(&self) {
        self.invalidated.set(false);
    }

    pub fn should_generate(&self) -> bool {
        self.should_generate
    }

    pub(crate) fn requires_generation(&self) -> bool {
        self.should_generate && !self.has_generated
    }

    pub(crate) fn requires_generation_completion(&self) -> bool {
        !self.has_generated
    }

    pub(crate) fn mark_loaded_from_storage(&mut self) {
        self.has_generated = true;
    }

    pub fn on_load(&mut self) {
        let load_callback = self.load_callback.clone();
        if let Some(load_callback) = load_callback {
            load_callback(self);
        }
    }

    pub fn set_load_callback(
        &mut self,
        load_callback: impl Fn(&mut Chunk) + Send + Sync + 'static,
    ) {
        self.load_callback = Some(Arc::new(load_callback));
    }

    pub fn clear_load_callback(&mut self) {
        self.load_callback = None;
    }

    pub fn on_generate(&mut self) {
        self.mark_generated();
        let generation_callback = self.generation_callback.clone();
        if let Some(generation_callback) = generation_callback {
            generation_callback(self);
        }
    }

    pub fn set_generation_callback(
        &mut self,
        generation_callback: impl Fn(&mut Chunk) + Send + Sync + 'static,
    ) {
        self.generation_callback = Some(Arc::new(generation_callback));
    }

    pub fn clear_generation_callback(&mut self) {
        self.generation_callback = None;
    }

    pub fn motion_blocking_heightmap(&self) -> Vec<i64> {
        self.refresh_heightmaps_if_needed();
        self.motion_blocking_heightmap.borrow().packed_heights()
    }

    pub fn world_surface_heightmap(&self) -> Vec<i64> {
        self.refresh_heightmaps_if_needed();
        self.world_surface_heightmap.borrow().packed_heights()
    }

    pub fn load_heightmaps_from_nbt(&mut self, heightmaps: &NbtCompound) {
        if let Some(Nbt::LongArray(heights)) = heightmaps.get("MOTION_BLOCKING") {
            self.motion_blocking_heightmap
                .borrow_mut()
                .load_motion_blocking_from_longs(heights);
        }
        if let Some(Nbt::LongArray(heights)) = heightmaps.get("WORLD_SURFACE") {
            self.world_surface_heightmap
                .borrow_mut()
                .load_world_surface_from_longs(heights);
        }
        self.heightmaps_need_complete_refresh.set(false);
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

    fn heightmaps(&self) -> Vec<HeightmapEntry> {
        self.refresh_heightmaps_if_needed();
        ChunkHeightmaps::entries(
            &self.motion_blocking_heightmap.borrow(),
            &self.world_surface_heightmap.borrow(),
        )
    }

    fn refresh_heightmaps_if_needed(&self) {
        if !self.heightmaps_need_complete_refresh.get() {
            return;
        }
        let refreshed_heightmaps = ChunkHeightmaps::from_sections(&self.sections);
        *self.motion_blocking_heightmap.borrow_mut() = refreshed_heightmaps.clone();
        *self.world_surface_heightmap.borrow_mut() = refreshed_heightmaps;
        self.heightmaps_need_complete_refresh.set(false);
    }

    fn refresh_heightmaps_after_block_change(&mut self, position: BlockPosition, block: Block) {
        self.refresh_heightmaps_if_needed();
        self.motion_blocking_heightmap
            .borrow_mut()
            .refresh_block(&self.sections, position, block);
        self.world_surface_heightmap
            .borrow_mut()
            .refresh_block(&self.sections, position, block);
    }

    fn reset_heightmaps(&mut self) {
        let empty_heightmaps = ChunkHeightmaps::from_sections(&[]);
        *self.motion_blocking_heightmap.borrow_mut() = empty_heightmaps.clone();
        *self.world_surface_heightmap.borrow_mut() = empty_heightmaps;
        self.heightmaps_need_complete_refresh.set(false);
    }

    fn store_block_instance(&mut self, position: BlockPosition, block_instance: BlockInstance) {
        let should_store = block_instance.has_nbt()
            || block_instance.handler().is_some()
            || block_instance.block().block_entity_type().is_some();
        if should_store {
            self.block_instances.insert(position, block_instance);
            return;
        }
        self.block_instances.remove(&position);
    }

    fn synchronize_block_entity(
        &mut self,
        position: BlockPosition,
        block_instance: &BlockInstance,
    ) {
        self.block_entities
            .retain(|block_entity| block_entity.position() != position);
        let Some(block_entity_type) = block_instance.block().block_entity_type() else {
            return;
        };
        self.block_entities.push(BlockEntity::new(
            position,
            block_entity_type,
            block_instance.nbt_or_empty(),
        ));
    }

    fn update_tickable_block(&mut self, position: BlockPosition, block_instance: &BlockInstance) {
        if block_instance
            .handler()
            .is_some_and(|handler| handler.is_tickable())
        {
            self.tickable_blocks
                .insert(position, block_instance.clone());
            return;
        }
        self.tickable_blocks.remove(&position);
    }

    fn dispatch_block_handler_destroy(
        &self,
        previous_block: &BlockInstance,
        new_block: &BlockInstance,
        position: BlockPosition,
        destroy: Option<BlockHandlerDestroy>,
    ) {
        let Some(handler) = previous_block.handler() else {
            return;
        };
        let world = self.world.unwrap_or_default();
        handler.on_destroy(destroy.unwrap_or_else(|| {
            BlockHandlerDestroy::new(
                previous_block.block(),
                new_block.block(),
                world,
                position,
                None,
            )
        }));
    }

    fn dispatch_block_handler_place(
        &self,
        previous_block: &BlockInstance,
        block: &BlockInstance,
        position: BlockPosition,
        placement: Option<BlockHandlerPlacement>,
    ) {
        let Some(handler) = block.handler() else {
            return;
        };
        let world = self.world.unwrap_or_default();
        handler.on_place(placement.unwrap_or_else(|| {
            BlockHandlerPlacement::new(block.block_state(), previous_block.block(), world, position)
        }));
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
