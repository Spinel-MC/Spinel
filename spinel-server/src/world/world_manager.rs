use crate::entity::Entity;
use crate::entity::{Player, PlayerHand};
use crate::network::client::instance::Client;
use crate::world::{ChunkLoader, World, WorldHandle};
use spinel_network::types::Identifier;
use spinel_registry::dimension_type::DimensionType;
use spinel_registry::{Registries, RegistryKey};
use std::io;
use std::net::SocketAddr;
use uuid::Uuid;

pub struct WorldManager {
    worlds: Vec<World>,
}

impl WorldManager {
    pub fn new() -> Self {
        Self { worlds: Vec::new() }
    }

    pub fn create_world(&mut self, name: Identifier) -> Uuid {
        let world = World::new(name);
        let world_uuid = world.uuid;
        self.register_world(world);
        world_uuid
    }

    pub fn create_world_with_loader(
        &mut self,
        name: Identifier,
        chunk_loader: impl ChunkLoader + 'static,
    ) -> Uuid {
        let mut world = World::new(name);
        world.set_chunk_loader(chunk_loader);
        let world_uuid = world.uuid;
        self.register_world(world);
        world_uuid
    }

    pub fn create_world_with_dimension(
        &mut self,
        dimension_type: RegistryKey<DimensionType>,
        cached_dimension_type: DimensionType,
    ) -> Uuid {
        self.create_world_with_dimension_and_loader(
            dimension_type,
            cached_dimension_type,
            crate::world::NoopChunkLoader,
        )
    }

    pub fn create_world_with_dimension_and_loader(
        &mut self,
        dimension_type: RegistryKey<DimensionType>,
        cached_dimension_type: DimensionType,
        chunk_loader: impl ChunkLoader + 'static,
    ) -> Uuid {
        let dimension_name = dimension_type.key().clone();
        let mut world =
            World::new_with_dimension(dimension_name, dimension_type, cached_dimension_type);
        world.set_chunk_loader(chunk_loader);
        let world_uuid = world.uuid;
        self.register_world(world);
        world_uuid
    }

    pub fn register_world(&mut self, mut world: World) {
        world.set_registered(true);
        self.worlds.push(world);
    }

    pub fn unregister_world(&mut self, world_uuid: Uuid) -> io::Result<Option<World>> {
        let Some(world_index) = self
            .worlds
            .iter()
            .position(|world| world.uuid == world_uuid)
        else {
            return Ok(None);
        };
        let chunk_positions = self.worlds[world_index]
            .chunks()
            .map(|chunk| crate::world::ChunkPosition::new(chunk.x(), chunk.z()))
            .collect::<Vec<_>>();
        for chunk_position in chunk_positions {
            self.worlds[world_index].unload_chunk(chunk_position)?;
        }
        self.worlds[world_index].set_registered(false);
        Ok(Some(self.worlds.remove(world_index)))
    }

    pub fn entity_world(
        &self,
        server: usize,
        entity_id: crate::entity::EntityId,
    ) -> Option<WorldHandle> {
        self.worlds
            .iter()
            .find(|world| world.entity_by_id(entity_id).is_some())
            .map(|world| WorldHandle::new(server, world.uuid()))
    }

    pub fn world(&self, world_uuid: Uuid) -> Option<&World> {
        self.worlds.iter().find(|world| world.uuid == world_uuid)
    }

    pub fn world_mut(&mut self, world_uuid: Uuid) -> Option<&mut World> {
        self.worlds
            .iter_mut()
            .find(|world| world.uuid == world_uuid)
    }

    pub fn worlds(&self) -> &[World] {
        &self.worlds
    }

    pub(crate) fn add_entity(&mut self, world_uuid: Uuid, entity: Entity) -> bool {
        let Some(world) = self.world_mut(world_uuid) else {
            return false;
        };
        world.add_entity(entity);
        true
    }

    pub(crate) fn tick(&mut self, registries: &Registries, server_ptr: usize) {
        self.worlds.iter_mut().for_each(|world| {
            world.use_server_event_dispatcher(server_ptr);
            world.tick_with_registries(registries);
        });
    }

    pub(crate) fn remove_entity_by_addr(&mut self, addr: &SocketAddr) -> io::Result<()> {
        self.worlds
            .iter_mut()
            .try_for_each(|world| world.remove_entity_by_addr(addr))
    }

    pub(crate) fn player_mut_for_client(&mut self, client: &Client) -> Option<&mut Player> {
        self.worlds
            .iter_mut()
            .find_map(|world| world.player_by_addr_mut(&client.addr))
    }

    pub(crate) fn player_pointer_for_client(&mut self, client: &Client) -> Option<*mut Player> {
        self.worlds
            .iter_mut()
            .find_map(|world| world.player_pointer_by_addr(&client.addr))
    }

    pub(crate) fn player_pointer_for_client_address(
        &mut self,
        address: &SocketAddr,
    ) -> Option<*mut Player> {
        self.worlds
            .iter_mut()
            .find_map(|world| world.player_pointer_by_addr(address))
    }

    pub(crate) fn loaded_block_for_client(
        &self,
        client: &Client,
        position: crate::world::BlockPosition,
    ) -> Option<crate::world::Block> {
        self.worlds
            .iter()
            .find(|world| world.player_by_addr(&client.addr).is_some())
            .and_then(|world| world.loaded_block_at(position))
    }

    pub(crate) fn block_position_is_loaded_for_client(
        &self,
        client: &Client,
        position: crate::world::BlockPosition,
    ) -> bool {
        self.worlds
            .iter()
            .find(|world| world.player_by_addr(&client.addr).is_some())
            .is_some_and(|world| world.block_position_is_loaded(position))
    }

