use crate::entity::{Entity, EntityTickContext, Player, PlayerChunk};
use crate::network::client::instance::Client;
use crate::network::connection_manager::ConnectionManager;
use crate::world::generator::{GenerationFork, Generator};
use crate::world::{BlockPosition, BlockSize, Chunk, ChunkPosition, GenerationUnit};
use spinel_core::network::clientbound::play::chunk_data::ChunkDataAndUpdateLightPacket;
use spinel_network::types::Identifier;
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

    pub fn chunk(&self, position: ChunkPosition) -> Option<&Chunk> {
        self.chunks.get(&position)
    }

    pub fn load_chunk(&mut self, position: ChunkPosition) -> &mut Chunk {
        self.chunks
            .entry(position)
            .or_insert_with(|| Chunk::new(position));
        self.generate_chunk(position);
        match self.chunks.entry(position) {
            Entry::Occupied(chunk_entry) => chunk_entry.into_mut(),
            Entry::Vacant(chunk_entry) => chunk_entry.insert(Chunk::new(position)),
        }
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
    ) -> Result<()> {
        let chunks = match self.player_by_addr(&client.addr) {
            Some(player) => player.spawn_chunks(),
            None => Vec::new(),
        };
        let chunk_packets = self.chunk_packets(chunks);
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
    ) -> Result<()> {
        let transition = self
            .player_by_addr(&client.addr)
            .and_then(|player| player.chunk_transition(x, y, z));
        let chunks = match transition.as_ref() {
            Some(transition) => transition.arriving.clone(),
            None => Vec::new(),
        };
        let chunk_packets = self.chunk_packets(chunks);
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        player.move_to(client, x, y, z, transition, chunk_packets)
    }

    pub fn tick(&mut self, connections: &ConnectionManager) -> Vec<SocketAddr> {
        let entity_tick_context = EntityTickContext { connections };
        self.entities
            .iter_mut()
            .filter_map(|entity| entity.tick(&entity_tick_context))
            .collect()
    }

    pub(crate) fn sync_ticks(&mut self, connections: &ConnectionManager, ticks_per_second: u32) {
        let entity_tick_context = EntityTickContext { connections };
        self.entities.iter_mut().for_each(|entity| match entity {
            Entity::Player(player) => player.sync_tick_rate(&entity_tick_context, ticks_per_second),
        });
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

    fn chunk_packets(&mut self, chunks: Vec<PlayerChunk>) -> Vec<ChunkDataAndUpdateLightPacket> {
        chunks
            .into_iter()
            .map(|chunk| self.chunk_packet(ChunkPosition::from(chunk)))
            .collect()
    }

    fn chunk_packet(&mut self, position: ChunkPosition) -> ChunkDataAndUpdateLightPacket {
        let chunk = self.load_chunk(position);
        ChunkDataAndUpdateLightPacket::new(chunk.x(), chunk.z(), chunk.data())
    }

    fn store_generation_fork(&mut self, fork: GenerationFork) {
        fork.target_positions().into_iter().for_each(|position| {
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
