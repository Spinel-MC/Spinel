use crate::entity::metadata::definitions;
use crate::entity::{
    Entity, EntityId, EntityPosition, GenericEntity, Player, PlayerChunk, PlayerChunkTransition,
};
use crate::events::instance_block_update::InstanceBlockUpdateEvent;
use crate::events::instance_chunk_load::InstanceChunkLoadEvent;
use crate::events::instance_chunk_unload::InstanceChunkUnloadEvent;
use crate::events::instance_register::InstanceRegisterEvent;
use crate::events::instance_section_invalidate::InstanceSectionInvalidateEvent;
use crate::events::instance_tick::InstanceTickEvent;
use crate::events::instance_tick_end::InstanceTickEndEvent;
use crate::events::instance_unregister::InstanceUnregisterEvent;
use crate::events::player_block_break::PlayerBlockBreakEvent;
use crate::events::player_move::PlayerMoveEvent;
use crate::events::player_spawn::PlayerSpawnEvent;
use crate::events::player_tick::PlayerTickEvent;
use crate::events::player_tick_end::PlayerTickEndEvent;
use crate::network::client::instance::Client;
use crate::world::generator::{FallibleGenerator, GenerateChunkError, GenerationFork, Generator};
use crate::world::{
    Biome, Block, BlockHandler, BlockHandlerDestroy, BlockHandlerInteraction,
    BlockHandlerPlacement, BlockHandlerRegistry, BlockHandlerTouch, BlockLookupCondition,
    BlockPlacementRule, BlockPlacementRuleRegistry, BlockPlacementState, BlockPosition, BlockSize,
    BlockUpdateState, BossBar, Chunk, ChunkLoader, ChunkPosition, ChunkSection, EntityTracker,
    EntityTrackerTarget, ExplosionSupplier, GenerationUnit, NoopChunkLoader, Weather, WorldBorder,
    WorldEventNode, WorldIdentity, WorldPointers, WorldScheduler, WorldSnapshot, WorldSoundEmitter,
};
use spinel_core::network::clientbound::play::block_action::BlockActionPacket;
use spinel_core::network::clientbound::play::block_entity_data::BlockEntityDataPacket;
use spinel_core::network::clientbound::play::block_update::BlockUpdatePacket;
use spinel_core::network::clientbound::play::chunk_data::ChunkDataAndUpdateLightPacket;
use spinel_core::network::clientbound::play::entity_head_look::EntityHeadLookPacket;
use spinel_core::network::clientbound::play::entity_position::EntityPositionPacket;
use spinel_core::network::clientbound::play::entity_position_and_rotation::EntityPositionAndRotationPacket;
use spinel_core::network::clientbound::play::entity_sound_effect::{
    EntitySoundEffectPacket, NetworkSoundEvent,
};
use spinel_core::network::clientbound::play::entity_status::EntityStatusPacket;
use spinel_core::network::clientbound::play::entity_teleport::EntityTeleportPacket;
use spinel_core::network::clientbound::play::game_event::{GameEvent, GameEventPacket};
use spinel_core::network::clientbound::play::player_info_remove::PlayerInfoRemovePacket;
use spinel_core::network::clientbound::play::player_info_update::PlayerInfoUpdatePacket;
use spinel_core::network::clientbound::play::remove_entities::RemoveEntitiesPacket;
use spinel_core::network::clientbound::play::set_entity_data::SetEntityDataPacket;
use spinel_core::network::clientbound::play::set_equipment::SetEquipmentPacket;
use spinel_core::network::clientbound::play::set_time::SetTimePacket;
use spinel_core::network::clientbound::play::sound_effect::{
    NetworkPositionedSoundEvent, SoundEffectPacket,
};
use spinel_core::network::clientbound::play::spawn_entity::EntityAngle;
use spinel_core::network::clientbound::play::spawn_entity::SpawnEntityPacket;
use spinel_core::network::clientbound::play::world_event::WorldEventPacket;
use spinel_nbt::{Nbt, NbtCompound, TagHandler, Taggable};
use spinel_network::types::entity_metadata::MetadataEntry;
use spinel_network::types::sound::SoundEvent;
use spinel_network::types::{Identifier, Position, TeleportFlags, Vector3d, Velocity};
use spinel_network::{DataType, PacketSender, PacketStruct};
use spinel_registry::block_entity_type::BlockEntityType;
use spinel_registry::dimension_type::DimensionType;
use spinel_registry::{EntityType, Registries, RegistryKey};
use spinel_utils::component::Component;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{Error, ErrorKind, Result};
use std::net::SocketAddr;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

const MAX_PLAYER_COORDINATE: f64 = 30_000_000.0;
const DEFAULT_TIME_SYNCHRONIZATION_TICKS: i32 = 20;
const DEFAULT_CHUNK_VIEW_DISTANCE: i32 = 8;
const DESTROY_BLOCK_WORLD_EVENT_ID: i32 = 2001;

pub struct ChunkSupplier {
    create_chunk: Box<dyn Fn(ChunkPosition) -> Chunk + Send + Sync>,
}