    pub(crate) fn refresh_block_for_client(
        &mut self,
        client: &mut Client,
        position: crate::world::BlockPosition,
    ) -> io::Result<()> {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        world.refresh_block_for_player(client, position)
    }

    pub(crate) fn set_block_for_client(
        &mut self,
        client: &Client,
        position: crate::world::BlockPosition,
        block: crate::world::Block,
    ) -> io::Result<bool> {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        world.set_block_for_player(client, position, block)
    }

    pub fn move_generic_entities_for_client(&mut self, client: &mut Client) -> io::Result<usize> {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        world.move_generic_entities_for_player(client)
    }

    pub fn remove_generic_entities_for_client(&mut self, client: &mut Client) -> io::Result<usize> {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        world.remove_generic_entities_for_player(client)
    }

    pub(crate) fn enter_player(
        &mut self,
        client: &mut Client,
        ticks_per_second: u32,
        registries: &Registries,
    ) -> io::Result<()> {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        world.enter_player(client, ticks_per_second, registries)
    }

    pub(crate) fn move_player(
        &mut self,
        client: &mut Client,
        x: f64,
        y: f64,
        z: f64,
        on_ground: bool,
        registries: &Registries,
    ) -> io::Result<()> {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        world.move_player(client, x, y, z, on_ground, registries)
    }

    pub(crate) fn move_player_with_view(
        &mut self,
        client: &mut Client,
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
        registries: &Registries,
    ) -> io::Result<()> {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        world.move_player_with_view(client, x, y, z, yaw, pitch, on_ground, registries)
    }

    pub(crate) fn look_player(
        &mut self,
        client: &Client,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    ) -> io::Result<()> {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        world.look_player(client, yaw, pitch, on_ground)
    }

    pub(crate) fn refresh_player_status(
        &mut self,
        client: &Client,
        on_ground: bool,
    ) -> io::Result<()> {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        world.refresh_player_status(client, on_ground)
    }

    pub(crate) fn animate_player_hand(
        &mut self,
        client: &Client,
        hand: PlayerHand,
    ) -> io::Result<()> {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        world.animate_player_hand(client, hand)
    }

    pub(crate) fn refresh_player_input(
        &mut self,
        client: &Client,
        forward: bool,
        backward: bool,
        left: bool,
        right: bool,
        jump: bool,
        shift: bool,
        sprint: bool,
    ) -> io::Result<()> {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        world.refresh_player_input(client, forward, backward, left, right, jump, shift, sprint)
    }

    pub(crate) fn set_player_sprinting(
        &mut self,
        client: &Client,
        sprinting: bool,
    ) -> io::Result<()> {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        world.set_player_sprinting(client, sprinting)
    }

    pub(crate) fn start_player_flying_with_elytra(&mut self, client: &Client) -> io::Result<()> {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        world.start_player_flying_with_elytra(client)
    }

    pub(crate) fn set_player_held_slot(
        &mut self,
        client: &mut Client,
        held_slot: i32,
        server: *mut crate::server::MinecraftServer,
    ) -> io::Result<bool> {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        world.set_player_held_slot(client, held_slot, server)
    }

    pub(crate) fn refresh_player_visible_equipment(&mut self, client: &Client) -> io::Result<()> {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        world.refresh_player_visible_equipment(client)
    }

    pub(crate) fn refresh_player_metadata(&mut self, client: &Client) -> io::Result<()> {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        world.refresh_player_metadata(client)
    }
}

impl Default for WorldManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::WorldManager;
    use crate::world::{Chunk, ChunkLoader, ChunkPosition};
    use spinel_network::types::Identifier;
    use spinel_registry::dimension_type::DimensionType;
    use std::io;

    struct ManagerTestChunkLoader;

    impl ChunkLoader for ManagerTestChunkLoader {
        fn load_chunk(&self, _position: ChunkPosition) -> io::Result<Option<Chunk>> {
            Ok(None)
        }

        fn save_chunk(&self, _chunk: &Chunk) -> io::Result<()> {
            Ok(())
        }

        fn unload_chunk(&self, _chunk: &mut Chunk) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn world_manager_create_and_register_worlds_match_minestom_instance_manager_surface() {
        let mut worlds = WorldManager::new();
        let first_world = worlds.create_world(Identifier::minecraft("overworld"));
        let second_world = worlds
            .create_world_with_loader(Identifier::minecraft("custom"), ManagerTestChunkLoader);
        let nether_world = worlds.create_world_with_dimension(
            DimensionType::THE_NETHER,
            DimensionType::builder()
                .vertical_bounds(-32, 256, 128)
                .build(),
        );
        let end_world = worlds.create_world_with_dimension_and_loader(
            DimensionType::THE_END,
            DimensionType::default(),
            ManagerTestChunkLoader,
        );

        assert_eq!(worlds.worlds().len(), 4);
        assert!(
            worlds
                .world(first_world)
                .is_some_and(|world| world.is_registered())
        );
        assert!(
            worlds
                .world(second_world)
                .is_some_and(|world| world.is_registered())
        );
        assert_eq!(
            worlds
                .world(nether_world)
                .map(|world| world.dimension_type().clone()),
            Some(DimensionType::THE_NETHER)
        );
        assert_eq!(
            worlds
                .world(end_world)
                .map(|world| world.dimension_name().clone()),
            Some(Identifier::minecraft("the_end"))
        );
    }
}
