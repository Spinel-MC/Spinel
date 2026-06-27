use crate::entity::player::PlayerMessageType;
use crate::entity::{Entity, EntityId, EntityPosition, PlayerChunk};
use crate::entity::{Player, PlayerHand};
use crate::network::client::instance::Client;
use crate::world::{
    BlockHandlerPlacement, BlockPosition, Chunk, ChunkLoadTicket, ChunkLoader, ChunkPosition,
    SharedWorld, World, WorldHandle,
};
use spinel_network::types::{ClientInformation, Identifier};
use spinel_registry::dimension_type::DimensionType;
use spinel_registry::{Registries, RegistryKey};
use spinel_utils::component::text::TextComponent;
use std::collections::VecDeque;
use std::io;
use std::net::SocketAddr;
use uuid::Uuid;

pub struct WorldManager {
    worlds: Vec<World>,
    shared_worlds: Vec<SharedWorld>,
    inactive_players: Vec<Player>,
    pending_player_world_transitions: VecDeque<PendingPlayerWorldTransition>,
    completed_player_world_transitions: Vec<u64>,
    next_player_world_transition_id: u64,
}

#[derive(Clone)]
struct PendingPlayerWorldTransition {
    id: u64,
    player_uuid: Uuid,
    current_world: Option<Uuid>,
    target_world: Uuid,
    position: EntityPosition,
    should_refresh_chunks: bool,
    chunks: Vec<PlayerChunk>,
    chunk_load_tickets: Vec<ChunkLoadTicket>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlayerWorldTransitionTicket {
    id: u64,
}

impl WorldManager {
    pub fn new() -> Self {
        Self {
            worlds: Vec::new(),
            shared_worlds: Vec::new(),
            inactive_players: Vec::new(),
            pending_player_world_transitions: VecDeque::new(),
            completed_player_world_transitions: Vec::new(),
            next_player_world_transition_id: 0,
        }
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
        world.dispatch_instance_register_event();
        self.worlds.push(world);
    }

