use crate::entity::{Entity, Player, PlayerChunk};
use crate::network::client::instance::Client;
use crate::world::generator::{GenerationFork, Generator};
use crate::world::{
    BlockPosition, BlockSize, Chunk, ChunkLoader, ChunkPosition, GenerationUnit, NoopChunkLoader,
};
use spinel_core::network::clientbound::play::chunk_data::ChunkDataAndUpdateLightPacket;
use spinel_network::types::Identifier;
use spinel_registry::Registries;
use std::collections::{HashMap, hash_map::Entry};
use std::io::{Error, ErrorKind, Result};
use std::net::SocketAddr;
use uuid::Uuid;

pub struct World {
    pub uuid: Uuid,
    pub name: Identifier,
    entities: Vec<Entity>,
    chunks: HashMap<ChunkPosition, Chunk>,
    pending_generation: HashMap<ChunkPosition, Vec<GenerationFork>>,
    generator: Option<Box<dyn Generator + Send + Sync>>,
    chunk_loader: Box<dyn ChunkLoader>,
}

impl World {
    pub fn new(name: Identifier) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
            entities: Vec::new(),
            chunks: HashMap::new(),
            pending_generation: HashMap::new(),
            generator: None,
            chunk_loader: Box::new(NoopChunkLoader),
        }
    }
    pub fn generator(&self) -> Option<&(dyn Generator + Send + Sync)> {
        self.generator.as_deref()
    }

    pub fn set_generator(
        &mut self,
        generator: impl Fn(&mut GenerationUnit) + Send + Sync + 'static,
    ) {
        self.generator = Some(Box::new(generator));
    }
    pub fn clear_generator(&mut self) {
        self.generator = None;
    }
    pub fn set_chunk_loader(&mut self, chunk_loader: impl ChunkLoader + 'static) {
        self.chunk_loader = Box::new(chunk_loader);
    }
    pub fn chunk(&self, position: ChunkPosition) -> Option<&Chunk> {
        self.chunks.get(&position)
    }
    pub fn load_chunk(&mut self, position: ChunkPosition) -> &mut Chunk {
        if !self.chunks.contains_key(&position) {
            let chunk = self
                .chunk_loader
                .load_chunk(position)
                .ok()
                .flatten()
                .unwrap_or_else(|| Chunk::new(position));
            self.chunks.insert(position, chunk);
        }
        self.generate_chunk(position);
        match self.chunks.entry(position) {
            Entry::Occupied(chunk_entry) => chunk_entry.into_mut(),
            Entry::Vacant(chunk_entry) => chunk_entry.insert(Chunk::new(position)),
        }
    }
    pub fn regenerate_chunk(&mut self, position: ChunkPosition) {
        if let Some(chunk) = self.chunks.get_mut(&position) {
            chunk.mark_not_generated();
            chunk.clear_invalidated();
        }
        self.generate_chunk(position);
    }
    pub fn generate_chunk(&mut self, position: ChunkPosition) {
        let Some(mut chunk) = self.chunks.remove(&position) else {
            return;
        };
        self.apply_generator(&mut chunk);
        self.chunks.insert(position, chunk);
    }

    pub(crate) fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub(crate) fn enter_player(
        &mut self,
        client: &mut Client,
        ticks_per_second: u32,
        registries: &Registries,
    ) -> Result<()> {
        let chunks = match self.player_by_addr(&client.addr) {
            Some(player) => player.spawn_chunks(),
            None => Vec::new(),
        };
        let chunk_packets = self.chunk_packets(chunks, registries)?;
        let world_name = self.name.clone();
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        player.enter_world(client, ticks_per_second, world_name, chunk_packets)
    }
    pub(crate) fn move_player(
        &mut self,
        client: &mut Client,
        x: f64,
        y: f64,
        z: f64,
        registries: &Registries,
    ) -> Result<()> {
        let transition = self
            .player_by_addr(&client.addr)
            .and_then(|player| player.chunk_transition(x, y, z));
        let chunks = match transition.as_ref() {
            Some(transition) => transition.arriving.clone(),
            None => Vec::new(),
        };
        let chunk_packets = self.chunk_packets(chunks, registries)?;
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        player.move_to(client, x, y, z, transition, chunk_packets)
    }

    pub fn tick(&mut self) {
        self.entities.iter_mut().for_each(Entity::tick);
    }

    pub(crate) fn remove_entity_by_addr(&mut self, addr: &SocketAddr) {
        self.entities.retain(|entity| match entity {
            Entity::Player(player) => player.addr != *addr,
        });
    }

    pub(crate) fn player_by_addr_mut(&mut self, addr: &SocketAddr) -> Option<&mut Player> {
        self.entities.iter_mut().find_map(|entity| match entity {
            Entity::Player(player) if player.addr == *addr => Some(player),
            Entity::Player(_) => None,
        })
    }

    pub(crate) fn player_by_addr(&self, addr: &SocketAddr) -> Option<&Player> {
        self.entities.iter().find_map(|entity| match entity {
            Entity::Player(player) if player.addr == *addr => Some(player),
            Entity::Player(_) => None,
        })
    }

    fn apply_generator(&mut self, chunk: &mut Chunk) {
        let Some(generator) = self.generator() else {
            return;
        };
        if !chunk.should_generate() {
            return;
        }
        let size = BlockSize::new(16, (chunk.sections().len() as i32) << 4, 16);
        let start = BlockPosition::new(chunk.x() << 4, -64, chunk.z() << 4);
        let mut unit = GenerationUnit::new(size, start, chunk.sections().to_vec());
        generator.generate(&mut unit);
        let (sections, forks) = unit.into_generation();
        chunk.replace_sections(sections);
        forks
            .into_iter()
            .for_each(|fork| self.store_generation_fork(fork));
        self.apply_pending_generation(chunk);
        chunk.mark_generated();
    }

    fn chunk_packets(
        &mut self,
        chunks: Vec<PlayerChunk>,
        registries: &Registries,
    ) -> Result<Vec<ChunkDataAndUpdateLightPacket>> {
        chunks
            .into_iter()
            .map(|chunk| self.chunk_packet(ChunkPosition::from(chunk), registries))
            .collect()
    }

    fn chunk_packet(
        &mut self,
        position: ChunkPosition,
        registries: &Registries,
    ) -> Result<ChunkDataAndUpdateLightPacket> {
        let chunk = self.load_chunk(position);
        Ok(ChunkDataAndUpdateLightPacket::with_light_data(
            chunk.x(),
            chunk.z(),
            chunk
                .data(registries)
                .map_err(|_| Error::new(ErrorKind::InvalidData, "Chunk has unregistered biome"))?,
            chunk.light_data(),
        ))
    }

    fn store_generation_fork(&mut self, fork: GenerationFork) {
        fork.target_positions().into_iter().for_each(|position| {
            if let Some(chunk) = self.chunks.get_mut(&position) {
                fork.apply_to(chunk);
                return;
            }
            self.pending_generation
                .entry(position)
                .or_default()
                .push(fork.clone());
        });
    }

    fn apply_pending_generation(&mut self, chunk: &mut Chunk) {
        let position = ChunkPosition::new(chunk.x(), chunk.z());
        if let Some(forks) = self.pending_generation.remove(&position) {
            forks.iter().for_each(|fork| fork.apply_to(chunk));
        }
    }
}
