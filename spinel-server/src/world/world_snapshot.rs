use crate::entity::{EntityId, EntityObservation};
use crate::world::{
    Biome, Block, BlockPosition, BlockState, CHUNK_SECTION_SIZE, CHUNK_SIZE_X, CHUNK_SIZE_Z, Chunk,
    ChunkPosition, ChunkSection, World, WorldBorder,
};
use spinel_nbt::{TagHandler, Taggable};
use spinel_network::types::Identifier;
use spinel_network::types::Vector3d;
use spinel_registry::RegistryKey;
use spinel_registry::dimension_type::DimensionType;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct WorldSnapshot {
    world: Uuid,
    name: Identifier,
    dimension_type: RegistryKey<DimensionType>,
    cached_dimension_type: DimensionType,
    world_age: i64,
    time: i64,
    world_border: WorldBorder,
    chunks: HashMap<ChunkPosition, ChunkSnapshot>,
    entity_ids: Vec<EntityId>,
    entities: Vec<EntityObservation>,
    tag_handler: TagHandler,
}

pub struct ChunkSnapshot {
    position: ChunkPosition,
    sections: Arc<Vec<ChunkSection>>,
    entity_ids: Vec<EntityId>,
    tag_handler: TagHandler,
}

impl WorldSnapshot {
    pub(crate) fn from_world(world: &World) -> Self {
        let chunks = world
            .chunks()
            .map(|chunk| {
                let position = ChunkPosition::new(chunk.x(), chunk.z());
                (position, ChunkSnapshot::from_chunk(world, chunk))
            })
            .collect();
        let entities = world
            .entities()
            .map(|entity| {
                EntityObservation::new(
                    entity.get_entity_id(),
                    entity.get_entity_type(),
                    entity.get_position(),
                    entity.is_removed(),
                )
            })
            .collect::<Vec<_>>();
        let entity_ids = entities.iter().map(|entity| entity.get_entity_id()).collect();
        Self {
            world: world.uuid(),
            name: world.name().clone(),
            dimension_type: world.get_dimension_type().clone(),
            cached_dimension_type: world.cached_dimension_type().clone(),
            world_age: world.world_age(),
            time: world.time(),
            world_border: world.get_world_border(),
            chunks,
            entity_ids,
            entities,
            tag_handler: world.tag_handler().readable_copy(),
        }
    }

    pub const fn world(&self) -> Uuid {
        self.world
    }

    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn get_dimension_type(&self) -> &RegistryKey<DimensionType> {
        &self.dimension_type
    }

    pub const fn cached_dimension_type(&self) -> &DimensionType {
        &self.cached_dimension_type
    }

    pub const fn world_age(&self) -> i64 {
        self.world_age
    }

    pub const fn time(&self) -> i64 {
        self.time
    }

    pub const fn get_world_border(&self) -> WorldBorder {
        self.world_border
    }

    pub fn is_chunk_loaded(&self, position: ChunkPosition) -> bool {
        self.chunks.contains_key(&position)
    }

    pub fn chunk(&self, position: ChunkPosition) -> Option<&ChunkSnapshot> {
        self.chunks.get(&position)
    }

    pub fn chunks(&self) -> impl Iterator<Item = &ChunkSnapshot> {
        self.chunks.values()
    }

    pub fn block(&self, position: BlockPosition) -> Block {
        self.block_state(position).block()
    }

    pub fn block_state(&self, position: BlockPosition) -> BlockState {
        self.chunks
            .get(&ChunkPosition::from(position))
            .map(|chunk| chunk.block_state(position))
            .unwrap_or_else(|| Block::AIR.default_state())
    }

    pub fn entity_ids(&self) -> &[EntityId] {
        &self.entity_ids
    }

    pub fn entities(&self) -> &[EntityObservation] {
        &self.entities
    }

    pub fn get_entity(&self, entity_id: EntityId) -> Option<EntityObservation> {
        self.entities
            .iter()
            .copied()
            .find(|entity| entity.get_entity_id() == entity_id)
    }

    pub fn has_line_of_sight(&self, source: EntityId, target: EntityId) -> bool {
        let Some(source) = self.get_entity(source) else {
            return false;
        };
        let Some(target) = self.get_entity(target) else {
            return false;
        };
        let source_eye = eye_position(source);
        let target_eye = eye_position(target);
        let delta_x = target_eye.x - source_eye.x;
        let delta_y = target_eye.y - source_eye.y;
        let delta_z = target_eye.z - source_eye.z;
        let distance = (delta_x * delta_x + delta_y * delta_y + delta_z * delta_z).sqrt();
        if distance == 0.0 {
            return true;
        }
        let direction = Vector3d {
            x: delta_x / distance,
            y: delta_y / distance,
            z: delta_z / distance,
        };
        let step_count = (distance * 4.0).ceil() as i32;
        !(0..=step_count)
            .map(|step| f64::from(step) * 0.25)
            .map(|ray_distance| {
                BlockPosition::new(
                    (source_eye.x + direction.x * ray_distance).floor() as i32,
                    (source_eye.y + direction.y * ray_distance).floor() as i32,
                    (source_eye.z + direction.z * ray_distance).floor() as i32,
                )
            })
            .any(|position| self.block(position) != Block::AIR)
    }
}

fn eye_position(entity: EntityObservation) -> Vector3d {
    Vector3d {
        x: entity.get_position().get_x(),
        y: entity.get_position().get_y() + entity.get_entity_type().get_eye_height(),
        z: entity.get_position().get_z(),
    }
}

impl ChunkSnapshot {
    fn from_chunk(world: &World, chunk: &Chunk) -> Self {
        let position = ChunkPosition::new(chunk.x(), chunk.z());
        let entity_ids = world
            .chunk_entities(position)
            .into_iter()
            .map(|entity| entity.get_entity_id())
            .collect();
        Self {
            position,
            sections: chunk.snapshot_sections(),
            entity_ids,
            tag_handler: chunk.tag_handler().readable_copy(),
        }
    }

    pub const fn position(&self) -> ChunkPosition {
        self.position
    }

    pub fn block(&self, position: BlockPosition) -> Block {
        self.block_state(position).block()
    }

    pub fn block_state(&self, position: BlockPosition) -> BlockState {
        self.sections
            .iter()
            .find(|section| section.y == position.y.div_euclid(CHUNK_SECTION_SIZE))
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
        self.sections
            .iter()
            .find(|section| section.y == position.y.div_euclid(CHUNK_SECTION_SIZE))
            .and_then(|section| {
                section.biome(
                    position.x.rem_euclid(CHUNK_SIZE_X) >> 2,
                    position.y.rem_euclid(CHUNK_SECTION_SIZE) >> 2,
                    position.z.rem_euclid(CHUNK_SIZE_Z) >> 2,
                )
            })
            .unwrap_or(Biome::PLAINS)
    }

    pub fn entity_ids(&self) -> &[EntityId] {
        &self.entity_ids
    }

    #[cfg(test)]
    pub(crate) fn section_storage_address(&self) -> usize {
        Arc::as_ptr(&self.sections) as usize
    }
}

impl Taggable for WorldSnapshot {
    fn tag_handler(&self) -> &TagHandler {
        &self.tag_handler
    }

    fn tag_handler_mut(&mut self) -> &mut TagHandler {
        &mut self.tag_handler
    }
}

impl Taggable for ChunkSnapshot {
    fn tag_handler(&self) -> &TagHandler {
        &self.tag_handler
    }

    fn tag_handler_mut(&mut self) -> &mut TagHandler {
        &mut self.tag_handler
    }
}
