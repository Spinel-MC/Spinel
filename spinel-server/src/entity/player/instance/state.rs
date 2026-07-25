use crate::entity::generic_entity::EntityAerodynamics;
use crate::entity::metadata::MetadataHolder;
use crate::entity::physics::EntityPhysicsResult;
use crate::entity::player::BelowNameTag;
use crate::entity::player::ChunkUpdateLimitChecker;
use crate::entity::player::PendingResourcePacks;
use crate::entity::player::chunks::PlayerChunk;
use crate::entity::player::input::PlayerInputs;
use crate::entity::player::position::PlayerPosition;
use crate::entity::player::skin::PlayerSkin;
use crate::entity::{
    EntityCollisionRules, EntityId, EntityLeash, EntityPosition, EntitySynchronization, EntityView,
    EquipmentSlot, LivingState, PlayerSpawnPoint,
};
use crate::inventory::{ClickPreprocessor, Inventory, PlayerInventory};
use crate::permission::{PermissionHandler, PermissionSet};
use crate::scheduler::ContextScheduler;
use spinel_core::entity::game_mode::GameMode;
use spinel_nbt::{TagHandler, Taggable};
use spinel_network::types::{ClientInformation, Identifier, Vector3d, Velocity};
use spinel_registry::dimension_type::DimensionType;
use spinel_registry::{EntityType, ItemStack, RegistryKey};
use spinel_utils::component::text::TextComponent;
use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};
use std::net::SocketAddr;
use uuid::Uuid;

use super::constants::*;
use super::death_location::PlayerDeathLocation;
use super::hand::PlayerHand;
use super::packet_queue::QueuedPlayerPacket;
use super::queued_chunk::QueuedPlayerChunk;

pub struct Player {
    pub(super) entity_id: EntityId,
    pub(super) entity_type: EntityType,
    pub uuid: Uuid,
    pub username: String,
    pub protocol_version: i32,
    pub addr: SocketAddr,
    pub(super) skin: Option<PlayerSkin>,
    pub(super) display_name: Option<TextComponent>,
    pub(super) below_name_tag: Option<BelowNameTag>,
    pub(super) listed: bool,
    pub(super) latency: i32,
    pub(crate) loaded_chunk: PlayerChunk,
    pub(crate) chunks_loaded_by_client: PlayerChunk,
    pub(in crate::entity::player) client_chunk_view_distance: i32,
    pub(in crate::entity::player) chunk_update_limit_checker: ChunkUpdateLimitChecker,
    pub(crate) position: PlayerPosition,
    pub(super) game_mode: GameMode,
    pub(super) pending_spawning_world: Option<Uuid>,
    pub(super) current_world: Option<Uuid>,
    pub(super) dimension_type: RegistryKey<DimensionType>,
    pub(super) world_name: Option<Identifier>,
    pub(super) hardcore: bool,
    pub(in crate::entity::player) living: LivingState,
    pub(super) food: i32,
    pub(super) food_saturation: f32,
    pub(super) death_location: Option<PlayerDeathLocation>,
    pub(super) enable_respawn_screen: bool,
    pub(super) experience: f32,
    pub(super) experience_level: i32,
    pub(super) total_experience: i32,
    pub(super) portal_cooldown: i32,
    pub(super) reduced_debug_screen_information: bool,
    pub(super) settings: ClientInformation,
    pub(super) permission_level: i32,
    pub(super) respawn_point: PlayerSpawnPoint,
    pub(super) inventory: PlayerInventory,
    pub(in crate::entity::player) attribute_equipment: BTreeMap<EquipmentSlot, ItemStack>,
    pub(super) open_inventory: Option<Inventory>,
    pub(super) anvil_rename_text: Option<String>,
    pub(super) debug_subscriptions: BTreeSet<i32>,
    pub(super) vehicle: Option<EntityId>,
    pub(super) velocity: Velocity,
    pub(super) passengers: BTreeSet<EntityId>,
    pub(super) leash: EntityLeash,
    pub(super) synchronization: EntitySynchronization,
    pub(super) vanished: bool,
    pub(in crate::entity::player) click_preprocessor: ClickPreprocessor,
    pub(super) held_slot: i32,
    pub(super) inputs: PlayerInputs,
    pub(super) flying: bool,
    pub(super) allow_flying: bool,
    pub(super) instant_break: bool,
    pub(super) has_entity_collision: bool,
    pub(super) prevents_block_placement: bool,
    pub(super) flying_speed: f32,
    pub(super) field_view_modifier: f32,
    pub(in crate::entity::player) metadata: MetadataHolder,
    pub(super) tag_handler: TagHandler,
    pub(super) has_entered_world: bool,
    pub(super) on_ground: bool,
    pub(super) aerodynamics: EntityAerodynamics,
    pub(super) gravity_tick_count: u64,
    pub(super) previous_physics_result: Option<EntityPhysicsResult>,
    pub(super) has_physics: bool,
    pub(super) last_sent_teleport_id: i32,
    pub(super) last_received_teleport_id: i32,
    pub(super) last_keep_alive: i64,
    pub(super) answer_keep_alive: bool,
    pub(in crate::entity::player) last_completed_client_tick: u64,
    pub(super) did_close_inventory: bool,
    pub(in crate::entity::player) client: Option<usize>,
    pub(super) statistics: BTreeMap<String, i32>,
    pub(in crate::entity::player) alive_ticks: u64,
    pub(super) delayed_remove_ticks: Option<u64>,
    pub(super) last_experience_pickup_tick: Option<i64>,
    pub(in crate::entity::player) item_use_hand: Option<PlayerHand>,
    pub(in crate::entity::player) start_item_use_time: u64,
    pub(in crate::entity::player) item_use_time: u64,
    pub(super) packet_queue: VecDeque<QueuedPlayerPacket>,
    pub(in crate::entity::player) pending_resource_packs: PendingResourcePacks,
    pub(in crate::entity::player) chunk_queue: VecDeque<QueuedPlayerChunk>,
    pub(in crate::entity::player) client_sent_chunks: HashSet<PlayerChunk>,
    pub(in crate::entity::player) chunk_queue_requires_sorting: bool,
    #[cfg(test)]
    pub(in crate::entity::player) chunk_queue_sort_count: usize,
    pub(in crate::entity::player) needs_chunk_position_sync: bool,
    pub(in crate::entity::player) max_chunk_batch_lead: i32,
    pub(in crate::entity::player) chunk_batch_lead: i32,
    pub(in crate::entity::player) target_chunks_per_tick: f32,
    pub(in crate::entity::player) pending_chunk_count: f32,
    pub(super) scheduler: ContextScheduler<Player>,
    pub(super) view: EntityView,
    pub(super) permissions: PermissionSet,
}

