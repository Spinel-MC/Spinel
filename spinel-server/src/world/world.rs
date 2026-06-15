use crate::entity::ai::CreatureAiAction;
use crate::entity::physics::{EntityMovement, EntityMovementPacket};
use crate::entity::player::{PlayerSkin, PlayerViewerSnapshot};
use crate::entity::{
    Damage, Entity, EntityId, EntityPosition, EntityTeleport, EquipmentSlot, ExperienceOrb,
    GenericEntity, ItemEntity, Player, PlayerChunk, PlayerChunkTransition, PlayerPose,
    TimedPotionEffect,
};
use crate::events::add_entity_to_instance::AddEntityToInstanceEvent;
use crate::events::entity_attack::EntityAttackEvent;
use crate::events::entity_damage::EntityDamageEvent;
use crate::events::entity_death::EntityDeathEvent;
use crate::events::entity_despawn::EntityDespawnEvent;
use crate::events::entity_equip::EntityEquipEvent;
use crate::events::entity_fire_extinguish::EntityFireExtinguishEvent;
use crate::events::entity_item_merge::EntityItemMergeEvent;
use crate::events::entity_potion_add::EntityPotionAddEvent;
use crate::events::entity_potion_remove::EntityPotionRemoveEvent;
use crate::events::entity_set_fire::EntitySetFireEvent;
use crate::events::entity_shoot::EntityShootEvent;
use crate::events::entity_spawn::EntitySpawnEvent;
use crate::events::entity_teleport::EntityTeleportEvent;
use crate::events::entity_tick::EntityTickEvent;
use crate::events::entity_velocity::EntityVelocityEvent;
use crate::events::instance_block_update::InstanceBlockUpdateEvent;
use crate::events::instance_chunk_load::InstanceChunkLoadEvent;
use crate::events::instance_chunk_unload::InstanceChunkUnloadEvent;
use crate::events::instance_register::InstanceRegisterEvent;
use crate::events::instance_section_invalidate::InstanceSectionInvalidateEvent;
use crate::events::instance_tick::InstanceTickEvent;
use crate::events::instance_tick_end::InstanceTickEndEvent;
use crate::events::instance_unregister::InstanceUnregisterEvent;
use crate::events::pickup_experience::PickupExperienceEvent;
use crate::events::pickup_item::PickupItemEvent;
use crate::events::player_block_break::PlayerBlockBreakEvent;
use crate::events::player_move::PlayerMoveEvent;
use crate::events::player_spawn::PlayerSpawnEvent;
use crate::events::player_stop_flying_with_elytra::PlayerStopFlyingWithElytraEvent;
use crate::events::player_tick::PlayerTickEvent;
use crate::events::player_tick_end::PlayerTickEndEvent;
use crate::events::projectile_collide_with_block::ProjectileCollideWithBlockEvent;
use crate::events::projectile_collide_with_entity::ProjectileCollideWithEntityEvent;
use crate::events::projectile_uncollide::ProjectileUncollideEvent;
use crate::events::remove_entity_from_instance::RemoveEntityFromInstanceEvent;
use crate::network::client::instance::Client;
use crate::scoreboard::Team;
use crate::world::chunk_loading_executor::ChunkLoadingExecutor;
use crate::world::generator::{FallibleGenerator, GenerateChunkError, GenerationFork, Generator};
use crate::world::world_lighting::WorldLighting;
use crate::world::{
    Biome, Block, BlockHandler, BlockHandlerDestroy, BlockHandlerInteraction,
    BlockHandlerPlacement, BlockHandlerRegistry, BlockHandlerTouch, BlockInstance,
    BlockLookupCondition, BlockPlacementRule, BlockPlacementRuleRegistry, BlockPlacementState,
    BlockPosition, BlockReplacement, BlockSize, BlockState, BlockUpdateState, BossBar, Chunk,
    ChunkLoader, ChunkPosition, EntityTracker, EntityTrackerTarget, ExplosionSupplier,
    GenerationUnit, NoopChunkLoader, Weather, WorldBorder, WorldEventNode, WorldIdentity,
    WorldPointers, WorldScheduler, WorldSnapshot, WorldSoundEmitter,
};
use spinel_core::entity::game_mode::GameMode;
use spinel_core::network::clientbound::play::block_action::BlockActionPacket;
use spinel_core::network::clientbound::play::block_entity_data::BlockEntityDataPacket;
use spinel_core::network::clientbound::play::block_update::BlockUpdatePacket;
use spinel_core::network::clientbound::play::chunk_data::ChunkDataAndUpdateLightPacket;
use spinel_core::network::clientbound::play::damage_event::DamageEventPacket;
use spinel_core::network::clientbound::play::entity_effect::EntityEffectPacket;
use spinel_core::network::clientbound::play::entity_head_look::EntityHeadLookPacket;
use spinel_core::network::clientbound::play::entity_position::EntityPositionPacket;
use spinel_core::network::clientbound::play::entity_position_and_rotation::EntityPositionAndRotationPacket;
use spinel_core::network::clientbound::play::entity_rotation::EntityRotationPacket;
use spinel_core::network::clientbound::play::entity_sound_effect::{
    EntitySoundEffectPacket, NetworkSoundEvent,
};
use spinel_core::network::clientbound::play::entity_status::EntityStatusPacket;
use spinel_core::network::clientbound::play::entity_teleport::EntityTeleportPacket;
use spinel_core::network::clientbound::play::entity_velocity::EntityVelocityPacket;
use spinel_core::network::clientbound::play::light_update::LightUpdatePacket;
use spinel_core::network::clientbound::play::player_info_remove::PlayerInfoRemovePacket;
use spinel_core::network::clientbound::play::player_info_update::PlayerInfoUpdatePacket;
use spinel_core::network::clientbound::play::remove_entities::RemoveEntitiesPacket;
use spinel_core::network::clientbound::play::remove_entity_effect::RemoveEntityEffectPacket;
use spinel_core::network::clientbound::play::set_entity_data::SetEntityDataPacket;
use spinel_core::network::clientbound::play::set_equipment::{
    EntityEquipmentEntry, SetEquipmentPacket,
};
use spinel_core::network::clientbound::play::set_player_team::SetPlayerTeamPacket;
use spinel_core::network::clientbound::play::set_time::SetTimePacket;
use spinel_core::network::clientbound::play::sound_effect::{
    NetworkPositionedSoundEvent, SoundEffectPacket,
};
use spinel_core::network::clientbound::play::spawn_entity::EntityAngle;
use spinel_core::network::clientbound::play::spawn_entity::SpawnEntityPacket;
use spinel_core::network::clientbound::play::take_item_entity::TakeItemEntityPacket;
use spinel_core::network::clientbound::play::update_attributes::UpdateAttributesPacket;
use spinel_core::network::clientbound::play::world_event::WorldEventPacket;
use spinel_core::raycast::RaycastBoundingBox;
use spinel_nbt::{NbtCompound, TagHandler, Taggable};
use spinel_network::types::entity_metadata::MetadataEntry;
use spinel_network::types::sound::SoundEvent;
use spinel_network::types::{
    ClientInformation, Identifier, Position, Slot, TeleportFlags, Vector3d, Velocity,
};
use spinel_network::{DataType, PacketSender, PacketStruct};
use spinel_registry::damage_type::DamageType;
use spinel_registry::dimension_type::DimensionType;
use spinel_registry::{EntityBoundingBox, EntityType, ItemStack, Registries, RegistryKey};
use spinel_utils::component::Component;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{Error, ErrorKind, Result};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::thread::JoinHandle;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

const MAX_PLAYER_COORDINATE: f64 = 30_000_000.0;
const DEFAULT_TIME_SYNCHRONIZATION_TICKS: i32 = 20;
const DEFAULT_CHUNK_VIEW_DISTANCE: i32 = 8;
const DESTROY_BLOCK_WORLD_EVENT_ID: i32 = 2001;
const ENTITY_VIEW_DISTANCE: i32 = 5;

#[derive(Clone)]
pub struct ChunkSupplier {
    create_chunk: Arc<dyn Fn(ChunkPosition) -> Chunk + Send + Sync>,
}

#[derive(Clone, Debug)]
pub struct ChunkLoadTicket {
    id: u64,
    position: ChunkPosition,
    is_completed: Arc<AtomicBool>,
}

pub struct EntityTeleportTicket {
    entity_id: EntityId,
    teleport: EntityTeleport,
    chunk_load_tickets: Vec<ChunkLoadTicket>,
    should_confirm: bool,
    completed: bool,
}

pub struct WorldIoTask {
    handle: Option<JoinHandle<Result<()>>>,
    completed: Option<Result<()>>,
}

struct CompletedChunkLoad {
    ticket: ChunkLoadTicket,
    prepared_chunk_load: Result<PreparedChunkLoad>,
}

struct PendingPlayerEntry {
    client: usize,
    ticks_per_second: u32,
    chunks: Vec<PlayerChunk>,
    chunk_load_tickets: Vec<ChunkLoadTicket>,
}

struct PreparedChunkLoad {
    chunk: Chunk,
    generation_forks: Vec<GenerationFork>,
    requires_generation_completion: bool,
}

impl ChunkSupplier {
    pub fn new(create_chunk: impl Fn(ChunkPosition) -> Chunk + Send + Sync + 'static) -> Self {
        Self {
            create_chunk: Arc::new(create_chunk),
        }
    }

    pub fn create_chunk(&self, position: ChunkPosition) -> Chunk {
        (self.create_chunk)(position)
    }
}

impl ChunkLoadTicket {
    pub const fn id(&self) -> u64 {
        self.id
    }

    pub const fn position(&self) -> ChunkPosition {
        self.position
    }

    pub fn is_completed(&self) -> bool {
        self.is_completed.load(Ordering::SeqCst)
    }

    fn complete(&self) {
        self.is_completed.store(true, Ordering::SeqCst);
    }
}

impl PartialEq for ChunkLoadTicket {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.position == other.position
    }
}

impl Eq for ChunkLoadTicket {}

impl EntityTeleportTicket {
    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub fn teleport(&self) -> &EntityTeleport {
        &self.teleport
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }
}

impl WorldIoTask {
    fn completed(result: Result<()>) -> Self {
        Self {
            handle: None,
            completed: Some(result),
        }
    }

    fn running(handle: JoinHandle<Result<()>>) -> Self {
        Self {
            handle: Some(handle),
            completed: None,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.completed.is_some() || self.handle.as_ref().is_some_and(JoinHandle::is_finished)
    }

    pub fn join(mut self) -> Result<()> {
        if let Some(result) = self.completed.take() {
            return result;
        }
        let Some(handle) = self.handle.take() else {
            return Ok(());
        };
        handle
            .join()
            .map_err(|_| Error::new(ErrorKind::Other, "World IO task panicked"))?
    }
}

impl Default for ChunkSupplier {
    fn default() -> Self {
        Self::new(Chunk::new)
    }
}

struct GenericEntityViewerSnapshot {
    player_info_packet: Option<PlayerInfoUpdatePacket>,
    spawn_packet: SpawnEntityPacket,
    velocity_packet: Option<EntityVelocityPacket>,
    metadata_packet: SetEntityDataPacket,
    equipment_packet: SetEquipmentPacket,
    head_look_packet: EntityHeadLookPacket,
    attributes_packet: Option<UpdateAttributesPacket>,
    effect_packets: Vec<EntityEffectPacket>,
}

impl GenericEntityViewerSnapshot {
    fn from_entity(entity: &GenericEntity) -> Self {
        Self {
            player_info_packet: (entity.entity_type() == EntityType::PLAYER).then(|| {
                PlayerInfoUpdatePacket::add_listed_player(
                    entity.uuid(),
                    format!("test_player_{}", entity.entity_id().value()),
                )
            }),
            spawn_packet: entity.spawn_packet(),
            velocity_packet: entity.has_velocity().then(|| entity.velocity_packet()),
            metadata_packet: entity.metadata_packet(),
            equipment_packet: entity.equipment_packet(),
            head_look_packet: entity.head_look_packet(),
            attributes_packet: entity
                .has_attributes()
                .then(|| entity.update_attributes_packet()),
            effect_packets: entity.effect_packets(),
        }
    }

    fn from_experience_orb(experience_orb: &ExperienceOrb) -> Self {
        let mut snapshot = Self::from_entity(experience_orb);
        snapshot.spawn_packet = experience_orb.spawn_packet();
        snapshot
    }

    fn from_item_entity(item_entity: &ItemEntity) -> Self {
        let mut snapshot = Self::from_entity(item_entity);
        snapshot.spawn_packet = item_entity.spawn_packet();
        snapshot
    }

    fn from_projectile(projectile: &crate::entity::ProjectileEntity) -> Self {
        let mut snapshot = Self::from_entity(projectile);
        snapshot.spawn_packet = projectile.spawn_packet();
        snapshot
    }

    fn dispatch_with_leashes(
        self,
        client: &mut Client,
        leash_packets: Vec<
            spinel_core::network::clientbound::play::attach_entity::AttachEntityPacket,
        >,
    ) -> Result<()> {
        if let Some(player_info_packet) = self.player_info_packet {
            player_info_packet.dispatch(client)?;
        }
        self.spawn_packet.dispatch(client)?;
        if let Some(velocity_packet) = self.velocity_packet {
            velocity_packet.dispatch(client)?;
        }
        self.metadata_packet.dispatch(client)?;
        leash_packets
            .into_iter()
            .try_for_each(|packet| packet.dispatch(client))?;
        self.head_look_packet.dispatch(client)?;
        self.equipment_packet.dispatch(client)?;
        if let Some(attributes_packet) = self.attributes_packet {
            attributes_packet.dispatch(client)?;
        }
        self.effect_packets
            .into_iter()
            .try_for_each(|packet| packet.dispatch(client))
    }
}

pub struct World {
    pub uuid: Uuid,
    pub name: Identifier,
    entities: Vec<Entity>,
    entity_tracker: EntityTracker,
    chunks: HashMap<ChunkPosition, Chunk>,
    block_handlers: BlockHandlerRegistry,
    block_placement_rules: BlockPlacementRuleRegistry,
    linked_shared_worlds: Vec<Uuid>,
    source_world: Option<Uuid>,
    last_block_change_time: u128,
    currently_changing_blocks: HashMap<BlockPosition, BlockState>,
    pending_generation: HashMap<ChunkPosition, Vec<GenerationFork>>,
    loading_chunks: HashSet<ChunkPosition>,
    async_chunk_loads: HashMap<ChunkPosition, ChunkLoadTicket>,
    completed_chunk_load_sender: mpsc::Sender<CompletedChunkLoad>,
    completed_chunk_load_receiver: mpsc::Receiver<CompletedChunkLoad>,
    prepared_chunk_loads: HashMap<u64, (ChunkLoadTicket, Result<PreparedChunkLoad>)>,
    next_chunk_load_ticket_id: u64,
    player_chunk_load_waiters: HashMap<ChunkPosition, Vec<SocketAddr>>,
    pending_player_entries: HashMap<SocketAddr, PendingPlayerEntry>,
    pending_entity_visibility_refreshes: VecDeque<EntityId>,
    pending_entity_visibility_refresh_keys: HashSet<EntityId>,
    generator: Option<Arc<dyn Generator + Send + Sync>>,
    explosion_supplier: Option<Box<dyn ExplosionSupplier>>,
    chunk_loader: Arc<dyn ChunkLoader>,
    chunk_supplier: ChunkSupplier,
    registered: bool,
    dimension_type: RegistryKey<DimensionType>,
    cached_dimension_type: DimensionType,
    dimension_name: Identifier,
    auto_chunk_load: bool,
    world_age: i64,
    time: i64,
    time_rate: i32,
    time_synchronization_ticks: i32,
    view_distance: i32,
    world_border: WorldBorder,
    boss_bars: Vec<BossBar>,
    scoreboard_teams: HashMap<String, Team>,
    weather: Weather,
    transitioning_weather: Weather,
    remaining_rain_transition_ticks: i32,
    remaining_thunder_transition_ticks: i32,
    tag_handler: TagHandler,
    scheduler: WorldScheduler,
    event_node: WorldEventNode,
    event_dispatcher: Option<usize>,
}

impl World {
    pub fn new(name: Identifier) -> Self {
        let (completed_chunk_load_sender, completed_chunk_load_receiver) = mpsc::channel();
        Self {
            uuid: Uuid::new_v4(),
            name: name.clone(),
            entities: Vec::new(),
            entity_tracker: EntityTracker::new(),
            chunks: HashMap::new(),
            block_handlers: BlockHandlerRegistry::default(),
            block_placement_rules: BlockPlacementRuleRegistry::default(),
            linked_shared_worlds: Vec::new(),
            source_world: None,
            last_block_change_time: current_time_nanos(),
            currently_changing_blocks: HashMap::new(),
            pending_generation: HashMap::new(),
            loading_chunks: HashSet::new(),
            async_chunk_loads: HashMap::new(),
            completed_chunk_load_sender,
            completed_chunk_load_receiver,
            prepared_chunk_loads: HashMap::new(),
            next_chunk_load_ticket_id: 0,
            player_chunk_load_waiters: HashMap::new(),
            pending_player_entries: HashMap::new(),
            pending_entity_visibility_refreshes: VecDeque::new(),
            pending_entity_visibility_refresh_keys: HashSet::new(),
            generator: None,
            explosion_supplier: None,
            chunk_loader: Arc::new(NoopChunkLoader),
            chunk_supplier: ChunkSupplier::default(),
            registered: false,
            dimension_type: DimensionType::OVERWORLD,
            cached_dimension_type: DimensionType::default(),
            dimension_name: name.clone(),
            auto_chunk_load: true,
            world_age: 0,
            time: 0,
            time_rate: 1,
            time_synchronization_ticks: DEFAULT_TIME_SYNCHRONIZATION_TICKS,
            view_distance: DEFAULT_CHUNK_VIEW_DISTANCE,
            world_border: WorldBorder::DEFAULT,
            boss_bars: Vec::new(),
            scoreboard_teams: HashMap::new(),
            weather: Weather::CLEAR,
            transitioning_weather: Weather::CLEAR,
            remaining_rain_transition_ticks: 0,
            remaining_thunder_transition_ticks: 0,
            tag_handler: TagHandler::new_handler(),
            scheduler: WorldScheduler::default(),
            event_node: WorldEventNode::default(),
            event_dispatcher: None,
        }
    }

    pub fn new_with_dimension(
        name: Identifier,
        dimension_type: RegistryKey<DimensionType>,
        cached_dimension_type: DimensionType,
    ) -> Self {
        Self {
            dimension_type,
            cached_dimension_type,
            ..Self::new(name.clone())
        }
    }

    pub const fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub const fn identity(&self) -> WorldIdentity {
        WorldIdentity::new(self.uuid)
    }

    pub const fn pointers(&self) -> WorldPointers {
        WorldPointers::new(self.uuid)
    }

    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub const fn is_registered(&self) -> bool {
        self.registered
    }

    pub(crate) fn set_registered(&mut self, registered: bool) {
        self.registered = registered;
    }

    pub fn dimension_type(&self) -> &RegistryKey<DimensionType> {
        &self.dimension_type
    }

    pub const fn cached_dimension_type(&self) -> &DimensionType {
        &self.cached_dimension_type
    }

    pub fn dimension_name(&self) -> &Identifier {
        &self.dimension_name
    }

    pub fn is_in_void(&self, position: EntityPosition) -> bool {
        position.y() < f64::from(self.cached_dimension_type.min_y - 64)
    }

    pub const fn world_age(&self) -> i64 {
        self.world_age
    }

    pub fn set_world_age(&mut self, world_age: i64) -> Result<()> {
        self.world_age = world_age;
        self.broadcast_time()
    }

    pub const fn time(&self) -> i64 {
        self.time
    }

    pub fn set_time(&mut self, time: i64) -> Result<()> {
        self.time = time;
        self.broadcast_time()
    }

    pub const fn time_rate(&self) -> i32 {
        self.time_rate
    }

