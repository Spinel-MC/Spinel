use crate::entity::Entity;
use crate::entity::Player;
use crate::network::client::instance::Client;
use crate::network::connection_manager::ConnectionManager;
use crate::world::World;
use spinel_network::types::Identifier;
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

    pub fn register_world(&mut self, world: World) {
        self.worlds.push(world);
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

    pub(crate) fn tick(&mut self, connections: &ConnectionManager) -> Vec<SocketAddr> {
        self.worlds
            .iter_mut()
            .flat_map(|world| world.tick(connections))
            .collect()
    }

    pub(crate) fn sync_ticks(&mut self, connections: &ConnectionManager, ticks_per_second: u32) {
        self.worlds
            .iter_mut()
            .for_each(|world| world.sync_ticks(connections, ticks_per_second));
    }

    pub(crate) fn remove_entity_by_addr(&mut self, addr: &SocketAddr) {
        self.worlds
            .iter_mut()
            .for_each(|world| world.remove_entity_by_addr(addr));
    }

    pub(crate) fn player_mut_for_client(&mut self, client: &Client) -> Option<&mut Player> {
        self.worlds
            .iter_mut()
            .find_map(|world| world.player_by_addr_mut(&client.addr))
    }

    pub(crate) fn enter_player(
        &mut self,
        client: &mut Client,
        ticks_per_second: u32,
    ) -> io::Result<()> {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        world.enter_player(client, ticks_per_second)
    }

    pub(crate) fn move_player(
        &mut self,
        client: &mut Client,
        x: f64,
        y: f64,
        z: f64,
    ) -> io::Result<()> {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        world.move_player(client, x, y, z)
    }
}

impl Default for WorldManager {
    fn default() -> Self {
        Self::new()
    }
}
