use crate::entity::ai::CreatureAiAction;
use crate::entity::physics::{EntityMovement, EntityMovementPacket};
use crate::entity::player::{PlayerSkin, PlayerViewerSnapshot};
use crate::entity::{
    Damage, Entity, EntityId, EntityPose, EntityPosition, EntityTeleport, EquipmentSlot,
    ExperienceOrb, GenericEntity, ItemEntity, Player, PlayerChunk, PlayerChunkTransition,
    TimedPotionEffect,
};
use crate::events::chunk_loader_error::ChunkLoaderErrorEvent;
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
use crate::events::instance::add_entity_to_instance::AddEntityToInstanceEvent;
use crate::events::instance::remove_entity_from_instance::RemoveEntityFromInstanceEvent;
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
use crate::events::world_block_update::WorldBlockUpdateEvent;
use crate::events::world_chunk_load::WorldChunkLoadEvent;
use crate::events::world_chunk_unload::WorldChunkUnloadEvent;
use crate::events::world_register::WorldRegisterEvent;
use crate::events::world_section_invalidate::WorldSectionInvalidateEvent;
use crate::events::world_tick::WorldTickEvent;
use crate::events::world_tick_end::WorldTickEndEvent;
use crate::events::world_unregister::WorldUnregisterEvent;
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
    ChunkLoader, ChunkLoaderFailure, ChunkLoaderOperation, ChunkPosition, ChunkSnapshot,
    EntityTracker, EntityTrackerTarget, ExplosionSupplier, GenerationUnit, NoopChunkLoader,
    Weather, WorldBorder, WorldEventNode, WorldIdentity, WorldPointers, WorldScheduler,
    WorldSnapshot, WorldSoundEmitter,
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
use spinel_registry::{
    EntityBoundingBox, EntityType, ItemStack, MobEffect, Registries, RegistryKey,
};
use spinel_utils::component::Component;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{Error, ErrorKind, Result};
use std::net::SocketAddr;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::thread::JoinHandle;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

const MAX_PLAYER_COORDINATE: f64 = 30_000_000.0;
const DEFAULT_TIME_SYNCHRONIZATION_TICKS: i32 = 20;
const DEFAULT_CHUNK_VIEW_DISTANCE: i32 = 8;
const AUTOMATIC_CHUNK_UNLOAD_GRACE_TICKS: u16 = 100;
const MAX_RETAINED_UNVIEWED_CHUNKS: usize = 1024;
const DESTROY_BLOCK_WORLD_EVENT_ID: i32 = 2001;
const ENTITY_VIEW_DISTANCE: i32 = 5;

pub struct World {
    pub uuid: Uuid,
    pub name: Identifier,
    entities: Vec<Entity>,
    entity_tracker: EntityTracker,
    chunks: HashMap<ChunkPosition, Chunk>,
    unviewed_chunk_ticks: HashMap<ChunkPosition, u16>,
    cached_snapshot_chunks: RefCell<Arc<HashMap<ChunkPosition, ChunkSnapshot>>>,
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
    prepared_chunk_loads: HashMap<
        u64,
        (
            ChunkLoadTicket,
            std::result::Result<PreparedChunkLoad, PreparedChunkLoadFailure>,
        ),
    >,
    next_chunk_load_ticket_id: u64,
    player_chunk_load_waiters: HashMap<ChunkPosition, Vec<SocketAddr>>,

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
    automatic_chunk_unload: bool,
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

include!("blocks.rs");
include!("border_control.rs");
include!("boss_bar_management.rs");
include!("chunk_lifecycle.rs");
include!("entities.rs");
include!("entity_movement.rs");
include!("entity_relationships.rs");
include!("explosions.rs");
include!("lifecycle.rs");
include!("living_entities.rs");
include!("packet_dispatch.rs");
include!("players.rs");
include!("projectiles.rs");
include!("scheduling.rs");
include!("scoreboard_teams.rs");
include!("ticking.rs");
include!("time_control.rs");
include!("visibility.rs");

impl Taggable for World {
    fn tag_handler(&self) -> &TagHandler {
        &self.tag_handler
    }

    fn tag_handler_mut(&mut self) -> &mut TagHandler {
        &mut self.tag_handler
    }
}