    pub fn set_time_rate(&mut self, time_rate: i32) -> Result<()> {
        if time_rate < 0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Time rate cannot be lower than 0",
            ));
        }
        self.time_rate = time_rate;
        Ok(())
    }

    pub const fn time_synchronization_ticks(&self) -> i32 {
        self.time_synchronization_ticks
    }

    pub fn set_time_synchronization_ticks(
        &mut self,
        time_synchronization_ticks: i32,
    ) -> Result<()> {
        if time_synchronization_ticks < 0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Time synchronization ticks cannot be lower than 0",
            ));
        }
        self.time_synchronization_ticks = time_synchronization_ticks;
        Ok(())
    }

    pub const fn view_distance(&self) -> i32 {
        self.view_distance
    }

    pub fn set_view_distance(&mut self, view_distance: i32) {
        self.view_distance = view_distance;
    }

    pub fn shared_worlds(&self) -> &[Uuid] {
        &self.linked_shared_worlds
    }

    pub fn has_shared_worlds(&self) -> bool {
        !self.linked_shared_worlds.is_empty()
    }

    pub(crate) fn add_shared_world(&mut self, world: Uuid) -> bool {
        if self.linked_shared_worlds.contains(&world) {
            return false;
        }
        self.linked_shared_worlds.push(world);
        true
    }

    pub(crate) fn set_source_world(&mut self, world: Uuid) {
        self.source_world = Some(world);
    }

    pub const fn source_world(&self) -> Option<Uuid> {
        self.source_world
    }

    pub const fn last_block_change_time(&self) -> u128 {
        self.last_block_change_time
    }

    pub fn refresh_last_block_change_time(&mut self) {
        self.last_block_change_time = current_time_nanos();
    }

    #[cfg(test)]
    pub(crate) fn block_change_guard_contains(
        &self,
        position: BlockPosition,
        block: Block,
    ) -> bool {
        self.currently_changing_blocks
            .get(&position)
            .is_some_and(|changed_block| changed_block.block() == block)
    }

    pub fn copy(&self) -> Self {
        let mut copied_world = Self::new_with_dimension(
            self.name.clone(),
            self.dimension_type.clone(),
            self.cached_dimension_type.clone(),
        );
        copied_world.dimension_name = self.dimension_name.clone();
        copied_world.source_world = Some(self.uuid);
        copied_world.last_block_change_time = self.last_block_change_time;
        copied_world.tag_handler = self.tag_handler.copy();
        copied_world.scoreboard_teams = self.scoreboard_teams.clone();
        self.chunks.iter().for_each(|(position, chunk)| {
            let mut copied_chunk = chunk.copy_for_position(*position);
            copied_chunk.set_world(copied_world.uuid);
            copied_world.chunks.insert(*position, copied_chunk);
            copied_world
                .entity_tracker
                .create_chunk_partition(*position);
        });
        copied_world
    }

    pub const fn world_border(&self) -> WorldBorder {
        self.world_border
    }

    pub fn block_position_is_inside_world_border(&self, position: BlockPosition) -> bool {
        self.world_border
            .contains(f64::from(position.x) + 0.5, f64::from(position.z) + 0.5)
    }

    pub fn block_position_has_placement_collision(&self, position: BlockPosition) -> bool {
        self.block_placement_collision_entity(position).is_some()
    }

    pub fn block_placement_collision_entity(&self, position: BlockPosition) -> Option<EntityId> {
        let block_center = Vector3d {
            x: f64::from(position.x) + 0.5,
            y: f64::from(position.y),
            z: f64::from(position.z) + 0.5,
        };
        let block_box = EntityBoundingBox::new(1.0, 1.0, 1.0);
        self.entities
            .iter()
            .find(|entity| match entity {
                Entity::Creature(entity) => {
                    entity.prevents_block_placement()
                        && entity.intersects_box_at(block_center, block_box)
                }
                Entity::ExperienceOrb(entity) => {
                    entity.prevents_block_placement()
                        && entity.intersects_box_at(block_center, block_box)
                }
                Entity::Generic(entity) => {
                    entity.prevents_block_placement()
                        && entity.intersects_box_at(block_center, block_box)
                }
                Entity::Item(entity) => {
                    entity.prevents_block_placement()
                        && entity.intersects_box_at(block_center, block_box)
                }
                Entity::Player(player) => {
                    player.prevents_block_placement()
                        && player_intersects_block(player.position(), block_center, block_box)
                }
                Entity::Projectile(entity) => {
                    entity.prevents_block_placement()
                        && entity.intersects_box_at(block_center, block_box)
                }
            })
            .map(Entity::entity_id)
    }

    pub(crate) fn chunk_is_read_only_at(&self, position: BlockPosition) -> bool {
        self.chunk(position.into())
            .is_some_and(|chunk| chunk.is_read_only())
    }

    pub(crate) fn refresh_chunk_for_client(
        &mut self,
        client: &Client,
        position: BlockPosition,
    ) -> bool {
        let player_chunk = ChunkPosition::from(position);
        self.player_by_addr_mut(&client.addr).is_some_and(|player| {
            player.send_loaded_chunk_position(PlayerChunk::new(player_chunk.x, player_chunk.z))
        })
    }

    pub fn set_world_border(&mut self, world_border: WorldBorder) -> Result<()> {
        self.set_world_border_with_transition(world_border, 0)
    }

    pub fn set_world_border_with_transition(
        &mut self,
        world_border: WorldBorder,
        transition_time: i64,
    ) -> Result<()> {
        self.world_border = world_border;
        let packet = self.create_initialize_world_border_packet_with_transition(transition_time);
        self.dispatch_packet_to_entered_players(packet)
    }

    pub fn create_initialize_world_border_packet(
        &self,
    ) -> spinel_core::network::clientbound::play::initialize_world_border::InitializeWorldBorderPacket
    {
        self.create_initialize_world_border_packet_with_transition(0)
    }

    fn create_initialize_world_border_packet_with_transition(
        &self,
        transition_time: i64,
    ) -> spinel_core::network::clientbound::play::initialize_world_border::InitializeWorldBorderPacket
    {
        self.world_border
            .initialize_packet(self.world_border.diameter(), transition_time)
    }

    pub fn show_boss_bar(&mut self, boss_bar: BossBar) -> Result<bool> {
        if self
            .boss_bars
            .iter()
            .any(|stored_bar| stored_bar.id() == boss_bar.id())
        {
            return Ok(false);
        }
        let packet = boss_bar.add_packet();
        self.boss_bars.push(boss_bar);
        self.dispatch_packet_to_entered_players(packet)?;
        Ok(true)
    }

    pub fn hide_boss_bar(&mut self, boss_bar_id: Uuid) -> Result<bool> {
        let Some(boss_bar_index) = self
            .boss_bars
            .iter()
            .position(|boss_bar| boss_bar.id() == boss_bar_id)
        else {
            return Ok(false);
        };
        let boss_bar = self.boss_bars.remove(boss_bar_index);
        self.dispatch_packet_to_entered_players(boss_bar.remove_packet())?;
        Ok(true)
    }

    pub fn boss_bars(&self) -> &[BossBar] {
        &self.boss_bars
    }

    pub fn register_scoreboard_team(&mut self, team: Team) -> Result<bool> {
        if self.scoreboard_teams.contains_key(team.name()) {
            return Ok(false);
        }
        self.dispatch_packet_to_entered_players(team.create_packet())?;
        self.scoreboard_teams.insert(team.name().to_owned(), team);
        Ok(true)
    }

    pub fn remove_scoreboard_team(&mut self, team_name: &str) -> Result<bool> {
        let Some(team) = self.scoreboard_teams.remove(team_name) else {
            return Ok(false);
        };
        self.entities.iter_mut().for_each(|entity| match entity {
            Entity::Generic(entity) if entity.team() == Some(team_name) => entity.set_team(None),
            Entity::Item(_) => {}
            Entity::Player(player) if player.team() == Some(team_name) => {
                player.set_scoreboard_team(None, None);
            }
            _ => {}
        });
        self.dispatch_packet_to_entered_players(team.remove_packet())?;
        Ok(true)
    }

    pub fn scoreboard_team(&self, team_name: &str) -> Option<&Team> {
        self.scoreboard_teams.get(team_name)
    }

    pub fn scoreboard_teams(&self) -> impl Iterator<Item = &Team> {
        self.scoreboard_teams.values()
    }

    pub fn set_entity_scoreboard_team(
        &mut self,
        entity_id: EntityId,
        team_name: Option<&str>,
    ) -> Result<bool> {
        let current_team_name = self
            .entity_by_id(entity_id)
            .and_then(entity_scoreboard_team_name)
            .map(str::to_owned);
        let requested_team_name = team_name.map(str::to_owned);
        if current_team_name == requested_team_name {
            return Ok(false);
        }
        if let Some(requested_team_name) = requested_team_name.as_deref() {
            if !self.scoreboard_teams.contains_key(requested_team_name) {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    format!("scoreboard team {requested_team_name} is not registered"),
                ));
            }
        }
        let mut previous_team = current_team_name
            .as_deref()
            .and_then(|team_name| self.scoreboard_teams.remove(team_name));
        let mut requested_team = requested_team_name
            .as_deref()
            .and_then(|team_name| self.scoreboard_teams.remove(team_name));
        let packets = self.apply_entity_scoreboard_team(
            entity_id,
            previous_team.as_mut(),
            requested_team.as_mut(),
        );
        if let Some(previous_team) = previous_team {
            self.scoreboard_teams
                .insert(previous_team.name().to_owned(), previous_team);
        }
        if let Some(requested_team) = requested_team {
            self.scoreboard_teams
                .insert(requested_team.name().to_owned(), requested_team);
        }
        let Some(packets) = packets else {
            return Ok(false);
        };
        packets.into_iter().try_for_each(|packet| {
            self.send_packet_to_player_viewers_and_self(entity_id, packet)
        })?;
        Ok(true)
    }

    fn apply_entity_scoreboard_team(
        &mut self,
        entity_id: EntityId,
        previous_team: Option<&mut Team>,
        requested_team: Option<&mut Team>,
    ) -> Option<Vec<SetPlayerTeamPacket>> {
        let entity = self.entity_by_id_mut(entity_id)?;
        Some(match entity {
            Entity::Creature(entity) => entity.set_scoreboard_team(previous_team, requested_team),
            Entity::ExperienceOrb(entity) => {
                entity.set_scoreboard_team(previous_team, requested_team)
            }
            Entity::Generic(entity) => entity.set_scoreboard_team(previous_team, requested_team),
            Entity::Item(_) => Vec::new(),
            Entity::Player(player) => player.set_scoreboard_team(previous_team, requested_team),
            Entity::Projectile(entity) => entity.set_scoreboard_team(previous_team, requested_team),
        })
    }

    pub const fn weather(&self) -> Weather {
        self.weather
    }

    pub fn set_weather(&mut self, weather: Weather) -> Result<()> {
        self.weather = weather;
        self.remaining_rain_transition_ticks = self.default_rain_transition_ticks(weather);
        self.remaining_thunder_transition_ticks = self.default_thunder_transition_ticks(weather);
        Ok(())
    }

    pub fn set_weather_with_transition(
        &mut self,
        weather: Weather,
        transition_ticks: i32,
    ) -> Result<()> {
        if transition_ticks < 1 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Transition ticks cannot be lower than 1",
            ));
        }
        self.weather = weather;
        self.remaining_rain_transition_ticks = transition_ticks;
        self.remaining_thunder_transition_ticks = transition_ticks;
        Ok(())
    }

    pub const fn transitioning_weather(&self) -> Weather {
        self.transitioning_weather
    }

    pub const fn remaining_rain_transition_ticks(&self) -> i32 {
        self.remaining_rain_transition_ticks
    }

    pub const fn remaining_thunder_transition_ticks(&self) -> i32 {
        self.remaining_thunder_transition_ticks
    }

    fn default_rain_transition_ticks(&self, weather: Weather) -> i32 {
        ((weather.rain_level() - self.transitioning_weather.rain_level()).abs() / 0.01).max(1.0)
            as i32
    }

    fn default_thunder_transition_ticks(&self, weather: Weather) -> i32 {
        ((weather.thunder_level() - self.transitioning_weather.thunder_level()).abs() / 0.01)
            .max(1.0) as i32
    }

    pub fn scheduler(&mut self) -> &mut WorldScheduler {
        &mut self.scheduler
    }

    pub fn event_node(&mut self) -> &mut WorldEventNode {
        &mut self.event_node
    }

    pub fn schedule_next_tick(
        &mut self,
        callback: impl FnMut(&mut World) + Send + 'static,
    ) -> crate::scheduler::Task {
        self.scheduler.schedule_next_tick(callback)
    }

    pub fn schedule_tick_end(
        &mut self,
        callback: impl FnMut(&mut World) + Send + 'static,
    ) -> crate::scheduler::Task {
        self.scheduler.schedule_tick_end(callback)
    }

    pub const fn time_packet(&self) -> SetTimePacket {
        SetTimePacket::new(self.world_age, self.time, self.time_rate != 0)
    }

    pub fn generator(&self) -> Option<&(dyn Generator + Send + Sync)> {
        self.generator.as_deref()
    }

    pub fn set_generator(
        &mut self,
        generator: impl Fn(&mut GenerationUnit) + Send + Sync + 'static,
    ) {
        self.generator = Some(Arc::new(generator));
    }
    pub fn set_fallible_generator(
        &mut self,
        generator: impl Fn(&mut GenerationUnit) -> std::result::Result<(), GenerateChunkError>
        + Send
        + Sync
        + 'static,
    ) {
        self.generator = Some(Arc::new(FallibleGenerator::new(generator)));
    }
    pub fn clear_generator(&mut self) {
        self.generator = None;
    }

    pub fn register_block_handler(&mut self, block: Block, handler: impl BlockHandler + 'static) {
        self.block_handlers.register(block, handler);
    }

    pub fn register_block_placement_rule(&mut self, rule: impl BlockPlacementRule + 'static) {
        self.block_placement_rules.register(rule);
    }

    pub fn block_is_self_replaceable(&self, replacement: BlockReplacement) -> bool {
        self.block_placement_rules
            .rule(replacement.block())
            .is_some_and(|rule| rule.is_self_replaceable(replacement))
    }

    pub fn explosion_supplier(&self) -> Option<&dyn ExplosionSupplier> {
        self.explosion_supplier.as_deref()
    }

    pub fn set_explosion_supplier(&mut self, supplier: impl ExplosionSupplier + 'static) {
        self.explosion_supplier = Some(Box::new(supplier));
    }

    pub fn clear_explosion_supplier(&mut self) {
        self.explosion_supplier = None;
    }

    pub fn explode(&mut self, center: EntityPosition, strength: f32) -> Result<Vec<BlockPosition>> {
        self.explode_with_data(center, strength, None)
    }

    pub fn explode_with_data(
        &mut self,
        center: EntityPosition,
        strength: f32,
        additional_data: Option<&NbtCompound>,
    ) -> Result<Vec<BlockPosition>> {
        let Some(explosion_supplier) = self.explosion_supplier() else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "No explosion supplier was set",
            ));
        };
        let explosion =
            explosion_supplier.create_explosion_with_data(center, strength, additional_data);
        explosion.apply(self)
    }

    pub fn update_snapshot(&self) -> WorldSnapshot {
        WorldSnapshot::from_world(self)
    }

    pub fn set_chunk_loader(&mut self, chunk_loader: impl ChunkLoader + 'static) {
        self.chunk_loader = Arc::new(chunk_loader);
    }

    pub fn chunk_loader(&self) -> &dyn ChunkLoader {
        self.chunk_loader.as_ref()
    }

    pub fn set_chunk_supplier(
        &mut self,
        create_chunk: impl Fn(ChunkPosition) -> Chunk + Send + Sync + 'static,
    ) {
        self.chunk_supplier = ChunkSupplier::new(create_chunk);
    }

    pub fn chunk_supplier(&self) -> &ChunkSupplier {
        &self.chunk_supplier
    }

    pub(crate) fn set_event_dispatcher(&mut self, server: usize) {
        self.event_dispatcher = Some(server);
    }

    pub fn load_instance(&self) -> Result<()> {
        self.chunk_loader.load_instance()
    }

    pub fn save_instance(&self) -> Result<()> {
        self.chunk_loader.save_instance()
    }

    pub fn save_chunk(&self, position: ChunkPosition) -> Result<bool> {
        let Some(chunk) = self.chunks.get(&position) else {
            return Ok(false);
        };
        self.chunk_loader.save_chunk(chunk)?;
        Ok(true)
    }

    pub fn save_chunks(&self) -> Result<()> {
        let chunks = self.chunks.values().collect::<Vec<_>>();
        self.chunk_loader.save_chunks(&chunks)
    }

    pub fn chunk(&self, position: ChunkPosition) -> Option<&Chunk> {
        self.chunks.get(&position)
    }

    pub fn chunks(&self) -> impl Iterator<Item = &Chunk> {
        self.chunks.values()
    }

    pub fn chunk_at(&self, x: f64, z: f64) -> Option<&Chunk> {
        self.chunk(ChunkPosition::new(
            (x.floor() as i32).div_euclid(16),
            (z.floor() as i32).div_euclid(16),
        ))
    }

    pub fn chunk_at_position(&self, position: impl Into<ChunkPosition>) -> Option<&Chunk> {
        self.chunk(position.into())
    }

    pub fn is_chunk_loaded(&self, position: ChunkPosition) -> bool {
        self.chunks
            .get(&position)
            .is_some_and(|chunk| chunk.is_loaded())
    }

    pub fn is_chunk_loaded_at(&self, position: impl Into<ChunkPosition>) -> bool {
        self.is_chunk_loaded(position.into())
    }

    pub fn enable_auto_chunk_load(&mut self, enable: bool) {
        self.auto_chunk_load = enable;
    }

    pub const fn has_enabled_auto_chunk_load(&self) -> bool {
        self.auto_chunk_load
    }

    pub fn load_chunk(&mut self, position: ChunkPosition) -> Result<&mut Chunk> {
        self.load_chunk_with_event_flag(position, true)
    }

    pub fn load_chunk_at(&mut self, position: impl Into<ChunkPosition>) -> Result<&mut Chunk> {
        self.load_chunk(position.into())
    }

    pub fn load_chunk_result(&mut self, position: ChunkPosition) -> Result<&mut Chunk> {
        self.load_chunk(position)
    }

    pub fn load_optional_chunk(&mut self, position: ChunkPosition) -> Option<&mut Chunk> {
        if self.chunks.contains_key(&position) {
            return self.chunks.get_mut(&position);
        }
        if !self.auto_chunk_load {
            return None;
        }
        self.load_chunk(position).ok()
    }

    pub fn load_optional_chunk_at(
        &mut self,
        position: impl Into<ChunkPosition>,
    ) -> Option<&mut Chunk> {
        self.load_optional_chunk(position.into())
    }

    pub fn load_optional_chunk_result(
        &mut self,
        position: ChunkPosition,
    ) -> Result<Option<&mut Chunk>> {
        if self.chunks.contains_key(&position) {
            return Ok(self.chunks.get_mut(&position));
        }
        if !self.auto_chunk_load {
            return Ok(None);
        }
        self.load_chunk(position).map(Some)
    }

    pub fn load_optional_chunks(
        &mut self,
        positions: &[ChunkPosition],
    ) -> Result<Vec<ChunkPosition>> {
        positions
            .iter()
            .copied()
            .map(|position| {
                self.load_optional_chunk_result(position)
                    .map(|chunk| chunk.map(|_| position))
            })
            .collect::<Result<Vec<_>>>()
            .map(|positions| positions.into_iter().flatten().collect())
    }

    pub fn teleport_player(
        &mut self,
        player_uuid: Uuid,
        position: EntityPosition,
        chunks: Option<Vec<i64>>,
        flags: TeleportFlags,
        should_confirm: bool,
    ) -> Result<Option<crate::entity::EntityTeleport>> {
        self.teleport_player_with_velocity(
            player_uuid,
            position,
            Velocity(Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            chunks,
            flags.with(TeleportFlags::DELTA_COORD),
            should_confirm,
        )
    }

    pub fn teleport_player_with_velocity(
        &mut self,
        player_uuid: Uuid,
        position: EntityPosition,
        velocity: Velocity,
        chunks: Option<Vec<i64>>,
        flags: TeleportFlags,
        should_confirm: bool,
    ) -> Result<Option<crate::entity::EntityTeleport>> {
        let Some(player) = self.player_by_uuid(player_uuid) else {
            return Ok(None);
        };
        let entity_id = player.entity_id();
        let previous_position = player.position();
        let destination = player.teleport_destination(position, flags);
        let chunk_transition = player.chunk_transition(
            destination.x(),
            destination.y(),
            destination.z(),
            self.view_distance,
        );
        self.dispatch_entity_teleport_event(entity_id, position, destination, flags);
        self.load_teleport_chunks(previous_position, destination, chunks.as_deref())?;
        let world_view_distance = self.view_distance;
        let teleport = {
            let player = self.player_by_uuid_mut(player_uuid).ok_or_else(|| {
                Error::new(ErrorKind::NotFound, "Player was removed before teleport")
            })?;
            player.refresh_chunks_after_teleport(chunk_transition, world_view_distance)?;
            player.teleport_with_velocity_chunks_and_flags(
                position,
                velocity,
                chunks,
                flags,
                should_confirm,
            )?
        };
        let entity_id = self
            .player_by_uuid(player_uuid)
            .map(Player::entity_id)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Player was removed after teleport"))?;
        self.entity_tracker.move_entity(entity_id, destination);
        self.refresh_passenger_positions(entity_id);
        self.schedule_entity_visibility_refresh(entity_id);
        let synchronization_packet = self
            .entity_by_id_mut(entity_id)
            .map(Entity::synchronize_position_packet)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Player was removed after teleport"))?;
        self.send_packet_to_entity_viewers(entity_id, synchronization_packet)?;
        Ok(Some(teleport))
    }

    pub fn teleport_entity(
        &mut self,
        entity_id: EntityId,
        position: EntityPosition,
        chunks: Option<Vec<i64>>,
        flags: TeleportFlags,
    ) -> Result<Option<EntityTeleport>> {
        self.teleport_entity_with_velocity(
            entity_id,
            position,
            Velocity(Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            chunks,
            flags.with(TeleportFlags::DELTA_COORD),
        )
    }

    pub fn teleport_entity_with_velocity(
        &mut self,
        entity_id: EntityId,
        position: EntityPosition,
        velocity: Velocity,
        chunks: Option<Vec<i64>>,
        flags: TeleportFlags,
    ) -> Result<Option<EntityTeleport>> {
        let Some(entity) = self.entity_by_id(entity_id) else {
            return Ok(None);
        };
        if let Entity::Player(player) = entity {
            return self.teleport_player_with_velocity(
                player.uuid,
                position,
                velocity,
                chunks,
                flags,
                true,
            );
        }
        let previous_position = entity.position();
        let teleport = EntityTeleport::resolve(
            previous_position,
            entity.velocity(),
            position,
            velocity,
            chunks,
            flags,
        );
        self.dispatch_entity_teleport_event(entity_id, position, teleport.position(), flags);
        self.load_teleport_chunks(previous_position, teleport.position(), teleport.chunks())?;
        let entity = self
            .entity_by_id_mut(entity_id)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Entity was removed before teleport"))?;
        entity.set_position(teleport.position());
        entity.set_velocity(teleport.velocity());
        self.entity_tracker
            .move_entity(entity_id, teleport.position());
        self.refresh_passenger_positions(entity_id);
        self.schedule_entity_visibility_refresh(entity_id);
        let synchronization_packet = self
            .entity_by_id_mut(entity_id)
            .map(Entity::synchronize_position_packet)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Entity was removed after teleport"))?;
        self.send_packet_to_entity_viewers(entity_id, synchronization_packet)?;
        Ok(Some(teleport))
    }

    pub fn teleport_entity_future(
        &mut self,
        entity_id: EntityId,
        position: EntityPosition,
        chunks: Option<Vec<i64>>,
        flags: TeleportFlags,
        should_confirm: bool,
    ) -> Result<Option<EntityTeleportTicket>> {
        self.teleport_entity_future_with_velocity(
            entity_id,
            position,
            Velocity(Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            chunks,
            flags.with(TeleportFlags::DELTA_COORD),
            should_confirm,
        )
    }

    pub fn teleport_entity_future_with_velocity(
        &mut self,
        entity_id: EntityId,
        position: EntityPosition,
        velocity: Velocity,
        chunks: Option<Vec<i64>>,
        flags: TeleportFlags,
        should_confirm: bool,
    ) -> Result<Option<EntityTeleportTicket>> {
        let Some(entity) = self.entity_by_id(entity_id) else {
            return Ok(None);
        };
        let previous_position = entity.position();
        let teleport = EntityTeleport::resolve(
            previous_position,
            entity.velocity(),
            position,
            velocity,
            chunks,
            flags,
        );
        self.dispatch_entity_teleport_event(entity_id, position, teleport.position(), flags);
        let chunk_load_tickets = self.begin_teleport_chunk_loads(
            previous_position,
            teleport.position(),
            teleport.chunks(),
        )?;
        Ok(Some(EntityTeleportTicket {
            entity_id,
            teleport,
            chunk_load_tickets,
            should_confirm,
            completed: false,
        }))
    }

    pub fn set_entity_velocity(&mut self, entity_id: EntityId, velocity: Velocity) -> Result<bool> {
        if self.entity_by_id(entity_id).is_none() {
            return Ok(false);
        }
        let Some(velocity) = self.dispatch_entity_velocity_event(entity_id, velocity) else {
            return Ok(false);
        };
        let entity = self.entity_by_id_mut(entity_id).ok_or_else(|| {
            Error::other(format!(
                "entity {entity_id:?} disappeared during velocity event dispatch"
            ))
        })?;
        entity.set_velocity(velocity);
        let velocity_packet = entity.velocity_packet();
        self.send_packet_to_player_viewers_and_self(entity_id, velocity_packet)?;
        Ok(true)
    }

    pub fn complete_entity_teleport(
        &mut self,
        ticket: &mut EntityTeleportTicket,
    ) -> Result<Option<EntityTeleport>> {
        if ticket.completed {
            return Ok(Some(ticket.teleport.clone()));
        }
        for chunk_load_ticket in &ticket.chunk_load_tickets {
            if !self.complete_chunk_load(chunk_load_ticket)? {
                return Ok(None);
            }
        }
        let entity_id = ticket.entity_id;
        let teleport = ticket.teleport.clone();
        let player_chunk_transition = match self.entity_by_id(entity_id) {
            Some(Entity::Player(player)) => player.chunk_transition(
                teleport.position().x(),
                teleport.position().y(),
                teleport.position().z(),
                self.view_distance,
            ),
            Some(_) => None,
            None => {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    "Entity was removed before teleport completion",
                ));
            }
        };
        if let Some(player_chunk_transition) = player_chunk_transition.as_ref() {
            let player_address = self
                .entity_by_id(entity_id)
                .and_then(|entity| match entity {
                    Entity::Player(player) => Some(player.address()),
                    _ => None,
                })
                .ok_or_else(|| Error::new(ErrorKind::NotFound, "Player was removed"))?;
            self.schedule_player_chunk_loads(player_address, &player_chunk_transition.arriving)?;
        }
        let world_view_distance = self.view_distance;
        let entity = self.entity_by_id_mut(entity_id).ok_or_else(|| {
            Error::new(
                ErrorKind::NotFound,
                "Entity was removed before teleport completion",
            )
        })?;
        match entity {
            Entity::Player(player) => {
                player
                    .refresh_chunks_after_teleport(player_chunk_transition, world_view_distance)?;
                player.apply_teleport(&teleport, ticket.should_confirm)?;
            }
            _ => {
                entity.set_position(teleport.position());
                entity.set_velocity(teleport.velocity());
            }
        }
        self.entity_tracker
            .move_entity(entity_id, teleport.position());
        self.refresh_passenger_positions(entity_id);
        self.schedule_entity_visibility_refresh(entity_id);
        let synchronization_packet = self
            .entity_by_id_mut(entity_id)
            .map(Entity::synchronize_position_packet)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Entity was removed after teleport"))?;
        self.send_packet_to_entity_viewers(entity_id, synchronization_packet)?;
        ticket.completed = true;
        Ok(Some(teleport))
    }

    pub fn retrieve_chunk<'world>(
        &'world self,
        current_chunk: Option<&'world Chunk>,
        position: impl Into<ChunkPosition>,
    ) -> Option<&'world Chunk> {
        let position = position.into();
        let current_chunk_matches = current_chunk.is_some_and(|chunk| {
            chunk.is_loaded() && chunk.x() == position.x && chunk.z() == position.z
        });
        if current_chunk_matches {
            return current_chunk;
        }
        self.chunk(position)
    }

    pub fn load_chunk_future(&mut self, position: ChunkPosition) -> Result<ChunkLoadTicket> {
        self.load_chunk_future_with_optional_flag(position, true)
            .and_then(|ticket| {
                ticket.ok_or_else(|| Error::new(ErrorKind::NotFound, "Chunk was not loaded"))
            })
    }

    pub fn load_optional_chunk_future(
        &mut self,
        position: ChunkPosition,
    ) -> Result<Option<ChunkLoadTicket>> {
        self.load_chunk_future_with_optional_flag(position, self.auto_chunk_load)
    }

    pub fn complete_chunk_load(&mut self, ticket: &ChunkLoadTicket) -> Result<bool> {
        if ticket.is_completed() {
            return Ok(true);
        }
        if self.chunks.contains_key(&ticket.position) {
            ticket.complete();
            self.async_chunk_loads.remove(&ticket.position);
            self.queue_waiting_players_for_loaded_chunk(ticket.position);
            return Ok(true);
        }
        self.receive_completed_chunk_loads();
        let Some((_, prepared_chunk_load)) = self.prepared_chunk_loads.remove(&ticket.id) else {
            return Ok(false);
        };
        self.async_chunk_loads.remove(&ticket.position);
        let PreparedChunkLoad {
            mut chunk,
            generation_forks,
            requires_generation_completion,
        } = match prepared_chunk_load {
            Ok(prepared_chunk_load) => prepared_chunk_load,
            Err(error) => {
                self.player_chunk_load_waiters.remove(&ticket.position);
                return Err(error);
            }
        };
        chunk.set_world(self.uuid);
        generation_forks
            .into_iter()
            .for_each(|fork| self.store_generation_fork(fork));
        self.chunks.insert(ticket.position, chunk);
        self.entity_tracker.create_chunk_partition(ticket.position);
        if requires_generation_completion {
            let Some(mut chunk) = self.chunks.remove(&ticket.position) else {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    "Prepared chunk disappeared before generation completion",
                ));
            };
            self.apply_pending_generation(&mut chunk);
            chunk.on_generate();
            self.chunks.insert(ticket.position, chunk);
            self.invalidate_generated_chunk_lighting(ticket.position);
        }
        if let Some(chunk) = self.chunks.get_mut(&ticket.position) {
            chunk.on_load();
        }
        self.dispatch_instance_chunk_load_event(ticket.position);
        ticket.complete();
        self.queue_waiting_players_for_loaded_chunk(ticket.position);
        Ok(true)
    }

    pub fn chunk_load_in_progress(&self, position: ChunkPosition) -> bool {
        self.async_chunk_loads.contains_key(&position)
    }

    pub fn save_instance_future(&self) -> WorldIoTask {
        self.optional_io_task(self.chunk_loader.supports_parallel_saving(), {
            let chunk_loader = self.chunk_loader.clone();
            move || chunk_loader.save_instance()
        })
    }

    pub fn save_chunk_future(&self, position: ChunkPosition) -> WorldIoTask {
        let Some(chunk) = self
            .chunks
            .get(&position)
            .map(|chunk| chunk.copy_for_position(position))
        else {
            return WorldIoTask::completed(Ok(()));
        };
        self.optional_io_task(self.chunk_loader.supports_parallel_saving(), {
            let chunk_loader = self.chunk_loader.clone();
            move || chunk_loader.save_chunk(&chunk)
        })
    }

    pub fn save_chunks_future(&self) -> WorldIoTask {
        let chunks = self
            .chunks
            .values()
            .map(|chunk| chunk.copy_for_position(ChunkPosition::new(chunk.x(), chunk.z())))
            .collect::<Vec<_>>();
        self.optional_io_task(self.chunk_loader.supports_parallel_saving(), {
            let chunk_loader = self.chunk_loader.clone();
            move || {
                let chunk_refs = chunks.iter().collect::<Vec<_>>();
                chunk_loader.save_chunks(&chunk_refs)
            }
        })
    }

    pub fn unload_chunk(&mut self, chunk: impl Into<ChunkPosition>) -> Result<bool> {
        let position = chunk.into();
        if !self.chunks.contains_key(&position) {
            return Ok(false);
        }
        self.send_chunk_unload_to_players(position)?;
        self.dispatch_instance_chunk_unload_event(position);
        self.remove_entities_in_chunk(position);
        self.entity_tracker.delete_chunk_partition(position);
        let Some(mut chunk) = self.chunks.remove(&position) else {
            return Ok(false);
        };
        chunk.unload();
        self.chunk_loader.unload_chunk(&mut chunk)?;
        Ok(true)
    }

    pub fn tick_chunks(&mut self, time: u64) -> usize {
        if self.chunks.values().any(Chunk::lighting_update_is_due) {
            WorldLighting::relight(
                &mut self.chunks,
                self.cached_dimension_type.has_skylight,
                None,
            );
        }
        let mut lighting_packets = Vec::new();
        let ticked_block_count = self
            .chunks
            .iter_mut()
            .filter(|(_, chunk)| chunk.is_loaded())
            .map(|(position, chunk)| {
                let light_data = chunk.tick_lighting();
                if let Some(light_data) = light_data {
                    lighting_packets.push((
                        *position,
                        LightUpdatePacket::new(position.x, position.z, light_data),
                    ));
                }
                chunk.tick(self.uuid, &self.block_handlers, time)
            })
            .sum();
        lighting_packets.into_iter().for_each(|(position, packet)| {
            let _ = self.dispatch_packet_to_chunk_viewers(position, packet);
        });
        ticked_block_count
    }

    fn dispatch_instance_chunk_load_event(&mut self, position: ChunkPosition) {
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let world = self as *mut World;
        InstanceChunkLoadEvent::new(world, position).dispatch(server);
    }

    fn dispatch_instance_chunk_unload_event(&mut self, position: ChunkPosition) {
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let world = self as *mut World;
        InstanceChunkUnloadEvent::new(world, position).dispatch(server);
    }

    pub(crate) fn dispatch_instance_register_event(&mut self) {
        self.dispatch_world_event_node("InstanceRegisterEvent");
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let world = self as *mut World;
        InstanceRegisterEvent::new(world).dispatch(server);
    }

    pub(crate) fn dispatch_instance_unregister_event(&mut self) {
        self.dispatch_world_event_node("InstanceUnregisterEvent");
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let world = self as *mut World;
        InstanceUnregisterEvent::new(world).dispatch(server);
    }

    fn dispatch_instance_section_invalidate_event(
        &mut self,
        section_x: i32,
        section_y: i32,
        section_z: i32,
    ) {
        self.dispatch_world_event_node("InstanceSectionInvalidateEvent");
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let world = self as *mut World;
        InstanceSectionInvalidateEvent::new(world, section_x, section_y, section_z)
            .dispatch(server);
    }

    fn dispatch_instance_block_update_event(&mut self, position: BlockPosition, block: Block) {
        self.dispatch_world_event_node("InstanceBlockUpdateEvent");
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let world = self as *mut World;
        InstanceBlockUpdateEvent::new(world, position, block).dispatch(server);
    }

    fn dispatch_instance_tick_event(&mut self) {
        self.dispatch_world_event_node("InstanceTickEvent");
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let world = self as *mut World;
        InstanceTickEvent::new(world, self.world_age).dispatch(server);
    }

    fn dispatch_instance_tick_end_event(&mut self) {
        self.dispatch_world_event_node("InstanceTickEndEvent");
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let world = self as *mut World;
        InstanceTickEndEvent::new(world, self.world_age).dispatch(server);
    }

    fn dispatch_world_event_node(&mut self, event_name: &'static str) {
        let mut event_node = std::mem::take(&mut self.event_node);
        event_node.dispatch(event_name, self);
        self.event_node = event_node;
    }

    pub(crate) fn dispatch_add_entity_to_instance_event(&mut self, entity: &mut Entity) -> bool {
        let Some(server_ptr) = self.event_dispatcher else {
            return false;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let world = self as *mut World;
        let entity = entity as *mut Entity;
        let mut event = AddEntityToInstanceEvent::new(world, entity);
        event.dispatch(server);
        event.is_cancelled()
    }

    fn dispatch_entity_spawn_event(&mut self, entity_id: EntityId) {
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let Some(entity) = self
            .entity_by_id_mut(entity_id)
            .map(|entity| entity as *mut Entity)
        else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let world = self as *mut World;
        EntitySpawnEvent::new(entity, world).dispatch(server);
    }

    fn dispatch_entity_despawn_event(&mut self, entity_id: EntityId) {
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let Some(entity) = self
            .entity_by_id_mut(entity_id)
            .map(|entity| entity as *mut Entity)
        else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        EntityDespawnEvent::new(entity).dispatch(server);
    }

    fn dispatch_remove_entity_from_instance_event(&mut self, entity_id: EntityId) {
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let Some(entity) = self
            .entity_by_id_mut(entity_id)
            .map(|entity| entity as *mut Entity)
        else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let world = self as *mut World;
        RemoveEntityFromInstanceEvent::new(world, entity).dispatch(server);
    }

    fn send_chunk_unload_to_players(&mut self, position: ChunkPosition) -> Result<()> {
        let player_chunk = PlayerChunk {
            x: position.x,
            z: position.z,
        };
        let world_view_distance = self.view_distance;
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player)
                    if player.has_entered_world()
                        && player.has_chunk_loaded_by_client(player_chunk, world_view_distance) =>
                {
                    Some(player)
                }
                _ => None,
            })
            .try_for_each(|player| {
                let Some(client) = player.client_mut().map(|client| client as *mut Client) else {
                    return Ok(());
                };
                let client = unsafe { &mut *client };
                player.forget_loaded_chunk(client, player_chunk)
            })
    }

    fn remove_entities_in_chunk(&mut self, position: ChunkPosition) {
        let removed_entity_ids = self
            .entities
            .iter()
            .filter(|entity| chunk_position_for_entity_position(entity.position()) == position)
            .map(Entity::entity_id)
            .collect::<Vec<_>>();
        removed_entity_ids.into_iter().for_each(|entity_id| {
            self.take_entity(entity_id);
        });
    }
    pub fn regenerate_chunk(&mut self, position: ChunkPosition) {
        if let Some(chunk) = self.chunks.get_mut(&position) {
            chunk.clear_invalidated();
        }
        self.generate_chunk(position);
    }
    pub fn generate_chunk(&mut self, position: ChunkPosition) {
        let _ = self.generate_chunk_result(position);
    }

    pub fn generate_chunk_result(&mut self, position: ChunkPosition) -> Result<bool> {
        self.load_chunk(position)?;
        let Some(generator) = self.generator.take() else {
            return Ok(self.chunks.contains_key(&position));
        };
        let generation_result =
            self.generate_loaded_chunk_with_result(position, generator.as_ref());
        self.generator = Some(generator);
        generation_result
    }

    pub fn generate_chunk_with_result(
        &mut self,
        position: ChunkPosition,
        generator: &(dyn Generator + Send + Sync),
    ) -> Result<bool> {
        self.load_chunk(position)?;
        self.generate_loaded_chunk_with_result(position, generator)
    }

    fn generate_loaded_chunk_with_result(
        &mut self,
        position: ChunkPosition,
        generator: &(dyn Generator + Send + Sync),
    ) -> Result<bool> {
        let Some(mut chunk) = self.chunks.remove(&position) else {
            return Ok(false);
        };
        let generation_result = self.apply_generation(&mut chunk, generator);
        self.chunks.insert(position, chunk);
        generation_result.map(|_| {
            self.invalidate_generated_chunk_lighting(position);
            self.queue_chunk_for_viewers(position);
            true
        })
    }

    pub fn add_entity(&mut self, mut entity: Entity) -> bool {
        if self.dispatch_add_entity_to_instance_event(&mut entity) {
            return false;
        }
        self.add_entity_after_instance_event(entity);
        true
    }

    pub(crate) fn add_entity_after_instance_event(&mut self, mut entity: Entity) {
        entity.set_world(self.uuid);
        self.entity_tracker.register(&entity);
        let entity_id = entity.entity_id();
        self.entities.push(entity);
        self.schedule_entity_visibility_refresh(entity_id);
        self.dispatch_entity_spawn_event(entity_id);
    }

    pub(crate) fn take_entity(&mut self, entity_id: EntityId) -> Option<Entity> {
        self.dispatch_entity_despawn_event(entity_id);
        self.take_entity_from_instance(entity_id)
    }

    pub(crate) fn take_entity_from_instance(&mut self, entity_id: EntityId) -> Option<Entity> {
        self.dispatch_remove_entity_from_instance_event(entity_id);
        self.detach_entity_passenger_relations(entity_id);
        self.detach_leashed_entities(entity_id);
        let _ = self.hide_entity_from_all_viewers(entity_id);
        let entity_index = self
            .entities
            .iter()
            .position(|entity| entity.entity_id() == entity_id)?;
        self.entity_tracker.unregister(entity_id);
        Some(self.entities.remove(entity_index))
    }

    pub(crate) fn take_player_by_uuid(&mut self, player_uuid: Uuid) -> Option<Player> {
        let entity_index = self.entities.iter().position(|entity| match entity {
            Entity::Player(player) => player.uuid() == player_uuid,
            Entity::Creature(_) => false,
            Entity::ExperienceOrb(_) => false,
            Entity::Generic(_) => false,
            Entity::Item(_) => false,
            Entity::Projectile(_) => false,
        })?;
        let player_id = self.entities[entity_index].entity_id();
        self.detach_entity_passenger_relations(player_id);
        self.detach_leashed_entities(player_id);
        let _ = self.hide_entity_from_all_viewers(player_id);
        let Entity::Player(player) = self.entities.remove(entity_index) else {
            return None;
        };
        self.entity_tracker.unregister(player.entity_id());
        Some(player)
    }

    pub fn entity_tracker(&self) -> &EntityTracker {
        &self.entity_tracker
    }

    #[cfg(test)]
    pub(crate) fn entity_tracker_mut(&mut self) -> &mut EntityTracker {
        &mut self.entity_tracker
    }

    pub fn entities(&self) -> impl Iterator<Item = &Entity> {
        self.entities.iter()
    }

    pub fn entity_by_id(&self, entity_id: EntityId) -> Option<&Entity> {
        self.entities
            .iter()
            .find(|entity| entity.entity_id() == entity_id)
    }

    pub fn creature_by_id_mut(
        &mut self,
        entity_id: EntityId,
    ) -> Option<&mut crate::entity::CreatureEntity> {
        self.entities.iter_mut().find_map(|entity| match entity {
            Entity::Creature(creature) if creature.entity_id() == entity_id => Some(creature),
            _ => None,
        })
    }

    pub(crate) fn entity_by_id_mut(&mut self, entity_id: EntityId) -> Option<&mut Entity> {
        self.entities
            .iter_mut()
            .find(|entity| entity.entity_id() == entity_id)
    }

    pub fn entity_by_uuid(&self, entity_uuid: Uuid) -> Option<&Entity> {
        self.entities
            .iter()
            .find(|entity| entity.uuid() == entity_uuid)
    }

    pub fn add_entity_viewer(
        &mut self,
        viewed_entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> Result<bool> {
        if viewed_entity_id == viewer_player_id {
            return Ok(false);
        }
        let Some(viewed_entity) = self.entity_by_id(viewed_entity_id) else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Entity must be in this world before adding viewers",
            ));
        };
        if viewed_entity.world() != Some(self.uuid) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Entity must be active in this world before adding viewers",
            ));
        }
        if !matches!(self.entity_by_id(viewer_player_id), Some(Entity::Player(_))) {
            return Err(Error::new(
                ErrorKind::NotFound,
                "Viewer player was not found in this world",
            ));
        }
        let Some(viewed_entity) = self.entity_by_id_mut(viewed_entity_id) else {
            return Ok(false);
        };
        if !viewed_entity.view_mut().manual_add(viewer_player_id) {
            return Ok(false);
        }
        self.send_single_entity_spawn_to_player(viewed_entity_id, viewer_player_id)?;
        Ok(true)
    }

    pub fn remove_entity_viewer(
        &mut self,
        viewed_entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> Result<bool> {
        if viewed_entity_id == viewer_player_id {
            return Ok(false);
        }
        let Some(viewed_entity) = self.entity_by_id(viewed_entity_id) else {
            return Ok(false);
        };
        if !viewed_entity
            .view()
            .manual_viewers()
            .contains(&viewer_player_id)
        {
            return Ok(false);
        }
        if !matches!(self.entity_by_id(viewer_player_id), Some(Entity::Player(_))) {
            return Err(Error::new(
                ErrorKind::NotFound,
                "Viewer player was not found in this world",
            ));
        }
        self.hide_single_visibility_pair(viewed_entity_id, viewer_player_id)?;
        Ok(true)
    }

    pub fn add_passenger(&mut self, vehicle_id: EntityId, passenger_id: EntityId) -> Result<bool> {
        if vehicle_id == passenger_id {
            return Ok(false);
        }
        let Some(vehicle) = self.entity_by_id(vehicle_id) else {
            return Ok(false);
        };
        if vehicle.vehicle() == Some(passenger_id) {
            return Ok(false);
        }
        let Some(passenger) = self.entity_by_id(passenger_id) else {
            return Ok(false);
        };
        if let Some(previous_vehicle_id) = passenger.vehicle() {
            self.remove_passenger(previous_vehicle_id, passenger_id)?;
        }
        let Some(vehicle_index) = self
            .entities
            .iter()
            .position(|entity| entity.entity_id() == vehicle_id)
        else {
            return Ok(false);
        };
        let Some(passenger_index) = self
            .entities
            .iter()
            .position(|entity| entity.entity_id() == passenger_id)
        else {
            return Ok(false);
        };
        let passenger_position =
            self.entities[vehicle_index].passenger_position(&self.entities[passenger_index]);
        let passenger_packet = {
            let (vehicle, passenger) =
                distinct_entities_mut(&mut self.entities, vehicle_index, passenger_index);
            vehicle.add_passenger(passenger_id);
            passenger.set_vehicle(vehicle_id);
            passenger.set_position(passenger_position);
            vehicle.passenger_packet()
        };
        self.entity_tracker
            .move_entity(passenger_id, passenger_position);
        self.schedule_entity_visibility_refresh(passenger_id);
        self.refresh_passenger_positions(passenger_id);
        self.send_packet_to_player_viewers_and_self(vehicle_id, passenger_packet)?;
        if let Some(position_sync_packet) = self
            .entity_by_id_mut(passenger_id)
            .map(Entity::synchronize_position_packet)
        {
            self.send_packet_to_player_viewers_and_self(passenger_id, position_sync_packet)?;
        }
        Ok(true)
    }

    pub fn remove_passenger(
        &mut self,
        vehicle_id: EntityId,
        passenger_id: EntityId,
    ) -> Result<bool> {
        if vehicle_id == passenger_id {
            return Ok(false);
        }
        let Some(vehicle_index) = self
            .entities
            .iter()
            .position(|entity| entity.entity_id() == vehicle_id)
        else {
            return Ok(false);
        };
        let passenger_packet = {
            let vehicle = &mut self.entities[vehicle_index];
            if !vehicle.remove_passenger(passenger_id) {
                return Ok(false);
            }
            vehicle.passenger_packet()
        };
        if let Some(passenger) = self.entity_by_id_mut(passenger_id) {
            passenger.clear_vehicle();
        }
        self.send_packet_to_player_viewers_and_self(vehicle_id, passenger_packet)?;
        if let Some(position_sync_packet) = self
            .entity_by_id_mut(passenger_id)
            .map(Entity::synchronize_position_packet)
        {
            self.send_packet_to_player_viewers_and_self(passenger_id, position_sync_packet)?;
        }
        Ok(true)
    }

    pub fn set_leash_holder(
        &mut self,
        entity_id: EntityId,
        leash_holder_id: Option<EntityId>,
    ) -> Result<bool> {
        let Some(previous_leash_holder_id) = self.entity_by_id(entity_id).map(Entity::leash_holder)
        else {
            return Ok(false);
        };
        if leash_holder_id.is_some_and(|holder_id| self.entity_by_id(holder_id).is_none()) {
            return Ok(false);
        }
        if let Some(previous_leash_holder_id) = previous_leash_holder_id {
            if let Some(previous_leash_holder) = self.entity_by_id_mut(previous_leash_holder_id) {
                previous_leash_holder.remove_leashed_entity(entity_id);
            }
        }
        if let Some(leash_holder_id) = leash_holder_id {
            if let Some(leash_holder) = self.entity_by_id_mut(leash_holder_id) {
                leash_holder.add_leashed_entity(entity_id);
            }
        }
        let Some(entity) = self.entity_by_id_mut(entity_id) else {
            return Ok(false);
        };
        entity.set_leash_holder(leash_holder_id);
        let attach_entity_packet = entity.attach_entity_packet();
        self.send_packet_to_player_viewers_and_self(entity_id, attach_entity_packet)?;
        Ok(true)
    }

    fn detach_entity_passenger_relations(&mut self, entity_id: EntityId) {
        let Some(entity) = self.entity_by_id(entity_id) else {
            return;
        };
        let passenger_ids = entity.passengers().iter().copied().collect::<Vec<_>>();
        let vehicle_id = entity.vehicle();
        passenger_ids.into_iter().for_each(|passenger_id| {
            let _ = self.remove_passenger(entity_id, passenger_id);
        });
        if let Some(vehicle_id) = vehicle_id {
            let _ = self.remove_passenger(vehicle_id, entity_id);
        }
    }

    fn detach_leashed_entities(&mut self, entity_id: EntityId) {
        let leashed_entity_ids = self
            .entity_by_id(entity_id)
            .map(|entity| {
                entity
                    .leashed_entities()
                    .iter()
                    .copied()
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        leashed_entity_ids
            .into_iter()
            .for_each(|leashed_entity_id| {
                let _ = self.set_leash_holder(leashed_entity_id, None);
            });
    }

    fn refresh_passenger_positions(&mut self, vehicle_id: EntityId) {
        let mut pending_vehicle_ids = vec![vehicle_id];
        let mut refreshed_vehicle_ids = std::collections::BTreeSet::new();
        while let Some(vehicle_id) = pending_vehicle_ids.pop() {
            if !refreshed_vehicle_ids.insert(vehicle_id) {
                continue;
            }
            let passenger_positions = self
                .entity_by_id(vehicle_id)
                .map(|vehicle| {
                    vehicle
                        .passengers()
                        .iter()
                        .filter_map(|passenger_id| {
                            self.entity_by_id(*passenger_id).map(|passenger| {
                                (*passenger_id, vehicle.passenger_position(passenger))
                            })
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            passenger_positions
                .into_iter()
                .for_each(|(passenger_id, passenger_position)| {
                    if let Some(passenger) = self.entity_by_id_mut(passenger_id) {
                        passenger.set_position(passenger_position);
                    }
                    self.entity_tracker
                        .move_entity(passenger_id, passenger_position);
                    self.schedule_entity_visibility_refresh(passenger_id);
                    pending_vehicle_ids.push(passenger_id);
                });
        }
    }

    pub fn players(&self) -> impl Iterator<Item = &Player> {
        self.entities.iter().filter_map(|entity| match entity {
            Entity::Player(player) => Some(player),
            Entity::Creature(_) => None,
            Entity::ExperienceOrb(_) => None,
            Entity::Generic(_) => None,
            Entity::Item(_) => None,
            Entity::Projectile(_) => None,
        })
    }

    pub fn chunk_entities(&self, position: ChunkPosition) -> Vec<&Entity> {
        self.entity_tracker
            .chunk_entities(position, EntityTrackerTarget::Entities)
            .into_iter()
            .filter_map(|entity_id| self.entity_by_id(entity_id))
            .collect()
    }

    pub fn nearby_entities(&self, position: EntityPosition, range: f64) -> Vec<&Entity> {
        self.entity_tracker
            .nearby_entities(position, range, EntityTrackerTarget::Entities)
            .into_iter()
            .filter_map(|entity_id| self.entity_by_id(entity_id))
            .collect()
    }

    pub fn creatures(&self) -> Vec<&GenericEntity> {
        self.entities
            .iter()
            .filter_map(|entity| match entity {
                Entity::Generic(entity) if entity.entity_type().is_living() => Some(entity),
                _ => None,
            })
            .collect()
    }

    pub fn experience_orbs(&self) -> Vec<&ExperienceOrb> {
        self.entity_tracker
            .entities(EntityTrackerTarget::ExperienceOrbs)
            .into_iter()
            .filter_map(|entity_id| match self.entity_by_id(entity_id) {
                Some(Entity::ExperienceOrb(entity)) => Some(entity),
                _ => None,
            })
            .collect()
    }

    pub fn viewable_chunk_players(&self, position: ChunkPosition) -> Vec<&Player> {
        self.entity_tracker
            .viewable(position, self.view_distance)
            .into_iter()
            .filter_map(|entity_id| match self.entity_by_id(entity_id) {
                Some(Entity::Player(player)) => Some(player),
                _ => None,
            })
            .collect()
    }

    pub fn player_by_uuid(&self, player_uuid: Uuid) -> Option<&Player> {
        self.players().find(|player| player.uuid() == player_uuid)
    }

    pub fn set_player_pose(&mut self, player_uuid: Uuid, pose: PlayerPose) -> bool {
        let Some(player) = self.player_by_uuid(player_uuid) else {
            return false;
        };
        if !player_pose_fits_at(self, player.position(), pose) {
            return false;
        }
        let Some(player) = self.player_by_uuid_mut(player_uuid) else {
            return false;
        };
        player.set_pose(pose);
        true
    }

    pub fn player_by_uuid_mut(&mut self, player_uuid: Uuid) -> Option<&mut Player> {
        self.entities.iter_mut().find_map(|entity| match entity {
            Entity::Creature(_) => None,
            Entity::ExperienceOrb(_) => None,
            Entity::Generic(_) => None,
            Entity::Item(_) => None,
            Entity::Player(player) if player.uuid() == player_uuid => Some(player),
            Entity::Player(_) => None,
            Entity::Projectile(_) => None,
        })
    }

    pub fn spawn_entity(
        &mut self,
        entity_type: EntityType,
        position: EntityPosition,
        nbt: Option<&NbtCompound>,
    ) -> Result<EntityId> {
        let mut entity = GenericEntity::new(entity_type);
        entity.set_position(position);
        if let Some(nbt) = nbt {
            entity.apply_summon_nbt(nbt);
        }
        let entity_id = entity.entity_id();
        if !self.add_entity(Entity::Generic(entity)) {
            return Err(Error::new(ErrorKind::Interrupted, "Entity add cancelled."));
        }
        Ok(entity_id)
    }

    pub fn spawn_item_entity(
        &mut self,
        item_stack: spinel_registry::ItemStack,
        position: EntityPosition,
    ) -> Result<EntityId> {
        let mut item_entity = ItemEntity::new(item_stack);
        item_entity.spawn(position);
        let entity_id = item_entity.entity_id();
        if !self.add_entity(Entity::Item(item_entity)) {
            return Err(Error::new(ErrorKind::Interrupted, "Entity add cancelled."));
        }
        Ok(entity_id)
    }

    pub fn spawn_experience_orb(
        &mut self,
        experience_count: i16,
        position: EntityPosition,
    ) -> Result<EntityId> {
        let mut experience_orb = ExperienceOrb::new(experience_count);
        experience_orb.set_position(position);
        let entity_id = experience_orb.entity_id();
        if !self.add_entity(Entity::ExperienceOrb(experience_orb)) {
            return Err(Error::new(ErrorKind::Interrupted, "Entity add cancelled."));
        }
        Ok(entity_id)
    }

    pub fn spawn_projectile(
        &mut self,
        shooter_id: Option<EntityId>,
        entity_type: EntityType,
        position: EntityPosition,
    ) -> Result<EntityId> {
        let mut projectile = crate::entity::ProjectileEntity::new(shooter_id, entity_type);
        projectile.set_position(position);
        let projectile_id = projectile.entity_id();
        if !self.add_entity(Entity::Projectile(projectile)) {
            return Err(Error::new(ErrorKind::Interrupted, "Entity add cancelled."));
        }
        Ok(projectile_id)
    }

    pub fn shoot_projectile(
        &mut self,
        projectile_id: EntityId,
        target: EntityPosition,
        power: f64,
        spread: f64,
    ) -> bool {
        let Some(shooter_id) = self
            .entity_by_id(projectile_id)
            .and_then(|entity| match entity {
                Entity::Projectile(projectile) => projectile.shooter(),
                _ => None,
            })
        else {
            return false;
        };
        let Some((shooter_position, shooter_eye_height)) = self
            .entity_by_id(shooter_id)
            .map(|shooter| (shooter.position(), shooter.eye_height()))
        else {
            return false;
        };
        let Some(shooter) = self
            .entity_by_id_mut(shooter_id)
            .map(|entity| entity as *mut Entity)
        else {
            return false;
        };
        let Some(projectile_entity) = self
            .entity_by_id_mut(projectile_id)
            .map(|entity| entity as *mut Entity)
        else {
            return false;
        };
        let mut event = EntityShootEvent::new(shooter, projectile_entity, target, power, spread);
        self.dispatch_entity_shoot_event(&mut event);
        let Some(Entity::Projectile(projectile)) = self.entity_by_id_mut(projectile_id) else {
            return false;
        };
        if event.is_cancelled() {
            projectile.remove();
            return false;
        }
        projectile.shoot_from(
            shooter_position.offset(0.0, shooter_eye_height, 0.0),
            event.target(),
            event.power(),
            event.spread(),
        );
        true
    }

    pub fn set_experience_orb_count(
        &mut self,
        entity_id: EntityId,
        experience_count: i16,
    ) -> Result<bool> {
        let viewer_ids = match self.entity_by_id(entity_id) {
            Some(Entity::ExperienceOrb(experience_orb)) => experience_orb.viewers(),
            _ => return Ok(false),
        };
        viewer_ids
            .iter()
            .try_for_each(|viewer_id| self.send_entity_remove_to_player(entity_id, *viewer_id))?;
        let Some(Entity::ExperienceOrb(experience_orb)) = self.entity_by_id_mut(entity_id) else {
            return Ok(false);
        };
        experience_orb.set_experience_count(experience_count);
        viewer_ids.into_iter().try_for_each(|viewer_id| {
            self.send_single_entity_spawn_to_player(entity_id, viewer_id)
        })?;
        Ok(true)
    }

    pub fn switch_entity_type(
        &mut self,
        entity_id: EntityId,
        entity_type: EntityType,
    ) -> Result<bool> {
        let Some(viewer_ids) = self.entity_by_id(entity_id).map(Entity::viewers) else {
            return Ok(false);
        };
        viewer_ids.iter().try_for_each(|viewer_id| {
            self.send_entity_switch_remove_to_player(entity_id, *viewer_id)
        })?;
        let Some(entity) = self.entity_by_id_mut(entity_id) else {
            return Ok(false);
        };
        if !entity.switch_entity_type(entity_type) {
            return Ok(false);
        }
        viewer_ids.into_iter().try_for_each(|viewer_id| {
            self.send_single_entity_spawn_to_player(entity_id, viewer_id)
        })?;
        Ok(true)
    }

    pub fn set_entity_position(&mut self, entity_id: EntityId, position: EntityPosition) -> bool {
        let Some(previous_position) = self.entity_by_id(entity_id).map(Entity::position) else {
            return false;
        };
        let Some(entity) = self.entity_by_id_mut(entity_id) else {
            return false;
        };
        entity.set_position(position);
        let current_position = entity.position();
        self.entity_tracker.move_entity(entity_id, current_position);
        self.refresh_passenger_positions(entity_id);
        if chunk_position_for_entity_position(previous_position)
            == chunk_position_for_entity_position(current_position)
        {
            return true;
        }
        self.schedule_entity_visibility_refresh(entity_id);
        true
    }

    pub fn steer_boat(
        &mut self,
        vehicle_id: EntityId,
        left_paddle_turning: bool,
        right_paddle_turning: bool,
    ) -> bool {
        let Some(Entity::Generic(vehicle)) = self.entity_by_id_mut(vehicle_id) else {
            return false;
        };
        if !vehicle.entity_type().path().contains("boat") {
            return false;
        }
        if vehicle.is_left_paddle_turning() != left_paddle_turning {
            vehicle.set_left_paddle_turning(left_paddle_turning);
        }
        if vehicle.is_right_paddle_turning() != right_paddle_turning {
            vehicle.set_right_paddle_turning(right_paddle_turning);
        }
        true
    }

    pub fn move_generic_entity(
        &mut self,
        entity_id: EntityId,
        position: EntityPosition,
        on_ground: bool,
    ) -> Result<bool> {
        let Some((previous_position, current_position, movement_packet, head_look_packet)) =
            self.move_generic_entity_state(entity_id, position, on_ground)
        else {
            return Ok(false);
        };
        self.entity_tracker.move_entity(entity_id, current_position);
        self.refresh_passenger_positions(entity_id);
        if chunk_position_for_entity_position(previous_position)
            != chunk_position_for_entity_position(current_position)
        {
            self.schedule_entity_visibility_refresh(entity_id);
        }
        self.send_packet_to_entity_viewers(entity_id, movement_packet)?;
        self.send_packet_to_entity_viewers(entity_id, head_look_packet)?;
        Ok(true)
    }

    pub fn look_generic_entity_at_position(
        &mut self,
        entity_id: EntityId,
        target: EntityPosition,
        on_ground: bool,
    ) -> Result<bool> {
        let Some((rotation_packet, head_look_packet)) =
            self.look_generic_entity_state_at_position(entity_id, target, on_ground)
        else {
            return Ok(false);
        };
        self.send_packet_to_entity_viewers(entity_id, rotation_packet)?;
        self.send_packet_to_entity_viewers(entity_id, head_look_packet)?;
        Ok(true)
    }

    pub fn swing_generic_entity_main_hand(&mut self, entity_id: EntityId) -> Result<bool> {
        let Some(animation_packet) = self.generic_entity_main_hand_animation(entity_id) else {
            return Ok(false);
        };
        self.send_packet_to_entity_viewers(entity_id, animation_packet)?;
        Ok(true)
    }

    pub fn creature_attack_entity(
        &mut self,
        creature_id: EntityId,
        target_id: EntityId,
        should_swing_main_hand: bool,
    ) -> Result<bool> {
        if !self.creature_can_attack_entity(creature_id, target_id) {
            return Ok(false);
        }
        if should_swing_main_hand {
            let Some(animation_packet) = self.creature_main_hand_animation(creature_id) else {
                return Ok(false);
            };
            self.send_packet_to_entity_viewers(creature_id, animation_packet)?;
        }
        self.dispatch_entity_attack_event(creature_id, target_id);
        Ok(true)
    }

    fn apply_creature_ai_action(&mut self, action: CreatureAiAction) {
        match action {
            CreatureAiAction::Attack {
                source,
                target,
                should_swing_main_hand,
            } => {
                let _ = self.creature_attack_entity(source, target, should_swing_main_hand);
            }
            CreatureAiAction::Shoot {
                shooter,
                mut projectile,
                target,
                power,
                spread,
            } => {
                let Some((shooter_position, shooter_eye_height)) = self
                    .entity_by_id(shooter)
                    .map(|entity| (entity.position(), entity.eye_height()))
                else {
                    return;
                };
                projectile.set_shooter(Some(shooter));
                projectile.set_position(shooter_position.offset(0.0, shooter_eye_height, 0.0));
                let projectile_id = projectile.entity_id();
                if !self.add_entity(Entity::Projectile(projectile)) {
                    return;
                }
                self.shoot_projectile(projectile_id, target, power, spread);
            }
        }
    }

    fn move_generic_entity_state(
        &mut self,
        entity_id: EntityId,
        position: EntityPosition,
        on_ground: bool,
    ) -> Option<(
        EntityPosition,
        EntityPosition,
        EntityPositionAndRotationPacket,
        EntityHeadLookPacket,
    )> {
        let Some(Entity::Generic(entity)) = self.entity_by_id_mut(entity_id) else {
            return None;
        };
        let previous_position = entity.position();
        entity.set_position(position);
        Some((
            previous_position,
            entity.position(),
            entity.position_and_rotation_delta_packet(previous_position, on_ground),
            entity.head_look_packet(),
        ))
    }

    fn look_generic_entity_state_at_position(
        &mut self,
        entity_id: EntityId,
        target: EntityPosition,
        on_ground: bool,
    ) -> Option<(EntityRotationPacket, EntityHeadLookPacket)> {
        let Some(Entity::Generic(entity)) = self.entity_by_id_mut(entity_id) else {
            return None;
        };
        entity.look_at_position(target);
        Some((entity.rotation_packet(on_ground), entity.head_look_packet()))
    }

    fn generic_entity_main_hand_animation(
        &self,
        entity_id: EntityId,
    ) -> Option<spinel_core::network::clientbound::play::entity_animation::EntityAnimationPacket>
    {
        let Some(Entity::Generic(entity)) = self.entity_by_id(entity_id) else {
            return None;
        };
        Some(entity.swing_main_hand())
    }

    fn creature_main_hand_animation(
        &self,
        entity_id: EntityId,
    ) -> Option<spinel_core::network::clientbound::play::entity_animation::EntityAnimationPacket>
    {
        let Some(Entity::Creature(entity)) = self.entity_by_id(entity_id) else {
            return None;
        };
        Some(entity.swing_main_hand())
    }

    fn creature_can_attack_entity(&self, creature_id: EntityId, target_id: EntityId) -> bool {
        if creature_id == target_id {
            return false;
        }
        let source_is_creature =
            matches!(self.entity_by_id(creature_id), Some(Entity::Creature(_)));
        source_is_creature && self.entity_by_id(target_id).is_some()
    }

    pub(crate) fn move_generic_entities_for_player(
        &mut self,
        client: &mut Client,
    ) -> Result<usize> {
        let moved_entities = self
            .entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Generic(entity) if !entity.is_removed() => Some(entity),
                _ => None,
            })
            .map(|entity| {
                let previous_position = entity.position();
                entity.set_position(previous_position.offset(0.0, 1.0, 0.0));
                entity.set_velocity(Velocity(Vector3d {
                    x: 0.0,
                    y: 0.25,
                    z: 0.0,
                }));
                entity.teleport_packet().dispatch(client)?;
                entity
                    .velocity_packet()
                    .dispatch(client)
                    .map(|_| (entity.entity_id(), entity.position()))
            })
            .collect::<Result<Vec<_>>>()?;
        moved_entities.iter().for_each(|(entity_id, position)| {
            self.entity_tracker.move_entity(*entity_id, *position);
        });
        moved_entities
            .iter()
            .try_for_each(|(entity_id, _)| self.refresh_visibility_for_entity(*entity_id))?;
        Ok(moved_entities.len())
    }

    pub(crate) fn remove_generic_entities_for_player(
        &mut self,
        client: &mut Client,
    ) -> Result<usize> {
        let removed_entities = self
            .entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Generic(entity) if !entity.is_removed() => {
                    entity.remove();
                    Some((entity.entity_id(), entity.entity_type(), entity.uuid()))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        let removed_entity_count = removed_entities.len();
        removed_entities
            .iter()
            .try_for_each(|(entity_id, entity_type, uuid)| {
                RemoveEntitiesPacket::new(vec![entity_id.value()]).dispatch(client)?;
                if *entity_type == EntityType::PLAYER {
                    PlayerInfoRemovePacket::new(*uuid).dispatch(client)?;
                }
                Ok::<(), std::io::Error>(())
            })?;
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player)
                    if player.addr != client.addr && player.has_entered_world() =>
                {
                    Some(player)
                }
                _ => None,
            })
            .filter_map(Player::client_mut)
            .try_for_each(|viewer_client| {
                removed_entities
                    .iter()
                    .try_for_each(|(entity_id, entity_type, uuid)| {
                        RemoveEntitiesPacket::new(vec![entity_id.value()])
                            .dispatch(viewer_client)?;
                        if *entity_type == EntityType::PLAYER {
                            PlayerInfoRemovePacket::new(*uuid).dispatch(viewer_client)?;
                        }
                        Ok::<(), std::io::Error>(())
                    })
            })?;
        removed_entities.iter().for_each(|(entity_id, _, _)| {
            self.entity_tracker.unregister(*entity_id);
        });
        self.entities.retain(|entity| match entity {
            Entity::Creature(entity) => !entity.is_removed(),
            Entity::ExperienceOrb(entity) => !entity.is_removed(),
            Entity::Generic(entity) => !entity.is_removed(),
            Entity::Item(entity) => !entity.is_removed(),
            Entity::Player(_) => true,
            Entity::Projectile(entity) => !entity.is_removed(),
        });
        Ok(removed_entity_count)
    }

    pub(crate) fn enter_player(
        &mut self,
        client: &mut Client,
        ticks_per_second: u32,
        registries: &Registries,
    ) -> Result<()> {
        self.use_client_event_dispatcher(client);
        let chunks = match self.player_by_addr(&client.addr) {
            Some(player) => player.spawn_chunks(self.view_distance),
            None => Vec::new(),
        };
        let chunk_load_tickets = chunks
            .iter()
            .copied()
            .map(ChunkPosition::from)
            .map(|position| self.load_optional_chunk_future(position))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        let all_chunks_are_loaded = chunks
            .iter()
            .copied()
            .map(ChunkPosition::from)
            .all(|position| self.is_chunk_loaded(position));
        if all_chunks_are_loaded {
            return self.finish_player_entry(client, ticks_per_second, registries, chunks);
        }
        self.pending_player_entries.insert(
            client.addr,
            PendingPlayerEntry {
                client: client as *mut Client as usize,
                ticks_per_second,
                chunks,
                chunk_load_tickets,
            },
        );
        Ok(())
    }

    fn finish_player_entry(
        &mut self,
        client: &mut Client,
        ticks_per_second: u32,
        registries: &Registries,
        chunks: Vec<PlayerChunk>,
    ) -> Result<()> {
        let dimension_type_id = registries
            .dynamic_registry_id(
                &spinel_registry::DIMENSION_TYPE_REGISTRY,
                self.dimension_type.key(),
            )
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::NotFound,
                    format!(
                        "Dimension type {} is not registered",
                        self.dimension_type.key()
                    ),
                )
            })?;
        let time_packet = self.time_packet();
        let weather = self.weather;
        let world_name = self.name.clone();
        let world_uuid = self.uuid;
        let world_border_packet = self
            .world_border
            .initialize_packet(self.world_border.diameter(), 0);
        let boss_bar_packets = self
            .boss_bars
            .iter()
            .map(BossBar::add_packet)
            .collect::<Vec<_>>();
        let (player, first_spawn, player_id, player_position) = {
            let dimension_type = self.dimension_type.clone();
            let Some(player) = self.player_by_addr_mut(&client.addr) else {
                return Err(Error::new(ErrorKind::NotFound, "Player not found."));
            };
            player.set_world(world_uuid);
            player.set_dimension_type(dimension_type);
            let first_spawn = !player.has_entered_world();
            player.unsafe_init_with_chunk_positions(
                client,
                ticks_per_second,
                dimension_type_id,
                world_name.clone(),
                chunks,
                world_border_packet,
                time_packet,
                weather,
            )?;
            boss_bar_packets
                .into_iter()
                .try_for_each(|packet| packet.dispatch(client))?;
            (
                player as *mut Player,
                first_spawn,
                player.entity_id(),
                player.position(),
            )
        };
        self.entity_tracker.move_entity(player_id, player_position);
        self.refresh_visibility_for_entity(player_id)?;
        self.send_pending_chunks_for_client(client, registries)?;
        dispatch_player_spawn_event(player, self as *mut World, first_spawn, client);
        self.synchronize_player_visibility(client)
    }
    pub(crate) fn move_player(
        &mut self,
        client: &mut Client,
        x: f64,
        y: f64,
        z: f64,
        on_ground: bool,
        _registries: &Registries,
    ) -> Result<()> {
        self.use_client_event_dispatcher(client);
        let world_view_distance = self.view_distance;
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        if !player.has_entered_world() {
            return Ok(());
        }
        let previous_position = player.position();
        if player_coordinate_is_too_large(x)
            || player_coordinate_is_too_large(y)
            || player_coordinate_is_too_large(z)
        {
            return player.kick(Component::text("You moved too far away!"));
        }
        if previous_position.x() == x && previous_position.y() == y && previous_position.z() == z {
            return Ok(());
        }
        if player.has_pending_teleport_confirmation() {
            return Ok(());
        }
        let packet_position =
            EntityPosition::new(x, y, z, previous_position.yaw(), previous_position.pitch());
        let Some(event_position) =
            self.process_player_move_event(client, previous_position, packet_position, on_ground)?
        else {
            return Ok(());
        };
        let pending_transition = self.player_by_addr(&client.addr).and_then(|player| {
            player.chunk_transition(
                event_position.x(),
                event_position.y(),
                event_position.z(),
                self.view_distance,
            )
        });
        if self.movement_enters_unloaded_chunk(pending_transition.as_ref()) {
            let Some(player) = self.player_by_addr_mut(&client.addr) else {
                return Err(Error::new(ErrorKind::NotFound, "Player not found."));
            };
            return player.synchronize_position_after_teleport(
                previous_position,
                Vector3d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                TeleportFlags::absolute(),
                true,
            );
        }
        let transition = self
            .player_by_addr_mut(&client.addr)
            .and_then(|player| player.accept_chunk_transition(pending_transition));
        let chunks = match transition.as_ref() {
            Some(transition) => transition.arriving.clone(),
            None => Vec::new(),
        };
        self.schedule_player_chunk_loads(client.addr, &chunks)?;
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let moving_player_id = player.entity_id();
        player.move_to_loaded_chunks(
            client,
            event_position.x(),
            event_position.y(),
            event_position.z(),
            on_ground,
            transition,
            chunks,
            world_view_distance,
        )?;
        let current_position = player.position();
        self.entity_tracker
            .move_entity(moving_player_id, current_position);
        self.refresh_visibility_for_entity(moving_player_id)?;
        self.broadcast_player_movement(
            moving_player_id,
            previous_position,
            current_position,
            on_ground,
        )
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
        _registries: &Registries,
    ) -> Result<()> {
        self.use_client_event_dispatcher(client);
        let world_view_distance = self.view_distance;
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        if !player.has_entered_world() {
            return Ok(());
        }
        let previous_position = player.position();
        if player_coordinate_is_too_large(x)
            || player_coordinate_is_too_large(y)
            || player_coordinate_is_too_large(z)
        {
            return player.kick(Component::text("You moved too far away!"));
        }
        let packet_position = EntityPosition::new(x, y, z, yaw, pitch);
        if previous_position == packet_position {
            return Ok(());
        }
        if player.has_pending_teleport_confirmation() {
            return Ok(());
        }
        let Some(event_position) =
            self.process_player_move_event(client, previous_position, packet_position, on_ground)?
        else {
            return Ok(());
        };
        let pending_transition = self.player_by_addr(&client.addr).and_then(|player| {
            player.chunk_transition(
                event_position.x(),
                event_position.y(),
                event_position.z(),
                self.view_distance,
            )
        });
        if self.movement_enters_unloaded_chunk(pending_transition.as_ref()) {
            let Some(player) = self.player_by_addr_mut(&client.addr) else {
                return Err(Error::new(ErrorKind::NotFound, "Player not found."));
            };
            return player.synchronize_position_after_teleport(
                previous_position,
                Vector3d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                TeleportFlags::absolute(),
                true,
            );
        }
        let transition = self
            .player_by_addr_mut(&client.addr)
            .and_then(|player| player.accept_chunk_transition(pending_transition));
        let chunks = match transition.as_ref() {
            Some(transition) => transition.arriving.clone(),
            None => Vec::new(),
        };
        self.schedule_player_chunk_loads(client.addr, &chunks)?;
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let moving_player_id = player.entity_id();
        player.move_to_with_view_loaded_chunks(
            client,
            event_position.x(),
            event_position.y(),
            event_position.z(),
            event_position.yaw(),
            event_position.pitch(),
            on_ground,
            transition,
            chunks,
            world_view_distance,
        )?;
        let current_position = player.position();
        self.entity_tracker
            .move_entity(moving_player_id, current_position);
        self.refresh_visibility_for_entity(moving_player_id)?;
        self.broadcast_player_movement(
            moving_player_id,
            previous_position,
            current_position,
            on_ground,
        )
    }

    fn process_player_move_event(
        &mut self,
        client: &mut Client,
        current_position: EntityPosition,
        packet_position: EntityPosition,
        on_ground: bool,
    ) -> Result<Option<EntityPosition>> {
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let player_ptr = player as *mut Player;
        let Some(server_ptr) = client.server_ptr else {
            return Ok(Some(packet_position));
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let mut event = PlayerMoveEvent::new(player_ptr, packet_position, on_ground);
        event.dispatch(server, client);
        let player = unsafe { &mut *player_ptr };
        if player.position() != current_position {
            return Ok(None);
        }
        if event.is_cancelled() {
            player.synchronize_position_after_teleport(
                current_position,
                Vector3d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                TeleportFlags::absolute(),
                true,
            )?;
            return Ok(None);
        }
        let event_position = event.new_position();
        if packet_position == event_position {
            return Ok(Some(event_position));
        }
        if packet_position.x() == event_position.x()
            && packet_position.y() == event_position.y()
            && packet_position.z() == event_position.z()
        {
            player.set_position_and_view(event_position);
            player.set_on_ground(on_ground);
            return Ok(None);
        }
        player.synchronize_position_after_teleport(
            event_position,
            Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            TeleportFlags::absolute(),
            true,
        )?;
        player.set_position(event_position);
        Ok(None)
    }

    pub(crate) fn animate_player_hand(
        &mut self,
        client: &Client,
        hand: crate::entity::PlayerHand,
    ) -> Result<()> {
        let Some(player) = self.player_by_addr(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let animating_player_id = player.entity_id();
        let animation_packet = player.animation_packet(hand);
        let animation_entity_id = animation_packet.entity_id;
        let animation = animation_packet.animation;
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player)
                    if player.entity_id() != animating_player_id && player.has_entered_world() =>
                {
                    Some(player)
                }
                _ => None,
            })
            .filter_map(Player::client_mut)
            .try_for_each(|viewer_client| {
                spinel_core::network::clientbound::play::entity_animation::EntityAnimationPacket {
                    entity_id: animation_entity_id,
                    animation,
                }
                .dispatch(viewer_client)
            })
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
    ) -> Result<()> {
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let player_entity_id = player.entity_id();
        if !player.refresh_input(forward, backward, left, right, jump, shift, sprint) {
            return Ok(());
        }
        let Some(metadata_packet) = player.dirty_metadata_packet() else {
            return Ok(());
        };
        let metadata_entity_id = metadata_packet.entity_id;
        let metadata_entries = metadata_packet.entries.0;
        self.broadcast_player_metadata(player_entity_id, metadata_entity_id, metadata_entries)
    }

    pub(crate) fn set_player_sprinting(&mut self, client: &Client, sprinting: bool) -> Result<()> {
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let player_entity_id = player.entity_id();
        if !player.set_sprinting(sprinting) {
            return Ok(());
        }
        let Some(metadata_packet) = player.dirty_metadata_packet() else {
            return Ok(());
        };
        let metadata_entity_id = metadata_packet.entity_id;
        let metadata_entries = metadata_packet.entries.0;
        self.broadcast_player_metadata(player_entity_id, metadata_entity_id, metadata_entries)
    }

    pub(crate) fn refresh_player_settings(
        &mut self,
        client: &mut Client,
        settings: ClientInformation,
    ) -> Result<()> {
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let current_center = player.chunks_loaded_by_client;
        let previous_view_distance = player.client_chunk_view_distance();
        if !player.refresh_settings(settings) {
            return Ok(());
        }
        let next_view_distance = player.client_chunk_view_distance();
        let previous_chunks = current_center
            .surrounding(previous_view_distance)
            .into_iter()
            .collect::<HashSet<_>>();
        let next_chunks = current_center
            .surrounding(next_view_distance)
            .into_iter()
            .collect::<HashSet<_>>();
        let arriving = next_chunks
            .difference(&previous_chunks)
            .copied()
            .collect::<Vec<_>>();
        let departing = previous_chunks
            .difference(&next_chunks)
            .copied()
            .collect::<Vec<_>>();
        self.schedule_player_chunk_loads(client.addr, &arriving)?;
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        player.update_chunks_after_view_distance_change(client, arriving, departing)
    }

    pub(crate) fn start_player_flying_with_elytra(&mut self, client: &Client) -> Result<()> {
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let player_entity_id = player.entity_id();
        if !player.set_flying_with_elytra(true) {
            return Ok(());
        }
        let Some(metadata_packet) = player.dirty_metadata_packet() else {
            return Ok(());
        };
        let metadata_entity_id = metadata_packet.entity_id;
        let metadata_entries = metadata_packet.entries.0;
        self.broadcast_player_metadata(player_entity_id, metadata_entity_id, metadata_entries)
    }

    pub(crate) fn set_player_held_slot(
        &mut self,
        client: &mut Client,
        held_slot: i32,
        server: *mut crate::server::MinecraftServer,
    ) -> Result<bool> {
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let player_entity_id = player.entity_id();
        let server = unsafe { &mut *server };
        if !player.change_held_slot(held_slot, server, client) {
            return Ok(false);
        }
        let equipment_packet = player.visible_equipment_packet();
        let equipment_entity_id = equipment_packet.entity_id;
        let equipment_entries = equipment_packet.equipment.0;
        let metadata_packet = player.dirty_metadata_packet();
        self.broadcast_player_equipment(player_entity_id, equipment_entity_id, equipment_entries)?;
        if let Some(metadata_packet) = metadata_packet {
            self.broadcast_player_metadata(
                player_entity_id,
                metadata_packet.entity_id,
                metadata_packet.entries.0,
            )?;
        }
        Ok(true)
    }

    pub(crate) fn refresh_player_metadata(&mut self, client: &Client) -> Result<()> {
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let player_entity_id = player.entity_id();
        let Some(metadata_packet) = player.dirty_metadata_packet() else {
            return Ok(());
        };
        let metadata_entity_id = metadata_packet.entity_id;
        let metadata_entries = metadata_packet.entries.0;
        self.broadcast_player_metadata(player_entity_id, metadata_entity_id, metadata_entries)
    }

    pub(crate) fn refresh_player_visible_equipment(&mut self, client: &Client) -> Result<()> {
        let Some(player) = self.player_by_addr(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let player_entity_id = player.entity_id();
        let equipment_packet = player.visible_equipment_packet();
        let equipment_entity_id = equipment_packet.entity_id;
        let equipment_entries = equipment_packet.equipment.0;
        self.broadcast_player_equipment(player_entity_id, equipment_entity_id, equipment_entries)
    }

    pub(crate) fn look_player(
        &mut self,
        client: &Client,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    ) -> Result<()> {
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        let previous_position = player.position();
        player.look(yaw, pitch);
        player.set_on_ground(on_ground);
        let current_position = player.position();
        let looking_player_id = player.entity_id();
        self.broadcast_player_movement(
            looking_player_id,
            previous_position,
            current_position,
            on_ground,
        )
    }

    pub(crate) fn refresh_player_status(
        &mut self,
        client: &mut Client,
        on_ground: bool,
    ) -> Result<()> {
        let (player, player_entity_id, metadata_packet, stopped_flying_with_elytra) = {
            let Some(player) = self.player_by_addr_mut(&client.addr) else {
                return Err(Error::new(ErrorKind::NotFound, "Player not found."));
            };
            let stopped_flying_with_elytra = player.refresh_on_ground(on_ground);
            (
                player as *mut Player,
                player.entity_id(),
                player.dirty_metadata_packet(),
                stopped_flying_with_elytra,
            )
        };

        if stopped_flying_with_elytra {
            self.dispatch_player_stop_flying_with_elytra_event(client, player);
        }

        let Some(metadata_packet) = metadata_packet else {
            return Ok(());
        };

        self.broadcast_player_metadata(
            player_entity_id,
            metadata_packet.entity_id,
            metadata_packet.entries.0,
        )?;

        Ok(())
    }

    fn dispatch_player_stop_flying_with_elytra_event(
        &mut self,
        client: &mut Client,
        player: *mut Player,
    ) {
        let Some(server_ptr) = client.server_ptr else {
            return;
        };
        let mut event = PlayerStopFlyingWithElytraEvent::new(player);
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server, client);
    }

    fn broadcast_player_movement(
        &mut self,
        moving_player_id: EntityId,
        previous_position: EntityPosition,
        current_position: EntityPosition,
        on_ground: bool,
    ) -> Result<()> {
        let moved_entity_id = moving_player_id.value();
        let moved_position = current_position.as_vector();
        let moved_yaw = current_position.yaw();
        let moved_pitch = current_position.pitch();
        let moved_delta_x =
            EntityPositionPacket::delta(current_position.x(), previous_position.x());
        let moved_delta_y =
            EntityPositionPacket::delta(current_position.y(), previous_position.y());
        let moved_delta_z =
            EntityPositionPacket::delta(current_position.z(), previous_position.z());
        let position_changed = current_position.x() != previous_position.x()
            || current_position.y() != previous_position.y()
            || current_position.z() != previous_position.z();
        if position_changed {
            self.refresh_passenger_positions(moving_player_id);
        }
        let view_changed = current_position.yaw() != previous_position.yaw()
            || current_position.pitch() != previous_position.pitch();
        let movement_requires_teleport = (current_position.x() - previous_position.x()).abs() > 8.0
            || (current_position.y() - previous_position.y()).abs() > 8.0
            || (current_position.z() - previous_position.z()).abs() > 8.0;
        let viewer_ids = self
            .entity_by_id(moving_player_id)
            .map(Entity::viewers)
            .unwrap_or_default();
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player)
                    if viewer_ids.contains(&player.entity_id()) && player.has_entered_world() =>
                {
                    Some(player)
                }
                _ => None,
            })
            .filter_map(Player::client_mut)
            .try_for_each(|viewer_client| {
                if movement_requires_teleport {
                    EntityTeleportPacket {
                        entity_id: moved_entity_id,
                        position: moved_position,
                        delta: Vector3d {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        yaw: moved_yaw,
                        pitch: moved_pitch,
                        flags: 0,
                        on_ground,
                    }
                    .dispatch(viewer_client)?;
                    return Ok(());
                }
                if position_changed && view_changed {
                    EntityPositionAndRotationPacket {
                        entity_id: moved_entity_id,
                        delta_x: moved_delta_x,
                        delta_y: moved_delta_y,
                        delta_z: moved_delta_z,
                        yaw: EntityAngle(moved_yaw),
                        pitch: EntityAngle(moved_pitch),
                        on_ground,
                    }
                    .dispatch(viewer_client)?;
                    return EntityHeadLookPacket {
                        entity_id: moved_entity_id,
                        head_yaw: EntityAngle(moved_yaw),
                    }
                    .dispatch(viewer_client);
                }
                if position_changed {
                    return EntityPositionAndRotationPacket {
                        entity_id: moved_entity_id,
                        delta_x: moved_delta_x,
                        delta_y: moved_delta_y,
                        delta_z: moved_delta_z,
                        yaw: EntityAngle(moved_yaw),
                        pitch: EntityAngle(moved_pitch),
                        on_ground,
                    }
                    .dispatch(viewer_client);
                }
                if !view_changed {
                    return Ok(());
                }
                EntityHeadLookPacket {
                    entity_id: moved_entity_id,
                    head_yaw: EntityAngle(moved_yaw),
                }
                .dispatch(viewer_client)?;
                EntityPositionAndRotationPacket {
                    entity_id: moved_entity_id,
                    delta_x: moved_delta_x,
                    delta_y: moved_delta_y,
                    delta_z: moved_delta_z,
                    yaw: EntityAngle(moved_yaw),
                    pitch: EntityAngle(moved_pitch),
                    on_ground,
                }
                .dispatch(viewer_client)
            })
    }

    fn broadcast_player_metadata(
        &mut self,
        changed_player_id: EntityId,
        metadata_entity_id: i32,
        metadata_entries: Vec<MetadataEntry>,
    ) -> Result<()> {
        let viewer_ids = self
            .entity_by_id(changed_player_id)
            .map(Entity::viewers)
            .unwrap_or_default();
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player)
                    if viewer_ids.contains(&player.entity_id()) && player.has_entered_world() =>
                {
                    Some(player)
                }
                _ => None,
            })
            .filter_map(Player::client_mut)
            .try_for_each(|viewer_client| {
                SetEntityDataPacket::new(metadata_entity_id, metadata_entries.clone())
                    .dispatch(viewer_client)
            })
    }

    pub fn send_packet_to_player_viewers_and_self<P>(
        &mut self,
        player_id: EntityId,
        packet: P,
    ) -> Result<()>
    where
        P: PacketStruct + DataType,
    {
        let Some(mut viewer_ids) = self.entity_by_id(player_id).map(Entity::viewers) else {
            return Ok(());
        };
        viewer_ids.insert(player_id);
        let mut payload = Vec::new();
        packet.encode(&mut payload)?;
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player)
                    if player.has_entered_world() && viewer_ids.contains(&player.entity_id()) =>
                {
                    Some(player)
                }
                _ => None,
            })
            .filter_map(Player::client_mut)
            .try_for_each(|client| client.send_packet(P::get_id(), &payload))
    }

    fn send_packet_to_entity_viewers<P>(&mut self, entity_id: EntityId, packet: P) -> Result<()>
    where
        P: PacketStruct + DataType,
    {
        let viewer_ids = self
            .entity_by_id(entity_id)
            .map(Entity::viewers)
            .unwrap_or_default();
        let mut payload = Vec::new();
        packet.encode(&mut payload)?;
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if viewer_ids.contains(&player.entity_id()) => Some(player),
                _ => None,
            })
            .filter_map(Player::client_mut)
            .try_for_each(|client| client.send_packet(P::get_id(), &payload))
    }

    fn refresh_visibility_for_entity(&mut self, entity_id: EntityId) -> Result<()> {
        let Some(position) = self.entity_by_id(entity_id).map(Entity::position) else {
            return Ok(());
        };
        let mut player_ids = self.entity_tracker.nearby_entities_by_chunk_range(
            position,
            ENTITY_VIEW_DISTANCE,
            EntityTrackerTarget::Players,
        );
        player_ids.extend(
            self.entity_by_id(entity_id)
                .map(Entity::viewers)
                .unwrap_or_default(),
        );
        player_ids.sort_unstable();
        player_ids.dedup();
        player_ids
            .into_iter()
            .try_for_each(|player_id| self.refresh_visibility_pair(entity_id, player_id))?;
        if !matches!(self.entity_by_id(entity_id), Some(Entity::Player(_))) {
            return Ok(());
        }
        let mut viewed_entity_ids = self.entity_tracker.nearby_entities_by_chunk_range(
            position,
            ENTITY_VIEW_DISTANCE,
            EntityTrackerTarget::Entities,
        );
        viewed_entity_ids.extend(
            self.entity_by_id(entity_id)
                .map(|entity| entity.view().viewed_entities().iter().copied())
                .into_iter()
                .flatten(),
        );
        viewed_entity_ids.sort_unstable();
        viewed_entity_ids.dedup();
        viewed_entity_ids
            .into_iter()
            .try_for_each(|viewed_entity_id| {
                self.refresh_visibility_pair(viewed_entity_id, entity_id)
            })
    }

    fn refresh_visibility_pair(
        &mut self,
        viewed_entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> Result<()> {
        if viewed_entity_id == viewer_player_id {
            return Ok(());
        }
        let Some(viewed_entity_index) = self
            .entities
            .iter()
            .position(|entity| entity.entity_id() == viewed_entity_id)
        else {
            return Ok(());
        };
        let Some(viewer_player_index) = self
            .entities
            .iter()
            .position(|entity| entity.entity_id() == viewer_player_id)
        else {
            return Ok(());
        };
        let (viewed_entity, viewer_player) =
            distinct_entities_mut(&mut self.entities, viewed_entity_index, viewer_player_index);
        let Entity::Player(viewer_player) = viewer_player else {
            return Ok(());
        };
        let viewed_player_is_vanished =
            matches!(viewed_entity, Entity::Player(player) if player.is_vanished());
        let should_be_visible = automatic_visibility_pair_is_allowed(
            viewed_entity,
            viewer_player,
            viewed_player_is_vanished,
        );
        let is_automatically_visible = viewed_entity
            .view()
            .automatic_viewers()
            .contains(&viewer_player_id);
        if should_be_visible && !viewed_entity.view().is_viewer(viewer_player_id) {
            viewed_entity.view_mut().automatic_add(viewer_player_id);
            viewer_player
                .view_mut()
                .register_viewed_entity(viewed_entity_id);
            return self.send_entity_chain_spawn_to_player(viewed_entity_id, viewer_player_id);
        }
        if !should_be_visible && is_automatically_visible {
            return self.hide_visibility_pair(viewed_entity_id, viewer_player_id);
        }
        Ok(())
    }

    fn hide_entity_from_all_viewers(&mut self, entity_id: EntityId) -> Result<()> {
        let viewer_ids = self
            .entity_by_id(entity_id)
            .map(Entity::viewers)
            .unwrap_or_default();
        viewer_ids
            .into_iter()
            .try_for_each(|viewer_id| self.hide_visibility_pair(entity_id, viewer_id))?;
        let viewed_entity_ids = self
            .entity_by_id(entity_id)
            .map(|entity| entity.view().viewed_entities().clone())
            .unwrap_or_default();
        viewed_entity_ids
            .into_iter()
            .try_for_each(|viewed_entity_id| self.hide_visibility_pair(viewed_entity_id, entity_id))
    }

    fn hide_visibility_pair(
        &mut self,
        viewed_entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> Result<()> {
        let mut pending_entity_ids = vec![viewed_entity_id];
        let mut hidden_entity_ids = HashSet::new();
        while let Some(entity_id) = pending_entity_ids.pop() {
            if !hidden_entity_ids.insert(entity_id) {
                continue;
            }
            let passenger_ids = self
                .entity_by_id(entity_id)
                .map(|entity| {
                    entity
                        .passengers()
                        .iter()
                        .rev()
                        .copied()
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            self.hide_single_visibility_pair(entity_id, viewer_player_id)?;
            pending_entity_ids.extend(
                passenger_ids
                    .into_iter()
                    .filter(|passenger_id| *passenger_id != viewer_player_id),
            );
        }
        Ok(())
    }

    fn hide_single_visibility_pair(
        &mut self,
        viewed_entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> Result<()> {
        if self
            .entity_by_id(viewed_entity_id)
            .is_none_or(|entity| !entity.view().is_viewer(viewer_player_id))
        {
            return Ok(());
        }
        let leash_detach_packets = self
            .entity_by_id(viewed_entity_id)
            .map(|entity| {
                entity
                    .leashed_entities()
                    .iter()
                    .map(|leashed_entity_id| {
                        spinel_core::network::clientbound::play::attach_entity::AttachEntityPacket {
                            attached_entity_id: leashed_entity_id.value(),
                            holding_entity_id: -1,
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let Some(viewed_entity_index) = self
            .entities
            .iter()
            .position(|entity| entity.entity_id() == viewed_entity_id)
        else {
            return Ok(());
        };
        let Some(viewer_player_index) = self
            .entities
            .iter()
            .position(|entity| entity.entity_id() == viewer_player_id)
        else {
            return Ok(());
        };
        let (viewed_entity, viewer_player) =
            distinct_entities_mut(&mut self.entities, viewed_entity_index, viewer_player_index);
        let Entity::Player(viewer_player) = viewer_player else {
            return Ok(());
        };
        viewed_entity.view_mut().automatic_remove(viewer_player_id);
        viewed_entity.view_mut().manual_remove(viewer_player_id);
        viewer_player
            .view_mut()
            .unregister_viewed_entity(viewed_entity_id);
        if let Some(client) =
            self.entity_by_id_mut(viewer_player_id)
                .and_then(|entity| match entity {
                    Entity::Player(player) => player.client_mut(),
                    _ => None,
                })
        {
            leash_detach_packets
                .into_iter()
                .try_for_each(|packet| packet.dispatch(client))?;
        }
        self.send_entity_remove_to_player(viewed_entity_id, viewer_player_id)
    }

    pub fn set_player_skin(
        &mut self,
        player_id: EntityId,
        skin: Option<PlayerSkin>,
    ) -> Result<bool> {
        let Some(Entity::Player(player)) = self.entity_by_id_mut(player_id) else {
            return Ok(false);
        };
        player.apply_skin(skin);
        self.refresh_player_skin(player_id)?;
        Ok(true)
    }

    pub fn set_player_vanished(&mut self, player_id: EntityId, vanished: bool) -> Result<bool> {
        let Some(Entity::Player(player)) = self.entity_by_id_mut(player_id) else {
            return Ok(false);
        };
        if player.is_vanished() == vanished {
            return Ok(true);
        }
        player.set_vanished(vanished);
        self.refresh_visibility_for_entity(player_id)?;
        Ok(true)
    }

    pub fn refresh_player_skin(&mut self, player_id: EntityId) -> Result<()> {
        let Some(Entity::Player(player)) = self.entity_by_id(player_id) else {
            return Ok(());
        };
        let player_uuid = player.uuid();
        let viewer_ids = player.viewers();
        let snapshot = PlayerViewerSnapshot::from_player(player);
        viewer_ids.into_iter().try_for_each(|viewer_id| {
            self.send_player_skin_refresh_to_viewer(player_id, player_uuid, &snapshot, viewer_id)
        })
    }

    fn send_player_skin_refresh_to_viewer(
        &mut self,
        player_id: EntityId,
        player_uuid: Uuid,
        snapshot: &PlayerViewerSnapshot,
        viewer_id: EntityId,
    ) -> Result<()> {
        let Some(client) = self
            .entity_by_id_mut(viewer_id)
            .and_then(|entity| match entity {
                Entity::Player(player) => player.client_mut(),
                _ => None,
            })
        else {
            return Ok(());
        };
        PlayerInfoRemovePacket::new(player_uuid).dispatch(client)?;
        RemoveEntitiesPacket::new(vec![player_id.value()]).dispatch(client)?;
        snapshot.dispatch(client)
    }

    fn send_entity_chain_spawn_to_player(
        &mut self,
        root_entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> Result<()> {
        let visible_chain = self.collect_visible_passenger_chain(root_entity_id, viewer_player_id);
        let visible_entity_ids = visible_chain.iter().copied().collect::<HashSet<_>>();
        let viewer_dispatches = visible_chain
            .iter()
            .filter_map(|entity_id| {
                self.entity_viewer_snapshot(*entity_id).map(|snapshot| {
                    (
                        snapshot,
                        self.leash_packets_for_new_viewer(*entity_id, viewer_player_id),
                    )
                })
            })
            .collect::<Vec<_>>();
        let passenger_packets = visible_chain
            .iter()
            .rev()
            .filter_map(|entity_id| self.entity_by_id(*entity_id))
            .filter(|entity| {
                entity
                    .passengers()
                    .iter()
                    .any(|passenger_id| visible_entity_ids.contains(passenger_id))
            })
            .map(Entity::passenger_packet)
            .collect::<Vec<_>>();
        let Some(client) =
            self.entity_by_id_mut(viewer_player_id)
                .and_then(|entity| match entity {
                    Entity::Player(player) => player.client_mut(),
                    _ => None,
                })
        else {
            return Ok(());
        };
        viewer_dispatches
            .into_iter()
            .try_for_each(|(snapshot, leash_packets)| {
                snapshot.dispatch_with_leashes(client, leash_packets)
            })?;
        passenger_packets
            .into_iter()
            .try_for_each(|packet| packet.dispatch(client))
    }

    fn send_single_entity_spawn_to_player(
        &mut self,
        viewed_entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> Result<()> {
        let Some(snapshot) = self.entity_viewer_snapshot(viewed_entity_id) else {
            return Ok(());
        };
        let leash_packets = self.leash_packets_for_new_viewer(viewed_entity_id, viewer_player_id);
        let Some(client) =
            self.entity_by_id_mut(viewer_player_id)
                .and_then(|entity| match entity {
                    Entity::Player(player) => player.client_mut(),
                    _ => None,
                })
        else {
            return Ok(());
        };
        snapshot.dispatch_with_leashes(client, leash_packets)
    }

    fn collect_visible_passenger_chain(
        &mut self,
        root_entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> Vec<EntityId> {
        let mut visible_chain = vec![root_entity_id];
        let mut collected_entity_ids = HashSet::from([root_entity_id]);
        let mut pending_entity_ids = self
            .entity_by_id(root_entity_id)
            .map(|entity| {
                entity
                    .passengers()
                    .iter()
                    .rev()
                    .copied()
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        while let Some(entity_id) = pending_entity_ids.pop() {
            if collected_entity_ids.contains(&entity_id)
                || !self.register_automatic_visibility_pair(entity_id, viewer_player_id)
            {
                continue;
            }
            collected_entity_ids.insert(entity_id);
            visible_chain.push(entity_id);
            if let Some(entity) = self.entity_by_id(entity_id) {
                pending_entity_ids.extend(entity.passengers().iter().rev().copied());
            }
        }
        visible_chain
    }

    fn register_automatic_visibility_pair(
        &mut self,
        viewed_entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> bool {
        if viewed_entity_id == viewer_player_id {
            return false;
        }
        let Some(viewed_entity_index) = self
            .entities
            .iter()
            .position(|entity| entity.entity_id() == viewed_entity_id)
        else {
            return false;
        };
        let Some(viewer_player_index) = self
            .entities
            .iter()
            .position(|entity| entity.entity_id() == viewer_player_id)
        else {
            return false;
        };
        let (viewed_entity, viewer_player) =
            distinct_entities_mut(&mut self.entities, viewed_entity_index, viewer_player_index);
        let Entity::Player(viewer_player) = viewer_player else {
            return false;
        };
        let viewed_player_is_vanished =
            matches!(viewed_entity, Entity::Player(player) if player.is_vanished());
        if viewed_entity.view().is_viewer(viewer_player_id)
            || !automatic_visibility_pair_is_allowed(
                viewed_entity,
                viewer_player,
                viewed_player_is_vanished,
            )
        {
            return false;
        }
        viewed_entity.view_mut().automatic_add(viewer_player_id);
        viewer_player
            .view_mut()
            .register_viewed_entity(viewed_entity_id);
        true
    }

    fn leash_packets_for_new_viewer(
        &self,
        entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> Vec<spinel_core::network::clientbound::play::attach_entity::AttachEntityPacket> {
        let Some(entity) = self.entity_by_id(entity_id) else {
            return Vec::new();
        };
        let mut packets = Vec::new();
        if entity.leash_holder().is_some_and(|leash_holder_id| {
            leash_holder_id == viewer_player_id
                || self
                    .entity_by_id(leash_holder_id)
                    .is_some_and(|holder| holder.view().is_viewer(viewer_player_id))
        }) {
            packets.push(entity.attach_entity_packet());
        }
        packets.extend(
            entity
                .leashed_entities()
                .iter()
                .filter_map(|leashed_entity_id| self.entity_by_id(*leashed_entity_id))
                .filter(|leashed_entity| leashed_entity.view().is_viewer(viewer_player_id))
                .map(Entity::attach_entity_packet),
        );
        packets
    }

    fn entity_viewer_snapshot(&self, viewed_entity_id: EntityId) -> Option<EntityViewerSnapshot> {
        self.entity_by_id(viewed_entity_id)
            .map(EntityViewerSnapshot::from_entity)
    }
}

enum EntityViewerSnapshot {
    Generic(GenericEntityViewerSnapshot),
    Player(PlayerViewerSnapshot),
}

impl EntityViewerSnapshot {
    fn from_entity(entity: &Entity) -> Self {
        match entity {
            Entity::Creature(entity) => {
                Self::Generic(GenericEntityViewerSnapshot::from_entity(entity))
            }
            Entity::ExperienceOrb(entity) => {
                Self::Generic(GenericEntityViewerSnapshot::from_experience_orb(entity))
            }
            Entity::Generic(entity) => {
                Self::Generic(GenericEntityViewerSnapshot::from_entity(entity))
            }
            Entity::Item(entity) => {
                Self::Generic(GenericEntityViewerSnapshot::from_item_entity(entity))
            }
            Entity::Player(player) => Self::Player(PlayerViewerSnapshot::from_player(player)),
            Entity::Projectile(entity) => {
                Self::Generic(GenericEntityViewerSnapshot::from_projectile(entity))
            }
        }
    }

    fn dispatch_with_leashes(
        self,
        client: &mut Client,
        leash_packets: Vec<
            spinel_core::network::clientbound::play::attach_entity::AttachEntityPacket,
        >,
    ) -> Result<()> {
        match self {
            Self::Generic(snapshot) => snapshot.dispatch_with_leashes(client, leash_packets),
            Self::Player(snapshot) => snapshot.dispatch_with_leashes(client, leash_packets),
        }
    }
}

impl World {
    fn send_entity_remove_to_player(
        &mut self,
        viewed_entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> Result<()> {
        let Some((viewed_entity_type, viewed_entity_uuid)) = self
            .entity_by_id(viewed_entity_id)
            .map(|entity| (entity.entity_type(), entity.uuid()))
        else {
            return Ok(());
        };
        let Some(client) =
            self.entity_by_id_mut(viewer_player_id)
                .and_then(|entity| match entity {
                    Entity::Player(player) => player.client_mut(),
                    _ => None,
                })
        else {
            return Ok(());
        };
        if viewed_entity_type == EntityType::PLAYER {
            PlayerInfoRemovePacket::new(viewed_entity_uuid).dispatch(client)?;
        }
        RemoveEntitiesPacket::new(vec![viewed_entity_id.value()]).dispatch(client)
    }

    fn send_entity_switch_remove_to_player(
        &mut self,
        viewed_entity_id: EntityId,
        viewer_player_id: EntityId,
    ) -> Result<()> {
        let Some(client) =
            self.entity_by_id_mut(viewer_player_id)
                .and_then(|entity| match entity {
                    Entity::Player(player) => player.client_mut(),
                    _ => None,
                })
        else {
            return Ok(());
        };
        RemoveEntitiesPacket::new(vec![viewed_entity_id.value()]).dispatch(client)
    }

    fn broadcast_player_equipment(
        &mut self,
        changed_player_id: EntityId,
        equipment_entity_id: i32,
        equipment_entries: Vec<
            spinel_core::network::clientbound::play::set_equipment::EntityEquipmentEntry,
        >,
    ) -> Result<()> {
        let viewer_ids = self
            .entity_by_id(changed_player_id)
            .map(Entity::viewers)
            .unwrap_or_default();
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player)
                    if viewer_ids.contains(&player.entity_id()) && player.has_entered_world() =>
                {
                    Some(player)
                }
                _ => None,
            })
            .filter_map(Player::client_mut)
            .try_for_each(|viewer_client| {
                SetEquipmentPacket::new(equipment_entity_id, equipment_entries.clone())
                    .dispatch(viewer_client)
            })
    }

    pub fn tick(&mut self) {
        let registries = Registries::new_vanilla();
        self.tick_with_registries(&registries);
    }

    pub(crate) fn tick_with_registries(&mut self, registries: &Registries) {
        self.process_next_tick_scheduler();
        self.tick_time();
        self.tick_weather();
        self.dispatch_instance_tick_event();
        let world_snapshot = self.update_snapshot();
        let mut player_addresses = Vec::new();
        let mut entity_touches = Vec::new();
        let mut moved_entities = Vec::new();
        let mut entity_movements = Vec::new();
        let mut mergeable_item_entity_ids = Vec::new();
        let mut experience_orb_ids = Vec::new();
        let mut projectile_paths = Vec::new();
        let mut expired_fire_entities = Vec::new();
        let mut expired_effects = Vec::new();
        let mut creature_ai_actions = Vec::new();
        let event_dispatcher = self.event_dispatcher;
        let item_use_completions = self
            .entities
            .iter_mut()
            .filter_map(|entity| {
                let entity_ptr = entity as *mut Entity;
                match entity {
                    Entity::Creature(entity) => {
                        if entity.fire_ticks() == 1 {
                            expired_fire_entities.push(entity.entity_id());
                        }
                        let previous_position = entity.position();
                        entity.tick_before_movement(&world_snapshot, self.world_age as u64);
                        if let Some(movement) = entity.movement_tick(&world_snapshot) {
                            entity_movements.push(movement);
                        }
                        entity.tick_after_movement(&world_snapshot, self.world_age as u64);
                        if let Some(movement) = entity.position_movement_after_tick() {
                            entity_movements.push(movement);
                        }
                        creature_ai_actions.extend(entity.take_ai_actions());
                        dispatch_entity_tick_event(entity_ptr, event_dispatcher);
                        if entity.position() != previous_position {
                            moved_entities.push((entity.entity_id(), entity.position()));
                        }
                        expired_effects.extend(
                            entity
                                .take_expired_effects()
                                .into_iter()
                                .map(|effect| (entity.entity_id(), effect)),
                        );
                        entity_touches.push((entity.entity_id(), entity.position()));
                        None
                    }
                    Entity::ExperienceOrb(entity) => {
                        if let Some(movement) = entity.movement_tick(&world_snapshot) {
                            entity_movements.push(movement);
                        }
                        entity.tick();
                        dispatch_entity_tick_event(entity_ptr, event_dispatcher);
                        experience_orb_ids.push(entity.entity_id());
                        entity_touches.push((entity.entity_id(), entity.position()));
                        None
                    }
                    Entity::Generic(entity) => {
                        if entity.fire_ticks() == 1 {
                            expired_fire_entities.push(entity.entity_id());
                        }
                        if let Some(movement) = entity.movement_tick(&world_snapshot) {
                            entity_movements.push(movement);
                        }
                        entity.tick();
                        dispatch_entity_tick_event(entity_ptr, event_dispatcher);
                        expired_effects.extend(
                            entity
                                .take_expired_effects()
                                .into_iter()
                                .map(|effect| (entity.entity_id(), effect)),
                        );
                        entity_touches.push((entity.entity_id(), entity.position()));
                        None
                    }
                    Entity::Item(entity) => {
                        if let Some(movement) = entity.movement_tick(&world_snapshot) {
                            entity_movements.push(movement);
                        }
                        entity.tick();
                        dispatch_entity_tick_event(entity_ptr, event_dispatcher);
                        if entity.should_check_merge(self.world_age as u64) {
                            mergeable_item_entity_ids.push(entity.entity_id());
                        }
                        entity_touches.push((entity.entity_id(), entity.position()));
                        None
                    }
                    Entity::Projectile(entity) => {
                        if entity.fire_ticks() == 1 {
                            expired_fire_entities.push(entity.entity_id());
                        }
                        let position_before_tick = entity.position();
                        if let Some(movement) = entity.movement_tick(&world_snapshot) {
                            entity_movements.push(movement);
                        }
                        entity.tick();
                        projectile_paths.push((
                            entity.entity_id(),
                            position_before_tick,
                            entity.position(),
                        ));
                        dispatch_entity_tick_event(entity_ptr, event_dispatcher);
                        expired_effects.extend(
                            entity
                                .take_expired_effects()
                                .into_iter()
                                .map(|effect| (entity.entity_id(), effect)),
                        );
                        entity_touches.push((entity.entity_id(), entity.position()));
                        None
                    }
                    Entity::Player(player) => {
                        player.movement_tick(&world_snapshot);
                        let item_use_completion = player.tick();
                        dispatch_player_tick_event(player);
                        dispatch_entity_tick_event(entity_ptr, event_dispatcher);
                        if player.fire_ticks() == 1 {
                            expired_fire_entities.push(player.entity_id());
                        }
                        player.tick_fire_ticks();
                        expired_effects.extend(
                            player
                                .tick_living_state()
                                .into_iter()
                                .map(|effect| (player.entity_id(), effect)),
                        );
                        entity_touches.push((player.entity_id(), player.position()));
                        if player.has_entered_world() && player.is_online() {
                            player_addresses.push(player.addr);
                        }
                        dispatch_player_tick_end_event(player);
                        item_use_completion
                    }
                }
            })
            .collect::<Vec<_>>();
        creature_ai_actions
            .into_iter()
            .for_each(|action| self.apply_creature_ai_action(action));
        experience_orb_ids
            .into_iter()
            .for_each(|experience_orb_id| self.tick_experience_orb(experience_orb_id));
        mergeable_item_entity_ids
            .into_iter()
            .for_each(|item_entity_id| self.merge_item_entity(item_entity_id));
        let synchronization_packets = self
            .entities
            .iter_mut()
            .filter_map(|entity| {
                entity
                    .scheduled_position_sync_packet()
                    .map(|position_packet| {
                        (
                            entity.entity_id(),
                            position_packet,
                            entity.velocity_packet(),
                        )
                    })
            })
            .collect::<Vec<_>>();
        let metadata_packets = self
            .entities
            .iter_mut()
            .filter_map(|entity| {
                entity
                    .dirty_metadata_packet()
                    .map(|packet| (entity.entity_id(), packet))
            })
            .collect::<Vec<_>>();
        moved_entities
            .into_iter()
            .for_each(|(entity_id, position)| {
                self.entity_tracker.move_entity(entity_id, position);
                self.refresh_passenger_positions(entity_id);
            });
        entity_movements
            .into_iter()
            .for_each(|movement| self.apply_entity_movement(movement));
        metadata_packets
            .into_iter()
            .for_each(|(entity_id, packet)| {
                let _ = self.send_packet_to_player_viewers_and_self(entity_id, packet);
            });
        synchronization_packets.into_iter().for_each(
            |(entity_id, position_packet, velocity_packet)| {
                let _ = self.send_packet_to_entity_viewers(entity_id, position_packet);
                let _ = self.send_packet_to_entity_viewers(entity_id, velocity_packet);
            },
        );
        projectile_paths.into_iter().for_each(
            |(projectile_id, position_before_tick, position_after_tick)| {
                self.process_projectile_collision(
                    projectile_id,
                    position_before_tick,
                    position_after_tick,
                );
            },
        );
        self.dispatch_expired_fire_events(expired_fire_entities);
        expired_effects.into_iter().for_each(|(entity_id, effect)| {
            let _ = self.dispatch_entity_effect_removal(entity_id, effect);
        });
        self.process_living_item_pickups();
        self.process_player_experience_pickups();
        entity_touches
            .into_iter()
            .for_each(|(entity_id, position)| self.touch_entity_blocks(entity_id, position));
        let _ = self.process_completed_chunk_loads();
        let _ = self.process_pending_player_entries(registries);
        let _ = self.process_pending_entity_visibility_refreshes();
        self.tick_chunks(self.world_age as u64);
        player_addresses.into_iter().for_each(|address| {
            let _ = self.send_pending_chunks_for_player_address(address, registries);
        });
        item_use_completions.into_iter().for_each(|completion| {
            let _ = self.finish_player_item_use(completion);
        });
        self.process_tick_end_scheduler();
        self.dispatch_instance_tick_end_event();
        self.currently_changing_blocks.clear();
    }

    fn apply_entity_movement(&mut self, movement: EntityMovement) {
        let entity_id = movement.entity_id();
        self.entity_tracker
            .move_entity(entity_id, movement.position());
        self.refresh_passenger_positions(entity_id);
        let (movement_packet, head_look_packet) = movement.into_packets();
        let Some(packet) = movement_packet else {
            return;
        };
        match packet {
            EntityMovementPacket::Position(packet) => {
                let _ = self.send_packet_to_entity_viewers(entity_id, packet);
            }
            EntityMovementPacket::Teleport(packet) => {
                let _ = self.send_packet_to_entity_viewers(entity_id, packet);
            }
        }
        if let Some(packet) = head_look_packet {
            let _ = self.send_packet_to_entity_viewers(entity_id, packet);
        }
    }

    fn process_projectile_collision(
        &mut self,
        projectile_id: EntityId,
        position_before_tick: EntityPosition,
        position_after_tick: EntityPosition,
    ) {
        let Some(projectile_state) = self.projectile_collision_state(projectile_id) else {
            return;
        };
        let collision = self.projectile_collision(
            projectile_id,
            position_before_tick,
            position_after_tick,
            projectile_state,
        );
        if self
            .entity_by_id(projectile_id)
            .is_none_or(projectile_entity_is_removed)
        {
            return;
        }
        match collision {
            ProjectileCollision::Stuck(collision_position) => {
                self.stick_projectile(projectile_id, collision_position)
            }
            ProjectileCollision::Free => self.unstick_projectile(projectile_id),
        }
    }

    fn projectile_collision_state(
        &self,
        projectile_id: EntityId,
    ) -> Option<ProjectileCollisionState> {
        let Entity::Projectile(projectile) = self.entity_by_id(projectile_id)? else {
            return None;
        };
        Some(ProjectileCollisionState {
            shooter_id: projectile.shooter(),
            alive_ticks: projectile.ticks(),
            bounding_box: projectile.bounding_box(),
            is_on_ground: projectile.is_on_ground(),
        })
    }

    fn projectile_collision(
        &mut self,
        projectile_id: EntityId,
        position_before_tick: EntityPosition,
        position_after_tick: EntityPosition,
        projectile_state: ProjectileCollisionState,
    ) -> ProjectileCollision {
        if entity_positions_share_point(position_before_tick, position_after_tick) {
            return self
                .loaded_block_at(block_position_for_entity(position_before_tick))
                .filter(|block| block.is_solid())
                .map_or(ProjectileCollision::Free, |_| {
                    ProjectileCollision::Stuck(position_before_tick)
                });
        }
        projectile_sample_positions(
            position_before_tick,
            position_after_tick,
            projectile_state.bounding_box.width(),
        )
        .into_iter()
        .find_map(|collision_position| {
            self.projectile_collision_at(projectile_id, collision_position, projectile_state)
        })
        .unwrap_or(ProjectileCollision::Free)
    }

    fn projectile_collision_at(
        &mut self,
        projectile_id: EntityId,
        collision_position: EntityPosition,
        projectile_state: ProjectileCollisionState,
    ) -> Option<ProjectileCollision> {
        let block = self
            .loaded_block_at(block_position_for_entity(collision_position))
            .unwrap_or(Block::AIR);
        if block.is_solid() {
            let mut event =
                ProjectileCollideWithBlockEvent::new(projectile_id, collision_position, block);
            self.dispatch_projectile_block_collision_event(&mut event);
            if self
                .entity_by_id(projectile_id)
                .is_none_or(projectile_entity_is_removed)
            {
                return Some(ProjectileCollision::Stuck(collision_position));
            }
            if !event.is_cancelled() {
                return Some(ProjectileCollision::Stuck(collision_position));
            }
        }
        let target_id =
            self.projectile_collision_target(projectile_id, collision_position, projectile_state)?;
        let mut event =
            ProjectileCollideWithEntityEvent::new(projectile_id, collision_position, target_id);
        self.dispatch_projectile_entity_collision_event(&mut event);
        (!event.is_cancelled() && projectile_state.is_on_ground)
            .then_some(ProjectileCollision::Stuck(collision_position))
    }

    fn projectile_collision_target(
        &self,
        projectile_id: EntityId,
        collision_position: EntityPosition,
        projectile_state: ProjectileCollisionState,
    ) -> Option<EntityId> {
        self.entity_tracker
            .chunk_entities(
                ChunkPosition::from(collision_position),
                EntityTrackerTarget::Entities,
            )
            .into_iter()
            .filter(|target_id| *target_id != projectile_id)
            .filter(|target_id| {
                projectile_state.alive_ticks >= 3 || projectile_state.shooter_id != Some(*target_id)
            })
            .filter_map(|target_id| {
                self.entity_by_id(target_id)
                    .filter(|target| entity_is_living(target))
                    .filter(|target| {
                        entity_boxes_intersect_at(
                            collision_position,
                            projectile_state.bounding_box,
                            target.position(),
                            target.bounding_box(),
                        )
                    })
                    .map(|_| target_id)
            })
            .next()
    }

    fn stick_projectile(&mut self, projectile_id: EntityId, collision_position: EntityPosition) {
        let Some(Entity::Projectile(projectile)) = self.entity_by_id_mut(projectile_id) else {
            return;
        };
        if projectile.is_on_ground() {
            return;
        }
        projectile.set_position(collision_position);
        projectile.set_on_ground(true);
        projectile.set_velocity(Velocity(Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }));
        projectile.set_no_gravity(true);
        projectile.set_was_stuck(true);
        let teleport_packet = projectile.synchronize_position_packet();
        let velocity_packet = projectile.velocity_packet();
        self.entity_tracker
            .move_entity(projectile_id, collision_position);
        self.refresh_passenger_positions(projectile_id);
        let _ = self.send_packet_to_player_viewers_and_self(projectile_id, teleport_packet);
        let _ = self.send_packet_to_player_viewers_and_self(projectile_id, velocity_packet);
    }

    fn unstick_projectile(&mut self, projectile_id: EntityId) {
        let Some(Entity::Projectile(projectile)) = self.entity_by_id_mut(projectile_id) else {
            return;
        };
        if !projectile.was_stuck() {
            return;
        }
        projectile.set_was_stuck(false);
        let was_on_ground = projectile.is_on_ground();
        projectile.set_no_gravity(was_on_ground);
        projectile.set_on_ground(false);
        self.dispatch_projectile_uncollide_event(ProjectileUncollideEvent::new(projectile_id));
    }

    fn dispatch_entity_shoot_event(&mut self, event: &mut EntityShootEvent) {
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
    }

    fn dispatch_projectile_block_collision_event(
        &mut self,
        event: &mut ProjectileCollideWithBlockEvent,
    ) {
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
        event.collision_mut().dispatch(server);
    }

    fn dispatch_projectile_entity_collision_event(
        &mut self,
        event: &mut ProjectileCollideWithEntityEvent,
    ) {
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
        event.collision_mut().dispatch(server);
    }

    fn dispatch_projectile_uncollide_event(&mut self, mut event: ProjectileUncollideEvent) {
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
    }

    pub fn set_entity_equipment(
        &mut self,
        entity_id: EntityId,
        equipment_slot: EquipmentSlot,
        item_stack: ItemStack,
    ) -> Result<bool> {
        let Some(item_stack) =
            self.dispatch_entity_equip_event(entity_id, equipment_slot, item_stack)
        else {
            return Ok(false);
        };
        let equipment_packet = SetEquipmentPacket::new(
            entity_id.value(),
            vec![EntityEquipmentEntry {
                slot: equipment_slot.entity_equipment_slot(),
                item: Slot::from_item_stack(&item_stack),
            }],
        );
        let Some((attributes_packet, should_send_to_self)) =
            self.apply_entity_equipment(entity_id, equipment_slot, item_stack)
        else {
            return Ok(false);
        };
        self.send_packet_to_entity_viewers(entity_id, equipment_packet)?;
        if should_send_to_self {
            self.send_packet_to_player_viewers_and_self(entity_id, attributes_packet)?;
        } else {
            self.send_packet_to_entity_viewers(entity_id, attributes_packet)?;
        }
        Ok(true)
    }

    pub fn add_entity_effect(
        &mut self,
        entity_id: EntityId,
        effect: TimedPotionEffect,
    ) -> Result<bool> {
        if !self.dispatch_entity_potion_add_event(entity_id, effect.clone()) {
            return Ok(false);
        }
        if self.entity_has_effect(entity_id, effect.effect_id()) {
            self.remove_entity_effect(entity_id, effect.effect_id())?;
        }
        let Some(packet) = self.apply_entity_effect(entity_id, effect) else {
            return Ok(false);
        };
        self.send_packet_to_player_viewers_and_self(entity_id, packet)?;
        Ok(true)
    }

    pub fn remove_entity_effect(&mut self, entity_id: EntityId, effect_id: i32) -> Result<bool> {
        let Some(effect) = self.entity_effect(entity_id, effect_id).cloned() else {
            return Ok(false);
        };
        self.dispatch_entity_effect_removal(entity_id, effect)?;
        Ok(true)
    }

    pub fn clear_entity_effects(&mut self, entity_id: EntityId) -> Result<usize> {
        let effect_ids = self
            .entity_effects(entity_id)
            .into_iter()
            .map(TimedPotionEffect::effect_id)
            .collect::<Vec<_>>();
        effect_ids.iter().try_for_each(|effect_id| {
            self.remove_entity_effect(entity_id, *effect_id).map(|_| ())
        })?;
        Ok(effect_ids.len())
    }

    pub fn set_entity_fire_ticks(&mut self, entity_id: EntityId, fire_ticks: i32) -> bool {
        let Some(current_fire_ticks) =
            self.entity_by_id(entity_id)
                .and_then(|entity| match entity {
                    Entity::Creature(entity) => Some(entity.fire_ticks()),
                    Entity::ExperienceOrb(entity) => Some(entity.fire_ticks()),
                    Entity::Generic(entity) => Some(entity.fire_ticks()),
                    Entity::Item(_) => None,
                    Entity::Player(player) => Some(player.fire_ticks()),
                    Entity::Projectile(entity) => Some(entity.fire_ticks()),
                })
        else {
            return false;
        };
        let requested_fire_ticks = fire_ticks.max(0);
        if requested_fire_ticks > 0 {
            let Some(approved_fire_ticks) =
                self.dispatch_entity_set_fire_event(entity_id, requested_fire_ticks)
            else {
                return false;
            };
            return self.apply_entity_fire_ticks(entity_id, approved_fire_ticks);
        }
        if current_fire_ticks != 0 && self.dispatch_entity_fire_extinguish_event(entity_id, false) {
            return self.apply_entity_cancelled_fire_extinguish(entity_id, 0);
        }
        self.apply_entity_fire_ticks(entity_id, requested_fire_ticks)
    }

    pub fn damage_entity(
        &mut self,
        registries: &Registries,
        entity_id: EntityId,
        mut damage: Damage,
    ) -> Result<bool> {
        if !damage.resolve_type(registries) {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!(
                    "damage type {} is not registered",
                    damage.damage_type().key()
                ),
            ));
        }
        if self.entity_rejects_damage(entity_id, &damage) {
            return Ok(false);
        }
        if damage.sound().is_none() {
            damage.set_sound(Some(damage.default_sound(self.entity_is_player(entity_id))));
        }
        let Some(damage) = self.dispatch_entity_damage_event(entity_id, damage) else {
            return Ok(false);
        };
        if damage.should_animate() {
            let damage_type_id = damage.damage_type_id(registries).ok_or_else(|| {
                Error::new(
                    ErrorKind::NotFound,
                    format!(
                        "damage type {} is not registered",
                        damage.damage_type().key()
                    ),
                )
            })?;
            self.send_packet_to_player_viewers_and_self(
                entity_id,
                DamageEventPacket {
                    target_entity_id: entity_id.value(),
                    damage_type_id,
                    source_entity_id: damage
                        .attacker()
                        .map(|attacker| attacker.value() + 1)
                        .unwrap_or(0),
                    source_direct_id: damage
                        .source()
                        .map(|source| source.value() + 1)
                        .unwrap_or(0),
                    source_position: damage.source_position(),
                },
            )?;
        }
        if let Some(sound) = damage.sound() {
            let sound_source_id = damage_sound_source_id(entity_id, self);
            self.play_entity_sound_except(None, sound, sound_source_id, entity_id, 1.0, 1.0, 0)?;
        }
        self.apply_entity_damage(entity_id, damage)?;
        Ok(true)
    }

    pub fn kill_entity(&mut self, entity_id: EntityId) -> Result<bool> {
        if self.entity_is_dead(entity_id) {
            return Ok(false);
        }
        if self
            .entity_by_id(entity_id)
            .is_some_and(|entity| matches!(entity, Entity::Player(_)))
        {
            self.apply_player_death_before_living_death(entity_id)?;
        }
        self.send_packet_to_player_viewers_and_self(
            entity_id,
            EntityStatusPacket {
                entity_id: entity_id.value(),
                status: 3,
            },
        )?;
        self.apply_living_death_state(entity_id)?;
        self.dispatch_entity_death_event(entity_id);
        Ok(true)
    }

    fn dispatch_expired_fire_events(&mut self, entity_ids: Vec<EntityId>) {
        entity_ids.into_iter().for_each(|entity_id| {
            if self.dispatch_entity_fire_extinguish_event(entity_id, true) {
                self.apply_entity_cancelled_fire_extinguish(entity_id, 0);
            }
        });
    }

    fn process_living_item_pickups(&mut self) {
        let living_entity_ids = self
            .entities
            .iter()
            .filter_map(|entity| match entity {
                Entity::Generic(entity)
                    if entity.entity_type().is_living() && entity.can_pickup_item() =>
                {
                    Some(entity.entity_id())
                }
                Entity::Player(player) if player.can_pickup_item() => Some(player.entity_id()),
                _ => None,
            })
            .collect::<Vec<_>>();
        living_entity_ids
            .into_iter()
            .for_each(|entity_id| self.process_living_entity_item_pickups(entity_id));
    }

    fn tick_experience_orb(&mut self, experience_orb_id: EntityId) {
        let current_tick = self.world_age;
        let Some(Entity::ExperienceOrb(experience_orb)) = self.entity_by_id_mut(experience_orb_id)
        else {
            return;
        };
        experience_orb.apply_gravity();
        let experience_orb_position = experience_orb.position();
        let current_target = experience_orb.target();
        let target_refresh_tick =
            current_tick - 20 + i64::from(experience_orb.entity_id().value() % 100);
        let should_refresh_target = experience_orb.last_target_update_tick() < target_refresh_tick;

        let target_is_missing_or_distant = current_target.is_none_or(|target_id| {
            self.entity_by_id(target_id).is_none_or(|target| {
                target.position().distance_squared(experience_orb_position) > 64.0
            })
        });
        let refreshed_target = (should_refresh_target && target_is_missing_or_distant)
            .then(|| self.closest_player_to(experience_orb_position, 8.0))
            .flatten();

        if should_refresh_target {
            let Some(Entity::ExperienceOrb(experience_orb)) =
                self.entity_by_id_mut(experience_orb_id)
            else {
                return;
            };
            if target_is_missing_or_distant {
                experience_orb.set_target(refreshed_target);
            }
            experience_orb.set_last_target_update_tick(current_tick);
        }

        let target = self
            .entity_by_id(experience_orb_id)
            .and_then(|entity| match entity {
                Entity::ExperienceOrb(experience_orb) => experience_orb.target(),
                _ => None,
            })
            .and_then(|target_id| match self.entity_by_id(target_id) {
                Some(Entity::Player(player)) if player.game_mode() != GameMode::Spectator => {
                    Some((target_id, player.position(), player.eye_height()))
                }
                _ => None,
            });
        let Some(Entity::ExperienceOrb(experience_orb)) = self.entity_by_id_mut(experience_orb_id)
        else {
            return;
        };
        match target {
            Some((_, target_position, eye_height)) => {
                experience_orb.apply_attraction(target_position, eye_height);
            }
            None => experience_orb.set_target(None),
        }
        experience_orb.apply_drag();
    }

    fn closest_player_to(
        &self,
        position: EntityPosition,
        maximum_distance: f64,
    ) -> Option<EntityId> {
        let maximum_distance_squared = maximum_distance * maximum_distance;
        self.players()
            .map(|player| {
                (
                    player.entity_id(),
                    player.position().distance_squared(position),
                )
            })
            .filter(|(_, distance_squared)| *distance_squared <= maximum_distance_squared)
            .min_by(|(_, first_distance), (_, second_distance)| {
                first_distance.total_cmp(second_distance)
            })
            .map(|(player_id, _)| player_id)
    }

    fn process_player_experience_pickups(&mut self) {
        let current_tick = self.world_age;
        let player_ids = self
            .entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.experience_pickup_is_ready(current_tick) => {
                    player.refresh_experience_pickup_cooldown(current_tick);
                    Some(player.entity_id())
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        player_ids.into_iter().for_each(|player_id| {
            self.process_player_experience_pickup(player_id);
        });
    }

    fn process_player_experience_pickup(&mut self, player_id: EntityId) {
        let Some((player_position, expanded_bounding_box)) =
            self.entity_by_id(player_id)
                .and_then(|entity| match entity {
                    Entity::Player(player) => {
                        Some((player.position(), player.expanded_bounding_box()))
                    }
                    _ => None,
                })
        else {
            return;
        };
        let experience_orb_ids = self.entity_tracker.nearby_entities(
            player_position,
            expanded_bounding_box.width(),
            EntityTrackerTarget::ExperienceOrbs,
        );
        experience_orb_ids
            .into_iter()
            .for_each(|experience_orb_id| {
                let Some(experience_count) =
                    self.entity_by_id(experience_orb_id)
                        .and_then(|entity| match entity {
                            Entity::ExperienceOrb(experience_orb)
                                if experience_orb.intersects_box_at(
                                    player_position.as_vector(),
                                    expanded_bounding_box,
                                ) =>
                            {
                                Some(experience_orb.experience_count())
                            }
                            _ => None,
                        })
                else {
                    return;
                };
                if self.dispatch_pickup_experience_event(
                    player_id,
                    experience_orb_id,
                    experience_count,
                ) {
                    return;
                }
                self.take_entity(experience_orb_id);
            });
    }

    fn dispatch_pickup_experience_event(
        &mut self,
        player_id: EntityId,
        experience_orb_id: EntityId,
        experience_count: i16,
    ) -> bool {
        let Some(server_ptr) = self.event_dispatcher else {
            return false;
        };
        let mut event = PickupExperienceEvent::new(player_id, experience_orb_id, experience_count);
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
        event.is_cancelled()
    }

    fn process_living_entity_item_pickups(&mut self, living_entity_id: EntityId) {
        if !self.refresh_living_item_pickup_cooldown(living_entity_id) {
            return;
        }
        let Some((position, expanded_bounding_box)) = self
            .entity_by_id(living_entity_id)
            .and_then(living_item_pickup_scan)
        else {
            return;
        };
        let item_entity_ids = self.entity_tracker.nearby_entities(
            position,
            expanded_bounding_box.width(),
            EntityTrackerTarget::Items,
        );
        item_entity_ids.into_iter().for_each(|item_entity_id| {
            if !self.item_entity_can_be_picked_up_by(
                living_entity_id,
                item_entity_id,
                position,
                expanded_bounding_box,
            ) {
                return;
            }
            if self.dispatch_pickup_item_event(living_entity_id, item_entity_id) {
                return;
            }
            let Some(pickup_item_count) = self
                .entity_by_id(item_entity_id)
                .and_then(item_entity)
                .map(|item_entity| item_entity.item_stack().amount())
            else {
                return;
            };
            let _ = self.send_packet_to_player_viewers_and_self(
                living_entity_id,
                TakeItemEntityPacket {
                    collected_entity_id: item_entity_id.value(),
                    collector_entity_id: living_entity_id.value(),
                    pickup_item_count,
                },
            );
            self.take_entity(item_entity_id);
        });
    }

    fn refresh_living_item_pickup_cooldown(&mut self, living_entity_id: EntityId) -> bool {
        let Some(entity) = self.entity_by_id_mut(living_entity_id) else {
            return false;
        };
        match entity {
            Entity::Generic(entity) if entity.item_pickup_cooldown() == 0 => {
                entity.set_item_pickup_cooldown(5);
                true
            }
            Entity::Player(player) if player.item_pickup_cooldown() == 0 => {
                player.set_item_pickup_cooldown(5);
                true
            }
            _ => false,
        }
    }

    fn item_entity_can_be_picked_up_by(
        &self,
        living_entity_id: EntityId,
        item_entity_id: EntityId,
        living_position: EntityPosition,
        expanded_bounding_box: EntityBoundingBox,
    ) -> bool {
        let Some(item_entity) = self.entity_by_id(item_entity_id).and_then(item_entity) else {
            return false;
        };
        if !item_entity.is_pickable() {
            return false;
        }
        if self
            .entity_by_id(living_entity_id)
            .is_some_and(|entity| matches!(entity, Entity::Player(_)))
            && !item_entity.is_viewer(living_entity_id)
        {
            return false;
        }
        item_entity.intersects_box_at(living_position.as_vector(), expanded_bounding_box)
    }

    fn dispatch_pickup_item_event(
        &mut self,
        living_entity_id: EntityId,
        item_entity_id: EntityId,
    ) -> bool {
        let Some(server_ptr) = self.event_dispatcher else {
            return false;
        };
        let mut event = PickupItemEvent::new(living_entity_id, item_entity_id);
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
        event.is_cancelled()
    }

    fn merge_item_entity(&mut self, source_item_entity_id: EntityId) {
        let Some((source_position, merge_range)) = self
            .entity_by_id(source_item_entity_id)
            .and_then(item_entity)
            .map(|item_entity| (item_entity.position(), item_entity.merge_range()))
        else {
            return;
        };
        let nearby_item_entity_ids = self.entity_tracker.nearby_entities(
            source_position,
            f64::from(merge_range),
            EntityTrackerTarget::Items,
        );
        nearby_item_entity_ids
            .into_iter()
            .filter(|merged_item_entity_id| *merged_item_entity_id != source_item_entity_id)
            .for_each(|merged_item_entity_id| {
                self.merge_item_entity_pair(source_item_entity_id, merged_item_entity_id);
            });
    }

    fn merge_item_entity_pair(
        &mut self,
        source_item_entity_id: EntityId,
        merged_item_entity_id: EntityId,
    ) {
        let Some(source_item_stack) = self
            .entity_by_id(source_item_entity_id)
            .and_then(item_entity)
            .filter(|item_entity| item_entity.is_pickable() && item_entity.is_mergeable())
            .map(|item_entity| item_entity.item_stack().clone())
        else {
            return;
        };
        let Some(merged_item_stack) = self
            .entity_by_id(merged_item_entity_id)
            .and_then(item_entity)
            .filter(|item_entity| item_entity.is_pickable() && item_entity.is_mergeable())
            .map(|item_entity| item_entity.item_stack().clone())
        else {
            return;
        };
        if !source_item_stack.is_similar(&merged_item_stack) {
            return;
        }
        let total_amount = source_item_stack.amount() + merged_item_stack.amount();
        if total_amount < 0 || total_amount > source_item_stack.max_stack_size() {
            return;
        }
        let result = source_item_stack.with_amount(total_amount);
        let Some(result) = self.dispatch_entity_item_merge_event(
            source_item_entity_id,
            merged_item_entity_id,
            result,
        ) else {
            return;
        };
        let Some(source_item_entity) =
            self.entity_by_id_mut(source_item_entity_id)
                .and_then(|entity| match entity {
                    Entity::Item(item_entity) => Some(item_entity),
                    _ => None,
                })
        else {
            return;
        };
        source_item_entity.set_item_stack(result);
        self.take_entity(merged_item_entity_id);
    }

    fn dispatch_entity_item_merge_event(
        &mut self,
        source_item_entity_id: EntityId,
        merged_item_entity_id: EntityId,
        result: ItemStack,
    ) -> Option<ItemStack> {
        let Some(server_ptr) = self.event_dispatcher else {
            return Some(result);
        };
        let Some(source_item_entity) = self
            .entity_by_id_mut(source_item_entity_id)
            .map(|entity| entity as *mut Entity)
        else {
            return Some(result);
        };
        let Some(merged_item_entity) = self
            .entity_by_id_mut(merged_item_entity_id)
            .map(|entity| entity as *mut Entity)
        else {
            return Some(result);
        };
        let mut event = EntityItemMergeEvent::new(source_item_entity, merged_item_entity, result);
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
        (!event.is_cancelled()).then(|| event.result().clone())
    }

    fn dispatch_entity_equip_event(
        &mut self,
        entity_id: EntityId,
        equipment_slot: EquipmentSlot,
        item_stack: ItemStack,
    ) -> Option<ItemStack> {
        let Some(server_ptr) = self.event_dispatcher else {
            return Some(item_stack);
        };
        let mut event = EntityEquipEvent::new(entity_id, item_stack, equipment_slot);
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
        Some(event.equipped_item().clone())
    }

    fn apply_entity_equipment(
        &mut self,
        entity_id: EntityId,
        equipment_slot: EquipmentSlot,
        item_stack: ItemStack,
    ) -> Option<(
        spinel_core::network::clientbound::play::update_attributes::UpdateAttributesPacket,
        bool,
    )> {
        let entity = self.entity_by_id_mut(entity_id)?;
        match entity {
            Entity::Generic(entity) if entity.entity_type().is_living() => {
                entity.set_equipment(equipment_slot, item_stack);
                Some((entity.update_attributes_packet(), false))
            }
            Entity::Player(player) => {
                if !player.set_equipment(equipment_slot, item_stack) {
                    return None;
                }
                Some((player.update_attributes_packet(), true))
            }
            _ => None,
        }
    }

    fn dispatch_entity_potion_add_event(
        &mut self,
        entity_id: EntityId,
        effect: TimedPotionEffect,
    ) -> bool {
        let Some(server_ptr) = self.event_dispatcher else {
            return true;
        };
        let Some(entity) = self
            .entity_by_id_mut(entity_id)
            .map(|entity| entity as *mut Entity)
        else {
            return true;
        };
        let mut event = EntityPotionAddEvent::new(entity, effect);
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
        !event.is_cancelled()
    }

    fn dispatch_entity_potion_remove_event(
        &mut self,
        entity_id: EntityId,
        effect: TimedPotionEffect,
    ) {
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let Some(entity) = self
            .entity_by_id_mut(entity_id)
            .map(|entity| entity as *mut Entity)
        else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        EntityPotionRemoveEvent::new(entity, effect).dispatch(server);
    }

    fn entity_effect(&self, entity_id: EntityId, effect_id: i32) -> Option<&TimedPotionEffect> {
        match self.entity_by_id(entity_id)? {
            Entity::Creature(entity) => entity.effect(effect_id),
            Entity::ExperienceOrb(entity) => entity.effect(effect_id),
            Entity::Generic(entity) => entity.effect(effect_id),
            Entity::Item(entity) => entity.effect(effect_id),
            Entity::Player(player) => player.effect(effect_id),
            Entity::Projectile(entity) => entity.effect(effect_id),
        }
    }

    fn entity_effects(&self, entity_id: EntityId) -> Vec<&TimedPotionEffect> {
        match self.entity_by_id(entity_id) {
            Some(Entity::Creature(entity)) => entity.active_effects(),
            Some(Entity::ExperienceOrb(entity)) => entity.active_effects(),
            Some(Entity::Generic(entity)) => entity.active_effects(),
            Some(Entity::Item(entity)) => entity.active_effects(),
            Some(Entity::Player(player)) => player.active_effects(),
            Some(Entity::Projectile(entity)) => entity.active_effects(),
            None => Vec::new(),
        }
    }

    fn entity_has_effect(&self, entity_id: EntityId, effect_id: i32) -> bool {
        self.entity_effect(entity_id, effect_id).is_some()
    }

    fn apply_entity_effect(
        &mut self,
        entity_id: EntityId,
        effect: TimedPotionEffect,
    ) -> Option<EntityEffectPacket> {
        match self.entity_by_id_mut(entity_id)? {
            Entity::Creature(entity) => Some(entity.add_effect(effect)),
            Entity::ExperienceOrb(entity) => Some(entity.add_effect(effect)),
            Entity::Generic(entity) => Some(entity.add_effect(effect)),
            Entity::Item(entity) => Some(entity.add_effect(effect)),
            Entity::Player(player) => Some(player.add_effect(effect)),
            Entity::Projectile(entity) => Some(entity.add_effect(effect)),
        }
    }

    fn apply_entity_effect_removal(
        &mut self,
        entity_id: EntityId,
        effect_id: i32,
    ) -> Option<RemoveEntityEffectPacket> {
        match self.entity_by_id_mut(entity_id)? {
            Entity::Creature(entity) => entity.remove_effect(effect_id),
            Entity::ExperienceOrb(entity) => entity.remove_effect(effect_id),
            Entity::Generic(entity) => entity.remove_effect(effect_id),
            Entity::Item(entity) => entity.remove_effect(effect_id),
            Entity::Player(player) => player.remove_effect(effect_id),
            Entity::Projectile(entity) => entity.remove_effect(effect_id),
        }
    }

    fn dispatch_entity_effect_removal(
        &mut self,
        entity_id: EntityId,
        effect: TimedPotionEffect,
    ) -> Result<()> {
        let packet = self
            .apply_entity_effect_removal(entity_id, effect.effect_id())
            .unwrap_or_else(|| effect.remove_packet(entity_id));
        self.send_packet_to_player_viewers_and_self(entity_id, packet)?;
        self.dispatch_entity_potion_remove_event(entity_id, effect);
        Ok(())
    }

    fn entity_is_dead(&self, entity_id: EntityId) -> bool {
        self.entity_by_id(entity_id)
            .is_none_or(|entity| match entity {
                Entity::Creature(entity) => entity.is_dead(),
                Entity::ExperienceOrb(_) => true,
                Entity::Generic(entity) => entity.is_dead(),
                Entity::Item(_) => true,
                Entity::Player(player) => player.is_dead(),
                Entity::Projectile(_) => true,
            })
    }

    fn entity_is_player(&self, entity_id: EntityId) -> bool {
        self.entity_by_id(entity_id)
            .is_some_and(|entity| matches!(entity, Entity::Player(_)))
    }

    fn apply_player_death_before_living_death(&mut self, entity_id: EntityId) -> Result<()> {
        let Some(Entity::Player(player)) = self.entity_by_id_mut(entity_id) else {
            return Ok(());
        };
        player.kill()
    }

    fn apply_living_death_state(&mut self, entity_id: EntityId) -> Result<()> {
        let passenger_ids = self
            .entity_by_id(entity_id)
            .map(|entity| entity.passengers().iter().copied().collect::<Vec<_>>())
            .unwrap_or_default();
        let Some(entity) = self.entity_by_id_mut(entity_id) else {
            return Ok(());
        };
        match entity {
            Entity::Creature(entity) => {
                entity.kill();
                entity.set_velocity(Velocity(Vector3d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }));
            }
            Entity::ExperienceOrb(_) => {}
            Entity::Generic(entity) => {
                entity.kill();
                entity.set_velocity(Velocity(Vector3d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }));
            }
            Entity::Item(_) => {}
            Entity::Player(player) => {
                player.set_pose(PlayerPose::Dying);
            }
            Entity::Projectile(_) => {}
        }
        passenger_ids.into_iter().try_for_each(|passenger_id| {
            self.remove_passenger(entity_id, passenger_id).map(|_| ())
        })?;
        Ok(())
    }

    fn dispatch_entity_death_event(&mut self, entity_id: EntityId) {
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let Some(entity) = self
            .entity_by_id_mut(entity_id)
            .map(|entity| entity as *mut Entity)
        else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        EntityDeathEvent::new(entity).dispatch(server);
    }

    fn dispatch_entity_attack_event(&mut self, entity_id: EntityId, target_id: EntityId) {
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        EntityAttackEvent::new(entity_id, target_id).dispatch(server);
    }

    fn dispatch_entity_teleport_event(
        &mut self,
        entity_id: EntityId,
        teleport_position: EntityPosition,
        new_position: EntityPosition,
        relative_flags: TeleportFlags,
    ) {
        let Some(server_ptr) = self.event_dispatcher else {
            return;
        };
        let Some(entity) = self
            .entity_by_id_mut(entity_id)
            .map(|entity| entity as *mut Entity)
        else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        EntityTeleportEvent::new(entity, teleport_position, new_position, relative_flags)
            .dispatch(server);
    }

    fn dispatch_entity_velocity_event(
        &mut self,
        entity_id: EntityId,
        velocity: Velocity,
    ) -> Option<Velocity> {
        let Some(server_ptr) = self.event_dispatcher else {
            return Some(velocity);
        };
        let Some(entity) = self
            .entity_by_id_mut(entity_id)
            .map(|entity| entity as *mut Entity)
        else {
            return Some(velocity);
        };
        let mut event = EntityVelocityEvent::new(entity, velocity);
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
        if event.is_cancelled() {
            return None;
        }
        Some(event.velocity())
    }

    fn load_teleport_chunks(
        &mut self,
        previous_position: EntityPosition,
        destination: EntityPosition,
        chunks: Option<&[i64]>,
    ) -> Result<()> {
        let explicit_chunk_positions = chunks
            .unwrap_or_default()
            .iter()
            .copied()
            .map(ChunkPosition::from_index)
            .collect::<Vec<_>>();
        if explicit_chunk_positions.is_empty() {
            let previous_chunk = ChunkPosition::from(previous_position);
            let destination_chunk = ChunkPosition::from(destination);
            if previous_chunk == destination_chunk {
                return Ok(());
            }
            self.load_optional_chunk_result(destination_chunk)?;
            return Ok(());
        }
        self.load_optional_chunks(&explicit_chunk_positions)?;
        Ok(())
    }

    fn begin_teleport_chunk_loads(
        &mut self,
        previous_position: EntityPosition,
        destination: EntityPosition,
        chunks: Option<&[i64]>,
    ) -> Result<Vec<ChunkLoadTicket>> {
        let explicit_chunk_positions = chunks
            .unwrap_or_default()
            .iter()
            .copied()
            .map(ChunkPosition::from_index)
            .collect::<Vec<_>>();
        if !explicit_chunk_positions.is_empty() {
            return explicit_chunk_positions
                .into_iter()
                .map(|position| self.load_optional_chunk_future(position))
                .collect::<Result<Vec<_>>>()
                .map(|tickets| tickets.into_iter().flatten().collect());
        }
        let previous_chunk = ChunkPosition::from(previous_position);
        let destination_chunk = ChunkPosition::from(destination);
        if previous_chunk == destination_chunk {
            return Ok(Vec::new());
        }
        self.load_optional_chunk_future(destination_chunk)
            .map(|ticket| ticket.into_iter().collect())
    }

    fn entity_rejects_damage(&self, entity_id: EntityId, damage: &Damage) -> bool {
        let Some(entity) = self.entity_by_id(entity_id) else {
            return true;
        };
        match entity {
            Entity::Creature(entity) => {
                entity.is_dead()
                    || (damage.damage_type() != &DamageType::OUT_OF_WORLD
                        && entity.is_immune_to_damage(&damage.damage_type().key().to_string()))
            }
            Entity::ExperienceOrb(_) => true,
            Entity::Generic(entity) => {
                entity.is_dead()
                    || (damage.damage_type() != &DamageType::OUT_OF_WORLD
                        && entity.is_immune_to_damage(&damage.damage_type().key().to_string()))
            }
            Entity::Item(_) => true,
            Entity::Player(player) => {
                player.is_dead()
                    || (damage.damage_type() != &DamageType::OUT_OF_WORLD
                        && player.is_invulnerable())
            }
            Entity::Projectile(_) => true,
        }
    }

    fn dispatch_entity_damage_event(
        &mut self,
        entity_id: EntityId,
        damage: Damage,
    ) -> Option<Damage> {
        let Some(server_ptr) = self.event_dispatcher else {
            return Some(damage);
        };
        let Some(entity) = self
            .entity_by_id_mut(entity_id)
            .map(|entity| entity as *mut Entity)
        else {
            return Some(damage);
        };
        let mut event = EntityDamageEvent::new(entity, damage);
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
        if event.is_cancelled() {
            return None;
        }
        Some(event.damage().clone())
    }

    fn apply_entity_damage(&mut self, entity_id: EntityId, damage: Damage) -> Result<bool> {
        let Some(entity) = self.entity_by_id_mut(entity_id) else {
            return Ok(false);
        };
        let should_kill = match entity {
            Entity::Creature(entity) => {
                entity.apply_damage(damage);
                entity.health() <= 0.0
            }
            Entity::ExperienceOrb(entity) => {
                entity.apply_damage(damage);
                entity.health() <= 0.0
            }
            Entity::Generic(entity) => {
                entity.apply_damage(damage);
                entity.health() <= 0.0
            }
            Entity::Item(_) => return Ok(false),
            Entity::Player(player) => {
                player.apply_damage(damage)?;
                player.health() <= 0.0
            }
            Entity::Projectile(_) => return Ok(false),
        };
        if should_kill {
            self.kill_entity(entity_id)?;
        }
        Ok(true)
    }

    fn dispatch_entity_set_fire_event(
        &mut self,
        entity_id: EntityId,
        fire_ticks: i32,
    ) -> Option<i32> {
        let Some(server_ptr) = self.event_dispatcher else {
            return Some(fire_ticks);
        };
        let Some(entity) = self
            .entity_by_id_mut(entity_id)
            .map(|entity| entity as *mut Entity)
        else {
            return Some(fire_ticks);
        };
        let mut event = EntitySetFireEvent::new(entity, fire_ticks);
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
        if event.is_cancelled() {
            return None;
        }
        Some(event.fire_ticks())
    }

    fn dispatch_entity_fire_extinguish_event(
        &mut self,
        entity_id: EntityId,
        natural: bool,
    ) -> bool {
        let Some(server_ptr) = self.event_dispatcher else {
            return false;
        };
        let Some(entity) = self
            .entity_by_id_mut(entity_id)
            .map(|entity| entity as *mut Entity)
        else {
            return false;
        };
        let mut event = EntityFireExtinguishEvent::new(entity, natural);
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        event.dispatch(server);
        event.is_cancelled()
    }

    fn apply_entity_fire_ticks(&mut self, entity_id: EntityId, fire_ticks: i32) -> bool {
        let Some(entity) = self.entity_by_id_mut(entity_id) else {
            return false;
        };
        match entity {
            Entity::Creature(entity) => entity.set_fire_ticks(fire_ticks),
            Entity::ExperienceOrb(entity) => entity.set_fire_ticks(fire_ticks),
            Entity::Generic(entity) => entity.set_fire_ticks(fire_ticks),
            Entity::Item(_) => return false,
            Entity::Player(player) => player.set_fire_ticks(fire_ticks),
            Entity::Projectile(entity) => entity.set_fire_ticks(fire_ticks),
        }
        true
    }

    fn apply_entity_cancelled_fire_extinguish(
        &mut self,
        entity_id: EntityId,
        fire_ticks: i32,
    ) -> bool {
        let Some(entity) = self.entity_by_id_mut(entity_id) else {
            return false;
        };
        match entity {
            Entity::Creature(entity) => {
                entity.set_fire_ticks_after_cancelled_extinguish(fire_ticks);
                entity.set_on_fire(true);
            }
            Entity::ExperienceOrb(entity) => {
                entity.set_fire_ticks_after_cancelled_extinguish(fire_ticks);
                entity.set_on_fire(true);
            }
            Entity::Generic(entity) => {
                entity.set_fire_ticks_after_cancelled_extinguish(fire_ticks);
                entity.set_on_fire(true);
            }
            Entity::Item(_) => return false,
            Entity::Player(player) => {
                player.set_fire_ticks_after_cancelled_extinguish(fire_ticks);
                player.set_on_fire(true);
            }
            Entity::Projectile(entity) => {
                entity.set_fire_ticks_after_cancelled_extinguish(fire_ticks);
                entity.set_on_fire(true);
            }
        }
        true
    }

    fn process_next_tick_scheduler(&mut self) {
        let mut scheduler = std::mem::take(&mut self.scheduler);
        scheduler.process_tick(self);
        self.scheduler = scheduler;
    }

    fn process_tick_end_scheduler(&mut self) {
        let mut scheduler = std::mem::take(&mut self.scheduler);
        scheduler.process_tick_end(self);
        self.scheduler = scheduler;
    }

    fn touch_entity_blocks(&self, entity_id: EntityId, position: EntityPosition) {
        let block_position = BlockPosition::new(
            position.x().floor() as i32,
            position.y().floor() as i32,
            position.z().floor() as i32,
        );
        let Some(block_instance) = self.loaded_block_instance_at(block_position) else {
            return;
        };
        let Some(handler) = block_instance.handler() else {
            return;
        };
        handler.on_touch(BlockHandlerTouch::new(
            block_instance.block(),
            self.uuid,
            block_position,
            entity_id,
        ));
    }

    fn tick_time(&mut self) {
        self.world_age += 1;
        self.time += self.time_rate as i64;
        if self.time_synchronization_ticks <= 0 {
            return;
        }
        if self.world_age % self.time_synchronization_ticks as i64 != 0 {
            return;
        }
        let _ = self.broadcast_time();
    }

    fn broadcast_time(&mut self) -> Result<()> {
        let time_packet = self.time_packet();
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.has_entered_world() => Some(player),
                _ => None,
            })
            .filter_map(Player::client_mut)
            .try_for_each(|client| {
                SetTimePacket::new(
                    time_packet.world_age,
                    time_packet.time,
                    time_packet.tick_day_time,
                )
                .dispatch(client)
            })
    }

    fn broadcast_weather(&mut self, previous_weather: Weather) -> Result<()> {
        let weather = self.transitioning_weather;
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.has_entered_world() => Some(player),
                _ => None,
            })
            .filter_map(Player::client_mut)
            .try_for_each(|client| {
                if previous_weather.is_raining() != weather.is_raining() {
                    weather.is_raining_packet().dispatch(client)?;
                }
                if previous_weather.rain_level() != weather.rain_level() {
                    weather.rain_level_packet().dispatch(client)?;
                }
                if previous_weather.thunder_level() == weather.thunder_level() {
                    return Ok(());
                }
                weather.thunder_level_packet().dispatch(client)
            })
    }

    fn tick_weather(&mut self) {
        if self.remaining_rain_transition_ticks <= 0 && self.remaining_thunder_transition_ticks <= 0
        {
            return;
        }
        let previous_weather = self.transitioning_weather;
        self.transitioning_weather = transition_weather(
            self.weather,
            self.transitioning_weather,
            self.remaining_rain_transition_ticks,
            self.remaining_thunder_transition_ticks,
        );
        let _ = self.broadcast_weather(previous_weather);
        self.remaining_rain_transition_ticks = (self.remaining_rain_transition_ticks - 1).max(0);
        self.remaining_thunder_transition_ticks =
            (self.remaining_thunder_transition_ticks - 1).max(0);
    }

    fn finish_player_item_use(
        &mut self,
        completion: crate::entity::player::PlayerItemUseCompletion,
    ) -> Result<()> {
        let _ = self.broadcast_entity_status(completion.entity_id, completion.status);
        let player = unsafe { &mut *completion.player };
        let Some(client) = player.client_mut().map(|client| client as *mut Client) else {
            return Ok(());
        };
        let client = unsafe { &mut *client };
        let Some(server_ptr) = client.server_ptr else {
            return self.refresh_player_metadata_by_entity_id(completion.entity_id);
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        player.finish_item_use(
            completion.hand,
            completion.item_stack,
            completion.duration,
            server,
            client,
        )?;
        self.refresh_player_metadata_by_entity_id(completion.entity_id)
    }

    fn refresh_player_metadata_by_entity_id(&mut self, entity_id: i32) -> Result<()> {
        let Some(player) = self.entities.iter_mut().find_map(|entity| match entity {
            Entity::Player(player) if player.entity_id().value() == entity_id => Some(player),
            _ => None,
        }) else {
            return Ok(());
        };
        let changed_player_id = player.entity_id();
        let Some(metadata_packet) = player.dirty_metadata_packet() else {
            return Ok(());
        };
        self.broadcast_player_metadata(
            changed_player_id,
            metadata_packet.entity_id,
            metadata_packet.entries.0,
        )
    }

    fn broadcast_entity_status(&mut self, entity_id: i32, status: i8) -> Result<()> {
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.has_entered_world() => Some(player),
                _ => None,
            })
            .filter_map(Player::client_mut)
            .try_for_each(|viewer_client| {
                EntityStatusPacket { entity_id, status }.dispatch(viewer_client)
            })
    }

    pub(crate) fn remove_entity_by_addr(&mut self, addr: &SocketAddr) -> Result<()> {
        let removed_players = self
            .entities
            .iter()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.addr == *addr => {
                    Some((player.entity_id(), player.uuid))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.addr != *addr && player.has_entered_world() => {
                    Some(player)
                }
                _ => None,
            })
            .filter_map(Player::client_mut)
            .try_for_each(|viewer_client| {
                removed_players.iter().try_for_each(|(entity_id, uuid)| {
                    PlayerInfoRemovePacket::new(*uuid).dispatch(viewer_client)?;
                    RemoveEntitiesPacket::new(vec![entity_id.value()]).dispatch(viewer_client)
                })
            })?;
        self.entities.retain(|entity| match entity {
            Entity::Creature(_) => true,
            Entity::ExperienceOrb(_) => true,
            Entity::Generic(_) => true,
            Entity::Item(_) => true,
            Entity::Player(player) => player.addr != *addr,
            Entity::Projectile(_) => true,
        });
        Ok(())
    }

    pub(crate) fn player_by_addr_mut(&mut self, addr: &SocketAddr) -> Option<&mut Player> {
        self.entities.iter_mut().find_map(|entity| match entity {
            Entity::Creature(_) => None,
            Entity::ExperienceOrb(_) => None,
            Entity::Generic(_) => None,
            Entity::Item(_) => None,
            Entity::Player(player) if player.addr == *addr => Some(player),
            Entity::Player(_) => None,
            Entity::Projectile(_) => None,
        })
    }

    pub(crate) fn player_by_addr(&self, addr: &SocketAddr) -> Option<&Player> {
        self.entities.iter().find_map(|entity| match entity {
            Entity::Creature(_) => None,
            Entity::ExperienceOrb(_) => None,
            Entity::Generic(_) => None,
            Entity::Item(_) => None,
            Entity::Player(player) if player.addr == *addr => Some(player),
            Entity::Player(_) => None,
            Entity::Projectile(_) => None,
        })
    }

    pub(crate) fn player_pointer_by_addr(&mut self, addr: &SocketAddr) -> Option<*mut Player> {
        self.player_by_addr_mut(addr)
            .map(|player| player as *mut Player)
    }

    pub(crate) fn send_player_remove_to_viewers(
        &mut self,
        player_id: EntityId,
        _player_uuid: Uuid,
    ) -> Result<()> {
        self.hide_entity_from_all_viewers(player_id)
    }

    pub(crate) fn synchronize_player_visibility(&mut self, client: &mut Client) -> Result<()> {
        let Some(joining_player_id) = self.player_by_addr(&client.addr).map(Player::entity_id)
        else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        self.refresh_visibility_for_entity(joining_player_id)
    }

    pub(crate) fn dispatch_player_spawn(
        &mut self,
        player_uuid: Uuid,
        first_spawn: bool,
        client: &mut Client,
    ) {
        let Some(player) = self.entities.iter_mut().find_map(|entity| match entity {
            Entity::Player(player) if player.uuid() == player_uuid => Some(player),
            _ => None,
        }) else {
            return;
        };
        dispatch_player_spawn_event(
            player as *mut Player,
            self as *mut World,
            first_spawn,
            client,
        );
    }

    pub fn block_at(&mut self, position: BlockPosition) -> Result<Block> {
        self.block_state_at(position).map(BlockState::block)
    }

    pub fn block_instance_at(&mut self, position: BlockPosition) -> Result<BlockInstance> {
        self.block_instance_at_with_condition(position, BlockLookupCondition::None)?
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Block instance was not found"))
    }

    pub fn block_state_at(&mut self, position: BlockPosition) -> Result<BlockState> {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        Ok(self.load_chunk(chunk_position)?.block_state(position))
    }

    pub fn block_at_with_condition(
        &mut self,
        position: BlockPosition,
        condition: BlockLookupCondition,
    ) -> Result<Option<Block>> {
        self.block_instance_at_with_condition(position, condition)
            .map(|block_instance| block_instance.map(|block_instance| block_instance.block()))
    }

    pub fn block_instance_at_with_condition(
        &mut self,
        position: BlockPosition,
        condition: BlockLookupCondition,
    ) -> Result<Option<BlockInstance>> {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        match condition {
            BlockLookupCondition::None => Ok(self
                .load_chunk(chunk_position)?
                .block_instance_with_condition(position, condition)),
            BlockLookupCondition::Cached | BlockLookupCondition::Type => Ok(self
                .chunks
                .get(&chunk_position)
                .filter(|chunk| chunk.is_loaded())
                .and_then(|chunk| chunk.block_instance_with_condition(position, condition))),
        }
    }

    pub fn biome_at(&mut self, position: BlockPosition) -> Result<RegistryKey<Biome>> {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        Ok(self.load_chunk(chunk_position)?.biome(position))
    }

    pub fn set_biome(
        &mut self,
        position: BlockPosition,
        biome: RegistryKey<Biome>,
    ) -> Result<bool> {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        let biome_was_set = self.load_chunk(chunk_position)?.set_biome(position, biome);
        if biome_was_set {
            self.refresh_last_block_change_time();
        }
        Ok(biome_was_set)
    }

    pub fn set_block(&mut self, position: BlockPosition, block: Block) -> Result<bool> {
        self.set_block_instance(position, block.into())
    }

    pub fn set_block_instance(
        &mut self,
        position: BlockPosition,
        block_instance: BlockInstance,
    ) -> Result<bool> {
        self.set_block_instance_with_handler(position, block_instance, None, None)
    }

    pub fn set_block_state(
        &mut self,
        position: BlockPosition,
        block_state: BlockState,
    ) -> Result<bool> {
        self.set_block_instance(position, block_state.into())
    }

    fn set_block_instance_with_handler(
        &mut self,
        position: BlockPosition,
        block_instance: BlockInstance,
        placement: Option<BlockHandlerPlacement>,
        destroy: Option<BlockHandlerDestroy>,
    ) -> Result<bool> {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        self.load_chunk(chunk_position)?;
        self.set_loaded_block_instance_with_handler(
            position,
            block_instance,
            placement,
            destroy,
            true,
            0,
        )
    }

    fn set_loaded_block_with_handler(
        &mut self,
        position: BlockPosition,
        block: Block,
        placement: Option<BlockHandlerPlacement>,
        destroy: Option<BlockHandlerDestroy>,
        do_block_updates: bool,
        update_distance: i32,
    ) -> Result<bool> {
        self.set_loaded_block_instance_with_handler(
            position,
            block.into(),
            placement,
            destroy,
            do_block_updates,
            update_distance,
        )
    }

    fn set_loaded_block_instance_with_handler(
        &mut self,
        position: BlockPosition,
        block_instance: BlockInstance,
        placement: Option<BlockHandlerPlacement>,
        destroy: Option<BlockHandlerDestroy>,
        do_block_updates: bool,
        update_distance: i32,
    ) -> Result<bool> {
        let block_state = self.block_state_after_placement_rule(
            block_instance.block_state(),
            position,
            placement.as_ref(),
            do_block_updates,
        );
        self.set_loaded_block_state_with_handler(
            position,
            block_instance.with_block_state(block_state),
            placement,
            destroy,
            do_block_updates,
            update_distance,
        )
    }

    fn set_loaded_block_state_with_handler(
        &mut self,
        position: BlockPosition,
        block_instance: impl Into<BlockInstance>,
        placement: Option<BlockHandlerPlacement>,
        destroy: Option<BlockHandlerDestroy>,
        do_block_updates: bool,
        update_distance: i32,
    ) -> Result<bool> {
        let block_instance = block_instance.into();
        let block_state = block_instance.block_state();
        let block = block_state.block();
        if self
            .currently_changing_blocks
            .get(&position)
            .is_some_and(|changed_block| *changed_block == block_state)
        {
            return Ok(false);
        }
        self.currently_changing_blocks.insert(position, block_state);
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        let Some(mut chunk) = self.chunks.remove(&chunk_position) else {
            return Ok(false);
        };
        let block_was_set = chunk
            .try_set_block_instance_with_handler(
                position,
                block_instance,
                Some(&self.block_handlers),
                placement,
                destroy,
            )
            .block_was_set();
        self.chunks.insert(chunk_position, chunk);
        if !block_was_set {
            return Ok(false);
        }
        self.refresh_last_block_change_time();
        if do_block_updates {
            self.execute_neighbor_block_placement_rules(position, update_distance)?;
        }
        self.broadcast_block_update(position, block_state)?;
        self.broadcast_block_entity_update(position)?;
        self.invalidate_neighbor_chunk_lighting(position);
        self.dispatch_instance_block_update_event(position, block);
        Ok(true)
    }

    fn invalidate_neighbor_chunk_lighting(&mut self, position: BlockPosition) {
        let Some(changed_chunk) = self.chunks.get(&ChunkPosition::from(position)) else {
            return;
        };
        if !changed_chunk.is_lighting_chunk() || changed_chunk.is_lighting_invalidation_frozen() {
            return;
        }
        let section_y = position.y.div_euclid(16);
        affected_lighting_chunk_positions(position)
            .into_iter()
            .for_each(|chunk_position| {
                if let Some(chunk) = self.chunks.get_mut(&chunk_position) {
                    ((section_y - 1)..=(section_y + 1)).for_each(|section_y| {
                        chunk.invalidate_section(section_y);
                    });
                    chunk.schedule_lighting_update();
                }
            });
    }

    fn invalidate_generated_chunk_lighting(&mut self, position: ChunkPosition) {
        if self
            .chunks
            .get(&position)
            .is_none_or(|chunk| !chunk.is_lighting_chunk())
        {
            return;
        }
        (-1..=1)
            .flat_map(|x_offset| {
                (-1..=1).map(move |z_offset| {
                    ChunkPosition::new(position.x + x_offset, position.z + z_offset)
                })
            })
            .for_each(|chunk_position| {
                if let Some(chunk) = self.chunks.get_mut(&chunk_position) {
                    (chunk.min_section()..chunk.max_section()).for_each(|section_y| {
                        chunk.invalidate_section(section_y);
                    });
                    chunk.schedule_generated_lighting_update();
                }
            });
    }

    fn block_state_after_placement_rule(
        &self,
        block_state: BlockState,
        position: BlockPosition,
        placement: Option<&BlockHandlerPlacement>,
        do_block_updates: bool,
    ) -> BlockState {
        if !do_block_updates {
            return block_state;
        }
        let Some(placement) = placement else {
            return block_state;
        };
        let block = block_state.block();
        let Some(rule) = self.block_placement_rules.rule(block) else {
            return block_state;
        };
        let player = placement
            .player()
            .and_then(|player_id| self.entity_by_id(player_id))
            .and_then(|entity| match entity {
                Entity::Player(player) => Some(player),
                Entity::Creature(_) => None,
                Entity::ExperienceOrb(_) => None,
                Entity::Generic(_) => None,
                Entity::Item(_) => None,
                Entity::Projectile(_) => None,
            });
        let result_block = rule.block_place(BlockPlacementState::new(
            block,
            placement.block_face(),
            position,
            placement.cursor_position(),
            player.map(Player::position),
            placement.player(),
            placement.hand(),
            player.is_some_and(Player::is_sneaking),
        ));
        match result_block {
            Some(result_block) if result_block == block => block_state,
            Some(result_block) => result_block.default_state(),
            None => Block::AIR.default_state(),
        }
    }

    fn execute_neighbor_block_placement_rules(
        &mut self,
        position: BlockPosition,
        update_distance: i32,
    ) -> Result<()> {
        crate::events::player_block_interact::BlockFace::update_faces()
            .into_iter()
            .try_for_each(|update_face| {
                let (normal_x, normal_y, normal_z) = update_face.normal();
                let neighbor_position = BlockPosition::new(
                    position.x + normal_x,
                    position.y + normal_y,
                    position.z + normal_z,
                );
                self.update_neighbor_block_from_rule(
                    neighbor_position,
                    update_face.opposite(),
                    update_distance,
                )
            })
    }

    fn update_neighbor_block_from_rule(
        &mut self,
        neighbor_position: BlockPosition,
        from_face: crate::events::player_block_interact::BlockFace,
        update_distance: i32,
    ) -> Result<()> {
        let Some(neighbor_block) = self.loaded_block_at(neighbor_position) else {
            return Ok(());
        };
        if block_is_air(neighbor_block) {
            return Ok(());
        }
        let Some(rule) = self.block_placement_rules.rule(neighbor_block) else {
            return Ok(());
        };
        if update_distance >= rule.max_update_distance() {
            return Ok(());
        }
        let new_neighbor_block = rule.block_update(BlockUpdateState::new(
            neighbor_position,
            neighbor_block,
            from_face,
        ));
        if neighbor_block == new_neighbor_block {
            return Ok(());
        }
        let chunk_position = ChunkPosition::new(
            neighbor_position.x.div_euclid(16),
            neighbor_position.z.div_euclid(16),
        );
        if !self.is_chunk_loaded(chunk_position) {
            return Ok(());
        }
        self.set_loaded_block_with_handler(
            neighbor_position,
            new_neighbor_block,
            None,
            None,
            true,
            update_distance + 1,
        )?;
        Ok(())
    }

    pub fn loaded_block_at(&self, position: BlockPosition) -> Option<Block> {
        self.loaded_block_state_at(position).map(BlockState::block)
    }

    pub fn loaded_block_instance_at(&self, position: BlockPosition) -> Option<BlockInstance> {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        self.chunks
            .get(&chunk_position)
            .filter(|chunk| chunk.is_loaded())
            .and_then(|chunk| {
                chunk.block_instance_with_condition(position, BlockLookupCondition::None)
            })
    }

    pub fn loaded_block_state_at(&self, position: BlockPosition) -> Option<BlockState> {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        self.chunks
            .get(&chunk_position)
            .filter(|chunk| chunk.is_loaded())
            .map(|chunk| chunk.block_state(position))
    }

    pub fn client_block_entity_nbt_at(&self, position: BlockPosition) -> Option<NbtCompound> {
        let block_instance = self.loaded_block_instance_at(position)?;
        self.client_block_entity_nbt(position, &block_instance)
    }

    pub fn target_block_position(
        &self,
        entity_id: EntityId,
        max_distance: i32,
    ) -> Option<BlockPosition> {
        self.line_of_sight(entity_id, max_distance)
            .into_iter()
            .next()
    }

    pub fn line_of_sight(&self, entity_id: EntityId, max_distance: i32) -> Vec<BlockPosition> {
        let Some(entity) = self.entity_by_id(entity_id) else {
            return Vec::new();
        };
        let eye_position = entity_eye_position(entity);
        let direction = view_direction(entity.position());
        self.ray_positions(eye_position, direction, max_distance as f64)
            .into_iter()
            .filter(|position| {
                self.loaded_block_at(*position)
                    .is_some_and(block_is_sight_block)
            })
            .fold(Vec::new(), |mut positions, position| {
                if positions.last() != Some(&position) {
                    positions.push(position);
                }
                positions
            })
    }

    pub fn has_line_of_sight(&self, source_id: EntityId, target_id: EntityId) -> bool {
        self.has_exact_line_of_sight(source_id, target_id, false)
    }

    pub fn has_exact_line_of_sight(
        &self,
        source_id: EntityId,
        target_id: EntityId,
        exact_view: bool,
    ) -> bool {
        let Some(source) = self.entity_by_id(source_id) else {
            return false;
        };
        let Some(target) = self.entity_by_id(target_id) else {
            return false;
        };
        let source_eye_position = entity_eye_position(source);
        let target_eye_position = entity_eye_position(target);
        let target_direction = normalized_vector_between(source_eye_position, target_eye_position);
        if exact_view && !vectors_are_aligned(view_direction(source.position()), target_direction) {
            return false;
        }
        !self
            .ray_positions(
                source_eye_position,
                target_direction,
                vector_distance(source_eye_position, target_eye_position),
            )
            .into_iter()
            .any(|position| {
                self.loaded_block_at(position)
                    .is_some_and(block_is_sight_block)
            })
    }

    pub fn line_of_sight_entity(
        &self,
        entity_id: EntityId,
        range: f64,
        predicate: impl Fn(&Entity) -> bool,
    ) -> Option<&Entity> {
        let source = self.entity_by_id(entity_id)?;
        let source_eye_position = entity_eye_position(source);
        let direction = view_direction(source.position());
        self.entities
            .iter()
            .filter(|entity| entity.entity_id() != entity_id)
            .filter(|entity| predicate(entity))
            .filter_map(|entity| {
                let target_eye_position = entity_eye_position(entity);
                let distance = vector_distance(source_eye_position, target_eye_position);
                if distance > range {
                    return None;
                }
                if !ray_reaches_entity(source_eye_position, direction, entity) {
                    return None;
                }
                if !self.has_exact_line_of_sight(entity_id, entity.entity_id(), false) {
                    return None;
                }
                Some((distance, entity))
            })
            .min_by(|(first_distance, _), (second_distance, _)| {
                first_distance.total_cmp(second_distance)
            })
            .map(|(_, entity)| entity)
    }

    fn ray_positions(
        &self,
        start: Vector3d,
        direction: Vector3d,
        max_distance: f64,
    ) -> Vec<BlockPosition> {
        let step_count = (max_distance.max(0.0) * 4.0).ceil() as i32;
        (0..=step_count)
            .map(|step| step as f64 * 0.25)
            .map(|distance| {
                BlockPosition::new(
                    (start.x + direction.x * distance).floor() as i32,
                    (start.y + direction.y * distance).floor() as i32,
                    (start.z + direction.z * distance).floor() as i32,
                )
            })
            .collect()
    }

    pub fn block_light(&mut self, position: BlockPosition) -> u8 {
        let chunk_position = ChunkPosition::from(position);
        let requested_chunk_uses_world_lighting = self
            .chunks
            .get(&chunk_position)
            .is_some_and(Chunk::is_lighting_chunk);
        if requested_chunk_uses_world_lighting
            && self.chunks.values().any(Chunk::lighting_is_invalidated)
        {
            WorldLighting::relight(
                &mut self.chunks,
                self.cached_dimension_type.has_skylight,
                None,
            );
        } else if let Some(chunk) = self.chunks.get_mut(&chunk_position) {
            chunk.relight_block_light_at(position.y);
        }
        self.chunks
            .get(&chunk_position)
            .filter(|chunk| chunk.is_loaded())
            .map(|chunk| chunk.block_light(position))
            .unwrap_or_default()
    }

    pub fn sky_light(&mut self, position: BlockPosition) -> u8 {
        let chunk_position = ChunkPosition::from(position);
        let requested_chunk_uses_world_lighting = self
            .chunks
            .get(&chunk_position)
            .is_some_and(Chunk::is_lighting_chunk);
        if requested_chunk_uses_world_lighting
            && self.chunks.values().any(Chunk::lighting_is_invalidated)
        {
            WorldLighting::relight(
                &mut self.chunks,
                self.cached_dimension_type.has_skylight,
                None,
            );
        } else if let Some(chunk) = self.chunks.get_mut(&chunk_position) {
            chunk.relight_sky_light_at(position.y);
        }
        self.chunks
            .get(&chunk_position)
            .filter(|chunk| chunk.is_loaded())
            .map(|chunk| chunk.sky_light(position))
            .unwrap_or_default()
    }

    pub fn relight_chunks(&mut self, positions: &[ChunkPosition]) -> Vec<ChunkPosition> {
        let has_loaded_requested_chunk = positions.iter().any(|position| {
            self.chunks
                .get(position)
                .is_some_and(|chunk| chunk.is_loaded() && chunk.is_lighting_chunk())
        });
        if !has_loaded_requested_chunk {
            return Vec::new();
        }
        WorldLighting::relight(
            &mut self.chunks,
            self.cached_dimension_type.has_skylight,
            Some(positions),
        )
    }

    pub fn invalidate_section(&mut self, section_x: i32, section_y: i32, section_z: i32) -> bool {
        let position = ChunkPosition::new(section_x, section_z);
        let Some(chunk) = self.chunks.get_mut(&position) else {
            return false;
        };
        if !chunk.invalidate_section(section_y) {
            return false;
        }
        self.dispatch_instance_section_invalidate_event(section_x, section_y, section_z);
        true
    }

    pub fn block_position_is_loaded(&self, position: BlockPosition) -> bool {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        self.is_chunk_loaded(chunk_position)
    }

    pub fn send_block_action(
        &mut self,
        position: BlockPosition,
        action_id: u8,
        action_param: u8,
    ) -> Result<()> {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        let Some(chunk) = self
            .chunks
            .get(&chunk_position)
            .filter(|chunk| chunk.is_loaded())
        else {
            return Err(Error::new(ErrorKind::NotFound, "Chunk is not loaded"));
        };
        let block = chunk.block(position);
        let packet = BlockActionPacket::new(
            Position {
                x: position.x,
                y: position.y,
                z: position.z,
            },
            action_id,
            action_param,
            block.state_id(),
        );
        self.dispatch_packet_to_chunk_viewers(chunk_position, packet)
    }

    pub fn play_sound_except(
        &mut self,
        excluded_player: Option<Uuid>,
        sound_event: SoundEvent,
        source_id: i32,
        position: EntityPosition,
        volume: f32,
        pitch: f32,
        seed: i64,
    ) -> Result<()> {
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.has_entered_world() => Some(player),
                _ => None,
            })
            .filter(|player| Some(player.uuid()) != excluded_player)
            .filter_map(Player::client_mut)
            .try_for_each(|client| {
                SoundEffectPacket {
                    sound_event: NetworkPositionedSoundEvent(sound_event.clone()),
                    source_id,
                    position: Vector3d {
                        x: position.x(),
                        y: position.y(),
                        z: position.z(),
                    },
                    volume,
                    pitch,
                    seed,
                }
                .dispatch(client)
            })
    }

    pub fn play_sound_except_emitter(
        &mut self,
        excluded_player: Option<Uuid>,
        sound_event: SoundEvent,
        source_id: i32,
        emitter: WorldSoundEmitter,
        volume: f32,
        pitch: f32,
        seed: i64,
    ) -> Result<()> {
        match emitter {
            WorldSoundEmitter::Entity(entity_id) => self.play_entity_sound_except(
                excluded_player,
                sound_event,
                source_id,
                entity_id,
                volume,
                pitch,
                seed,
            ),
            WorldSoundEmitter::SelfPlayer => self.play_self_emitter_sound_except(
                excluded_player,
                sound_event,
                source_id,
                volume,
                pitch,
                seed,
            ),
        }
    }

    fn play_entity_sound_except(
        &mut self,
        excluded_player: Option<Uuid>,
        sound_event: SoundEvent,
        source_id: i32,
        entity_id: EntityId,
        volume: f32,
        pitch: f32,
        seed: i64,
    ) -> Result<()> {
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.has_entered_world() => Some(player),
                _ => None,
            })
            .filter(|player| Some(player.uuid()) != excluded_player)
            .filter_map(Player::client_mut)
            .try_for_each(|client| {
                EntitySoundEffectPacket {
                    sound_event: NetworkSoundEvent(sound_event.clone()),
                    source_id,
                    entity_id: entity_id.value() as i32,
                    volume,
                    pitch,
                    seed,
                }
                .dispatch(client)
            })
    }

    fn play_self_emitter_sound_except(
        &mut self,
        excluded_player: Option<Uuid>,
        sound_event: SoundEvent,
        source_id: i32,
        volume: f32,
        pitch: f32,
        seed: i64,
    ) -> Result<()> {
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.has_entered_world() => Some(player),
                _ => None,
            })
            .filter(|player| Some(player.uuid()) != excluded_player)
            .try_for_each(|player| {
                let entity_id = player.entity_id().value() as i32;
                let Some(client) = player.client_mut() else {
                    return Ok(());
                };
                EntitySoundEffectPacket {
                    sound_event: NetworkSoundEvent(sound_event.clone()),
                    source_id,
                    entity_id,
                    volume,
                    pitch,
                    seed,
                }
                .dispatch(client)
            })
    }

    pub fn send_chunk_to_viewers(
        &mut self,
        position: ChunkPosition,
        _registries: &Registries,
    ) -> Result<()> {
        self.queue_chunk_for_viewers(position);
        Ok(())
    }

    fn queue_chunk_for_viewers(&mut self, position: ChunkPosition) {
        let Some(chunk) = self.chunks.get(&position).filter(|chunk| chunk.is_loaded()) else {
            return;
        };
        let viewer_ids = chunk.viewers().collect::<HashSet<_>>();
        if viewer_ids.is_empty() {
            return;
        }
        let player_chunk = PlayerChunk::new(position.x, position.z);
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if viewer_ids.contains(&player.entity_id().value()) => {
                    Some(player)
                }
                _ => None,
            })
            .for_each(|player| {
                player.send_loaded_chunk_position(player_chunk);
            });
    }

    fn dispatch_packet_to_chunk_viewers<P>(
        &mut self,
        position: ChunkPosition,
        packet: P,
    ) -> Result<()>
    where
        P: DataType + PacketStruct,
    {
        let Some(chunk) = self.chunks.get(&position) else {
            return Ok(());
        };
        let viewer_ids = chunk.viewers().collect::<HashSet<_>>();
        let mut payload = Vec::new();
        packet.encode(&mut payload)?;
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if viewer_ids.contains(&player.entity_id().value()) => {
                    Some(player)
                }
                _ => None,
            })
            .try_for_each(|player| {
                let Some(client) = player.client_mut() else {
                    return Ok(());
                };
                client.send_packet(P::get_id(), &payload)
            })
    }

    fn dispatch_packet_to_entered_players<P>(&mut self, packet: P) -> Result<()>
    where
        P: DataType + PacketStruct,
    {
        self.dispatch_packet_to_players(packet)
    }

    pub(crate) fn dispatch_packet_to_players<P>(&mut self, packet: P) -> Result<()>
    where
        P: DataType + PacketStruct,
    {
        let mut payload = Vec::new();
        packet.encode(&mut payload)?;
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.has_entered_world() => Some(player),
                _ => None,
            })
            .filter_map(Player::client_mut)
            .try_for_each(|client| client.send_packet(P::get_id(), &payload))
    }

    pub(crate) fn refresh_block_for_player(
        &mut self,
        client: &mut Client,
        position: BlockPosition,
    ) -> Result<()> {
        let block_state = self.block_state_at(position)?;
        BlockUpdatePacket::new(
            Position {
                x: position.x,
                y: position.y,
                z: position.z,
            },
            block_state.state_id(),
        )
        .dispatch(client)
    }

    pub(crate) fn refresh_block_entity_for_player(
        &mut self,
        client: &mut Client,
        position: BlockPosition,
    ) -> Result<()> {
        let Some(block_instance) = self.loaded_block_instance_at(position) else {
            return Ok(());
        };
        let Some(block_entity_type) = block_instance.block().block_entity_type() else {
            return Ok(());
        };
        BlockEntityDataPacket::new(
            Position {
                x: position.x,
                y: position.y,
                z: position.z,
            },
            block_entity_type,
            self.client_block_entity_nbt(position, &block_instance),
        )
        .dispatch(client)
    }

    pub fn place_block(&mut self, placement: BlockHandlerPlacement) -> bool {
        self.place_block_with_updates(placement, true)
    }

    pub fn place_block_with_updates(
        &mut self,
        placement: BlockHandlerPlacement,
        do_block_updates: bool,
    ) -> bool {
        let chunk_position = ChunkPosition::new(
            placement.block_position().x.div_euclid(16),
            placement.block_position().z.div_euclid(16),
        );
        if !self.is_chunk_loaded(chunk_position) {
            return false;
        }
        let block_state = self.block_state_after_placement_rule(
            placement.block_state(),
            placement.block_position(),
            Some(&placement),
            do_block_updates,
        );
        self.set_loaded_block_state_with_handler(
            placement.block_position(),
            block_state,
            Some(placement),
            None,
            do_block_updates,
            0,
        )
        .unwrap_or(false)
    }

    pub fn break_block(
        &mut self,
        player_id: EntityId,
        position: BlockPosition,
        block_face: crate::events::player_block_interact::BlockFace,
    ) -> bool {
        self.break_block_with_updates(player_id, position, block_face, true)
    }

    pub fn break_block_with_updates(
        &mut self,
        player_id: EntityId,
        position: BlockPosition,
        block_face: crate::events::player_block_interact::BlockFace,
        do_block_updates: bool,
    ) -> bool {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        let Some(chunk) = self.chunks.get(&chunk_position) else {
            return false;
        };
        if chunk.is_read_only() || !chunk.is_loaded() {
            return false;
        }
        let Some(block) = self.loaded_block_at(position) else {
            return false;
        };
        if block == Block::AIR {
            self.send_loaded_chunk_to_player(player_id, chunk_position);
            return false;
        }
        let Some(player) = self.player_pointer_for_block_break(player_id) else {
            return false;
        };
        let Some(result_block) =
            self.dispatch_player_block_break_event(player, block, position, block_face)
        else {
            return false;
        };
        let destroy =
            BlockHandlerDestroy::new(block, result_block, self.uuid, position, Some(player_id));
        let block_was_broken = self
            .set_loaded_block_with_handler(
                position,
                result_block,
                None,
                Some(destroy),
                do_block_updates,
                0,
            )
            .unwrap_or(false);
        if !block_was_broken {
            return false;
        }
        if do_block_updates {
            let _ =
                self.dispatch_block_break_effect_except(chunk_position, position, block, player_id);
        }
        true
    }

    pub fn interact_block_handler(
        &self,
        player_id: EntityId,
        hand: crate::entity::PlayerHand,
        block_face: crate::events::player_block_interact::BlockFace,
        position: BlockPosition,
        cursor_position: (f32, f32, f32),
    ) -> bool {
        let Some(block_instance) = self.loaded_block_instance_at(position) else {
            return true;
        };
        let Some(handler) = block_instance.handler() else {
            return true;
        };
        handler.on_interact(BlockHandlerInteraction::new(
            block_instance.block(),
            self.uuid,
            block_face,
            position,
            EntityPosition::new(
                f64::from(cursor_position.0),
                f64::from(cursor_position.1),
                f64::from(cursor_position.2),
                0.0,
                0.0,
            ),
            player_id,
            hand,
        ))
    }

    fn player_pointer_for_block_break(&mut self, player_id: EntityId) -> Option<*mut Player> {
        self.entities.iter_mut().find_map(|entity| match entity {
            Entity::Player(player) if player.entity_id() == player_id => {
                Some(player as *mut Player)
            }
            _ => None,
        })
    }

    fn dispatch_player_block_break_event(
        &mut self,
        player: *mut Player,
        block: Block,
        position: BlockPosition,
        block_face: crate::events::player_block_interact::BlockFace,
    ) -> Option<Block> {
        let Some(server_ptr) = self.event_dispatcher else {
            return Some(Block::AIR);
        };
        let Some(client) = (unsafe { &mut *player })
            .client_mut()
            .map(|client| client as *mut Client)
        else {
            return Some(Block::AIR);
        };
        let mut event = PlayerBlockBreakEvent::new(player, block, Block::AIR, position, block_face);
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let client = unsafe { &mut *client };
        event.dispatch(server, client);
        if event.is_cancelled() {
            return None;
        }
        Some(event.result_block())
    }

    fn send_loaded_chunk_to_player(&mut self, player_id: EntityId, position: ChunkPosition) {
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.entity_id() == player_id => Some(player),
                _ => None,
            })
            .for_each(|player| {
                player.send_loaded_chunk_position(PlayerChunk::new(position.x, position.z));
            });
    }

    fn dispatch_block_break_effect_except(
        &mut self,
        chunk_position: ChunkPosition,
        position: BlockPosition,
        block: Block,
        excluded_player: EntityId,
    ) -> Result<()> {
        let packet = WorldEventPacket::new(
            DESTROY_BLOCK_WORLD_EVENT_ID,
            Position {
                x: position.x,
                y: position.y,
                z: position.z,
            },
            block.state_id(),
            false,
        );
        self.dispatch_packet_to_chunk_viewers_except(chunk_position, packet, excluded_player)
    }

    fn dispatch_packet_to_chunk_viewers_except<P>(
        &mut self,
        position: ChunkPosition,
        packet: P,
        excluded_player: EntityId,
    ) -> Result<()>
    where
        P: DataType + PacketStruct,
    {
        let Some(chunk) = self.chunks.get(&position) else {
            return Ok(());
        };
        let viewer_ids = chunk.viewers().collect::<HashSet<_>>();
        let mut payload = Vec::new();
        packet.encode(&mut payload)?;
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player)
                    if player.entity_id() != excluded_player
                        && viewer_ids.contains(&player.entity_id().value()) =>
                {
                    Some(player)
                }
                _ => None,
            })
            .filter_map(Player::client_mut)
            .try_for_each(|client| client.send_packet(P::get_id(), &payload))
    }

    fn broadcast_block_update(
        &mut self,
        position: BlockPosition,
        block_state: BlockState,
    ) -> Result<()> {
        let block_position = Position {
            x: position.x,
            y: position.y,
            z: position.z,
        };
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.has_entered_world() => Some(player),
                _ => None,
            })
            .filter_map(Player::client_mut)
            .try_for_each(|viewer_client| {
                BlockUpdatePacket::new(block_position, block_state.state_id())
                    .dispatch(viewer_client)
            })
    }

    fn broadcast_block_entity_update(&mut self, position: BlockPosition) -> Result<()> {
        let Some(block_instance) = self.loaded_block_instance_at(position) else {
            return Ok(());
        };
        let Some(block_entity_type) = block_instance.block().block_entity_type() else {
            return Ok(());
        };
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        let block_entity_nbt = self.client_block_entity_nbt(position, &block_instance);
        let packet = BlockEntityDataPacket::new(
            Position {
                x: position.x,
                y: position.y,
                z: position.z,
            },
            block_entity_type,
            block_entity_nbt,
        );
        self.dispatch_packet_to_chunk_viewers(chunk_position, packet)
    }

    fn client_block_entity_nbt(
        &self,
        _position: BlockPosition,
        block_instance: &BlockInstance,
    ) -> Option<NbtCompound> {
        block_instance.block().block_entity_type()?;
        let block_entity_nbt = block_instance.nbt_or_empty();
        let Some(handler) = block_instance.handler() else {
            return Some(block_entity_nbt);
        };
        let tags = handler.block_entity_tags();
        if tags.is_empty() {
            return Some(NbtCompound::new());
        }
        let mut filtered_nbt = NbtCompound::new();
        tags.into_iter().for_each(|tag| {
            tag.write(&mut filtered_nbt, tag.read(&block_entity_nbt));
        });
        Some(filtered_nbt)
    }

    fn apply_generator(&mut self, chunk: &mut Chunk) -> Result<()> {
        if !chunk.requires_generation_completion() {
            return Ok(());
        }
        let generation_result = if chunk.requires_generation() {
            let Some(generator) = self.generator.take() else {
                self.apply_pending_generation(chunk);
                chunk.on_generate();
                return Ok(());
            };
            let generation_result = self.apply_generation(chunk, generator.as_ref());
            self.generator = Some(generator);
            generation_result
        } else {
            self.apply_pending_generation(chunk);
            Ok(())
        };
        chunk.on_generate();
        generation_result
    }

    fn apply_generation(
        &mut self,
        chunk: &mut Chunk,
        generator: &(dyn Generator + Send + Sync),
    ) -> Result<()> {
        generate_chunk(chunk, generator)?
            .into_iter()
            .for_each(|fork| self.store_generation_fork(fork));
        self.apply_pending_generation(chunk);
        Ok(())
    }

    fn finish_new_chunk_generation(&mut self, position: ChunkPosition) -> Result<bool> {
        let Some(mut chunk) = self.chunks.remove(&position) else {
            return Ok(false);
        };
        let chunk_will_generate = chunk.requires_generation_completion();
        let generation_result = self.apply_generator(&mut chunk);
        self.chunks.insert(position, chunk);
        if chunk_will_generate {
            self.invalidate_generated_chunk_lighting(position);
        }
        generation_result.map(|_| true)
    }

    fn schedule_player_chunk_loads(
        &mut self,
        player_address: SocketAddr,
        chunks: &[PlayerChunk],
    ) -> Result<()> {
        for chunk in chunks {
            let position = ChunkPosition::from(*chunk);
            if self.is_chunk_loaded(position) {
                self.queue_loaded_chunk_for_player(player_address, *chunk);
                continue;
            }
            let Some(ticket) = self.load_optional_chunk_future(position)? else {
                continue;
            };
            self.player_chunk_load_waiters
                .entry(position)
                .or_default()
                .push(player_address);
            let _ = self.complete_chunk_load(&ticket)?;
        }
        Ok(())
    }

    fn schedule_entity_visibility_refresh(&mut self, entity_id: EntityId) {
        if self
            .pending_entity_visibility_refresh_keys
            .insert(entity_id)
        {
            self.pending_entity_visibility_refreshes
                .push_back(entity_id);
        }
    }

    pub(crate) fn process_pending_entity_visibility_refreshes(&mut self) -> Result<()> {
        let mut pending_entity_ids = VecDeque::new();
        std::mem::swap(
            &mut pending_entity_ids,
            &mut self.pending_entity_visibility_refreshes,
        );
        self.pending_entity_visibility_refresh_keys.clear();
        while let Some(entity_id) = pending_entity_ids.pop_front() {
            self.refresh_visibility_for_entity(entity_id)?;
        }
        Ok(())
    }

    pub(crate) fn process_completed_chunk_loads(&mut self) -> Result<()> {
        self.receive_completed_chunk_loads();
        let completed_tickets = self
            .prepared_chunk_loads
            .values()
            .map(|(ticket, _)| ticket.clone())
            .collect::<Vec<_>>();
        for ticket in completed_tickets {
            self.complete_chunk_load(&ticket)?;
        }
        Ok(())
    }

    fn process_pending_player_entries(&mut self, registries: &Registries) -> Result<()> {
        let pending_entries = self
            .pending_player_entries
            .iter()
            .map(|(player_address, pending_entry)| {
                (*player_address, pending_entry.chunk_load_tickets.clone())
            })
            .collect::<Vec<_>>();
        for (player_address, chunk_load_tickets) in pending_entries {
            let mut all_chunk_loads_are_complete = true;
            for ticket in chunk_load_tickets {
                if !self.complete_chunk_load(&ticket)? {
                    all_chunk_loads_are_complete = false;
                }
            }
            if !all_chunk_loads_are_complete {
                continue;
            }
            let Some(pending_entry) = self.pending_player_entries.remove(&player_address) else {
                continue;
            };
            let client = unsafe { &mut *(pending_entry.client as *mut Client) };
            self.finish_player_entry(
                client,
                pending_entry.ticks_per_second,
                registries,
                pending_entry.chunks,
            )?;
        }
        Ok(())
    }

    fn receive_completed_chunk_loads(&mut self) {
        while let Ok(completed_chunk_load) = self.completed_chunk_load_receiver.try_recv() {
            self.prepared_chunk_loads.insert(
                completed_chunk_load.ticket.id,
                (
                    completed_chunk_load.ticket,
                    completed_chunk_load.prepared_chunk_load,
                ),
            );
        }
    }

    fn queue_waiting_players_for_loaded_chunk(&mut self, position: ChunkPosition) {
        let Some(player_addresses) = self.player_chunk_load_waiters.remove(&position) else {
            return;
        };
        let chunk = PlayerChunk::new(position.x, position.z);
        player_addresses.into_iter().for_each(|player_address| {
            self.queue_loaded_chunk_for_player(player_address, chunk);
        });
    }

    fn queue_loaded_chunk_for_player(&mut self, player_address: SocketAddr, chunk: PlayerChunk) {
        let Some(player) = self.player_by_addr_mut(&player_address) else {
            return;
        };
        player.queue_loaded_chunk(chunk);
    }

    fn loaded_chunk_packet(
        chunks: &mut HashMap<ChunkPosition, Chunk>,
        has_skylight: bool,
        position: ChunkPosition,
        registries: &Registries,
    ) -> Result<Option<ChunkDataAndUpdateLightPacket>> {
        let lighting_requires_update = chunks
            .get(&position)
            .is_some_and(Chunk::lighting_is_invalidated);
        if lighting_requires_update {
            WorldLighting::relight(chunks, has_skylight, Some(&[position]));
        }
        let Some(chunk) = chunks.get(&position) else {
            return Ok(None);
        };
        if !chunk.is_loaded() {
            return Ok(None);
        }
        Ok(Some(ChunkDataAndUpdateLightPacket::with_light_data(
            chunk.x(),
            chunk.z(),
            chunk
                .data(registries)
                .map_err(|_| Error::new(ErrorKind::InvalidData, "Chunk has unregistered biome"))?,
            chunk.light_data(),
        )))
    }

    pub(crate) fn send_pending_chunks_for_client(
        &mut self,
        client: &mut Client,
        registries: &Registries,
    ) -> Result<()> {
        let Some(player) = self.player_pointer_by_addr(&client.addr) else {
            return Ok(());
        };
        let chunks = &mut self.chunks as *mut HashMap<ChunkPosition, Chunk>;
        let has_skylight = self.cached_dimension_type.has_skylight;
        unsafe { &mut *player }.send_pending_chunks_with(client, |queued_chunk| {
            let position = ChunkPosition::from(queued_chunk.chunk);
            Self::loaded_chunk_packet(unsafe { &mut *chunks }, has_skylight, position, registries)
        })
    }

    fn send_pending_chunks_for_player_address(
        &mut self,
        address: SocketAddr,
        registries: &Registries,
    ) -> Result<()> {
        let Some(player) = self.player_pointer_by_addr(&address) else {
            return Ok(());
        };
        let Some(client) = (unsafe { &mut *player }).client_mut() else {
            return Ok(());
        };
        let client = client as *mut Client;
        let chunks = &mut self.chunks as *mut HashMap<ChunkPosition, Chunk>;
        let has_skylight = self.cached_dimension_type.has_skylight;
        unsafe { &mut *player }.send_pending_chunks_with(unsafe { &mut *client }, |queued_chunk| {
            let position = ChunkPosition::from(queued_chunk.chunk);
            Self::loaded_chunk_packet(unsafe { &mut *chunks }, has_skylight, position, registries)
        })
    }

    fn movement_enters_unloaded_chunk(&self, transition: Option<&PlayerChunkTransition>) -> bool {
        let Some(transition) = transition else {
            return false;
        };
        let target_position = ChunkPosition::from(transition.next);
        !self.auto_chunk_load && !self.is_chunk_loaded(target_position)
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

    fn use_client_event_dispatcher(&mut self, client: &Client) {
        if let Some(server_ptr) = client.server_ptr {
            self.event_dispatcher = Some(server_ptr);
        }
    }

    pub(crate) fn use_server_event_dispatcher(&mut self, server_ptr: usize) {
        self.event_dispatcher = Some(server_ptr);
    }

    fn load_chunk_future_with_optional_flag(
        &mut self,
        position: ChunkPosition,
        should_load_missing_chunk: bool,
    ) -> Result<Option<ChunkLoadTicket>> {
        if self.chunks.contains_key(&position) {
            let ticket = self.next_completed_chunk_load_ticket(position);
            return Ok(Some(ticket));
        }
        if !should_load_missing_chunk {
            return Ok(None);
        }
        if let Some(ticket) = self.async_chunk_loads.get(&position).cloned() {
            return Ok(Some(ticket));
        }
        let ticket = self.next_chunk_load_ticket(position);
        let supports_parallel_loading = self.chunk_loader.supports_parallel_loading();
        let chunk_loader = self.chunk_loader.clone();
        let chunk_supplier = self.chunk_supplier.clone();
        let generator = self.generator.clone();
        let synchronously_loaded_chunk = if supports_parallel_loading {
            None
        } else {
            Some(chunk_loader.load_chunk(position)?)
        };
        let completed_chunk_load_sender = self.completed_chunk_load_sender.clone();
        let executor_ticket = ticket.clone();
        ChunkLoadingExecutor::global().execute(move || {
            let prepared_chunk_load = prepare_chunk_load(
                position,
                chunk_loader,
                chunk_supplier,
                generator,
                synchronously_loaded_chunk,
            );
            let _ = completed_chunk_load_sender.send(CompletedChunkLoad {
                ticket: executor_ticket,
                prepared_chunk_load,
            });
        });
        self.async_chunk_loads.insert(position, ticket.clone());
        Ok(Some(ticket))
    }

    fn next_completed_chunk_load_ticket(&mut self, position: ChunkPosition) -> ChunkLoadTicket {
        let ticket = self.next_chunk_load_ticket(position);
        ticket.complete();
        ticket
    }

    fn next_chunk_load_ticket(&mut self, position: ChunkPosition) -> ChunkLoadTicket {
        self.next_chunk_load_ticket_id += 1;
        ChunkLoadTicket {
            id: self.next_chunk_load_ticket_id,
            position,
            is_completed: Arc::new(AtomicBool::new(false)),
        }
    }

    fn optional_io_task(
        &self,
        should_run_parallel: bool,
        task: impl FnOnce() -> Result<()> + Send + 'static,
    ) -> WorldIoTask {
        if !should_run_parallel {
            return WorldIoTask::completed(task());
        }
        WorldIoTask::running(std::thread::spawn(task))
    }

    fn load_chunk_with_event_flag(
        &mut self,
        position: ChunkPosition,
        should_dispatch_load_event: bool,
    ) -> Result<&mut Chunk> {
        if self.loading_chunks.contains(&position) {
            return self.chunks.get_mut(&position).ok_or_else(|| {
                Error::new(
                    ErrorKind::WouldBlock,
                    "Chunk load is already in progress for this position",
                )
            });
        }
        let chunk_was_missing = !self.chunks.contains_key(&position);
        if chunk_was_missing {
            self.loading_chunks.insert(position);
            let load_result = self.load_or_create_chunk(position);
            self.loading_chunks.remove(&position);
            load_result?;
        }
        self.finish_new_chunk_generation(position)?;
        if chunk_was_missing {
            let chunk = self.chunks.get_mut(&position).ok_or_else(|| {
                Error::new(
                    ErrorKind::NotFound,
                    "Loaded chunk disappeared before on-load callback",
                )
            })?;
            chunk.on_load();
        }
        if chunk_was_missing && should_dispatch_load_event {
            self.dispatch_instance_chunk_load_event(position);
        }
        self.chunks
            .get_mut(&position)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Chunk was not loaded"))
    }

    fn load_or_create_chunk(&mut self, position: ChunkPosition) -> Result<()> {
        let mut chunk = match self.chunk_loader.load_chunk(position)? {
            Some(mut chunk) => {
                chunk.mark_loaded_from_storage();
                chunk
            }
            None => self.chunk_supplier.create_chunk(position),
        };
        chunk.set_world(self.uuid);
        self.chunks.insert(position, chunk);
        self.entity_tracker.create_chunk_partition(position);
        Ok(())
    }
}