#[derive(Clone, Copy)]
struct PendingPlayerChunkLoad {
    player_address: SocketAddr,
    chunk: PlayerChunk,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChunkLoadTicket {
    id: u64,
    position: ChunkPosition,
}

pub struct WorldIoTask {
    handle: Option<JoinHandle<Result<()>>>,
    completed: Option<Result<()>>,
}

struct AsyncChunkLoad {
    handle: JoinHandle<Result<Option<Chunk>>>,
}

impl ChunkSupplier {
    pub fn new(create_chunk: impl Fn(ChunkPosition) -> Chunk + Send + Sync + 'static) -> Self {
        Self {
            create_chunk: Box::new(create_chunk),
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

struct PlayerViewerSnapshot {
    player_info_packet: PlayerInfoUpdatePacket,
    spawn_packet: SpawnEntityPacket,
    metadata_entries: Vec<MetadataEntry>,
    equipment_packet: SetEquipmentPacket,
    head_look_packet: EntityHeadLookPacket,
}

impl PlayerViewerSnapshot {
    fn from_player(player: &Player) -> Self {
        Self {
            player_info_packet: player.player_info_packet(),
            spawn_packet: player.spawn_packet(),
            metadata_entries: player.metadata_packet().entries.0,
            equipment_packet: player.visible_equipment_packet(),
            head_look_packet: player.head_look_packet(),
        }
    }

    fn dispatch(&self, client: &mut Client) -> Result<()> {
        self.player_info_packet.clone().dispatch(client)?;
        self.spawn_packet.clone().dispatch(client)?;
        SetEntityDataPacket::new(self.spawn_packet.entity_id, self.metadata_entries.clone())
            .dispatch(client)?;
        SetEquipmentPacket::new(
            self.spawn_packet.entity_id,
            self.equipment_packet.equipment.0.clone(),
        )
        .dispatch(client)?;
        EntityHeadLookPacket {
            entity_id: self.head_look_packet.entity_id,
            head_yaw: self.head_look_packet.head_yaw,
        }
        .dispatch(client)
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
    currently_changing_blocks: HashMap<BlockPosition, Block>,
    pending_generation: HashMap<ChunkPosition, Vec<GenerationFork>>,
    loading_chunks: HashSet<ChunkPosition>,
    async_chunk_loads: HashMap<ChunkPosition, ChunkLoadTicket>,
    async_chunk_load_handles: HashMap<u64, AsyncChunkLoad>,
    completed_chunk_loads: HashSet<u64>,
    next_chunk_load_ticket_id: u64,
    pending_player_chunk_loads: VecDeque<PendingPlayerChunkLoad>,
    pending_player_chunk_load_keys: HashSet<(SocketAddr, PlayerChunk)>,
    generator: Option<Box<dyn Generator + Send + Sync>>,
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
            async_chunk_load_handles: HashMap::new(),
            completed_chunk_loads: HashSet::new(),
            next_chunk_load_ticket_id: 0,
            pending_player_chunk_loads: VecDeque::new(),
            pending_player_chunk_load_keys: HashSet::new(),
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
            .is_some_and(|changed_block| *changed_block == block)
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

    pub fn schedule_next_tick(&mut self, callback: impl FnOnce(&mut World) + Send + 'static) {
        self.scheduler.schedule_next_tick(callback);
    }

    pub fn schedule_tick_end(&mut self, callback: impl FnOnce(&mut World) + Send + 'static) {
        self.scheduler.schedule_tick_end(callback);
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
        self.generator = Some(Box::new(generator));
    }
    pub fn set_fallible_generator(
        &mut self,
        generator: impl Fn(&mut GenerationUnit) -> std::result::Result<(), GenerateChunkError>
        + Send
        + Sync
        + 'static,
    ) {
        self.generator = Some(Box::new(FallibleGenerator::new(generator)));
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

    pub fn is_chunk_loaded(&self, position: ChunkPosition) -> bool {
        self.chunks
            .get(&position)
            .is_some_and(|chunk| chunk.is_loaded())
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

    pub fn complete_chunk_load(&mut self, ticket: ChunkLoadTicket) -> Result<bool> {
        if self.completed_chunk_loads.contains(&ticket.id) {
            return Ok(true);
        }
        if self.chunks.contains_key(&ticket.position) {
            self.completed_chunk_loads.insert(ticket.id);
            self.async_chunk_loads.remove(&ticket.position);
            return Ok(true);
        }
        let Some(async_load) = self.async_chunk_load_handles.get(&ticket.id) else {
            return Ok(false);
        };
        if !async_load.handle.is_finished() {
            return Ok(false);
        }
        let Some(async_load) = self.async_chunk_load_handles.remove(&ticket.id) else {
            return Ok(false);
        };
        let load_result = async_load
            .handle
            .join()
            .map_err(|_| Error::new(ErrorKind::Other, "Chunk load task panicked"))?;
        self.async_chunk_loads.remove(&ticket.position);
        let mut chunk = match load_result? {
            Some(chunk) => chunk,
            None => self.chunk_supplier.create_chunk(ticket.position),
        };
        chunk.set_world(self.uuid);
        self.chunks.insert(ticket.position, chunk);
        self.entity_tracker.create_chunk_partition(ticket.position);
        self.generate_chunk_result(ticket.position)?;
        self.dispatch_instance_chunk_load_event(ticket.position);
        self.completed_chunk_loads.insert(ticket.id);
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

    pub fn unload_chunk(&mut self, position: ChunkPosition) -> Result<bool> {
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

    pub fn tick_chunks(&self, time: u64) -> usize {
        self.chunks
            .values()
            .filter(|chunk| chunk.is_loaded())
            .map(|chunk| chunk.tick(self.uuid, &self.block_handlers, time))
            .sum()
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
            self.entity_tracker.unregister(entity_id);
        });
        self.entities
            .retain(|entity| chunk_position_for_entity_position(entity.position()) != position);
    }
    pub fn regenerate_chunk(&mut self, position: ChunkPosition) {
        if let Some(chunk) = self.chunks.get_mut(&position) {
            chunk.mark_not_generated();
            chunk.clear_invalidated();
        }
        self.generate_chunk(position);
    }
    pub fn generate_chunk(&mut self, position: ChunkPosition) {
        let _ = self.generate_chunk_result(position);
    }

    pub fn generate_chunk_result(&mut self, position: ChunkPosition) -> Result<bool> {
        let Some(mut chunk) = self.chunks.remove(&position) else {
            return Ok(false);
        };
        let generation_result = self.apply_generator(&mut chunk);
        self.chunks.insert(position, chunk);
        generation_result.map(|_| true)
    }

    pub(crate) fn add_entity(&mut self, mut entity: Entity) {
        entity.set_world(self.uuid);
        self.entity_tracker.register(&entity);
        self.entities.push(entity);
    }

    pub(crate) fn take_entity(&mut self, entity_id: EntityId) -> Option<Entity> {
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
            Entity::Generic(_) => false,
        })?;
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

    pub fn entity_by_uuid(&self, entity_uuid: Uuid) -> Option<&Entity> {
        self.entities
            .iter()
            .find(|entity| entity.uuid() == entity_uuid)
    }

    pub fn players(&self) -> impl Iterator<Item = &Player> {
        self.entities.iter().filter_map(|entity| match entity {
            Entity::Player(player) => Some(player),
            Entity::Generic(_) => None,
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

    pub fn experience_orbs(&self) -> Vec<&GenericEntity> {
        self.entity_tracker
            .entities(EntityTrackerTarget::ExperienceOrbs)
            .into_iter()
            .filter_map(|entity_id| match self.entity_by_id(entity_id) {
                Some(Entity::Generic(entity)) => Some(entity),
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

    pub fn spawn_entity(
        &mut self,
        entity_type: EntityType,
        position: EntityPosition,
        nbt: Option<&NbtCompound>,
    ) -> Result<EntityId> {
        let mut entity = GenericEntity::new(entity_type);
        entity.set_position(position);
        apply_entity_nbt(&mut entity, nbt);
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.has_entered_world() => Some(player),
                _ => None,
            })
            .try_for_each(|player| {
                let viewer_id = player.entity_id();
                let Some(viewer_client) = player.client_mut() else {
                    return Ok(());
                };
                entity.show_to_viewer(viewer_id, viewer_client).map(|_| ())
            })?;
        let entity_id = entity.entity_id();
        self.add_entity(Entity::Generic(entity));
        Ok(entity_id)
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
            Entity::Generic(entity) => !entity.is_removed(),
            Entity::Player(_) => true,
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
        self.schedule_player_chunk_loads(client.addr, &chunks);
        let time_packet = self.time_packet();
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
            let Some(player) = self.player_by_addr_mut(&client.addr) else {
                return Err(Error::new(ErrorKind::NotFound, "Player not found."));
            };
            player.set_world(world_uuid);
            let first_spawn = !player.has_entered_world();
            player.unsafe_init_with_chunk_positions(
                client,
                ticks_per_second,
                world_name.clone(),
                chunks,
                time_packet,
            )?;
            world_border_packet.dispatch(client)?;
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
        self.send_pending_chunks_for_client(client, registries)?;
        dispatch_player_spawn_event(player, world_name, first_spawn, client);
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
        if player_coordinate_is_too_large(x)
            || player_coordinate_is_too_large(y)
            || player_coordinate_is_too_large(z)
        {
            return player.kick(Component::text("You moved too far away!"));
        }
        let previous_position = player.position();
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
        let transition = self.player_by_addr(&client.addr).and_then(|player| {
            player.chunk_transition(
                event_position.x(),
                event_position.y(),
                event_position.z(),
                self.view_distance,
            )
        });
        if self.movement_enters_unloaded_chunk(transition.as_ref()) {
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
        let chunks = match transition.as_ref() {
            Some(transition) => transition.arriving.clone(),
            None => Vec::new(),
        };
        self.schedule_player_chunk_loads(client.addr, &chunks);
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
        if player_coordinate_is_too_large(x)
            || player_coordinate_is_too_large(y)
            || player_coordinate_is_too_large(z)
        {
            return player.kick(Component::text("You moved too far away!"));
        }
        let previous_position = player.position();
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
        let transition = self.player_by_addr(&client.addr).and_then(|player| {
            player.chunk_transition(
                event_position.x(),
                event_position.y(),
                event_position.z(),
                self.view_distance,
            )
        });
        if self.movement_enters_unloaded_chunk(transition.as_ref()) {
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
        let chunks = match transition.as_ref() {
            Some(transition) => transition.arriving.clone(),
            None => Vec::new(),
        };
        self.load_player_chunks(&chunks)?;
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

    pub(crate) fn refresh_player_status(&mut self, client: &Client, on_ground: bool) -> Result<()> {
        let Some(player) = self.player_by_addr_mut(&client.addr) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };
        player.set_on_ground(on_ground);
        Ok(())
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
        let view_changed = current_position.yaw() != previous_position.yaw()
            || current_position.pitch() != previous_position.pitch();
        let movement_requires_teleport = (current_position.x() - previous_position.x()).abs() > 8.0
            || (current_position.y() - previous_position.y()).abs() > 8.0
            || (current_position.z() - previous_position.z()).abs() > 8.0;
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player)
                    if player.entity_id() != moving_player_id && player.has_entered_world() =>
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
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player)
                    if player.entity_id() != changed_player_id && player.has_entered_world() =>
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

    fn broadcast_player_equipment(
        &mut self,
        changed_player_id: EntityId,
        equipment_entity_id: i32,
        equipment_entries: Vec<
            spinel_core::network::clientbound::play::set_equipment::EntityEquipmentEntry,
        >,
    ) -> Result<()> {
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player)
                    if player.entity_id() != changed_player_id && player.has_entered_world() =>
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
        let mut player_addresses = Vec::new();
        let mut entity_touches = Vec::new();
        let item_use_completions = self
            .entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Generic(entity) => {
                    entity.tick();
                    entity_touches.push((entity.entity_id(), entity.position()));
                    None
                }
                Entity::Player(player) => {
                    dispatch_player_tick_event(player);
                    let item_use_completion = player.tick();
                    entity_touches.push((player.entity_id(), player.position()));
                    if player.has_entered_world() {
                        player_addresses.push(player.addr);
                    }
                    dispatch_player_tick_end_event(player);
                    item_use_completion
                }
            })
            .collect::<Vec<_>>();
        entity_touches
            .into_iter()
            .for_each(|(entity_id, position)| self.touch_entity_blocks(entity_id, position));
        let _ = self.process_pending_player_chunk_loads();
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

    fn process_next_tick_scheduler(&mut self) {
        let callbacks = self.scheduler.take_next_tick_callbacks();
        callbacks.into_iter().for_each(|callback| callback(self));
    }

    fn process_tick_end_scheduler(&mut self) {
        let callbacks = self.scheduler.take_tick_end_callbacks();
        callbacks.into_iter().for_each(|callback| callback(self));
    }

    fn touch_entity_blocks(&self, entity_id: EntityId, position: EntityPosition) {
        let block_position = BlockPosition::new(
            position.x().floor() as i32,
            position.y().floor() as i32,
            position.z().floor() as i32,
        );
        let Some(block) = self.loaded_block_at(block_position) else {
            return;
        };
        let Some(handler) = self.block_handlers.handler(block) else {
            return;
        };
        handler.on_touch(BlockHandlerTouch::new(
            block,
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
                if previous_weather.has_rain() != weather.has_rain() {
                    let rain_event = if weather.has_rain() {
                        GameEvent::BeginRaining
                    } else {
                        GameEvent::EndRaining
                    };
                    GameEventPacket::from(rain_event).dispatch(client)?;
                }
                GameEventPacket::from(GameEvent::RainLevelChange(weather.rain_level()))
                    .dispatch(client)?;
                GameEventPacket::from(GameEvent::ThunderLevelChange(weather.thunder_level()))
                    .dispatch(client)
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
            Entity::Generic(_) => true,
            Entity::Player(player) => player.addr != *addr,
        });
        Ok(())
    }

    pub(crate) fn player_by_addr_mut(&mut self, addr: &SocketAddr) -> Option<&mut Player> {
        self.entities.iter_mut().find_map(|entity| match entity {
            Entity::Generic(_) => None,
            Entity::Player(player) if player.addr == *addr => Some(player),
            Entity::Player(_) => None,
        })
    }

    pub(crate) fn player_by_addr(&self, addr: &SocketAddr) -> Option<&Player> {
        self.entities.iter().find_map(|entity| match entity {
            Entity::Generic(_) => None,
            Entity::Player(player) if player.addr == *addr => Some(player),
            Entity::Player(_) => None,
        })
    }

    pub(crate) fn player_pointer_by_addr(&mut self, addr: &SocketAddr) -> Option<*mut Player> {
        self.player_by_addr_mut(addr)
            .map(|player| player as *mut Player)
    }

    pub(crate) fn send_player_remove_to_viewers(
        &mut self,
        player_id: EntityId,
        player_uuid: Uuid,
    ) -> Result<()> {
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player) if player.has_entered_world() => Some(player),
                _ => None,
            })
            .filter_map(Player::client_mut)
            .try_for_each(|viewer_client| {
                PlayerInfoRemovePacket::new(player_uuid).dispatch(viewer_client)?;
                RemoveEntitiesPacket::new(vec![player_id.value()]).dispatch(viewer_client)
            })
    }

    pub(crate) fn synchronize_player_visibility(&mut self, client: &mut Client) -> Result<()> {
        let client_address = client.addr;
        let Some(joining_player) = self.player_by_addr(&client_address) else {
            return Err(Error::new(ErrorKind::NotFound, "Player not found."));
        };

        let joining_player_id = joining_player.entity_id();
        let visible_player_packets = self
            .entities
            .iter()
            .filter_map(|entity| match entity {
                Entity::Player(player)
                    if player.addr != client_address && player.has_entered_world() =>
                {
                    Some(PlayerViewerSnapshot::from_player(player))
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        visible_player_packets
            .into_iter()
            .try_for_each(|player_snapshot| player_snapshot.dispatch(client))?;

        let joining_player_snapshot = PlayerViewerSnapshot::from_player(joining_player);
        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Player(player)
                    if player.addr != client_address && player.has_entered_world() =>
                {
                    Some(player)
                }
                _ => None,
            })
            .filter_map(Player::client_mut)
            .try_for_each(|viewer_client| joining_player_snapshot.dispatch(viewer_client))?;

        self.entities
            .iter_mut()
            .filter_map(|entity| match entity {
                Entity::Generic(entity) if !entity.is_removed() => Some(entity),
                _ => None,
            })
            .try_for_each(|entity| entity.show_to_viewer(joining_player_id, client).map(|_| ()))?;

        Ok(())
    }

    pub(crate) fn dispatch_player_spawn(
        &mut self,
        player_uuid: Uuid,
        first_spawn: bool,
        client: &mut Client,
    ) {
        let world_name = self.name.clone();
        let Some(player) = self.entities.iter_mut().find_map(|entity| match entity {
            Entity::Player(player) if player.uuid() == player_uuid => Some(player),
            _ => None,
        }) else {
            return;
        };
        dispatch_player_spawn_event(player as *mut Player, world_name, first_spawn, client);
    }

    pub fn block_at(&mut self, position: BlockPosition) -> Result<Block> {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        Ok(self.load_chunk(chunk_position)?.block(position))
    }

    pub fn block_at_with_condition(
        &mut self,
        position: BlockPosition,
        condition: BlockLookupCondition,
    ) -> Result<Option<Block>> {
        match condition {
            BlockLookupCondition::None => self.block_at(position).map(Some),
            BlockLookupCondition::Cached | BlockLookupCondition::Type => {
                Ok(self.loaded_block_at(position))
            }
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
        self.set_block_with_handler(position, block, None, None)
    }

    fn set_block_with_handler(
        &mut self,
        position: BlockPosition,
        block: Block,
        placement: Option<BlockHandlerPlacement>,
        destroy: Option<BlockHandlerDestroy>,
    ) -> Result<bool> {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        self.load_chunk(chunk_position)?;
        self.set_loaded_block_with_handler(position, block, placement, destroy, true, 0)
    }

    fn set_loaded_block_with_handler(
        &mut self,
        position: BlockPosition,
        mut block: Block,
        placement: Option<BlockHandlerPlacement>,
        destroy: Option<BlockHandlerDestroy>,
        do_block_updates: bool,
        update_distance: i32,
    ) -> Result<bool> {
        block =
            self.block_after_placement_rule(block, position, placement.as_ref(), do_block_updates);
        if self
            .currently_changing_blocks
            .get(&position)
            .is_some_and(|changed_block| *changed_block == block)
        {
            return Ok(false);
        }
        self.currently_changing_blocks.insert(position, block);
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        let Some(mut chunk) = self.chunks.remove(&chunk_position) else {
            return Ok(false);
        };
        let block_was_set = chunk
            .try_set_block_with_handler(
                position,
                block,
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
        self.broadcast_block_update(position, block)?;
        self.broadcast_block_entity_update(position, block)?;
        self.dispatch_instance_block_update_event(position, block);
        Ok(true)
    }

    fn block_after_placement_rule(
        &self,
        block: Block,
        position: BlockPosition,
        placement: Option<&BlockHandlerPlacement>,
        do_block_updates: bool,
    ) -> Block {
        if !do_block_updates {
            return block;
        }
        let Some(placement) = placement else {
            return block;
        };
        let Some(rule) = self.block_placement_rules.rule(block) else {
            return block;
        };
        let player = placement
            .player()
            .and_then(|player_id| self.entity_by_id(player_id))
            .and_then(|entity| match entity {
                Entity::Player(player) => Some(player),
                Entity::Generic(_) => None,
            });
        rule.block_place(BlockPlacementState::new(
            block,
            placement.block_face(),
            position,
            placement.cursor_position(),
            player.map(Player::position),
            placement.player(),
            placement.hand(),
            player.is_some_and(Player::is_sneaking),
        ))
        .unwrap_or(Block::AIR)
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
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        self.chunks
            .get(&chunk_position)
            .filter(|chunk| chunk.is_loaded())
            .map(|chunk| chunk.block(position))
    }

    pub fn block_light(&self, position: BlockPosition) -> u8 {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        self.chunks
            .get(&chunk_position)
            .filter(|chunk| chunk.is_loaded())
            .map(|chunk| chunk.block_light(position))
            .unwrap_or_default()
    }

    pub fn sky_light(&self, position: BlockPosition) -> u8 {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        self.chunks
            .get(&chunk_position)
            .filter(|chunk| chunk.is_loaded())
            .map(|chunk| chunk.sky_light(position))
            .unwrap_or_default()
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
        let Some(chunk) = self.chunks.get(&position).filter(|chunk| chunk.is_loaded()) else {
            return Ok(());
        };
        let viewer_ids = chunk.viewers().collect::<HashSet<_>>();
        if viewer_ids.is_empty() {
            return Ok(());
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
        Ok(())
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
        let block = self.block_at(position)?;
        BlockUpdatePacket::new(
            Position {
                x: position.x,
                y: position.y,
                z: position.z,
            },
            block.state_id(),
        )
        .dispatch(client)
    }

    pub(crate) fn refresh_block_entity_for_player(
        &mut self,
        client: &mut Client,
        position: BlockPosition,
    ) -> Result<()> {
        let Some(block) = self.loaded_block_at(position) else {
            return Ok(());
        };
        if !ChunkSection::block_can_own_block_entity(block) {
            return Ok(());
        }
        let Some(block_entity_type) = self.block_entity_packet_type(block) else {
            return Ok(());
        };
        BlockEntityDataPacket::new(
            Position {
                x: position.x,
                y: position.y,
                z: position.z,
            },
            block_entity_type,
            self.client_block_entity_nbt(position, block),
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
        self.set_loaded_block_with_handler(
            placement.block_position(),
            placement.block(),
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
        let Some(block) = self.loaded_block_at(position) else {
            return true;
        };
        let Some(handler) = self.block_handlers.handler(block) else {
            return true;
        };
        handler.on_interact(BlockHandlerInteraction::new(
            block,
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

    fn broadcast_block_update(&mut self, position: BlockPosition, block: Block) -> Result<()> {
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
                BlockUpdatePacket::new(block_position, block.state_id()).dispatch(viewer_client)
            })
    }

    fn broadcast_block_entity_update(
        &mut self,
        position: BlockPosition,
        block: Block,
    ) -> Result<()> {
        if !ChunkSection::block_can_own_block_entity(block) {
            return Ok(());
        }
        let Some(block_entity_type) = self.block_entity_packet_type(block) else {
            return Ok(());
        };
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        let block_entity_nbt = self.client_block_entity_nbt(position, block);
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

    fn block_entity_packet_type(&self, block: Block) -> Option<BlockEntityType> {
        self.block_handlers
            .handler(block)
            .and_then(|handler| BlockEntityType::from_id(handler.block_entity_action().into()))
            .or_else(|| block_entity_type_for_block(block))
    }

    fn client_block_entity_nbt(
        &self,
        position: BlockPosition,
        block: Block,
    ) -> Option<NbtCompound> {
        let chunk_position =
            ChunkPosition::new(position.x.div_euclid(16), position.z.div_euclid(16));
        let block_entity = self
            .chunks
            .get(&chunk_position)
            .and_then(|chunk| chunk.block_entity(position))?;
        let Some(handler) = self.block_handlers.handler(block) else {
            return Some(block_entity.nbt().clone());
        };
        let tags = handler.block_entity_tags();
        if tags.is_empty() {
            return Some(NbtCompound::new());
        }
        let mut filtered_nbt = NbtCompound::new();
        tags.into_iter().for_each(|tag| {
            tag.write(&mut filtered_nbt, tag.read(block_entity.nbt()));
        });
        Some(filtered_nbt)
    }

    fn apply_generator(&mut self, chunk: &mut Chunk) -> Result<()> {
        let Some(generator) = self.generator() else {
            return Ok(());
        };
        if !chunk.should_generate() {
            return Ok(());
        }
        let size = BlockSize::new(16, (chunk.sections().len() as i32) << 4, 16);
        let start = BlockPosition::new(chunk.x() << 4, -64, chunk.z() << 4);
        let mut unit = GenerationUnit::new(size, start, chunk.sections().to_vec());
        generator
            .generate(&mut unit)
            .map_err(|error| Error::new(ErrorKind::Other, error))?;
        let (sections, forks) = unit.into_generation();
        chunk.replace_sections(sections);
        forks
            .into_iter()
            .for_each(|fork| self.store_generation_fork(fork));
        self.apply_pending_generation(chunk);
        chunk.on_generate();
        Ok(())
    }

    fn load_player_chunks(&mut self, chunks: &[PlayerChunk]) -> Result<()> {
        chunks
            .iter()
            .try_for_each(|chunk| self.load_chunk(ChunkPosition::from(*chunk)).map(|_| ()))
    }

    fn schedule_player_chunk_loads(&mut self, player_address: SocketAddr, chunks: &[PlayerChunk]) {
        chunks.iter().for_each(|chunk| {
            let request_key = (player_address, *chunk);
            if self.pending_player_chunk_load_keys.insert(request_key) {
                self.pending_player_chunk_loads
                    .push_back(PendingPlayerChunkLoad {
                        player_address,
                        chunk: *chunk,
                    });
            }
        });
    }

    fn process_pending_player_chunk_loads(&mut self) -> Result<()> {
        let mut pending_chunk_loads = VecDeque::new();
        std::mem::swap(
            &mut pending_chunk_loads,
            &mut self.pending_player_chunk_loads,
        );
        self.pending_player_chunk_load_keys.clear();
        while let Some(pending_chunk_load) = pending_chunk_loads.pop_front() {
            self.process_pending_player_chunk_load(pending_chunk_load)?;
        }
        Ok(())
    }

    fn process_pending_player_chunk_load(
        &mut self,
        pending_chunk_load: PendingPlayerChunkLoad,
    ) -> Result<()> {
        let Some(player) = self.player_by_addr(&pending_chunk_load.player_address) else {
            return Ok(());
        };
        if !pending_chunk_load.chunk.is_within_view_distance(
            player.chunks_loaded_by_client,
            player.effective_chunk_view_distance(self.view_distance),
        ) {
            return Ok(());
        }
        self.load_chunk(ChunkPosition::from(pending_chunk_load.chunk))?;
        let world_view_distance = self.view_distance;
        let Some(player) = self.player_by_addr_mut(&pending_chunk_load.player_address) else {
            return Ok(());
        };
        player.queue_loaded_chunk_if_current_view(pending_chunk_load.chunk, world_view_distance);
        Ok(())
    }

    fn loaded_chunk_packet(
        &self,
        position: ChunkPosition,
        registries: &Registries,
    ) -> Result<Option<ChunkDataAndUpdateLightPacket>> {
        let Some(chunk) = self.chunks.get(&position) else {
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
        let world = self as *const Self;
        unsafe { &mut *player }.send_pending_chunks_with(client, |queued_chunk| {
            let position = ChunkPosition::from(queued_chunk.chunk);
            unsafe { &*world }.loaded_chunk_packet(position, registries)
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
        let world = self as *const Self;
        unsafe { &mut *player }.send_pending_chunks_with(unsafe { &mut *client }, |queued_chunk| {
            let position = ChunkPosition::from(queued_chunk.chunk);
            unsafe { &*world }.loaded_chunk_packet(position, registries)
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
        if let Some(ticket) = self.async_chunk_loads.get(&position).copied() {
            return Ok(Some(ticket));
        }
        let ticket = self.next_chunk_load_ticket(position);
        if !self.chunk_loader.supports_parallel_loading() {
            self.load_chunk(position)?;
            self.completed_chunk_loads.insert(ticket.id);
            self.async_chunk_loads.remove(&position);
            return Ok(Some(ticket));
        }
        let chunk_loader = self.chunk_loader.clone();
        let handle = std::thread::spawn(move || chunk_loader.load_chunk(position));
        self.async_chunk_loads.insert(position, ticket);
        self.async_chunk_load_handles
            .insert(ticket.id, AsyncChunkLoad { handle });
        Ok(Some(ticket))
    }

    fn next_completed_chunk_load_ticket(&mut self, position: ChunkPosition) -> ChunkLoadTicket {
        let ticket = self.next_chunk_load_ticket(position);
        self.completed_chunk_loads.insert(ticket.id);
        ticket
    }

    fn next_chunk_load_ticket(&mut self, position: ChunkPosition) -> ChunkLoadTicket {
        self.next_chunk_load_ticket_id += 1;
        ChunkLoadTicket {
            id: self.next_chunk_load_ticket_id,
            position,
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
            let load_result = self.retrieve_chunk(position);
            self.loading_chunks.remove(&position);
            load_result?;
        }
        self.generate_chunk_result(position)?;
        if chunk_was_missing && should_dispatch_load_event {
            self.dispatch_instance_chunk_load_event(position);
        }
        self.chunks
            .get_mut(&position)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Chunk was not loaded"))
    }

    fn retrieve_chunk(&mut self, position: ChunkPosition) -> Result<()> {
        let mut chunk = match self.chunk_loader.load_chunk(position)? {
            Some(chunk) => chunk,
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

fn apply_entity_nbt(entity: &mut GenericEntity, nbt: Option<&NbtCompound>) {
    let Some(nbt) = nbt else {
        return;
    };
    if matches!(nbt.get("HasVisualFire"), Some(Nbt::Byte(1))) {
        entity
            .metadata_mut()
            .set_flag(&definitions::is_on_fire(), true);
    }
}

fn player_coordinate_is_too_large(coordinate: f64) -> bool {
    coordinate.abs() > MAX_PLAYER_COORDINATE
}

fn chunk_position_for_entity_position(position: EntityPosition) -> ChunkPosition {
    ChunkPosition::new(
        (position.x().floor() as i32).div_euclid(16),
        (position.z().floor() as i32).div_euclid(16),
    )
}

fn block_entity_type_for_block(block: Block) -> Option<BlockEntityType> {
    let block_name = format!("{block:?}").to_ascii_lowercase();
    let block_name = block_name.as_str();
    if block_name.ends_with("_bed") {
        return Some(BlockEntityType::Bed);
    }
    if block_name.ends_with("_banner") || block_name.ends_with("_wall_banner") {
        return Some(BlockEntityType::Banner);
    }
    if block_name.ends_with("_hanging_sign") || block_name.ends_with("_wall_hanging_sign") {
        return Some(BlockEntityType::HangingSign);
    }
    if block_name.ends_with("_sign") || block_name.ends_with("_wall_sign") {
        return Some(BlockEntityType::Sign);
    }
    if block_name.ends_with("_shulker_box") {
        return Some(BlockEntityType::ShulkerBox);
    }
    if block_name.ends_with("_head")
        || block_name.ends_with("_wall_head")
        || block_name.ends_with("_skull")
        || block_name.ends_with("_wall_skull")
    {
        return Some(BlockEntityType::Skull);
    }
    match block_name {
        "barrel" => Some(BlockEntityType::Barrel),
        "beacon" => Some(BlockEntityType::Beacon),
        "beehive" | "bee_nest" => Some(BlockEntityType::Beehive),
        "bell" => Some(BlockEntityType::Bell),
        "blast_furnace" => Some(BlockEntityType::BlastFurnace),
        "brewing_stand" => Some(BlockEntityType::BrewingStand),
        "calibrated_sculk_sensor" => Some(BlockEntityType::CalibratedSculkSensor),
        "campfire" => Some(BlockEntityType::Campfire),
        "chest" => Some(BlockEntityType::Chest),
        "chiseled_bookshelf" => Some(BlockEntityType::ChiseledBookshelf),
        "comparator" => Some(BlockEntityType::Comparator),
        "conduit" => Some(BlockEntityType::Conduit),
        "crafter" => Some(BlockEntityType::Crafter),
        "decorated_pot" => Some(BlockEntityType::DecoratedPot),
        "dispenser" => Some(BlockEntityType::Dispenser),
        "dropper" => Some(BlockEntityType::Dropper),
        "enchanting_table" => Some(BlockEntityType::EnchantingTable),
        "ender_chest" => Some(BlockEntityType::EnderChest),
        "end_gateway" => Some(BlockEntityType::EndGateway),
        "end_portal" => Some(BlockEntityType::EndPortal),
        "furnace" => Some(BlockEntityType::Furnace),
        "hopper" => Some(BlockEntityType::Hopper),
        "jigsaw" => Some(BlockEntityType::Jigsaw),
        "jukebox" => Some(BlockEntityType::Jukebox),
        "lectern" => Some(BlockEntityType::Lectern),
        "mob_spawner" => Some(BlockEntityType::MobSpawner),
        "piston" => Some(BlockEntityType::Piston),
        "sculk_catalyst" => Some(BlockEntityType::SculkCatalyst),
        "sculk_sensor" => Some(BlockEntityType::SculkSensor),
        "sculk_shrieker" => Some(BlockEntityType::SculkShrieker),
        "smoker" => Some(BlockEntityType::Smoker),
        "structure_block" => Some(BlockEntityType::StructureBlock),
        "trapped_chest" => Some(BlockEntityType::TrappedChest),
        _ => None,
    }
}

fn block_is_air(block: Block) -> bool {
    matches!(block, Block::AIR | Block::CAVE_AIR | Block::VOID_AIR)
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
    Weather::new(rain_level, thunder_level)
}

fn dispatch_player_spawn_event(
    player: *mut Player,
    world_name: Identifier,
    first_spawn: bool,
    client: &mut Client,
) {
    let Some(server_ptr) = client.server_ptr else {
        return;
    };
    let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
    PlayerSpawnEvent::new(player, world_name, first_spawn).dispatch(server, client);
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

#[cfg(test)]
mod tests {
    use super::World;
    use crate::entity::{Entity, EntityId, EntityPosition, Player, PlayerChunk};
    use crate::events::player_move::PlayerMoveEvent;
    use crate::network::client::instance::Client;
    use crate::server::MinecraftServer;
    use crate::world::{
        Biome, Block, BlockPosition, Chunk, ChunkLoader, ChunkPosition, GenerateChunkError,
        SetChunkBlockResult,
    };
    use spinel_core::network::clientbound::play::disconnect::PlayDisconnectPacket;
    use spinel_core::network::clientbound::play::forget_level_chunk::ForgetLevelChunkPacket;
    use spinel_core::network::clientbound::play::sync_player_pos::SyncPlayerPositionPacket;
    use spinel_macros::event_listener;
    use spinel_network::types::Identifier;
    use spinel_network::{ConnectionState, DataType, VarIntWrapper};
    use spinel_registry::EntityType;
    use spinel_registry::Registries;
    use spinel_registry::dimension_type::DimensionType;
    use std::io::{self, Cursor, Error, ErrorKind, Read};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
    use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::{Duration, Instant};
    use uuid::Uuid;

    const PLAYER_MOVE_TEST_PASSTHROUGH: i32 = 0;
    const PLAYER_MOVE_TEST_CANCEL: i32 = 1;
    const PLAYER_MOVE_TEST_VIEW: i32 = 2;
    const PLAYER_MOVE_TEST_COORDINATES: i32 = 3;
    const PLAYER_MOVE_TEST_TELEPORT: i32 = 4;

    static PLAYER_MOVE_TEST_BEHAVIOR: AtomicI32 = AtomicI32::new(PLAYER_MOVE_TEST_PASSTHROUGH);
    static PLAYER_MOVE_TEST_LOCK: Mutex<()> = Mutex::new(());

    #[event_listener]
    fn player_move_test_listener(event: &mut PlayerMoveEvent, _server: &mut MinecraftServer) {
        match PLAYER_MOVE_TEST_BEHAVIOR.load(Ordering::SeqCst) {
            PLAYER_MOVE_TEST_CANCEL => event.set_cancelled(true),
            PLAYER_MOVE_TEST_VIEW => {
                let packet_position = event.new_position();
                event.set_new_position(EntityPosition::new(
                    packet_position.x(),
                    packet_position.y(),
                    packet_position.z(),
                    90.0,
                    45.0,
                ));
            }
            PLAYER_MOVE_TEST_COORDINATES => {
                let packet_position = event.new_position();
                event.set_new_position(EntityPosition::new(
                    packet_position.x() + 1.0,
                    packet_position.y(),
                    packet_position.z(),
                    packet_position.yaw(),
                    packet_position.pitch(),
                ));
            }
            PLAYER_MOVE_TEST_TELEPORT => event
                .player()
                .set_position(EntityPosition::new(8.0, 64.0, 8.0, 0.0, 0.0)),
            _ => {}
        }
    }

    struct PlayerMoveBehaviorScope<'a> {
        _lock: std::sync::MutexGuard<'a, ()>,
    }

    impl Drop for PlayerMoveBehaviorScope<'_> {
        fn drop(&mut self) {
            PLAYER_MOVE_TEST_BEHAVIOR.store(PLAYER_MOVE_TEST_PASSTHROUGH, Ordering::SeqCst);
        }
    }

    struct FailingChunkLoader;

    impl ChunkLoader for FailingChunkLoader {
        fn load_chunk(&self, _position: ChunkPosition) -> io::Result<Option<Chunk>> {
            Err(Error::new(ErrorKind::Other, "load failed"))
        }

        fn save_chunk(&self, _chunk: &Chunk) -> io::Result<()> {
            Err(Error::new(ErrorKind::Other, "save failed"))
        }

        fn unload_chunk(&self, _chunk: &mut Chunk) -> io::Result<()> {
            Err(Error::new(ErrorKind::Other, "unload failed"))
        }
    }

    fn player_move_behavior_scope(behavior: i32) -> PlayerMoveBehaviorScope<'static> {
        let lock = PLAYER_MOVE_TEST_LOCK.lock().unwrap();
        PLAYER_MOVE_TEST_BEHAVIOR.store(behavior, Ordering::SeqCst);
        PlayerMoveBehaviorScope { _lock: lock }
    }

    fn test_client_pair() -> (Client, TcpStream) {
        let listener =
            TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
        let addr = listener.local_addr().unwrap();
        let stream = std::net::TcpStream::connect(addr).unwrap();
        let (peer_stream, _) = listener.accept().unwrap();
        peer_stream
            .set_read_timeout(Some(Duration::from_secs(1)))
            .unwrap();
        let mut client = Client::new(stream, addr);
        client.state = ConnectionState::Play;
        (client, peer_stream)
    }

    fn read_packet_frame(peer_stream: &mut TcpStream) -> (i32, Vec<u8>) {
        let frame_length = VarIntWrapper::decode(peer_stream).unwrap().0 as usize;
        let mut frame = vec![0; frame_length];
        peer_stream.read_exact(&mut frame).unwrap();
        let mut frame_cursor = Cursor::new(frame);
        let packet_id = VarIntWrapper::decode(&mut frame_cursor).unwrap().0;
        let payload_start = frame_cursor.position() as usize;
        let payload = frame_cursor.into_inner()[payload_start..].to_vec();
        (packet_id, payload)
    }

    fn world_with_entered_player(client: &mut Client) -> World {
        let mut world = World::new(Identifier::minecraft("overworld"));
        let mut player = Player::new(Uuid::nil(), "Player".to_string(), 0, client.addr);
        player.set_client(client);
        player.set_world(world.uuid());
        player.set_position(EntityPosition::new(0.0, 64.0, 0.0, 0.0, 0.0));
        player.mark_entered_world();
        world.add_entity(Entity::Player(player));
        world
    }

    fn attach_server_to_client(server: &mut MinecraftServer, client: &mut Client) {
        client.server_ptr = Some(server as *mut MinecraftServer as usize);
    }

    fn player_position(world: &World, client: &Client) -> EntityPosition {
        world.player_by_addr(&client.addr).unwrap().position()
    }

    #[test]
    fn optional_chunk_load_respects_auto_chunk_loading_like_minestom() {
        let mut world = World::new(Identifier::minecraft("overworld"));
        let chunk_position = ChunkPosition::new(2, 3);

        world.enable_auto_chunk_load(false);

        assert!(world.load_optional_chunk(chunk_position).is_none());
        assert!(world.chunk(chunk_position).is_none());

        world.enable_auto_chunk_load(true);

        assert!(world.load_optional_chunk(chunk_position).is_some());
        assert!(world.chunk(chunk_position).is_some());
    }

    #[test]
    fn chunk_unload_missing_chunk_is_minestom_noop() {
        let mut world = World::new(Identifier::minecraft("overworld"));

        assert!(!world.unload_chunk(ChunkPosition::new(4, 5)).unwrap());
    }

    #[test]
    fn explicit_chunk_unload_sends_forget_packet_to_loaded_viewer() {
        let (mut client, mut peer_stream) = test_client_pair();
        let mut world = world_with_entered_player(&mut client);
        let registries = Registries::new_vanilla();

        world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
        let packet = world
            .chunk(ChunkPosition::new(0, 0))
            .unwrap()
            .full_data_packet(&registries)
            .unwrap();
        let player = world.player_by_addr_mut(&client.addr).unwrap();
        player.send_chunk(packet);
        player.send_pending_chunks().unwrap();
        let _ = read_packet_frame(&mut peer_stream);
        let _ = read_packet_frame(&mut peer_stream);
        let _ = read_packet_frame(&mut peer_stream);
        let _ = read_packet_frame(&mut peer_stream);

        assert!(world.unload_chunk(ChunkPosition::new(0, 0)).unwrap());

        let (packet_id, _) = read_packet_frame(&mut peer_stream);

        assert_eq!(packet_id, ForgetLevelChunkPacket::get_id());
    }

    #[test]
    fn fast_linear_movement_loads_full_minestom_view_radius() {
        let (mut client, _peer_stream) = test_client_pair();
        let mut world = world_with_entered_player(&mut client);
        let registries = Registries::new_vanilla();
        world.set_view_distance(1);
        world.set_generator(|unit| {
            unit.modifier().fill_height(-64, -63, Block::STONE);
        });

        world
            .move_player(&mut client, 80.0, 64.0, 0.0, true, &registries)
            .unwrap();
        world.process_pending_player_chunk_loads().unwrap();

        PlayerChunk::new(5, 0)
            .surrounding(2)
            .into_iter()
            .for_each(|chunk| {
                assert!(
                    world.is_chunk_loaded(ChunkPosition::from(chunk)),
                    "chunk {chunk:?} was not loaded"
                );
            });
    }

    #[test]
    fn sharp_turn_before_chunk_load_completion_loads_latest_view_only() {
        let (mut client, _peer_stream) = test_client_pair();
        let mut world = world_with_entered_player(&mut client);
        let registries = Registries::new_vanilla();
        world.set_view_distance(1);
        world.set_generator(|unit| {
            unit.modifier().fill_height(-64, -63, Block::STONE);
        });

        world
            .move_player(&mut client, 80.0, 64.0, 0.0, true, &registries)
            .unwrap();
        world
            .move_player(&mut client, 0.0, 64.0, 0.0, true, &registries)
            .unwrap();
        world.process_pending_player_chunk_loads().unwrap();

        PlayerChunk::new(0, 0)
            .surrounding(2)
            .into_iter()
            .for_each(|chunk| {
                assert!(
                    world.is_chunk_loaded(ChunkPosition::from(chunk)),
                    "chunk {chunk:?} was not loaded"
                );
            });
        PlayerChunk::new(5, 0)
            .surrounding(2)
            .into_iter()
            .filter(|chunk| !chunk.is_within_view_distance(PlayerChunk::new(0, 0), 2))
            .for_each(|chunk| {
                assert!(
                    !world.is_chunk_loaded(ChunkPosition::from(chunk)),
                    "stale trail chunk {chunk:?} was loaded"
                );
            });
    }

    #[test]
    #[ignore]
    fn measure_fast_movement_load_vs_throttled_send_cost() {
        let (mut client, mut peer_stream) = test_client_pair();
        let mut world = world_with_entered_player(&mut client);
        let registries = Registries::new_vanilla();
        world.set_view_distance(4);
        world.set_generator(|unit| {
            unit.modifier().fill_height(-64, 64, Block::STONE);
        });
        let drain_thread = thread::spawn(move || {
            let mut drained_bytes = 0usize;
            let mut buffer = [0u8; 8192];
            loop {
                match peer_stream.read(&mut buffer) {
                    Ok(0) => return drained_bytes,
                    Ok(bytes_read) => drained_bytes += bytes_read,
                    Err(error)
                        if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) =>
                    {
                        return drained_bytes;
                    }
                    Err(_) => return drained_bytes,
                }
            }
        });

        let movement_start = Instant::now();
        world
            .move_player(&mut client, 320.0, 64.0, 0.0, true, &registries)
            .unwrap();
        let movement_elapsed = movement_start.elapsed();
        let loaded_chunks = world.chunks().count();
        let queued_before_tick = world
            .player_by_addr(&client.addr)
            .unwrap()
            .queued_chunk_count();

        let tick_start = Instant::now();
        world.tick_with_registries(&registries);
        let tick_elapsed = tick_start.elapsed();
        let player = world.player_by_addr(&client.addr).unwrap();
        let queued_after_tick = player.queued_chunk_count();
        let chunk_batch_lead = player.chunk_batch_lead();
        let target_chunks_per_tick = player.target_chunks_per_tick();
        drop(world);
        drop(client);
        let drained_bytes = drain_thread.join().unwrap();

        println!(
            "loaded_chunks={} queued_before_tick={} move_load={:?} first_throttled_tick={:?} queued_after_tick={} chunk_batch_lead={} target_chunks_per_tick={} drained_bytes={}",
            loaded_chunks,
            queued_before_tick,
            movement_elapsed,
            tick_elapsed,
            queued_after_tick,
            chunk_batch_lead,
            target_chunks_per_tick,
            drained_bytes
        );
    }

    #[test]
    #[ignore]
    fn measure_fast_client_movement_chunk_generation_per_second() {
        let (mut client, mut peer_stream) = test_client_pair();
        let mut world = world_with_entered_player(&mut client);
        let registries = Registries::new_vanilla();
        let generated_chunks = Arc::new(AtomicUsize::new(0));
        let generated_chunk_counter = Arc::clone(&generated_chunks);
        world.set_view_distance(4);
        world.set_generator(move |unit| {
            generated_chunk_counter.fetch_add(1, Ordering::SeqCst);
            unit.modifier().fill_height(0, 3, Block::GRASS_BLOCK);
        });
        let drain_thread = thread::spawn(move || {
            let mut drained_bytes = 0usize;
            let mut buffer = [0u8; 16384];
            loop {
                match peer_stream.read(&mut buffer) {
                    Ok(0) => return drained_bytes,
                    Ok(bytes_read) => drained_bytes += bytes_read,
                    Err(error)
                        if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) =>
                    {
                        return drained_bytes;
                    }
                    Err(_) => return drained_bytes,
                }
            }
        });

        let movement_count = 40;
        let movement_start = Instant::now();
        for movement_step in 1..=movement_count {
            world
                .move_player(
                    &mut client,
                    movement_step as f64 * 24.0,
                    64.0,
                    0.0,
                    true,
                    &registries,
                )
                .unwrap();
            world
                .player_by_addr_mut(&client.addr)
                .unwrap()
                .on_chunk_batch_received(64.0);
            world.tick_with_registries(&registries);
        }
        let movement_elapsed = movement_start.elapsed();
        let generated_chunk_count = generated_chunks.load(Ordering::SeqCst);
        let generated_chunks_per_second =
            generated_chunk_count as f64 / movement_elapsed.as_secs_f64();
        let loaded_chunks = world.chunks().count();
        let player = world.player_by_addr(&client.addr).unwrap();
        let queued_chunks = player.queued_chunk_count();
        let chunk_batch_lead = player.chunk_batch_lead();
        let target_chunks_per_tick = player.target_chunks_per_tick();
        drop(world);
        drop(client);
        let drained_bytes = drain_thread.join().unwrap();

        println!(
            "movement_count={} generated_chunks={} elapsed={:?} generated_chunks_per_second={:.2} loaded_chunks={} queued_chunks={} chunk_batch_lead={} target_chunks_per_tick={} drained_bytes={}",
            movement_count,
            generated_chunk_count,
            movement_elapsed,
            generated_chunks_per_second,
            loaded_chunks,
            queued_chunks,
            chunk_batch_lead,
            target_chunks_per_tick,
            drained_bytes
        );
    }

    #[test]
    #[ignore]
    fn measure_fast_client_movement_chunk_pipeline_split() {
        let (mut client, mut peer_stream) = test_client_pair();
        let mut world = world_with_entered_player(&mut client);
        let registries = Registries::new_vanilla();
        let generated_chunks = Arc::new(AtomicUsize::new(0));
        let generated_chunk_counter = Arc::clone(&generated_chunks);
        world.set_view_distance(4);
        world.set_generator(move |unit| {
            generated_chunk_counter.fetch_add(1, Ordering::SeqCst);
            unit.modifier().fill_height(0, 3, Block::GRASS_BLOCK);
        });
        let drain_thread = thread::spawn(move || {
            let mut drained_bytes = 0usize;
            let mut buffer = [0u8; 16384];
            loop {
                match peer_stream.read(&mut buffer) {
                    Ok(0) => return drained_bytes,
                    Ok(bytes_read) => drained_bytes += bytes_read,
                    Err(error)
                        if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) =>
                    {
                        return drained_bytes;
                    }
                    Err(_) => return drained_bytes,
                }
            }
        });

        let movement_count = 40;
        let mut movement_elapsed = Duration::ZERO;
        let mut pending_load_elapsed = Duration::ZERO;
        let mut pending_send_elapsed = Duration::ZERO;
        let pipeline_start = Instant::now();
        for movement_step in 1..=movement_count {
            let movement_start = Instant::now();
            world
                .move_player(
                    &mut client,
                    movement_step as f64 * 24.0,
                    64.0,
                    0.0,
                    true,
                    &registries,
                )
                .unwrap();
            movement_elapsed += movement_start.elapsed();

            let pending_load_start = Instant::now();
            world.process_pending_player_chunk_loads().unwrap();
            pending_load_elapsed += pending_load_start.elapsed();

            let pending_send_start = Instant::now();
            world
                .send_pending_chunks_for_player_address(client.addr, &registries)
                .unwrap();
            pending_send_elapsed += pending_send_start.elapsed();

            world
                .player_by_addr_mut(&client.addr)
                .unwrap()
                .on_chunk_batch_received(64.0);
        }
        let pipeline_elapsed = pipeline_start.elapsed();
        let generated_chunk_count = generated_chunks.load(Ordering::SeqCst);
        let generated_chunks_per_second =
            generated_chunk_count as f64 / pipeline_elapsed.as_secs_f64();
        let loaded_chunks = world.chunks().count();
        let player = world.player_by_addr(&client.addr).unwrap();
        let queued_chunks = player.queued_chunk_count();
        drop(world);
        drop(client);
        let drained_bytes = drain_thread.join().unwrap();

        println!(
            "movement_count={} generated_chunks={} generated_chunks_per_second={:.2} total={:?} movement={:?} pending_load={:?} pending_send={:?} loaded_chunks={} queued_chunks={} drained_bytes={}",
            movement_count,
            generated_chunk_count,
            generated_chunks_per_second,
            pipeline_elapsed,
            movement_elapsed,
            pending_load_elapsed,
            pending_send_elapsed,
            loaded_chunks,
            queued_chunks,
            drained_bytes
        );
    }

    #[test]
    fn too_large_player_coordinate_kicks_player_like_minestom() {
        let (mut client, mut peer_stream) = test_client_pair();
        let mut world = world_with_entered_player(&mut client);
        let registries = Registries::new_vanilla();

        world
            .move_player(&mut client, 30_000_001.0, 64.0, 0.0, true, &registries)
            .unwrap();

        let (packet_id, _) = read_packet_frame(&mut peer_stream);

        assert_eq!(packet_id, PlayDisconnectPacket::get_id());
        assert!(!client.is_online());
    }

    #[test]
    fn movement_is_suppressed_while_teleport_confirmation_is_pending() {
        let (mut client, mut peer_stream) = test_client_pair();
        let mut world = world_with_entered_player(&mut client);
        let registries = Registries::new_vanilla();

        world
            .player_by_addr_mut(&client.addr)
            .unwrap()
            .next_teleport_id();
        world
            .move_player(&mut client, 1.0, 64.0, 0.0, true, &registries)
            .unwrap();

        peer_stream
            .set_read_timeout(Some(Duration::from_millis(25)))
            .unwrap();
        let mut trailing_packet_byte = [0u8; 1];

        assert_eq!(player_position(&world, &client).x(), 0.0);
        assert!(peer_stream.read(&mut trailing_packet_byte).is_err());
    }

    #[test]
    fn movement_into_unloaded_destination_chunk_teleports_player_back() {
        let (mut client, mut peer_stream) = test_client_pair();
        let mut world = world_with_entered_player(&mut client);
        let registries = Registries::new_vanilla();

        world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
        world.enable_auto_chunk_load(false);
        world
            .move_player(&mut client, 16.0, 64.0, 0.0, true, &registries)
            .unwrap();

        let (packet_id, _) = read_packet_frame(&mut peer_stream);

        assert_eq!(packet_id, SyncPlayerPositionPacket::get_id());
        assert_eq!(player_position(&world, &client).x(), 0.0);
    }

    #[test]
    fn cancelled_player_move_event_sends_correction_packet() {
        let _scope = player_move_behavior_scope(PLAYER_MOVE_TEST_CANCEL);
        let (mut client, mut peer_stream) = test_client_pair();
        let mut server = MinecraftServer::new();
        attach_server_to_client(&mut server, &mut client);
        let mut world = world_with_entered_player(&mut client);
        let registries = Registries::new_vanilla();

        world
            .move_player(&mut client, 1.0, 64.0, 0.0, true, &registries)
            .unwrap();

        let (packet_id, _) = read_packet_frame(&mut peer_stream);

        assert_eq!(packet_id, SyncPlayerPositionPacket::get_id());
        assert_eq!(player_position(&world, &client).x(), 0.0);
    }

    #[test]
    fn player_move_event_mutated_yaw_and_pitch_update_player_view() {
        let _scope = player_move_behavior_scope(PLAYER_MOVE_TEST_VIEW);
        let (mut client, mut peer_stream) = test_client_pair();
        let mut server = MinecraftServer::new();
        attach_server_to_client(&mut server, &mut client);
        let mut world = world_with_entered_player(&mut client);
        let registries = Registries::new_vanilla();

        world
            .move_player_with_view(&mut client, 1.0, 64.0, 0.0, 0.0, 0.0, true, &registries)
            .unwrap();

        peer_stream
            .set_read_timeout(Some(Duration::from_millis(25)))
            .unwrap();
        let mut trailing_packet_byte = [0u8; 1];
        let position = player_position(&world, &client);

        assert_eq!(position.x(), 1.0);
        assert_eq!(position.yaw(), 90.0);
        assert_eq!(position.pitch(), 45.0);
        assert!(peer_stream.read(&mut trailing_packet_byte).is_err());
    }

    #[test]
    fn player_move_event_mutated_coordinates_teleport_player_to_event_position() {
        let _scope = player_move_behavior_scope(PLAYER_MOVE_TEST_COORDINATES);
        let (mut client, mut peer_stream) = test_client_pair();
        let mut server = MinecraftServer::new();
        attach_server_to_client(&mut server, &mut client);
        let mut world = world_with_entered_player(&mut client);
        let registries = Registries::new_vanilla();

        world
            .move_player(&mut client, 1.0, 64.0, 0.0, true, &registries)
            .unwrap();

        let (packet_id, _) = read_packet_frame(&mut peer_stream);

        assert_eq!(packet_id, SyncPlayerPositionPacket::get_id());
        assert_eq!(player_position(&world, &client).x(), 2.0);
    }

    #[test]
    fn player_move_event_triggered_teleport_stops_original_movement() {
        let _scope = player_move_behavior_scope(PLAYER_MOVE_TEST_TELEPORT);
        let (mut client, mut peer_stream) = test_client_pair();
        let mut server = MinecraftServer::new();
        attach_server_to_client(&mut server, &mut client);
        let mut world = world_with_entered_player(&mut client);
        let registries = Registries::new_vanilla();

        world
            .move_player(&mut client, 1.0, 64.0, 0.0, true, &registries)
            .unwrap();

        peer_stream
            .set_read_timeout(Some(Duration::from_millis(25)))
            .unwrap();
        let mut trailing_packet_byte = [0u8; 1];

        assert_eq!(player_position(&world, &client).x(), 8.0);
        assert!(peer_stream.read(&mut trailing_packet_byte).is_err());
    }

    #[test]
    fn chunk_loader_errors_propagate_through_fallible_load_api() {
        let mut world = World::new(Identifier::minecraft("overworld"));
        world.set_chunk_loader(FailingChunkLoader);

        let load_error = match world.load_chunk(ChunkPosition::new(1, 1)) {
            Ok(_) => panic!("loader error should propagate"),
            Err(error) => error,
        };

        assert_eq!(load_error.kind(), ErrorKind::Other);
        assert!(world.chunk(ChunkPosition::new(1, 1)).is_none());
    }

    #[test]
    fn loader_miss_uses_world_chunk_supplier_like_minestom() {
        let mut world = World::new(Identifier::minecraft("overworld"));
        world.set_chunk_supplier(|_| Chunk::new(ChunkPosition::new(7, -9)));

        let chunk = world.load_chunk(ChunkPosition::new(1, 1)).unwrap();

        assert_eq!(chunk.x(), 7);
        assert_eq!(chunk.z(), -9);
        assert_eq!(
            world
                .chunk_supplier()
                .create_chunk(ChunkPosition::new(3, 4))
                .x(),
            7
        );
    }

    #[test]
    fn in_flight_chunk_load_guard_rejects_duplicate_reentry() {
        let mut world = World::new(Identifier::minecraft("overworld"));
        let chunk_position = ChunkPosition::new(1, 1);
        world.loading_chunks.insert(chunk_position);

        let load_error = match world.load_chunk(chunk_position) {
            Ok(_) => panic!("in-flight chunk load should reject duplicate reentry"),
            Err(error) => error,
        };

        assert_eq!(load_error.kind(), ErrorKind::WouldBlock);
        assert!(world.chunk(chunk_position).is_none());
    }

    #[test]
    fn generation_errors_propagate_through_fallible_chunk_load_api() {
        let mut world = World::new(Identifier::minecraft("overworld"));
        world.set_fallible_generator(|_| {
            Err(GenerateChunkError::GeneratorFailed {
                reason: "test generator failed".to_string(),
            })
        });

        let load_error = match world.load_chunk(ChunkPosition::new(1, 1)) {
            Ok(_) => panic!("generator error should propagate"),
            Err(error) => error,
        };

        assert_eq!(load_error.kind(), ErrorKind::Other);
        assert!(world.chunk(ChunkPosition::new(1, 1)).is_some());
        assert!(
            world
                .chunk(ChunkPosition::new(1, 1))
                .is_some_and(Chunk::should_generate)
        );
    }

    #[test]
    fn chunk_state_accessors_match_minestom_chunk_api_capability() {
        let mut chunk = Chunk::new(ChunkPosition::new(2, -3));
        let copied_chunk = chunk.copy_for_position(ChunkPosition::new(4, 5));
        let registries = spinel_registry::Registries::new_vanilla();

        assert_ne!(chunk.identifier(), copied_chunk.identifier());
        assert_eq!(chunk.x(), 2);
        assert_eq!(chunk.z(), -3);
        assert_eq!(chunk.min_section(), -4);
        assert_eq!(chunk.max_section(), 20);
        assert_eq!(
            chunk.world_position(),
            crate::world::BlockPosition::new(32, 0, -48)
        );
        assert!(chunk.section_at_block_y(-64).is_some());
        assert!(chunk.section_at_block_y(319).is_some());
        assert!(chunk.section_at_block_y(320).is_none());
        assert!(chunk.section(-4).is_some());
        assert!(chunk.section(20).is_none());

        assert_eq!(
            chunk.try_set_block(crate::world::BlockPosition::new(1, 64, 1), Block::STONE),
            SetChunkBlockResult::Changed
        );
        assert!(chunk.is_invalidated());
        let chunk_packet = chunk.full_data_packet(&registries).unwrap();

        assert_eq!(chunk_packet.chunk_x, 2);
        assert_eq!(chunk_packet.chunk_z, -3);
        assert_eq!(chunk_packet.chunk_data.sections.len(), 24);
        chunk.reset();

        assert_eq!(
            chunk.block(crate::world::BlockPosition::new(1, 64, 1)),
            Block::AIR
        );
        assert!(chunk.is_invalidated());
        assert!(chunk.should_generate());
        chunk.on_generate();
        assert!(!chunk.should_generate());

        chunk.set_read_only(true);

        assert_eq!(
            chunk.try_set_block(crate::world::BlockPosition::new(1, 64, 1), Block::STONE),
            SetChunkBlockResult::ReadOnly
        );
        assert!(chunk.section_mut(4).is_none());
    }

    #[test]
    fn chunk_biome_accessors_invalidate_cached_chunk_data() {
        let registries = spinel_registry::Registries::new_vanilla();
        let mut chunk = Chunk::new(ChunkPosition::new(0, 0));

        let empty_chunk_data = chunk.data(&registries).unwrap();

        assert_eq!(
            chunk.biome(crate::world::BlockPosition::new(0, 64, 0)),
            spinel_registry::Biome::PLAINS
        );
        assert!(chunk.set_biome(
            crate::world::BlockPosition::new(0, 64, 0),
            spinel_registry::Biome::BADLANDS
        ));

        let changed_chunk_data = chunk.data(&registries).unwrap();
        let empty_biomes = &empty_chunk_data.sections[8].biomes;
        let changed_biomes = &changed_chunk_data.sections[8].biomes;

        assert_ne!(empty_biomes.bits_per_entry, changed_biomes.bits_per_entry);
        assert_ne!(empty_biomes.palette, changed_biomes.palette);

        chunk.fill_biome(spinel_registry::Biome::BAMBOO_JUNGLE);

        assert_eq!(
            chunk.biome(crate::world::BlockPosition::new(4, 64, 4)),
            spinel_registry::Biome::BAMBOO_JUNGLE
        );
    }

    #[test]
    fn chunk_cache_and_heightmaps_invalidate_after_block_mutation() {
        let registries = spinel_registry::Registries::new_vanilla();
        let mut chunk = Chunk::new(ChunkPosition::new(0, 0));

        let empty_chunk_data = chunk.data(&registries).unwrap();

        assert_eq!(empty_chunk_data.sections[4].block_count, 0);

        assert_eq!(
            chunk.try_set_block(crate::world::BlockPosition::new(1, 64, 1), Block::STONE),
            SetChunkBlockResult::Changed
        );

        let changed_chunk_data = chunk.data(&registries).unwrap();

        assert_eq!(changed_chunk_data.sections[8].block_count, 1);
        let empty_heightmap_data = empty_chunk_data
            .heightmaps
            .iter()
            .flat_map(|heightmap| heightmap.data.iter())
            .copied()
            .collect::<Vec<_>>();
        let changed_heightmap_data = changed_chunk_data
            .heightmaps
            .iter()
            .flat_map(|heightmap| heightmap.data.iter())
            .copied()
            .collect::<Vec<_>>();

        assert_ne!(empty_heightmap_data, changed_heightmap_data);
    }

    #[test]
    fn chunk_loads_heightmaps_from_nbt_like_minestom() {
        let mut chunk = Chunk::new(ChunkPosition::new(0, 0));
        let mut heightmaps = spinel_nbt::NbtCompound::new();
        heightmaps.insert(
            "MOTION_BLOCKING".to_string(),
            spinel_nbt::Nbt::LongArray(vec![1; 37].into_boxed_slice()),
        );

        chunk.load_heightmaps_from_nbt(&heightmaps);

        assert_eq!(
            &chunk.motion_blocking_heightmap()[0..36],
            vec![1; 36].as_slice()
        );
    }

    #[test]
    fn chunk_viewer_membership_matches_minestom_no_op_edges() {
        let mut chunk = Chunk::new(ChunkPosition::new(0, 0));
        let viewer = EntityId::next();

        assert!(chunk.add_viewer(viewer));
        assert!(!chunk.add_viewer(viewer));
        assert_eq!(chunk.viewers().collect::<Vec<_>>(), vec![viewer.value()]);
        assert!(chunk.remove_viewer(viewer));
        assert!(!chunk.remove_viewer(viewer));
        assert!(chunk.viewers().next().is_none());
    }

    #[test]
    fn chunk_light_data_has_full_chunk_section_shape() {
        let chunk = Chunk::new(ChunkPosition::new(0, 0));
        let light_data = chunk.light_data();

        assert_eq!(light_data.sky_light_mask, vec![0x01ff_fffe]);
        assert_eq!(light_data.block_light_mask, vec![0x01ff_fffe]);
        assert_eq!(light_data.empty_sky_light_mask, vec![0x0200_0001]);
        assert_eq!(light_data.empty_block_light_mask, vec![0x0200_0001]);
        assert_eq!(light_data.sky_light_arrays.len(), 24);
        assert_eq!(light_data.block_light_arrays.len(), 24);
        assert!(
            light_data
                .sky_light_arrays
                .iter()
                .all(|section_light| section_light.len() == 2048)
        );
        assert!(
            light_data
                .block_light_arrays
                .iter()
                .all(|section_light| section_light.len() == 2048)
        );
    }

    #[test]
    fn added_entities_record_their_current_world_membership() {
        let mut world = World::new(Identifier::minecraft("overworld"));
        let world_uuid = world.uuid;
        let entity = Entity::new(EntityType::ZOMBIE);

        world.add_entity(entity);

        assert_eq!(world.entities[0].world(), Some(world_uuid));
    }

    #[test]
    fn world_dimension_registration_and_void_api_match_minestom_instance_surface() {
        let dimension_type = DimensionType::THE_NETHER;
        let cached_dimension_type = DimensionType::builder()
            .vertical_bounds(-32, 256, 128)
            .build();
        let world = World::new_with_dimension(
            Identifier::minecraft("the_nether"),
            dimension_type.clone(),
            cached_dimension_type,
        );

        assert!(!world.is_registered());
        assert_eq!(world.dimension_type(), &dimension_type);
        assert_eq!(world.cached_dimension_type().min_y, -32);
        assert_eq!(world.dimension_name(), &Identifier::minecraft("the_nether"));
        assert!(world.is_in_void(EntityPosition::new(0.0, -97.0, 0.0, 0.0, 0.0)));
        assert!(!world.is_in_void(EntityPosition::new(0.0, -96.0, 0.0, 0.0, 0.0)));
    }

    #[test]
    fn world_entity_and_player_lookup_api_matches_minestom_instance_surface() {
        let mut world = World::new(Identifier::minecraft("overworld"));
        let generic_entity = Entity::new(EntityType::ZOMBIE);
        let generic_entity_id = generic_entity.entity_id();
        let generic_entity_uuid = generic_entity.uuid();
        let player_socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565);
        let player = Player::new(Uuid::nil(), "Player".to_string(), 0, player_socket);
        let player_id = player.entity_id();
        let player_uuid = player.uuid();

        world.add_entity(generic_entity);
        world.add_entity(Entity::Player(player));

        assert_eq!(world.entities().count(), 2);
        assert_eq!(
            world.entity_by_id(generic_entity_id).map(Entity::uuid),
            Some(generic_entity_uuid)
        );
        assert_eq!(
            world.entity_by_uuid(player_uuid).map(Entity::entity_id),
            Some(player_id)
        );
        assert_eq!(world.players().count(), 1);
        assert_eq!(
            world.player_by_uuid(player_uuid).map(Player::entity_id),
            Some(player_id)
        );
    }

    #[test]
    fn world_chunk_and_block_api_match_loaded_chunk_semantics() {
        let mut world = World::new(Identifier::minecraft("overworld"));
        let chunk_position = ChunkPosition::new(0, 0);

        assert_eq!(world.uuid(), world.uuid);
        assert_eq!(world.name(), &Identifier::minecraft("overworld"));
        assert!(!world.is_chunk_loaded(chunk_position));
        assert!(world.chunk_at(0.0, 0.0).is_none());
        assert!(
            world
                .loaded_block_at(crate::world::BlockPosition::new(1, 64, 1))
                .is_none()
        );

        let block = world
            .block_at(crate::world::BlockPosition::new(1, 64, 1))
            .unwrap();

        assert_eq!(block, Block::AIR);
        assert_eq!(
            world.biome_at(BlockPosition::new(1, 64, 1)).unwrap(),
            Biome::PLAINS
        );
        assert!(
            world
                .set_block(BlockPosition::new(1, 64, 1), Block::STONE)
                .unwrap()
        );
        assert!(world.is_chunk_loaded(chunk_position));
        assert!(world.chunk_at(0.0, 0.0).is_some());
        assert_eq!(world.chunks().count(), 1);
        assert_eq!(
            world.loaded_block_at(crate::world::BlockPosition::new(1, 64, 1)),
            Some(Block::STONE)
        );
        assert!(world.block_position_is_loaded(crate::world::BlockPosition::new(1, 64, 1)));
    }

    #[test]
    fn world_time_api_matches_minestom_defaults_and_validation() {
        let mut world = World::new(Identifier::minecraft("overworld"));

        assert_eq!(world.world_age(), 0);
        assert_eq!(world.time(), 0);
        assert_eq!(world.time_rate(), 1);
        assert_eq!(world.time_synchronization_ticks(), 20);
        assert_eq!(world.view_distance(), 8);
        assert_eq!(world.time_packet().world_age, 0);
        assert_eq!(world.time_packet().time, 0);
        assert!(world.time_packet().tick_day_time);

        world.set_world_age(40).unwrap();
        world.set_time(6000).unwrap();
        world.set_time_rate(0).unwrap();
        world.set_time_synchronization_ticks(0).unwrap();
        world.set_view_distance(12);

        assert_eq!(world.world_age(), 40);
        assert_eq!(world.time(), 6000);
        assert_eq!(world.time_rate(), 0);
        assert_eq!(world.time_synchronization_ticks(), 0);
        assert_eq!(world.view_distance(), 12);
        assert!(!world.time_packet().tick_day_time);
        assert!(world.set_time_rate(-1).is_err());
        assert!(world.set_time_synchronization_ticks(-1).is_err());
    }

    #[test]
    fn world_tick_advances_time_like_minestom() {
        let mut world = World::new(Identifier::minecraft("overworld"));

        world.tick();

        assert_eq!(world.world_age(), 1);
        assert_eq!(world.time(), 1);

        world.set_time_rate(0).unwrap();
        world.tick();

        assert_eq!(world.world_age(), 2);
        assert_eq!(world.time(), 1);
    }
}