    pub fn create_shared_world(&mut self, source_world: Uuid) -> io::Result<Uuid> {
        let source = self
            .world(source_world)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Source world not found."))?;
        if !source.is_registered() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Source world is not registered.",
            ));
        }
        let shared_world = SharedWorld::new(source_world, World::new(source.name().clone()));
        let shared_world_uuid = shared_world.uuid();
        self.register_shared_world(shared_world)?;
        Ok(shared_world_uuid)
    }

    pub fn register_shared_world(&mut self, mut shared_world: SharedWorld) -> io::Result<Uuid> {
        let Some(source_world) = self.world_mut(shared_world.source_world()) else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Shared world source not found.",
            ));
        };
        shared_world.set_registered(true);
        let shared_world_uuid = shared_world.uuid();
        source_world.add_shared_world(shared_world_uuid);
        self.shared_worlds.push(shared_world);
        Ok(shared_world_uuid)
    }

    pub fn copy_world(&mut self, source_world: Uuid) -> io::Result<Uuid> {
        let source = self
            .world(source_world)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Source world not found."))?;
        let copied_world = source.copy();
        let copied_world_uuid = copied_world.uuid();
        self.register_world(copied_world);
        Ok(copied_world_uuid)
    }

    pub fn instance_uuids(&self) -> Vec<Uuid> {
        self.worlds
            .iter()
            .map(World::uuid)
            .chain(self.shared_worlds.iter().map(SharedWorld::uuid))
            .collect()
    }

    pub fn worlds_are_linked(&self, first_world: Uuid, second_world: Uuid) -> bool {
        self.chunk_source_world(first_world)
            .zip(self.chunk_source_world(second_world))
            .is_some_and(|(first_source, second_source)| {
                first_world != second_world && first_source == second_source
            })
    }

    pub fn shared_world(&self, world_uuid: Uuid) -> Option<&SharedWorld> {
        self.shared_worlds
            .iter()
            .find(|world| world.uuid() == world_uuid)
    }

    pub fn shared_worlds(&self) -> &[SharedWorld] {
        &self.shared_worlds
    }

    pub fn unregister_world(&mut self, world_uuid: Uuid) -> io::Result<Option<World>> {
        let Some(world_index) = self
            .worlds
            .iter()
            .position(|world| world.uuid == world_uuid)
        else {
            return Ok(None);
        };
        let has_online_players = self.worlds[world_index].players().any(Player::is_online);
        if has_online_players {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "You cannot unregister an instance with players inside.",
            ));
        }
        let chunk_positions = self.worlds[world_index]
            .chunks()
            .map(|chunk| crate::world::ChunkPosition::new(chunk.x(), chunk.z()))
            .collect::<Vec<_>>();
        for chunk_position in chunk_positions {
            self.worlds[world_index].unload_chunk(chunk_position)?;
        }
        self.worlds[world_index].dispatch_instance_unregister_event();
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

    pub fn set_entity_world(&mut self, entity_id: EntityId, target_world: Uuid) -> io::Result<()> {
        let position = self
            .entity_position(entity_id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Entity not found."))?;
        self.set_entity_world_at_position(entity_id, target_world, position)
    }

    pub fn add_inactive_player(&mut self, player: Player) -> bool {
        if self
            .inactive_players
            .iter()
            .any(|stored_player| stored_player.get_uuid() == player.get_uuid())
        {
            return false;
        }
        if self.player_world_uuid(player.get_uuid()).is_some() {
            return false;
        }
        self.inactive_players.push(player);
        true
    }

    pub fn inactive_player(&self, player_uuid: Uuid) -> Option<&Player> {
        self.inactive_players
            .iter()
            .find(|player| player.get_uuid() == player_uuid)
    }

    pub fn set_entity_world_at_point(
        &mut self,
        entity_id: EntityId,
        target_world: Uuid,
        position: BlockPosition,
    ) -> io::Result<()> {
        let previous_position = self
            .entity_position(entity_id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Entity not found."))?;
        self.set_entity_world_at_position(
            entity_id,
            target_world,
            EntityPosition::new(
                f64::from(position.x),
                f64::from(position.y),
                f64::from(position.z),
                previous_position.get_yaw(),
                previous_position.get_pitch(),
            ),
        )
    }

    pub fn set_entity_world_at_position(
        &mut self,
        entity_id: EntityId,
        target_world: Uuid,
        position: EntityPosition,
    ) -> io::Result<()> {
        self.ensure_world_is_registered(target_world)?;
        let current_world = self
            .entity_world_uuid(entity_id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Entity not found."))?;
        self.load_chunk_for_world(target_world, chunk_position_for_entity(position))?;
        let source_world = self
            .world_mut_ptr(current_world)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Source world not found."))?;
        let target_world = self
            .world_mut_ptr(target_world)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Target world not found."))?;
        if source_world == target_world {
            let source_world = unsafe { &mut *source_world };
            let entity = source_world
                .entity_by_id_mut(entity_id)
                .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Entity not found."))?;
            entity.set_position(position);
            return Ok(());
        }
        let source_world = unsafe { &mut *source_world };
        let target_world = unsafe { &mut *target_world };
        let entity = source_world
            .entity_by_id_mut(entity_id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Entity not found."))?;
        if target_world.dispatch_add_entity_to_instance_event(entity) {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "Entity add cancelled.",
            ));
        }
        let Some(mut entity) = source_world.take_entity_from_instance(entity_id) else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Entity not found."));
        };
        entity.set_position(position);
        target_world.add_entity_after_instance_event(entity);
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn add_passenger(
        &mut self,
        vehicle_id: EntityId,
        passenger_id: EntityId,
    ) -> io::Result<bool> {
        if vehicle_id == passenger_id {
            return Ok(false);
        }
        let vehicle_world = self
            .entity_world_uuid(vehicle_id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Vehicle not found."))?;
        let passenger_world = self
            .entity_world_uuid(passenger_id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Passenger not found."))?;
        if vehicle_world != passenger_world {
            let vehicle_position = self
                .entity_position(vehicle_id)
                .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Vehicle not found."))?;
            self.set_entity_world_at_position(passenger_id, vehicle_world, vehicle_position)?;
        }
        self.world_mut(vehicle_world)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Vehicle world not found."))?
            .add_passenger(vehicle_id, passenger_id)
    }

    pub fn set_player_world(&mut self, player_uuid: Uuid, target_world: Uuid) -> io::Result<()> {
        self.set_player_world_future(player_uuid, target_world)
            .map(|_| ())
    }

    pub fn set_player_world_future(
        &mut self,
        player_uuid: Uuid,
        target_world: Uuid,
    ) -> io::Result<PlayerWorldTransitionTicket> {
        let position = self
            .player_set_instance_default_position(player_uuid)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Player not found."))?;
        self.set_player_world_at_position_future(player_uuid, target_world, position)
    }

    pub fn set_player_world_at_position(
        &mut self,
        player_uuid: Uuid,
        target_world: Uuid,
        position: EntityPosition,
    ) -> io::Result<()> {
        self.set_player_world_at_position_future(player_uuid, target_world, position)
            .map(|_| ())
    }

    pub fn set_player_world_at_position_future(
        &mut self,
        player_uuid: Uuid,
        target_world: Uuid,
        position: EntityPosition,
    ) -> io::Result<PlayerWorldTransitionTicket> {
        self.ensure_world_is_registered(target_world)?;
        let current_world = self.player_world_uuid(player_uuid);
        if current_world == Some(target_world) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Instance should be different than the current one",
            ));
        }
        if current_world.is_none() && self.inactive_player(player_uuid).is_none() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        }
        let should_refresh_chunks = current_world.is_none()
            || !self.worlds_are_linked(current_world.unwrap(), target_world)
            || !player_position_is_in_same_chunk(
                self.player_position(player_uuid).unwrap_or(position),
                position,
            );
        let chunks = if should_refresh_chunks {
            self.player_target_chunks(player_uuid, target_world, position)?
        } else {
            Vec::new()
        };
        let chunk_load_tickets = if should_refresh_chunks {
            let target_world = self
                .world_mut(target_world)
                .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "World not found."))?;
            chunks
                .iter()
                .copied()
                .map(ChunkPosition::from)
                .map(|position| target_world.load_optional_chunk_future(position))
                .collect::<io::Result<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect()
        } else {
            Vec::new()
        };
        let ticket = self.next_player_world_transition_ticket();
        self.pending_player_world_transitions
            .push_back(PendingPlayerWorldTransition {
                id: ticket.id,
                player_uuid,
                current_world,
                target_world,
                position,
                should_refresh_chunks,
                chunks,
                chunk_load_tickets,
            });
        Ok(ticket)
    }

    pub fn player_world_transition_is_complete(&self, ticket: PlayerWorldTransitionTicket) -> bool {
        self.completed_player_world_transitions.contains(&ticket.id)
    }

    fn next_player_world_transition_ticket(&mut self) -> PlayerWorldTransitionTicket {
        self.next_player_world_transition_id += 1;
        PlayerWorldTransitionTicket {
            id: self.next_player_world_transition_id,
        }
    }

    pub fn pending_player_world_transition_count(&self) -> usize {
        self.pending_player_world_transitions.len()
    }

    fn process_pending_player_world_transitions(
        &mut self,
        registries: &Registries,
    ) -> io::Result<()> {
        let mut transitions = VecDeque::new();
        std::mem::swap(&mut transitions, &mut self.pending_player_world_transitions);
        while let Some(transition) = transitions.pop_front() {
            self.process_pending_player_world_transition(transition, registries)?;
        }
        Ok(())
    }

    fn process_pending_player_world_transition(
        &mut self,
        transition: PendingPlayerWorldTransition,
        registries: &Registries,
    ) -> io::Result<()> {
        if transition.should_refresh_chunks {
            let target_world = self
                .world_mut(transition.target_world)
                .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "World not found."))?;
            let mut all_chunk_loads_are_complete = true;
            for ticket in &transition.chunk_load_tickets {
                if !target_world.complete_chunk_load(ticket)? {
                    all_chunk_loads_are_complete = false;
                }
            }
            if !all_chunk_loads_are_complete {
                self.pending_player_world_transitions.push_back(transition);
                return Ok(());
            }
        }
        let current_world = transition.current_world;
        if current_world == Some(transition.target_world) {
            self.completed_player_world_transitions.push(transition.id);
            return Ok(());
        }
        if let Some((player_id, player_uuid)) = self
            .world(current_world.unwrap_or(transition.target_world))
            .and_then(|world| world.player_by_uuid(transition.player_uuid))
            .map(|player| (player.get_entity_id(), player.get_uuid()))
        {
            self.world_mut(current_world.unwrap())
                .unwrap()
                .send_player_remove_to_viewers(player_id, player_uuid)?;
        }
        let Some(mut player) = self.take_transition_player(transition.player_uuid, current_world)
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        let target_view_distance = self
            .world(transition.target_world)
            .map(World::view_distance)
            .unwrap_or_default();
        let target_world_name = self
            .world(transition.target_world)
            .map(|world| world.name().clone())
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Target world not found."))?;
        let target_dimension_type = self
            .world(transition.target_world)
            .map(|world| world.get_dimension_type().clone())
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Target world not found."))?;
        let current_world_name =
            current_world.and_then(|world| self.world(world).map(|world| world.name().clone()));
        let target_time_packet = self
            .world(transition.target_world)
            .map(World::time_packet)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Target world not found."))?;
        let target_world_border_packet = self
            .world(transition.target_world)
            .map(World::create_initialize_world_border_packet)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Target world not found."))?;
        let target_weather = self
            .world(transition.target_world)
            .map(World::weather)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Target world not found."))?;
        let _ = player.set_instance_position(
            transition.position,
            target_view_distance,
            transition.should_refresh_chunks,
        );
        let chunks = transition.chunks.clone();
        player.set_dimension_type(target_dimension_type);
        let first_spawn = !player.has_entered_world();
        if transition.should_refresh_chunks {
            let client_ptr = player
                .get_client_mut()
                .filter(|client| client.state == spinel_network::ConnectionState::Play)
                .map(|client| client as *mut Client);
            if let Some(client_ptr) = client_ptr {
                let dimension_change = current_world_name
                    .as_ref()
                    .is_some_and(|current_world_name| current_world_name != &target_world_name);
                player.spawn_after_instance_transition(
                    unsafe { &mut *client_ptr },
                    target_world_name,
                    chunks.clone(),
                    target_time_packet,
                    target_world_border_packet,
                    target_weather,
                    first_spawn,
                    dimension_change,
                    transition.should_refresh_chunks,
                )?;
            }
        }
        if transition.should_refresh_chunks {
            chunks.into_iter().for_each(|chunk| {
                player.send_loaded_chunk_position(chunk);
            });
        }
        let target_world = self
            .world_mut(transition.target_world)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Target world not found."))?;
        let player_address = player.addr;
        target_world.add_entity(Entity::Player(player));
        if let Some(client_ptr) = target_world
            .player_by_addr_mut(&player_address)
            .and_then(Player::get_client_mut)
            .map(|client| client as *mut Client)
        {
            let client = unsafe { &mut *client_ptr };
            let _ = target_world.send_pending_chunks_for_client(client, registries);
            let _ = target_world.synchronize_player_visibility(client);
            target_world.dispatch_player_spawn(transition.player_uuid, first_spawn, client);
        }
        self.completed_player_world_transitions.push(transition.id);
        Ok(())
    }

    fn player_position(&self, player_uuid: Uuid) -> Option<EntityPosition> {
        self.worlds
            .iter()
            .chain(self.shared_worlds.iter().map(SharedWorld::world))
            .find_map(|world| world.player_by_uuid(player_uuid).map(Player::get_position))
    }

    fn player_world_uuid(&self, player_uuid: Uuid) -> Option<Uuid> {
        self.worlds
            .iter()
            .chain(self.shared_worlds.iter().map(SharedWorld::world))
            .find(|world| world.player_by_uuid(player_uuid).is_some())
            .map(World::uuid)
    }

    fn player_set_instance_default_position(&self, player_uuid: Uuid) -> Option<EntityPosition> {
        if let Some(position) = self
            .worlds
            .iter()
            .chain(self.shared_worlds.iter().map(SharedWorld::world))
            .find_map(|world| {
                world
                    .player_by_uuid(player_uuid)
                    .map(|player| player.get_position())
            })
        {
            return Some(position);
        }
        self.inactive_player(player_uuid).map(|player| {
            let respawn_point = player.get_respawn_point();
            EntityPosition::new(
                respawn_point.x,
                respawn_point.y,
                respawn_point.z,
                respawn_point.yaw,
                respawn_point.pitch,
            )
        })
    }

    fn take_transition_player(
        &mut self,
        player_uuid: Uuid,
        current_world: Option<Uuid>,
    ) -> Option<Player> {
        if let Some(current_world) = current_world {
            return self
                .world_mut(current_world)
                .and_then(|world| world.take_player_by_uuid(player_uuid));
        }
        let player_index = self
            .inactive_players
            .iter()
            .position(|player| player.get_uuid() == player_uuid)?;
        Some(self.inactive_players.remove(player_index))
    }

    fn player_for_target_chunks(&self, player_uuid: Uuid) -> Option<&Player> {
        self.worlds
            .iter()
            .chain(self.shared_worlds.iter().map(SharedWorld::world))
            .find_map(|world| world.player_by_uuid(player_uuid))
            .or_else(|| self.inactive_player(player_uuid))
    }

    fn player_target_chunks(
        &self,
        player_uuid: Uuid,
        target_world: Uuid,
        position: EntityPosition,
    ) -> io::Result<Vec<PlayerChunk>> {
        let player = self
            .player_for_target_chunks(player_uuid)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Player not found."))?;
        let target_view_distance = self
            .world(target_world)
            .map(World::view_distance)
            .unwrap_or_default();
        let center = PlayerChunk::new(
            (position.get_x().floor() as i32).div_euclid(16),
            (position.get_z().floor() as i32).div_euclid(16),
        );
        Ok(center.surrounding(player.get_effective_chunk_view_distance(target_view_distance)))
    }

    fn entity_position(&self, entity_id: EntityId) -> Option<EntityPosition> {
        self.worlds
            .iter()
            .chain(self.shared_worlds.iter().map(SharedWorld::world))
            .find_map(|world| world.entity_by_id(entity_id).map(Entity::get_position))
    }

    fn entity_world_uuid(&self, entity_id: EntityId) -> Option<Uuid> {
        self.worlds
            .iter()
            .chain(self.shared_worlds.iter().map(SharedWorld::world))
            .find(|world| world.entity_by_id(entity_id).is_some())
            .map(World::uuid)
    }

    fn ensure_world_is_registered(&self, world_uuid: Uuid) -> io::Result<()> {
        let Some(world) = self.world(world_uuid) else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "World not found."));
        };
        if !world.is_registered() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "World is not registered.",
            ));
        }
        Ok(())
    }

    pub fn world(&self, world_uuid: Uuid) -> Option<&World> {
        self.worlds
            .iter()
            .find(|world| world.uuid == world_uuid)
            .or_else(|| {
                self.shared_worlds
                    .iter()
                    .find(|world| world.uuid() == world_uuid)
                    .map(SharedWorld::world)
            })
    }

    pub fn world_mut(&mut self, world_uuid: Uuid) -> Option<&mut World> {
        if let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.uuid == world_uuid)
        {
            return Some(world);
        }
        self.shared_worlds
            .iter_mut()
            .find(|world| world.uuid() == world_uuid)
            .map(SharedWorld::world_mut)
    }

    fn world_mut_ptr(&mut self, world_uuid: Uuid) -> Option<*mut World> {
        if let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.uuid() == world_uuid)
        {
            return Some(world as *mut World);
        }
        self.shared_worlds
            .iter_mut()
            .find(|world| world.uuid() == world_uuid)
            .map(|world| world.world_mut() as *mut World)
    }

    pub fn worlds(&self) -> &[World] {
        &self.worlds
    }

    fn chunk_source_world(&self, world_uuid: Uuid) -> Option<Uuid> {
        if self.worlds.iter().any(|world| world.uuid() == world_uuid) {
            return Some(world_uuid);
        }
        self.shared_world(world_uuid).map(SharedWorld::source_world)
    }

    fn chunk_source_world_mut(&mut self, world_uuid: Uuid) -> Option<&mut World> {
        let source_world = self.chunk_source_world(world_uuid)?;
        self.worlds
            .iter_mut()
            .find(|world| world.uuid() == source_world)
    }

    pub fn load_chunk_for_world(
        &mut self,
        world_uuid: Uuid,
        position: ChunkPosition,
    ) -> io::Result<&mut Chunk> {
        let Some(world) = self.chunk_source_world_mut(world_uuid) else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "World not found."));
        };
        world.load_chunk(position)
    }

    pub fn chunk_for_world(&self, world_uuid: Uuid, position: ChunkPosition) -> Option<&Chunk> {
        let source_world = self.chunk_source_world(world_uuid)?;
        self.worlds
            .iter()
            .find(|world| world.uuid() == source_world)
            .and_then(|world| world.chunk(position))
    }

    pub fn unload_chunk_for_world(
        &mut self,
        world_uuid: Uuid,
        position: ChunkPosition,
    ) -> io::Result<bool> {
        let Some(world) = self.chunk_source_world_mut(world_uuid) else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "World not found."));
        };
        world.unload_chunk(position)
    }

    pub fn save_chunk_for_world(
        &mut self,
        world_uuid: Uuid,
        position: ChunkPosition,
    ) -> io::Result<bool> {
        let Some(world) = self.chunk_source_world_mut(world_uuid) else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "World not found."));
        };
        world.save_chunk(position)
    }

    pub fn save_chunks_for_world(&mut self, world_uuid: Uuid) -> io::Result<()> {
        let Some(world) = self.chunk_source_world_mut(world_uuid) else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "World not found."));
        };
        world.save_chunks()
    }

    pub fn generate_chunk_for_world(
        &mut self,
        world_uuid: Uuid,
        position: ChunkPosition,
    ) -> io::Result<bool> {
        let Some(world) = self.chunk_source_world_mut(world_uuid) else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "World not found."));
        };
        world.generate_chunk_result(position)
    }

    pub(crate) fn add_entity(&mut self, world_uuid: Uuid, entity: Entity) -> bool {
        let Some(world) = self.world_mut(world_uuid) else {
            return false;
        };
        world.add_entity(entity)
    }

    pub(crate) fn tick(&mut self, registries: &Registries, server_ptr: usize) {
        let _ = self.process_pending_player_world_transitions(registries);
        self.worlds.iter_mut().for_each(|world| {
            world.use_server_event_dispatcher(server_ptr);
            world.tick_with_registries(registries);
        });
        self.shared_worlds.iter_mut().for_each(|world| {
            world.world_mut().use_server_event_dispatcher(server_ptr);
            world.world_mut().tick_with_registries(registries);
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

    pub(crate) fn online_player_uuids(&self) -> Vec<Uuid> {
        self.worlds
            .iter()
            .flat_map(World::players)
            .filter(|player| player.is_online())
            .map(Player::get_uuid)
            .collect()
    }

    pub(crate) fn send_chat_message_to_recipients(
        &mut self,
        recipients: &[Uuid],
        sender: Uuid,
        message: TextComponent,
    ) -> io::Result<()> {
        self.worlds.iter_mut().try_for_each(|world| {
            recipients.iter().try_for_each(|recipient| {
                let Some(player) = world.player_by_uuid_mut(*recipient) else {
                    return Ok(());
                };
                player.send_message_from(sender, message.clone(), PlayerMessageType::Chat)
            })
        })
    }

    pub(crate) fn move_client_world_entity(
        &mut self,
        client: &Client,
        entity_id: EntityId,
        position: EntityPosition,
    ) -> bool {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return false;
        };
        world.set_entity_position(entity_id, position)
    }

    pub(crate) fn steer_client_boat(
        &mut self,
        client: &Client,
        vehicle_id: EntityId,
        left_paddle_turning: bool,
        right_paddle_turning: bool,
    ) -> bool {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return false;
        };
        world.steer_boat(vehicle_id, left_paddle_turning, right_paddle_turning)
    }

    pub(crate) fn world_uuid_for_client(&self, client: &Client) -> Option<Uuid> {
        self.worlds
            .iter()
            .find(|world| world.player_by_addr(&client.addr).is_some())
            .map(World::uuid)
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

    pub(crate) fn loaded_block_state_for_client(
        &self,
        client: &Client,
        position: crate::world::BlockPosition,
    ) -> Option<crate::world::BlockState> {
        self.worlds
            .iter()
            .find(|world| world.player_by_addr(&client.addr).is_some())
            .and_then(|world| world.loaded_block_state_at(position))
    }

    pub(crate) fn client_block_entity_nbt_for_client(
        &self,
        client: &Client,
        position: crate::world::BlockPosition,
    ) -> Option<spinel_nbt::NbtCompound> {
        self.worlds
            .iter()
            .find(|world| world.player_by_addr(&client.addr).is_some())
            .and_then(|world| world.client_block_entity_nbt_at(position))
    }

    pub(crate) fn block_is_self_replaceable_for_client(
        &self,
        client: &Client,
        replacement: crate::world::BlockReplacement,
    ) -> bool {
        self.worlds
            .iter()
            .find(|world| world.player_by_addr(&client.addr).is_some())
            .is_some_and(|world| world.block_is_self_replaceable(replacement))
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

    pub(crate) fn block_position_is_inside_world_border_for_client(
        &self,
        client: &Client,
        position: crate::world::BlockPosition,
    ) -> bool {
        self.worlds
            .iter()
            .find(|world| world.player_by_addr(&client.addr).is_some())
            .is_some_and(|world| world.block_position_is_inside_world_border(position))
    }

    pub(crate) fn block_placement_collision_entity_for_client(
        &self,
        client: &Client,
        position: crate::world::BlockPosition,
    ) -> Option<crate::entity::EntityId> {
        self.worlds
            .iter()
            .find(|world| world.player_by_addr(&client.addr).is_some())
            .and_then(|world| world.block_placement_collision_entity(position))
    }

    pub(crate) fn chunk_is_read_only_for_client(
        &self,
        client: &Client,
        position: crate::world::BlockPosition,
    ) -> bool {
        self.worlds
            .iter()
            .find(|world| world.player_by_addr(&client.addr).is_some())
            .is_some_and(|world| world.chunk_is_read_only_at(position))
    }

    pub(crate) fn refresh_chunk_for_client(
        &mut self,
        client: &Client,
        position: crate::world::BlockPosition,
    ) -> bool {
        self.worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
            .is_some_and(|world| world.refresh_chunk_for_client(client, position))
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

    pub(crate) fn refresh_block_entity_for_client(
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
        world.refresh_block_entity_for_player(client, position)
    }

    pub(crate) fn place_block_for_client(
        &mut self,
        client: &Client,
        placement: BlockHandlerPlacement,
        do_block_updates: bool,
    ) -> io::Result<bool> {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        Ok(world.place_block_with_updates(placement, do_block_updates))
    }

    pub(crate) fn break_block_for_client(
        &mut self,
        client: &Client,
        player_id: EntityId,
        position: crate::world::BlockPosition,
        block_face: crate::events::player_block_interact::BlockFace,
        do_block_updates: bool,
    ) -> io::Result<bool> {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        Ok(world.break_block_with_updates(player_id, position, block_face, do_block_updates))
    }

    pub(crate) fn interact_block_handler_for_client(
        &mut self,
        client: &Client,
        player_id: EntityId,
        hand: PlayerHand,
        position: BlockPosition,
        block_face: crate::events::player_block_interact::BlockFace,
        cursor_position: (f32, f32, f32),
    ) -> io::Result<bool> {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        Ok(world.interact_block_handler(player_id, hand, block_face, position, cursor_position))
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
        client: &mut Client,
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

    pub(crate) fn refresh_player_settings(
        &mut self,
        client: &mut Client,
        settings: ClientInformation,
    ) -> io::Result<()> {
        let Some(world) = self
            .worlds
            .iter_mut()
            .find(|world| world.player_by_addr(&client.addr).is_some())
        else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Player not found."));
        };
        world.refresh_player_settings(client, settings)
    }
}

impl Default for WorldManager {
    fn default() -> Self {
        Self::new()
    }
}

fn chunk_position_for_entity(position: EntityPosition) -> ChunkPosition {
    ChunkPosition::new(
        (position.get_x().floor() as i32).div_euclid(16),
        (position.get_z().floor() as i32).div_euclid(16),
    )
}

fn player_position_is_in_same_chunk(first: EntityPosition, second: EntityPosition) -> bool {
    chunk_position_for_entity(first) == chunk_position_for_entity(second)
}