impl Taggable for World {
    fn tag_handler(&self) -> &TagHandler {
        &self.tag_handler
    }

    fn tag_handler_mut(&mut self) -> &mut TagHandler {
        &mut self.tag_handler
    }
}

fn player_coordinate_is_too_large(coordinate: f64) -> bool {
    coordinate.abs() > MAX_PLAYER_COORDINATE
}

fn prepare_chunk_load(
    position: ChunkPosition,
    chunk_loader: Arc<dyn ChunkLoader>,
    chunk_supplier: ChunkSupplier,
    generator: Option<Arc<dyn Generator + Send + Sync>>,
    synchronously_loaded_chunk: Option<Option<Chunk>>,
) -> Result<PreparedChunkLoad> {
    let loaded_chunk = match synchronously_loaded_chunk {
        Some(loaded_chunk) => loaded_chunk,
        None => chunk_loader.load_chunk(position)?,
    };
    let mut chunk = match loaded_chunk {
        Some(mut chunk) => {
            chunk.mark_loaded_from_storage();
            chunk
        }
        None => chunk_supplier.create_chunk(position),
    };
    let requires_generation_completion = chunk.requires_generation_completion();
    let generation_forks = match (chunk.requires_generation(), generator) {
        (true, Some(generator)) => generate_chunk(&mut chunk, generator.as_ref())?,
        _ => Vec::new(),
    };
    Ok(PreparedChunkLoad {
        chunk,
        generation_forks,
        requires_generation_completion,
    })
}

