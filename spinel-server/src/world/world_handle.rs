use crate::entity::{EntityId, EntityPosition};
use crate::server::MinecraftServer;
use crate::world::{Block, BlockPosition, ChunkPosition};
use spinel_nbt::NbtCompound;
use spinel_registry::EntityType;
use std::io;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldHandle {
    server: usize,
    uuid: Uuid,
}

impl WorldHandle {
    pub(crate) const fn new(server: usize, uuid: Uuid) -> Self {
        Self { server, uuid }
    }

    pub const fn uuid(self) -> Uuid {
        self.uuid
    }

    pub fn spawn_entity(
        self,
        entity_type: EntityType,
        position: EntityPosition,
        nbt: Option<&NbtCompound>,
    ) -> io::Result<EntityId> {
        let server = unsafe { &mut *(self.server as *mut MinecraftServer) };
        let Some(world) = server.world_manager.world_mut(self.uuid) else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "World not found."));
        };
        world.spawn_entity(entity_type, position, nbt)
    }

    pub fn load_chunk(self, position: ChunkPosition) -> io::Result<bool> {
        let server = unsafe { &mut *(self.server as *mut MinecraftServer) };
        let Some(world) = server.world_manager.world_mut(self.uuid) else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "World not found."));
        };
        world.set_event_dispatcher(self.server);
        world.load_chunk_result(position)?;
        Ok(true)
    }

    pub fn load_optional_chunk(self, position: ChunkPosition) -> io::Result<bool> {
        let server = unsafe { &mut *(self.server as *mut MinecraftServer) };
        let Some(world) = server.world_manager.world_mut(self.uuid) else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "World not found."));
        };
        world.set_event_dispatcher(self.server);
        Ok(world.load_optional_chunk_result(position)?.is_some())
    }

    pub fn unload_chunk(self, position: ChunkPosition) -> io::Result<bool> {
        let server = unsafe { &mut *(self.server as *mut MinecraftServer) };
        let Some(world) = server.world_manager.world_mut(self.uuid) else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "World not found."));
        };
        world.set_event_dispatcher(self.server);
        world.unload_chunk(position)
    }

    pub fn block_at(self, position: BlockPosition) -> io::Result<Block> {
        let server = unsafe { &mut *(self.server as *mut MinecraftServer) };
        let Some(world) = server.world_manager.world_mut(self.uuid) else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "World not found."));
        };
        world.block_at(position)
    }

    pub fn loaded_block_at(self, position: BlockPosition) -> io::Result<Option<Block>> {
        let server = unsafe { &mut *(self.server as *mut MinecraftServer) };
        let Some(world) = server.world_manager.world(self.uuid) else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "World not found."));
        };
        Ok(world.loaded_block_at(position))
    }

    pub fn block_position_is_loaded(self, position: BlockPosition) -> io::Result<bool> {
        let server = unsafe { &mut *(self.server as *mut MinecraftServer) };
        let Some(world) = server.world_manager.world(self.uuid) else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "World not found."));
        };
        Ok(world.block_position_is_loaded(position))
    }
}
