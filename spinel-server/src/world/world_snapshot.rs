use crate::entity::EntityId;
use crate::world::{Biome, Block, BlockPosition, Chunk, ChunkPosition, World};
use spinel_nbt::{TagHandler, Taggable};
use spinel_network::types::Identifier;
use spinel_registry::RegistryKey;
use spinel_registry::dimension_type::DimensionType;
use std::collections::HashMap;
use uuid::Uuid;

pub struct WorldSnapshot {
    world: Uuid,
    name: Identifier,
    dimension_type: RegistryKey<DimensionType>,
    world_age: i64,
    time: i64,
    chunks: HashMap<ChunkPosition, ChunkSnapshot>,
    entity_ids: Vec<EntityId>,
    tag_handler: TagHandler,
}

pub struct ChunkSnapshot {
    position: ChunkPosition,
    chunk: Chunk,
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
        let entity_ids = world.entities().map(|entity| entity.entity_id()).collect();
        Self {
            world: world.uuid(),
            name: world.name().clone(),
            dimension_type: world.dimension_type().clone(),
            world_age: world.world_age(),
            time: world.time(),
            chunks,
            entity_ids,
            tag_handler: world.tag_handler().readable_copy(),
        }
    }

    pub const fn world(&self) -> Uuid {
        self.world
    }

    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn dimension_type(&self) -> &RegistryKey<DimensionType> {
        &self.dimension_type
    }

    pub const fn world_age(&self) -> i64 {
        self.world_age
    }

    pub const fn time(&self) -> i64 {
        self.time
    }

    pub fn chunk(&self, position: ChunkPosition) -> Option<&ChunkSnapshot> {
        self.chunks.get(&position)
    }

    pub fn chunks(&self) -> impl Iterator<Item = &ChunkSnapshot> {
        self.chunks.values()
    }

    pub fn entity_ids(&self) -> &[EntityId] {
        &self.entity_ids
    }
}

impl ChunkSnapshot {
    fn from_chunk(world: &World, chunk: &Chunk) -> Self {
        let position = ChunkPosition::new(chunk.x(), chunk.z());
        let entity_ids = world
            .chunk_entities(position)
            .into_iter()
            .map(|entity| entity.entity_id())
            .collect();
        Self {
            position,
            chunk: chunk.copy_for_position(position),
            entity_ids,
            tag_handler: chunk.tag_handler().readable_copy(),
        }
    }

    pub const fn position(&self) -> ChunkPosition {
        self.position
    }

    pub fn block(&self, position: BlockPosition) -> Block {
        self.chunk.block(position)
    }

    pub fn biome(&self, position: BlockPosition) -> RegistryKey<Biome> {
        self.chunk.biome(position)
    }

    pub fn entity_ids(&self) -> &[EntityId] {
        &self.entity_ids
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