fn generate_chunk(
    chunk: &mut Chunk,
    generator: &(dyn Generator + Send + Sync),
) -> Result<Vec<GenerationFork>> {
    let size = BlockSize::new(16, (chunk.sections().len() as i32) << 4, 16);
    let start = BlockPosition::new(chunk.x() << 4, -64, chunk.z() << 4);
    let mut unit = GenerationUnit::new(size, start, chunk.sections().to_vec());
    generator.generate(&mut unit).map_err(Error::other)?;
    let (sections, generation_forks) = unit.into_generation();
    chunk.replace_sections(sections);
    Ok(generation_forks)
}

fn chunk_position_for_entity_position(position: EntityPosition) -> ChunkPosition {
    ChunkPosition::new(
        (position.x().floor() as i32).div_euclid(16),
        (position.z().floor() as i32).div_euclid(16),
    )
}

fn entity_positions_are_within_view_distance(
    viewed_entity_position: EntityPosition,
    viewer_position: EntityPosition,
) -> bool {
    let viewed_entity_chunk = chunk_position_for_entity_position(viewed_entity_position);
    let viewer_chunk = chunk_position_for_entity_position(viewer_position);
    (viewed_entity_chunk.x - viewer_chunk.x).abs() <= ENTITY_VIEW_DISTANCE
        && (viewed_entity_chunk.z - viewer_chunk.z).abs() <= ENTITY_VIEW_DISTANCE
}