impl Player {
    pub const PLAYER_PACKET_PER_TICK: usize = 50;

    pub const PLAYER_PACKET_QUEUE_SIZE: usize = 1000;

    pub fn new(uuid: Uuid, username: String, protocol_version: i32, addr: SocketAddr) -> Self {
        let respawn_point = PlayerSpawnPoint::default();
        let position = PlayerPosition::from(respawn_point);
        let entity_id = EntityId::next();
        let collision_rules = EntityCollisionRules::from_entity_type(EntityType::PLAYER);
        Self {
            entity_id,
            entity_type: EntityType::PLAYER,
            uuid,
            username,
            protocol_version,
            addr,
            skin: None,
            display_name: None,
            below_name_tag: None,
            listed: true,
            latency: 0,
            loaded_chunk: PlayerChunk::from_position(position),
            chunks_loaded_by_client: PlayerChunk::from_position(position),
            client_chunk_view_distance: DEFAULT_CLIENT_CHUNK_VIEW_DISTANCE,
            chunk_update_limit_checker: ChunkUpdateLimitChecker::new(
                PLAYER_CHUNK_UPDATE_LIMITER_HISTORY_SIZE,
            ),
            position,
            game_mode: GameMode::Survival,
            pending_spawning_world: None,
            current_world: None,
            dimension_type: DimensionType::OVERWORLD,
            world_name: None,
            hardcore: false,
            living: LivingState::new(EntityType::PLAYER),
            food: 20,
            food_saturation: 5.0,
            death_location: None,
            enable_respawn_screen: true,
            experience: 0.0,
            experience_level: 0,
            total_experience: 0,
            portal_cooldown: 0,
            reduced_debug_screen_information: false,
            settings: ClientInformation::default(),
            permission_level: 0,
            respawn_point,
            inventory: PlayerInventory::new(),
            attribute_equipment: BTreeMap::new(),
            open_inventory: None,
            anvil_rename_text: None,
            debug_subscriptions: BTreeSet::new(),
            vehicle: None,
            velocity: Velocity(Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            passengers: BTreeSet::new(),
            leash: EntityLeash::new(),
            synchronization: EntitySynchronization::new(EntityPosition::new(
                position.x,
                position.y,
                position.z,
                position.yaw,
                position.pitch,
            )),
            vanished: false,
            click_preprocessor: ClickPreprocessor::default(),
            held_slot: 0,
            inputs: PlayerInputs::default(),
            flying: false,
            allow_flying: false,
            instant_break: false,
            has_entity_collision: collision_rules.has_entity_collision(),
            prevents_block_placement: collision_rules.can_prevent_block_placement(),
            flying_speed: 0.05,
            field_view_modifier: 0.1,
            metadata: MetadataHolder::default(),
            tag_handler: TagHandler::new_handler(),
            has_entered_world: false,
            on_ground: false,
            aerodynamics: EntityAerodynamics::new(
                EntityType::PLAYER.horizontal_air_resistance(),
                EntityType::PLAYER.vertical_air_resistance(),
                EntityType::PLAYER.acceleration(),
            ),
            gravity_tick_count: 0,
            previous_physics_result: None,
            has_physics: true,
            last_sent_teleport_id: 0,
            last_received_teleport_id: 0,
            last_keep_alive: 0,
            answer_keep_alive: false,
            last_completed_client_tick: 0,
            did_close_inventory: false,
            client: None,
            statistics: BTreeMap::new(),
            alive_ticks: 0,
            delayed_remove_ticks: None,
            last_experience_pickup_tick: None,
            item_use_hand: None,
            start_item_use_time: 0,
            item_use_time: 0,
            packet_queue: VecDeque::new(),
            pending_resource_packs: PendingResourcePacks::new(),
            chunk_queue: VecDeque::new(),
            client_sent_chunks: HashSet::new(),
            chunk_queue_requires_sorting: false,
            #[cfg(test)]
            chunk_queue_sort_count: 0,
            needs_chunk_position_sync: true,
            max_chunk_batch_lead: 1,
            chunk_batch_lead: 0,
            target_chunks_per_tick: 9.0,
            pending_chunk_count: 0.0,
            scheduler: ContextScheduler::new(),
            view: EntityView::new(entity_id),
            permissions: PermissionSet::new(),
        }
    }
}

impl Taggable for Player {
    fn tag_handler(&self) -> &TagHandler {
        &self.tag_handler
    }

    fn tag_handler_mut(&mut self) -> &mut TagHandler {
        &mut self.tag_handler
    }
}

impl PermissionHandler for Player {
    fn get_permission_set(&self) -> &PermissionSet {
        &self.permissions
    }

    fn get_permission_set_mut(&mut self) -> &mut PermissionSet {
        &mut self.permissions
    }
}