fn automatic_visibility_pair_is_allowed(
    viewed_entity: &Entity,
    viewer_player: &Player,
    viewed_player_is_vanished: bool,
) -> bool {
    viewer_player.has_entered_world()
        && !viewed_player_is_vanished
        && viewer_player.vehicle() != Some(viewed_entity.entity_id())
        && entity_positions_are_within_view_distance(
            viewed_entity.position(),
            viewer_player.position(),
        )
        && viewed_entity.view().is_auto_viewable()
        && viewer_player.view().is_auto_viewer()
        && viewed_entity
            .view()
            .viewable_rule_allows(viewer_player.entity_id())
        && viewer_player
            .view()
            .viewer_rule_allows(viewed_entity.entity_id())
}

fn block_is_air(block: Block) -> bool {
    matches!(block, Block::AIR | Block::CAVE_AIR | Block::VOID_AIR)
}

fn affected_lighting_chunk_positions(position: BlockPosition) -> Vec<ChunkPosition> {
    let changed_chunk = ChunkPosition::from(position);
    (-1..=1)
        .flat_map(|x_offset| {
            (-1..=1).map(move |z_offset| {
                ChunkPosition::new(changed_chunk.x + x_offset, changed_chunk.z + z_offset)
            })
        })
        .collect()
}

fn block_is_sight_block(block: Block) -> bool {
    !block_is_air(block)
}

fn entity_eye_position(entity: &Entity) -> Vector3d {
    let position = entity.position();
    Vector3d {
        x: position.x(),
        y: position.y() + entity.eye_height(),
        z: position.z(),
    }
}

fn view_direction(position: EntityPosition) -> Vector3d {
    let yaw = position.yaw().to_radians() as f64;
    let pitch = position.pitch().to_radians() as f64;
    let pitch_cosine = pitch.cos();
    Vector3d {
        x: -yaw.sin() * pitch_cosine,
        y: -pitch.sin(),
        z: yaw.cos() * pitch_cosine,
    }
}

fn normalized_vector_between(start: Vector3d, end: Vector3d) -> Vector3d {
    let distance = vector_distance(start, end);
    if distance == 0.0 {
        return Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }
    Vector3d {
        x: (end.x - start.x) / distance,
        y: (end.y - start.y) / distance,
        z: (end.z - start.z) / distance,
    }
}

fn vector_distance(start: Vector3d, end: Vector3d) -> f64 {
    let delta_x = end.x - start.x;
    let delta_y = end.y - start.y;
    let delta_z = end.z - start.z;
    (delta_x.mul_add(delta_x, delta_y.mul_add(delta_y, delta_z * delta_z))).sqrt()
}

fn vectors_are_aligned(first: Vector3d, second: Vector3d) -> bool {
    let dot_product = first
        .x
        .mul_add(second.x, first.y.mul_add(second.y, first.z * second.z));
    dot_product > 0.995
}

fn ray_reaches_entity(start: Vector3d, direction: Vector3d, entity: &Entity) -> bool {
    let target = entity_eye_position(entity);
    let target_distance = vector_distance(start, target);
    if target_distance == 0.0 {
        return true;
    }
    let bounding_box = entity.bounding_box();
    let ray_direction = Vector3d {
        x: direction.x * target_distance,
        y: direction.y * target_distance,
        z: direction.z * target_distance,
    };
    RaycastBoundingBox::from_center_dimensions(
        entity.position().as_vector(),
        bounding_box.width(),
        bounding_box.height(),
        bounding_box.depth(),
    )
    .ray_intersection(start, ray_direction)
    .is_some()
}

fn player_intersects_block(
    player_position: EntityPosition,
    block_center: Vector3d,
    block_box: EntityBoundingBox,
) -> bool {
    let player_box = EntityType::PLAYER.bounding_box();
    let player_start = Vector3d {
        x: player_position.x() - player_box.width() / 2.0,
        y: player_position.y(),
        z: player_position.z() - player_box.depth() / 2.0,
    };
    let player_end = Vector3d {
        x: player_position.x() + player_box.width() / 2.0,
        y: player_position.y() + player_box.height(),
        z: player_position.z() + player_box.depth() / 2.0,
    };
    let block_start = Vector3d {
        x: block_center.x - block_box.width() / 2.0,
        y: block_center.y,
        z: block_center.z - block_box.depth() / 2.0,
    };
    let block_end = Vector3d {
        x: block_center.x + block_box.width() / 2.0,
        y: block_center.y + block_box.height(),
        z: block_center.z + block_box.depth() / 2.0,
    };

    player_start.x <= block_end.x
        && player_end.x >= block_start.x
        && player_start.y <= block_end.y
        && player_end.y >= block_start.y
        && player_start.z <= block_end.z
        && player_end.z >= block_start.z
}

fn player_pose_fits_at(world: &World, player_position: EntityPosition, pose: PlayerPose) -> bool {
    let Some(player_box) = pose.bounding_box(EntityType::PLAYER.bounding_box()) else {
        return false;
    };
    let player_start = Vector3d {
        x: player_position.x() - player_box.width() / 2.0,
        y: player_position.y(),
        z: player_position.z() - player_box.depth() / 2.0,
    };
    let player_end = Vector3d {
        x: player_position.x() + player_box.width() / 2.0,
        y: player_position.y() + player_box.height(),
        z: player_position.z() + player_box.depth() / 2.0,
    };
    pose_block_positions(player_start, player_end)
        .into_iter()
        .all(
            |block_position| match world.loaded_block_at(block_position) {
                Some(block) if block != Block::SCAFFOLDING && block.is_solid() => !boxes_intersect(
                    player_start,
                    player_end,
                    block_start(block_position),
                    block_end(block_position),
                ),
                _ => true,
            },
        )
}

fn pose_block_positions(player_start: Vector3d, player_end: Vector3d) -> Vec<BlockPosition> {
    let min_x = player_start.x.floor() as i32;
    let min_y = player_start.y.floor() as i32;
    let min_z = player_start.z.floor() as i32;
    let max_x = player_end.x.floor() as i32;
    let max_y = player_end.y.floor() as i32;
    let max_z = player_end.z.floor() as i32;
    (min_x..=max_x)
        .flat_map(|x| {
            (min_y..=max_y)
                .flat_map(move |y| (min_z..=max_z).map(move |z| BlockPosition::new(x, y, z)))
        })
        .collect()
}

fn block_start(block_position: BlockPosition) -> Vector3d {
    Vector3d {
        x: f64::from(block_position.x),
        y: f64::from(block_position.y),
        z: f64::from(block_position.z),
    }
}

fn block_end(block_position: BlockPosition) -> Vector3d {
    Vector3d {
        x: f64::from(block_position.x) + 1.0,
        y: f64::from(block_position.y) + 1.0,
        z: f64::from(block_position.z) + 1.0,
    }
}

fn boxes_intersect(
    first_start: Vector3d,
    first_end: Vector3d,
    second_start: Vector3d,
    second_end: Vector3d,
) -> bool {
    first_start.x <= second_end.x
        && first_end.x >= second_start.x
        && first_start.y <= second_end.y
        && first_end.y >= second_start.y
        && first_start.z <= second_end.z
        && first_end.z >= second_start.z
}

#[derive(Clone, Copy)]
struct ProjectileCollisionState {
    shooter_id: Option<EntityId>,
    alive_ticks: u64,
    bounding_box: spinel_registry::EntityBoundingBox,
    is_on_ground: bool,
}

enum ProjectileCollision {
    Free,
    Stuck(EntityPosition),
}

fn projectile_sample_positions(
    position_before_tick: EntityPosition,
    position_after_tick: EntityPosition,
    projectile_width: f64,
) -> Vec<EntityPosition> {
    let delta_x = position_after_tick.x() - position_before_tick.x();
    let delta_y = position_after_tick.y() - position_before_tick.y();
    let delta_z = position_after_tick.z() - position_before_tick.z();
    let distance = delta_x
        .mul_add(delta_x, delta_y.mul_add(delta_y, delta_z * delta_z))
        .sqrt();
    let sample_distance = projectile_width / 2.0;
    let sample_count = (distance / sample_distance).ceil() as usize;
    if sample_count == 0 {
        return Vec::new();
    }
    let direction_x = delta_x / distance * sample_distance;
    let direction_y = delta_y / distance * sample_distance;
    let direction_z = delta_z / distance * sample_distance;
    (0..sample_count)
        .map(|sample_index| {
            if sample_index == sample_count - 1 {
                return position_after_tick;
            }
            let sample_multiplier = (sample_index + 1) as f64;
            position_before_tick.offset(
                direction_x * sample_multiplier,
                direction_y * sample_multiplier,
                direction_z * sample_multiplier,
            )
        })
        .collect()
}

fn block_position_for_entity(position: EntityPosition) -> BlockPosition {
    BlockPosition::new(
        position.x().floor() as i32,
        position.y().floor() as i32,
        position.z().floor() as i32,
    )
}

fn entity_is_living(entity: &Entity) -> bool {
    match entity {
        Entity::Creature(_) | Entity::Player(_) => true,
        Entity::Generic(entity) => entity.entity_type().is_living(),
        Entity::ExperienceOrb(_) | Entity::Item(_) | Entity::Projectile(_) => false,
    }
}

fn projectile_entity_is_removed(entity: &Entity) -> bool {
    match entity {
        Entity::Projectile(entity) => entity.is_removed(),
        Entity::Creature(_)
        | Entity::ExperienceOrb(_)
        | Entity::Generic(_)
        | Entity::Item(_)
        | Entity::Player(_) => true,
    }
}

fn entity_positions_share_point(
    first_position: EntityPosition,
    second_position: EntityPosition,
) -> bool {
    first_position.x() == second_position.x()
        && first_position.y() == second_position.y()
        && first_position.z() == second_position.z()
}

fn entity_boxes_intersect_at(
    first_position: EntityPosition,
    first_bounding_box: spinel_registry::EntityBoundingBox,
    second_position: EntityPosition,
    second_bounding_box: spinel_registry::EntityBoundingBox,
) -> bool {
    boxes_intersect(
        entity_box_start(first_position, first_bounding_box),
        entity_box_end(first_position, first_bounding_box),
        entity_box_start(second_position, second_bounding_box),
        entity_box_end(second_position, second_bounding_box),
    )
}

fn entity_box_start(
    position: EntityPosition,
    bounding_box: spinel_registry::EntityBoundingBox,
) -> Vector3d {
    Vector3d {
        x: position.x() + bounding_box.minimum_x(),
        y: position.y() + bounding_box.minimum_y(),
        z: position.z() + bounding_box.minimum_z(),
    }
}

fn entity_box_end(
    position: EntityPosition,
    bounding_box: spinel_registry::EntityBoundingBox,
) -> Vector3d {
    Vector3d {
        x: position.x() + bounding_box.maximum_x(),
        y: position.y() + bounding_box.maximum_y(),
        z: position.z() + bounding_box.maximum_z(),
    }
}

fn current_time_nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default()
}

fn transition_weather(
    target_weather: Weather,
    current_weather: Weather,
    remaining_rain_transition_ticks: i32,
    remaining_thunder_transition_ticks: i32,
) -> Weather {
    let rain_level = current_weather.rain_level()
        + (target_weather.rain_level() - current_weather.rain_level())
            * (1.0 / remaining_rain_transition_ticks.max(1) as f32);
    let thunder_level = current_weather.thunder_level()
        + (target_weather.thunder_level() - current_weather.thunder_level())
            * (1.0 / remaining_thunder_transition_ticks.max(1) as f32);
    Weather::from_valid_levels(rain_level, thunder_level)
}

fn dispatch_player_spawn_event(
    player: *mut Player,
    world: *mut World,
    first_spawn: bool,
    client: &mut Client,
) {
    let Some(server_ptr) = client.server_ptr else {
        return;
    };
    let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
    PlayerSpawnEvent::new(player, world, first_spawn).dispatch(server, client);
}

fn entity_scoreboard_team_name(entity: &Entity) -> Option<&str> {
    match entity {
        Entity::Creature(entity) => entity.team(),
        Entity::ExperienceOrb(entity) => entity.team(),
        Entity::Generic(entity) => entity.team(),
        Entity::Item(_) => None,
        Entity::Player(player) => player.team(),
        Entity::Projectile(entity) => entity.team(),
    }
}

fn living_item_pickup_scan(entity: &Entity) -> Option<(EntityPosition, EntityBoundingBox)> {
    match entity {
        Entity::Creature(entity) => Some((entity.position(), entity.expanded_bounding_box())),
        Entity::Generic(entity) if entity.entity_type().is_living() => {
            Some((entity.position(), entity.expanded_bounding_box()))
        }
        Entity::Player(player) => Some((player.position(), player.expanded_bounding_box())),
        _ => None,
    }
}

fn item_entity(entity: &Entity) -> Option<&ItemEntity> {
    match entity {
        Entity::Item(item_entity) => Some(item_entity),
        _ => None,
    }
}

fn damage_sound_source_id(entity_id: EntityId, world: &World) -> i32 {
    match world.entity_by_id(entity_id) {
        Some(Entity::Player(_)) => 1,
        _ => 5,
    }
}

fn dispatch_player_tick_event(player: &mut Player) {
    let player_ptr = player as *mut Player;
    let Some(client) = player.client_mut() else {
        return;
    };
    let Some(server_ptr) = client.server_ptr else {
        return;
    };
    let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
    PlayerTickEvent::new(player_ptr).dispatch(server, client);
}

fn dispatch_entity_tick_event(entity: *mut Entity, server_ptr: Option<usize>) {
    let Some(server_ptr) = server_ptr else {
        return;
    };
    let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
    EntityTickEvent::new(entity).dispatch(server);
}

fn dispatch_player_tick_end_event(player: &mut Player) {
    let player_ptr = player as *mut Player;
    let Some(client) = player.client_mut() else {
        return;
    };
    let Some(server_ptr) = client.server_ptr else {
        return;
    };
    let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
    PlayerTickEndEvent::new(player_ptr).dispatch(server, client);
}

fn distinct_entities_mut(
    entities: &mut [Entity],
    first_index: usize,
    second_index: usize,
) -> (&mut Entity, &mut Entity) {
    if first_index < second_index {
        let (before_second, from_second) = entities.split_at_mut(second_index);
        return (&mut before_second[first_index], &mut from_second[0]);
    }
    let (before_first, from_first) = entities.split_at_mut(first_index);
    (&mut from_first[0], &mut before_first[second_index])
}
