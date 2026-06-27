use crate::entity::EntityPose;
use crate::entity::generic_entity::EntityAerodynamics;
use crate::entity::metadata::{MetadataHolder, definitions};
use crate::entity::physics::{EntityPhysicsResult, knockback_velocity, simulate_movement};
use crate::entity::player::BelowNameTag;
use crate::entity::player::ChunkUpdateLimitChecker;
use crate::entity::player::PendingResourcePacks;
use crate::entity::player::PlayerMeta;
use crate::entity::player::PlayerViewerSnapshot;
use crate::entity::player::chunks::PlayerChunk;
use crate::entity::player::input::PlayerInputs;
use crate::entity::player::position::PlayerPosition;
use crate::entity::player::skin::PlayerSkin;
use crate::entity::{
    Damage, EntityCollisionRules, EntityId, EntityIdentity, EntityLeash, EntityPointers,
    EntityPosition, EntitySynchronization, EntityTeleport, EntityView, EquipmentSlot, LivingState,
    PlayerSnapshot, PlayerSpawnPoint,
};
use crate::events::inventory_close::InventoryCloseEvent;
use crate::events::item_drop::ItemDropEvent;
use crate::events::player_change_held_slot::PlayerChangeHeldSlotEvent;
use crate::events::player_death::PlayerDeathEvent;
use crate::events::player_game_mode_change::PlayerGameModeChangeEvent;
use crate::events::player_respawn::PlayerRespawnEvent;
use crate::events::player_swap_item::PlayerSwapItemEvent;
use crate::inventory::{ClickPreprocessor, Inventory, PlayerInventory};
use crate::network::client::instance::Client;
use crate::scheduler::{ContextScheduler, Task, TaskSchedule};
use crate::scoreboard::Team;
use crate::world::{BossBar, ChunkPosition, WorldHandle, WorldSnapshot};
use spinel_core::entity::game_mode::GameMode;
use spinel_core::network::clientbound::play::advancements::Notification;
use spinel_core::network::clientbound::play::chunk_data::ChunkDataAndUpdateLightPacket;
use spinel_core::network::clientbound::play::clear_dialog::ClearDialogPacket;
use spinel_core::network::clientbound::play::clear_titles::ClearTitlesPacket;
use spinel_core::network::clientbound::play::container_close::ContainerClosePacket;
use spinel_core::network::clientbound::play::entity_animation::{
    EntityAnimation, EntityAnimationPacket,
};
use spinel_core::network::clientbound::play::entity_head_look::EntityHeadLookPacket;
use spinel_core::network::clientbound::play::entity_sound_effect::{
    EntitySoundEffectPacket, NetworkSoundEvent,
};
use spinel_core::network::clientbound::play::entity_status::EntityStatusPacket;
use spinel_core::network::clientbound::play::entity_velocity::EntityVelocityPacket;
use spinel_core::network::clientbound::play::game_event::{
    GameEvent, GameEventPacket, RespawnScreenState,
};
use spinel_core::network::clientbound::play::open_book::OpenBookPacket;
use spinel_core::network::clientbound::play::player_abilities::PlayerAbilitiesPacket;
use spinel_core::network::clientbound::play::player_combat_kill::PlayerCombatKillPacket;
use spinel_core::network::clientbound::play::player_info_remove::PlayerInfoRemovePacket;
use spinel_core::network::clientbound::play::player_info_update::PlayerInfoUpdatePacket;
use spinel_core::network::clientbound::play::player_look_at::{FacePoint, PlayerLookAtPacket};
use spinel_core::network::clientbound::play::plugin_message::PlayCustomPayloadPacket;
use spinel_core::network::clientbound::play::recipe_book_add::RecipeBookAddPacket;
use spinel_core::network::clientbound::play::remove_entities::RemoveEntitiesPacket;
use spinel_core::network::clientbound::play::respawn::RespawnPacket;
use spinel_core::network::clientbound::play::server_difficulty::ServerDifficultyPacket;
use spinel_core::network::clientbound::play::set_camera::SetCameraPacket;
use spinel_core::network::clientbound::play::set_entity_data::SetEntityDataPacket;
use spinel_core::network::clientbound::play::set_equipment::{
    EntityEquipmentEntry, SetEquipmentPacket,
};
use spinel_core::network::clientbound::play::set_experience::SetExperiencePacket;
use spinel_core::network::clientbound::play::set_health::SetHealthPacket;
use spinel_core::network::clientbound::play::set_held_slot::SetHeldSlotPacket;
use spinel_core::network::clientbound::play::set_passengers::SetPassengersPacket;
use spinel_core::network::clientbound::play::set_subtitle_text::SetSubtitleTextPacket;
use spinel_core::network::clientbound::play::set_title_text::SetTitleTextPacket;
use spinel_core::network::clientbound::play::set_titles_animation::SetTitlesAnimationPacket;
use spinel_core::network::clientbound::play::show_dialog::ShowDialogPacket;
use spinel_core::network::clientbound::play::sound_effect::{
    NetworkPositionedSoundEvent, SoundEffectPacket,
};
use spinel_core::network::clientbound::play::spawn_entity::{EntityAngle, SpawnEntityPacket};
use spinel_core::network::clientbound::play::start_configuration::StartConfigurationPacket;
use spinel_core::network::clientbound::play::stop_sound::StopSoundPacket;
use spinel_core::network::clientbound::play::sync_player_pos::SyncPlayerPositionPacket;
use spinel_core::network::clientbound::play::system_chat::SystemChatPacket;
use spinel_core::network::clientbound::play::tab_list::TabListPacket;
use spinel_core::network::clientbound::play::ticking_state::TickingStatePacket;
use spinel_core::network::clientbound::play::ticking_step::TickingStepPacket;
use spinel_core::network::clientbound::play::update_recipes::UpdateRecipesPacket;
use spinel_core::network::clientbound::play::world_event::WorldEventPacket;
use spinel_nbt::{TagHandler, Taggable};
use spinel_network::types::MainHand;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::sound::SoundEvent;
use spinel_network::types::{
    ClientInformation, GlobalPos, Identifier, IntList, Particle, Position, Slot, TeleportFlags,
    Vector3d, Velocity,
};
use spinel_network::{ConnectionState, PacketSender, PacketStruct};
use spinel_registry::dialog::Dialog;
use spinel_registry::dimension_type::DimensionType;
use spinel_registry::{Attribute, EntityBoundingBox, EntityType, ItemStack, RegistryKey};
use spinel_utils::component::color::{NamedTextColor, TextColor};
use spinel_utils::component::events::{HoverEntity, HoverEvent};
use spinel_utils::component::text::TextComponent;
use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};
use std::io;
use std::net::SocketAddr;
use uuid::Uuid;

const MIN_CHUNKS_PER_TICK: f32 = 0.01;
const MAX_CHUNKS_PER_TICK: f32 = 64.0;
const CHUNKS_PER_TICK_MULTIPLIER: f32 = 1.0;
const DEFAULT_CLIENT_CHUNK_VIEW_DISTANCE: i32 = 8;
const PLAYER_CHUNK_UPDATE_LIMITER_HISTORY_SIZE: usize = 5;
const SERVER_TICKS_PER_SECOND: f64 = 20.0;

pub struct Player {
    entity_id: EntityId,
    entity_type: EntityType,
    pub uuid: Uuid,
    pub username: String,
    pub protocol_version: i32,
    pub addr: SocketAddr,
    skin: Option<PlayerSkin>,
    display_name: Option<TextComponent>,
    below_name_tag: Option<BelowNameTag>,
    listed: bool,
    latency: i32,
    pub(crate) loaded_chunk: PlayerChunk,
    pub(crate) chunks_loaded_by_client: PlayerChunk,
    client_chunk_view_distance: i32,
    pub(super) chunk_update_limit_checker: ChunkUpdateLimitChecker,
    pub(crate) position: PlayerPosition,
    game_mode: GameMode,
    pending_spawning_world: Option<Uuid>,
    current_world: Option<Uuid>,
    dimension_type: RegistryKey<DimensionType>,
    world_name: Option<Identifier>,
    hardcore: bool,
    pub(super) living: LivingState,
    food: i32,
    food_saturation: f32,
    death_location: Option<PlayerDeathLocation>,
    enable_respawn_screen: bool,
    experience: f32,
    experience_level: i32,
    total_experience: i32,
    portal_cooldown: i32,
    reduced_debug_screen_information: bool,
    settings: ClientInformation,
    permission_level: i32,
    respawn_point: PlayerSpawnPoint,
    inventory: PlayerInventory,
    pub(super) attribute_equipment: BTreeMap<EquipmentSlot, ItemStack>,
    open_inventory: Option<Inventory>,
    anvil_rename_text: Option<String>,
    debug_subscriptions: BTreeSet<i32>,
    vehicle: Option<EntityId>,
    velocity: Velocity,
    passengers: BTreeSet<EntityId>,
    leash: EntityLeash,
    synchronization: EntitySynchronization,
    vanished: bool,
    pub(super) click_preprocessor: ClickPreprocessor,
    held_slot: i32,
    inputs: PlayerInputs,
    flying: bool,
    allow_flying: bool,
    instant_break: bool,
    has_entity_collision: bool,
    prevents_block_placement: bool,
    flying_speed: f32,
    field_view_modifier: f32,
    pub(super) metadata: MetadataHolder,
    tag_handler: TagHandler,
    has_entered_world: bool,
    on_ground: bool,
    aerodynamics: EntityAerodynamics,
    gravity_tick_count: u64,
    previous_physics_result: Option<EntityPhysicsResult>,
    has_physics: bool,
    last_sent_teleport_id: i32,
    last_received_teleport_id: i32,
    last_keep_alive: i64,
    answer_keep_alive: bool,
    pub(super) last_completed_client_tick: u64,
    did_close_inventory: bool,
    pub(super) client: Option<usize>,
    pub(super) item_cooldowns: BTreeMap<String, u64>,
    statistics: BTreeMap<String, i32>,
    pub(super) alive_ticks: u64,
    last_experience_pickup_tick: Option<i64>,
    pub(super) item_use_hand: Option<PlayerHand>,
    pub(super) start_item_use_time: u64,
    pub(super) item_use_time: u64,
    packet_queue: VecDeque<QueuedPlayerPacket>,
    pub(super) pending_resource_packs: PendingResourcePacks,
    pub(super) chunk_queue: VecDeque<QueuedPlayerChunk>,
    pub(super) client_sent_chunks: HashSet<PlayerChunk>,
    pub(super) needs_chunk_position_sync: bool,
    pub(super) max_chunk_batch_lead: i32,
    pub(super) chunk_batch_lead: i32,
    pub(super) target_chunks_per_tick: f32,
    pub(super) pending_chunk_count: f32,
    scheduler: ContextScheduler<Player>,
    view: EntityView,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlayerDeathLocation {
    dimension: Identifier,
    position: EntityPosition,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueuedPlayerPacket {
    pub state: ConnectionState,
    pub packet_id: i32,
    pub payload: Vec<u8>,
}

pub(crate) struct QueuedPlayerChunk {
    pub(crate) chunk: PlayerChunk,
    pub(super) packet: Option<ChunkDataAndUpdateLightPacket>,
}

impl QueuedPlayerChunk {
    pub(super) fn new(packet: ChunkDataAndUpdateLightPacket) -> Self {
        Self {
            chunk: PlayerChunk::new(packet.chunk_x, packet.chunk_z),
            packet: Some(packet),
        }
    }

    pub(super) fn from_chunk(chunk: PlayerChunk) -> Self {
        Self {
            chunk,
            packet: None,
        }
    }
}

impl PlayerDeathLocation {
    pub fn new(dimension: Identifier, position: EntityPosition) -> Self {
        Self {
            dimension,
            position,
        }
    }

    pub fn get_dimension(&self) -> &Identifier {
        &self.dimension
    }

    pub const fn get_position(&self) -> EntityPosition {
        self.position
    }
}

impl QueuedPlayerPacket {
    pub fn new(state: ConnectionState, packet_id: i32, payload: Vec<u8>) -> Self {
        Self {
            state,
            packet_id,
            payload,
        }
    }
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
            item_cooldowns: BTreeMap::new(),
            statistics: BTreeMap::new(),
            alive_ticks: 0,
            last_experience_pickup_tick: None,
            item_use_hand: None,
            start_item_use_time: 0,
            item_use_time: 0,
            packet_queue: VecDeque::new(),
            pending_resource_packs: PendingResourcePacks::new(),
            chunk_queue: VecDeque::new(),
            client_sent_chunks: HashSet::new(),
            needs_chunk_position_sync: true,
            max_chunk_batch_lead: 1,
            chunk_batch_lead: 0,
            target_chunks_per_tick: 9.0,
            pending_chunk_count: 0.0,
            scheduler: ContextScheduler::new(),
            view: EntityView::new(entity_id),
        }
    }

    pub fn set_pending_options(&mut self, spawning_world: Uuid, hardcore: bool) {
        self.pending_spawning_world = Some(spawning_world);
        self.hardcore = hardcore;
    }

    pub const fn get_pending_spawning_world(&self) -> Option<Uuid> {
        self.pending_spawning_world
    }

    pub fn get_scheduler(&mut self) -> &mut ContextScheduler<Player> {
        &mut self.scheduler
    }

    pub fn schedule_next_tick(
        &mut self,
        callback: impl FnMut(&mut Player) -> TaskSchedule + Send + 'static,
    ) -> Task {
        self.scheduler.schedule_next_tick(callback)
    }

    pub const fn get_current_world(&self) -> Option<Uuid> {
        self.current_world
    }

    pub(crate) fn set_world(&mut self, world: Uuid) {
        self.current_world = Some(world);
    }

    pub(crate) fn set_dimension_type(&mut self, dimension_type: RegistryKey<DimensionType>) {
        self.dimension_type = dimension_type;
    }

    pub fn get_dimension_type(&self) -> &RegistryKey<DimensionType> {
        &self.dimension_type
    }

    pub const fn is_hardcore(&self) -> bool {
        self.hardcore
    }

    pub fn unsafe_init(
        &mut self,
        client: &mut Client,
        ticks_per_second: u32,
        dimension_type_id: i32,
        world_name: Identifier,
        chunk_packets: Vec<
            spinel_core::network::clientbound::play::chunk_data::ChunkDataAndUpdateLightPacket,
        >,
        world_border_packet: spinel_core::network::clientbound::play::initialize_world_border::InitializeWorldBorderPacket,
        time_packet: spinel_core::network::clientbound::play::set_time::SetTimePacket,
        weather: crate::world::Weather,
    ) -> io::Result<()> {
        self.pending_spawning_world = None;
        self.living.revive();
        self.world_name = Some(world_name.clone());
        self.enter_world(
            client,
            ticks_per_second,
            dimension_type_id,
            world_name,
            chunk_packets,
            world_border_packet,
            time_packet,
            weather,
        )
    }

    pub(crate) fn unsafe_init_with_chunk_positions(
        &mut self,
        client: &mut Client,
        ticks_per_second: u32,
        dimension_type_id: i32,
        world_name: Identifier,
        chunks: Vec<PlayerChunk>,
        world_border_packet: spinel_core::network::clientbound::play::initialize_world_border::InitializeWorldBorderPacket,
        time_packet: spinel_core::network::clientbound::play::set_time::SetTimePacket,
        weather: crate::world::Weather,
    ) -> io::Result<()> {
        self.pending_spawning_world = None;
        self.living.revive();
        self.world_name = Some(world_name.clone());
        self.enter_world_with_chunk_positions(
            client,
            ticks_per_second,
            dimension_type_id,
            world_name,
            chunks,
            world_border_packet,
            time_packet,
            weather,
        )
    }

    pub(super) fn prepare_instance_spawn(&mut self, world_name: Identifier) {
        self.pending_spawning_world = None;
        self.living.revive();
        self.world_name = Some(world_name);
    }

    pub fn start_configuration_phase(&mut self) -> io::Result<()> {
        let Some(client) = self.get_client_mut() else {
            return Ok(());
        };
        if client.state != ConnectionState::Play {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "player must be in play state for reconfiguration",
            ));
        }
        StartConfigurationPacket.dispatch(client)?;
        client.state = ConnectionState::Configuration;
        self.has_entered_world = false;
        Ok(())
    }
    pub const fn get_alive_ticks(&self) -> u64 {
        self.alive_ticks
    }

    pub fn is_removed(&self) -> bool {
        self.is_dead()
    }

    pub const fn is_dead(&self) -> bool {
        self.living.is_dead()
    }

    pub fn get_last_damage(&self) -> Option<&Damage> {
        self.living.get_last_damage()
    }

    pub const fn get_health(&self) -> f32 {
        self.living.get_health()
    }

    pub const fn get_fire_ticks(&self) -> i32 {
        self.living.get_fire_ticks()
    }

    pub fn is_on_fire(&self) -> bool {
        self.metadata.get_flag(&definitions::is_on_fire())
    }

    pub fn set_fire_ticks(&mut self, fire_ticks: i32) {
        self.living.set_fire_ticks(fire_ticks);
        self.set_on_fire(self.living.get_fire_ticks() > 0);
    }

    pub(crate) fn set_fire_ticks_after_cancelled_extinguish(&mut self, fire_ticks: i32) {
        self.living.set_fire_ticks(fire_ticks);
    }

    pub(crate) fn tick_fire_ticks(&mut self) {
        self.living.tick_fire_ticks();
        self.set_on_fire(self.living.get_fire_ticks() > 0);
    }

    pub fn set_health(&mut self, health: f32) -> io::Result<()> {
        self.living.set_health(health);
        self.metadata.set(
            &definitions::living_entity::get_health(),
            MetadataValue::Float(self.living.get_health()),
        );
        self.sync_health()?;
        if self.living.get_health() <= 0.0 {
            self.kill()?;
        }
        Ok(())
    }

    pub(crate) fn apply_damage(&mut self, damage: Damage) -> io::Result<()> {
        let mut remaining_damage = damage.get_amount();
        let additional_hearts = self.get_additional_hearts();
        if additional_hearts > 0.0 {
            if remaining_damage > additional_hearts {
                remaining_damage -= additional_hearts;
                self.set_additional_hearts(0.0);
            } else {
                self.set_additional_hearts(additional_hearts - remaining_damage);
                remaining_damage = 0.0;
            }
        }
        self.living.store_last_damage(damage);
        self.living
            .set_health(self.living.get_health() - remaining_damage);
        self.metadata.set(
            &definitions::living_entity::get_health(),
            MetadataValue::Float(self.living.get_health()),
        );
        self.sync_health()
    }

    pub fn get_player_metadata(&self) -> &MetadataHolder {
        &self.metadata
    }

    pub fn get_additional_hearts(&self) -> f32 {
        match self
            .metadata
            .get_value(&definitions::get_additional_hearts())
        {
            MetadataValue::Float(additional_hearts) => additional_hearts,
            _ => 0.0,
        }
    }

    pub fn set_additional_hearts(&mut self, additional_hearts: f32) {
        self.metadata.set(
            &definitions::get_additional_hearts(),
            MetadataValue::Float(additional_hearts.max(0.0)),
        );
    }

    pub const fn get_food(&self) -> i32 {
        self.food
    }

    pub fn set_food(&mut self, food: i32) -> io::Result<()> {
        self.food = food.clamp(0, 20);
        self.sync_health()
    }

    pub const fn get_food_saturation(&self) -> f32 {
        self.food_saturation
    }

    pub fn set_food_saturation(&mut self, food_saturation: f32) -> io::Result<()> {
        self.food_saturation = food_saturation.clamp(0.0, self.food as f32);
        self.sync_health()
    }

    pub fn set_death_location(&mut self, position: EntityPosition) {
        let dimension = self
            .world_name
            .clone()
            .unwrap_or_else(|| Identifier::minecraft("overworld"));
        self.death_location = Some(PlayerDeathLocation::new(dimension, position));
    }

    pub fn set_death_location_in_dimension(
        &mut self,
        dimension: Identifier,
        position: EntityPosition,
    ) {
        self.death_location = Some(PlayerDeathLocation::new(dimension, position));
    }

    pub fn get_death_location(&self) -> Option<&PlayerDeathLocation> {
        self.death_location.as_ref()
    }

    pub const fn is_respawn_screen_enabled(&self) -> bool {
        self.enable_respawn_screen
    }

    pub fn set_respawn_screen_enabled(&mut self, enable_respawn_screen: bool) -> io::Result<()> {
        self.enable_respawn_screen = enable_respawn_screen;
        let state = if enable_respawn_screen {
            RespawnScreenState::Enabled
        } else {
            RespawnScreenState::ImmediateRespawn
        };
        self.dispatch_game_event(GameEvent::SetRespawnScreen(state))
    }

    pub const fn get_experience(&self) -> f32 {
        self.experience
    }

    pub fn set_experience(&mut self, experience: f32) -> io::Result<()> {
        self.experience = experience.clamp(0.0, 1.0);
        self.sync_experience()
    }

    pub const fn get_experience_level(&self) -> i32 {
        self.experience_level
    }

    pub fn set_experience_level(&mut self, experience_level: i32) -> io::Result<()> {
        self.experience_level = experience_level.max(0);
        self.sync_experience()
    }

    pub const fn get_total_experience(&self) -> i32 {
        self.total_experience
    }

    pub fn set_total_experience(&mut self, total_experience: i32) -> io::Result<()> {
        self.total_experience = total_experience.max(0);
        self.sync_experience()
    }

    pub const fn get_portal_cooldown(&self) -> i32 {
        self.portal_cooldown
    }

    pub fn set_portal_cooldown(&mut self, portal_cooldown: i32) {
        self.portal_cooldown = portal_cooldown.max(0);
    }

    pub const fn has_reduced_debug_screen_information(&self) -> bool {
        self.reduced_debug_screen_information
    }

    pub fn set_reduced_debug_screen_information(
        &mut self,
        reduced_debug_screen_information: bool,
    ) -> io::Result<()> {
        self.reduced_debug_screen_information = reduced_debug_screen_information;
        let status = if reduced_debug_screen_information {
            22
        } else {
            23
        };
        self.send_packet(EntityStatusPacket {
            entity_id: self.get_entity_id().get_value(),
            status,
        })
    }

    pub const fn get_settings(&self) -> &ClientInformation {
        &self.settings
    }

    pub fn refresh_settings(&mut self, settings: ClientInformation) -> bool {
        let previous_view_distance = self.client_chunk_view_distance;
        self.client_chunk_view_distance = settings.view_distance.clamp(2, 32) as i32;
        self.settings = settings;
        self.settings.view_distance = self.settings.view_distance.clamp(2, 32);
        previous_view_distance != self.client_chunk_view_distance
    }

    pub fn get_locale(&self) -> &str {
        &self.settings.locale
    }

    pub fn set_locale(&mut self, locale: impl Into<String>) {
        let mut settings = self.settings.clone();
        settings.locale = locale.into();
        self.refresh_settings(settings);
    }

    pub fn kill(&mut self) -> io::Result<()> {
        if self.living.is_dead() {
            return Ok(());
        }
        self.living.kill();
        self.velocity = Velocity(Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        let default_death_text = self
            .living
            .get_last_damage()
            .and_then(Damage::build_death_screen_text)
            .unwrap_or_else(|| TextComponent::literal("Killed by poor programming."));
        let default_chat_message = self
            .living
            .get_last_damage()
            .and_then(|damage| damage.build_death_message(self.get_username()))
            .unwrap_or_else(|| {
                TextComponent::literal(format!(
                    "{} was killed by poor programming.",
                    self.get_username()
                ))
            });
        let (death_text, chat_message) =
            self.dispatch_player_death_event(default_death_text, default_chat_message);
        let entity_id = self.entity_id.get_value();
        if let Some(client) = self.get_client_mut() {
            if client.state == ConnectionState::Play
                && let Some(death_text) = death_text
            {
                PlayerCombatKillPacket::new(entity_id, death_text).dispatch(client)?;
            }
        }
        self.broadcast_death_message(chat_message)?;
        if self.current_world.is_some() {
            self.set_death_location(self.get_position());
        }
        Ok(())
    }

    pub fn respawn(&mut self) -> io::Result<bool> {
        if !self.living.is_dead() {
            return Ok(false);
        }
        let respawn_dimension = self
            .world_name
            .clone()
            .unwrap_or_else(|| Identifier::minecraft("overworld"));
        let game_mode = self.game_mode;
        if let Some(client) = self.get_client_mut()
            && client.state == ConnectionState::Play
        {
            RespawnPacket::new(game_mode, respawn_dimension).dispatch(client)?;
            GameEventPacket::from(GameEvent::StartWaitingForLevelChunks).dispatch(client)?;
            ServerDifficultyPacket::normal(false).dispatch(client)?;
            SetHealthPacket::new(20.0, 20, 5.0).dispatch(client)?;
            SetExperiencePacket::new(0.0, 0, 0).dispatch(client)?;
            self.refresh_abilities()?;
        }
        let respawn_position = self.dispatch_player_respawn_event();
        self.living.revive();
        self.refresh_pose();
        self.position = PlayerPosition::from(respawn_position);
        Ok(true)
    }

    pub fn is_online(&self) -> bool {
        self.get_client().is_some_and(Client::is_online)
    }

    pub const fn can_pickup_item(&self) -> bool {
        self.living.can_pickup_item()
    }

    pub fn set_can_pickup_item(&mut self, can_pickup_item: bool) {
        self.living.set_can_pickup_item(can_pickup_item);
    }

    pub const fn has_entity_collision(&self) -> bool {
        self.has_entity_collision
    }

    pub const fn can_prevent_block_placement(&self) -> bool {
        self.prevents_block_placement
    }

    pub const fn get_permission_level(&self) -> i32 {
        self.permission_level
    }

    pub fn set_permission_level(&mut self, permission_level: i32) -> io::Result<()> {
        self.permission_level = permission_level.clamp(0, 4);
        self.send_packet(EntityStatusPacket {
            entity_id: self.get_entity_id().get_value(),
            status: (24 + self.permission_level) as i8,
        })
    }

    pub fn refresh_recipes(&mut self) -> io::Result<()> {
        let Some(client) = self.get_client_mut() else {
            return Ok(());
        };
        UpdateRecipesPacket.dispatch(client)?;
        RecipeBookAddPacket::reset_empty().dispatch(client)
    }

    pub fn send_packet<P>(&mut self, packet: P) -> io::Result<()>
    where
        P: PacketStruct,
    {
        let Some(client) = self.get_client_mut() else {
            return Ok(());
        };
        if client.state != P::get_state() {
            return Ok(());
        }

        let mut payload = Vec::new();
        packet.encode(&mut payload)?;
        client.send_packet(P::get_id(), &payload)
    }

    pub fn send_packets<P>(&mut self, packets: impl IntoIterator<Item = P>) -> io::Result<()>
    where
        P: PacketStruct,
    {
        packets
            .into_iter()
            .try_for_each(|packet| self.send_packet(packet))
    }

    pub fn send_raw_packet(&mut self, packet_id: i32, payload: &[u8]) -> io::Result<()> {
        let Some(client) = self.get_client_mut() else {
            return Ok(());
        };

        client.send_packet(packet_id, payload)
    }

    pub fn send_plugin_message(
        &mut self,
        channel: impl Into<String>,
        data: Vec<u8>,
    ) -> io::Result<()> {
        self.send_packet(PlayCustomPayloadPacket::new(channel, data))
    }

    pub fn send_plugin_message_string(
        &mut self,
        channel: impl Into<String>,
        data: impl Into<String>,
    ) -> io::Result<()> {
        self.send_plugin_message(channel, data.into().into_bytes())
    }

    pub fn send_system_message(&mut self, message: TextComponent) -> io::Result<()> {
        self.send_packet(SystemChatPacket::new(message, false))
    }

    pub fn send_action_bar(&mut self, message: TextComponent) -> io::Result<()> {
        self.send_packet(SystemChatPacket::new(message, true))
    }

    pub fn send_message_from(
        &mut self,
        _source: Uuid,
        message: TextComponent,
        message_type: PlayerMessageType,
    ) -> io::Result<()> {
        if !message_type.is_accepted_by_chat_mode(self.settings.chat_mode) {
            return Ok(());
        }
        self.send_packet(SystemChatPacket::new(
            message,
            message_type == PlayerMessageType::ActionBar,
        ))
    }

    pub fn can_receive_chat_message(&self) -> bool {
        self.settings.chat_mode == 0
    }

    pub fn can_receive_chat_command(&self) -> bool {
        self.settings.chat_mode != 2
    }

    pub fn send_chat_rejection_message(&mut self) -> io::Result<()> {
        self.send_packet(SystemChatPacket::new(
            TextComponent::translatable("chat.cannotSend")
                .color(spinel_utils::component::color::TextColor::from_named(
                    spinel_utils::component::color::NamedTextColor::Red,
                ))
                .build(),
            false,
        ))
    }

    pub fn open_book(&mut self, hand: PlayerHand) -> io::Result<()> {
        self.send_packet(OpenBookPacket::new(hand.get_protocol_id()))
    }

    pub fn show_dialog(&mut self, dialog: &RegistryKey<Dialog>) -> io::Result<()> {
        let packet = ShowDialogPacket::from_vanilla_dialog(dialog).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "unknown registered dialog")
        })?;
        self.send_packet(packet)
    }

    pub fn send_player_list_header_and_footer(
        &mut self,
        header: TextComponent,
        footer: TextComponent,
    ) -> io::Result<()> {
        self.send_packet(TabListPacket::new(header, footer))
    }

    pub fn send_title(&mut self, title: TextComponent) -> io::Result<()> {
        self.send_packet(SetTitleTextPacket::new(title))
    }

    pub fn send_subtitle(&mut self, subtitle: TextComponent) -> io::Result<()> {
        self.send_packet(SetSubtitleTextPacket::new(subtitle))
    }

    pub fn set_title_animation(
        &mut self,
        fade_in: i32,
        stay: i32,
        fade_out: i32,
    ) -> io::Result<()> {
        self.send_packet(SetTitlesAnimationPacket::new(fade_in, stay, fade_out))
    }

    pub fn clear_title(&mut self) -> io::Result<()> {
        self.send_packet(ClearTitlesPacket::clear())
    }

    pub fn close_dialog(&mut self) -> io::Result<()> {
        self.send_packet(ClearDialogPacket)
    }

    pub fn send_notification(&mut self, notification: Notification) -> io::Result<()> {
        let timestamp_millis = self.alive_ticks as i64 * 50;
        self.send_packet(notification.add_packet(timestamp_millis))?;
        self.send_packet(notification.remove_packet())
    }

    pub fn get_below_name_tag(&self) -> Option<&BelowNameTag> {
        self.below_name_tag.as_ref()
    }

    pub fn set_below_name_tag(&mut self, below_name_tag: Option<BelowNameTag>) -> io::Result<()> {
        if self.below_name_tag == below_name_tag {
            return Ok(());
        }

        if let Some(previous_below_name_tag) = self.below_name_tag.take() {
            self.send_packet(previous_below_name_tag.remove_packet())?;
        }

        self.below_name_tag = below_name_tag;
        if let Some(current_below_name_tag) = self.below_name_tag.clone() {
            self.send_packet(current_below_name_tag.create_packet())?;
            self.send_packet(current_below_name_tag.get_display_packet())?;
        }

        Ok(())
    }

    pub fn get_team(&self) -> Option<&str> {
        self.living.get_team()
    }

    pub fn set_scoreboard_team(
        &mut self,
        mut previous_team: Option<&mut Team>,
        mut new_team: Option<&mut Team>,
    ) -> Vec<spinel_core::network::clientbound::play::set_player_team::SetPlayerTeamPacket> {
        let member = self.username.clone();
        let mut packets = Vec::new();
        if let Some(previous_team_name) = self.living.get_team().map(str::to_owned) {
            self.living.set_team(None);
            let should_remove_previous_member = new_team
                .as_ref()
                .is_none_or(|current_team| current_team.name() != previous_team_name);
            if should_remove_previous_member {
                if let Some(previous_team) = previous_team.as_mut() {
                    if let Some(packet) = previous_team.remove_member(&member) {
                        packets.push(packet);
                    }
                } else {
                    packets.push(
                        spinel_core::network::clientbound::play::set_player_team::SetPlayerTeamPacket {
                            team_name: previous_team_name,
                            action: spinel_core::network::clientbound::play::set_player_team::TeamAction::RemoveEntities(vec![member.clone()]),
                        },
                    );
                }
            }
        }
        if let Some(current_team) = new_team.as_mut() {
            self.living.set_team(Some(current_team.name().to_owned()));
            if let Some(packet) = current_team.add_member(member) {
                packets.push(packet);
            }
        }
        packets
    }

    pub fn reset_title(&mut self) -> io::Result<()> {
        self.send_packet(ClearTitlesPacket::reset())
    }

    pub fn play_sound(&mut self, sound_event: SoundEvent) -> io::Result<()> {
        self.play_sound_at_position(sound_event, self.get_position().as_vector())
    }

    pub fn play_sound_at_position(
        &mut self,
        sound_event: SoundEvent,
        position: Vector3d,
    ) -> io::Result<()> {
        self.send_packet(SoundEffectPacket {
            sound_event: NetworkPositionedSoundEvent(sound_event),
            source_id: 0,
            position,
            volume: 1.0,
            pitch: 1.0,
            seed: 0,
        })
    }

    pub fn play_sound_from_entity(
        &mut self,
        sound_event: SoundEvent,
        entity_id: EntityId,
    ) -> io::Result<()> {
        self.send_packet(EntitySoundEffectPacket {
            sound_event: NetworkSoundEvent(sound_event),
            source_id: 0,
            entity_id: entity_id.get_value(),
            volume: 1.0,
            pitch: 1.0,
            seed: 0,
        })
    }

    pub fn stop_sound(&mut self, source: Option<i32>, sound: Option<Identifier>) -> io::Result<()> {
        self.send_packet(StopSoundPacket::new(source, sound))
    }

    pub fn play_effect(
        &mut self,
        effect_id: i32,
        position: Position,
        data: i32,
        global_event: bool,
    ) -> io::Result<()> {
        self.send_packet(WorldEventPacket::new(
            effect_id,
            position,
            data,
            global_event,
        ))
    }

    pub fn show_boss_bar(&mut self, boss_bar: &BossBar) -> io::Result<()> {
        self.send_packet(boss_bar.add_packet())
    }

    pub fn hide_boss_bar(&mut self, boss_bar: &BossBar) -> io::Result<()> {
        self.send_packet(boss_bar.remove_packet())
    }

    pub fn spectate(&mut self, entity_id: EntityId) -> io::Result<()> {
        self.send_packet(SetCameraPacket::new(entity_id.get_value()))
    }

    pub fn stop_spectating(&mut self) -> io::Result<()> {
        self.send_packet(SetCameraPacket::new(self.get_entity_id().get_value()))
    }

    pub fn add_packet_to_queue(&mut self, packet: QueuedPlayerPacket) -> bool {
        if self.packet_queue.len() >= Self::PLAYER_PACKET_QUEUE_SIZE {
            let _ = self.kick(Self::too_many_packets_text());
            return false;
        }
        self.packet_queue.push_back(packet);
        true
    }

    pub(crate) fn pop_next_packet_from_queue(&mut self) -> Option<QueuedPlayerPacket> {
        self.packet_queue.pop_front()
    }

    pub fn interpret_packet_queue(
        &mut self,
        server: &mut crate::server::MinecraftServer,
        client: &mut Client,
    ) -> usize {
        let mut interpreted_packets = 0;
        while interpreted_packets < Self::PLAYER_PACKET_PER_TICK {
            let Some(queued_packet) = self.packet_queue.pop_front() else {
                return interpreted_packets;
            };
            client.state = queued_packet.state;
            server.dispatch_packet(queued_packet.packet_id, client, queued_packet.payload);
            interpreted_packets += 1;
        }
        interpreted_packets
    }

    pub fn get_queued_packet_count(&self) -> usize {
        self.packet_queue.len()
    }

    fn too_many_packets_text() -> TextComponent {
        TextComponent::literal_with_color(
            "Too Many Packets",
            TextColor::from_named(NamedTextColor::Red),
        )
    }

    pub fn on_chunk_batch_received(&mut self, desired_chunks_per_tick: f32) {
        self.chunk_batch_lead -= 1;
        self.target_chunks_per_tick = if desired_chunks_per_tick.is_nan() {
            MIN_CHUNKS_PER_TICK
        } else {
            (desired_chunks_per_tick * CHUNKS_PER_TICK_MULTIPLIER)
                .clamp(MIN_CHUNKS_PER_TICK, MAX_CHUNKS_PER_TICK)
        };
        if self.max_chunk_batch_lead == 1 {
            self.max_chunk_batch_lead = 10;
        }
    }

    pub const fn get_chunk_batch_lead(&self) -> i32 {
        self.chunk_batch_lead
    }

    pub const fn get_max_chunk_batch_lead(&self) -> i32 {
        self.max_chunk_batch_lead
    }

    pub const fn get_target_chunks_per_tick(&self) -> f32 {
        self.target_chunks_per_tick
    }

    pub const fn get_pending_chunk_count(&self) -> f32 {
        self.pending_chunk_count
    }

    pub const fn get_client_chunk_view_distance(&self) -> i32 {
        self.client_chunk_view_distance
    }

    pub fn set_client_chunk_view_distance(&mut self, client_chunk_view_distance: i32) {
        self.client_chunk_view_distance = client_chunk_view_distance.max(0);
    }

    pub fn get_effective_chunk_view_distance(&self, world_view_distance: i32) -> i32 {
        self.client_chunk_view_distance
            .min(world_view_distance)
            .max(0)
            + 1
    }

    pub const fn get_entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub const fn get_entity_type(&self) -> EntityType {
        self.entity_type
    }

    pub fn switch_entity_type(&mut self, entity_type: EntityType) {
        self.entity_type = entity_type;
        self.metadata = MetadataHolder::default();
        self.aerodynamics = EntityAerodynamics::from_entity_type(entity_type);
        self.refresh_collision_rules();
    }

    fn refresh_collision_rules(&mut self) {
        let collision_rules = EntityCollisionRules::from_entity_type(self.entity_type);
        self.has_entity_collision = collision_rules.has_entity_collision();
        self.prevents_block_placement = collision_rules.can_prevent_block_placement();
    }

    pub const fn get_view(&self) -> &EntityView {
        &self.view
    }

    pub const fn get_view_mut(&mut self) -> &mut EntityView {
        &mut self.view
    }

    pub fn get_viewers(&self) -> BTreeSet<EntityId> {
        self.view.get_viewers()
    }

    pub fn is_viewer(&self, viewer_id: EntityId) -> bool {
        self.view.is_viewer(viewer_id)
    }

    pub const fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub const fn get_identity(&self) -> EntityIdentity {
        EntityIdentity::new(self.uuid)
    }
    pub fn get_entity_meta_mut(&mut self) -> PlayerMeta<'_> {
        PlayerMeta::new(self)
    }

    pub const fn get_pointers(&self) -> EntityPointers {
        EntityPointers::new(self.uuid, self.entity_id)
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn as_hover_event(&self) -> HoverEvent {
        HoverEvent::ShowEntity(HoverEntity {
            entity_type: self.entity_type.key().to_string(),
            id: self.uuid.to_string(),
            name: Some(Box::new(TextComponent::literal(self.get_username()))),
        })
    }

    pub fn update_snapshot(&self, updater: impl FnOnce(&mut PlayerSnapshot)) -> PlayerSnapshot {
        let mut snapshot = PlayerSnapshot::new(
            self.entity_id,
            self.uuid,
            self.username.clone(),
            self.get_position(),
            self.current_world,
            self.game_mode,
            self.skin.clone(),
            self.display_name.clone(),
            self.statistics
                .iter()
                .map(|(statistic, value)| (statistic.clone(), *value))
                .collect(),
        );
        updater(&mut snapshot);
        snapshot
    }

    pub fn get_statistic_value_map(&self) -> &BTreeMap<String, i32> {
        &self.statistics
    }

    pub fn get_statistic_value(&self, statistic: &str) -> i32 {
        self.statistics.get(statistic).copied().unwrap_or_default()
    }

    pub fn set_statistic_value(&mut self, statistic: impl Into<String>, value: i32) {
        self.statistics.insert(statistic.into(), value.max(0));
    }

    pub fn increment_statistic_value(&mut self, statistic: impl Into<String>, amount: i32) -> i32 {
        let statistic = statistic.into();
        let value = self
            .get_statistic_value(&statistic)
            .saturating_add(amount)
            .max(0);
        self.statistics.insert(statistic, value);
        value
    }

    pub fn get_skin(&self) -> Option<&PlayerSkin> {
        self.skin.as_ref()
    }

    pub fn set_skin(&mut self, skin: Option<PlayerSkin>) -> io::Result<()> {
        self.skin = skin;
        if !self.has_entered_world() {
            return Ok(());
        }
        let player_uuid = self.get_uuid();
        let player_id = self.get_entity_id();
        let add_player_packet = self.get_player_info_packet();
        let viewer_snapshot = PlayerViewerSnapshot::from_player(self);

        self.refresh_skin_for_self(player_uuid, add_player_packet.clone(), &viewer_snapshot)?;
        self.broadcast_to_play_clients(|client| {
            PlayerInfoRemovePacket::new(player_uuid).dispatch(client)
        })?;
        self.dispatch_to_viewer_clients(|client| {
            RemoveEntitiesPacket::new(vec![player_id.get_value()]).dispatch(client)
        })?;
        self.broadcast_to_play_clients(|client| add_player_packet.clone().dispatch(client))?;
        self.dispatch_to_viewer_clients(|client| {
            viewer_snapshot.dispatch_without_player_info(client)
        })
    }

    pub(crate) fn apply_skin(&mut self, skin: Option<PlayerSkin>) {
        self.skin = skin;
    }

    pub fn get_display_name(&self) -> Option<&TextComponent> {
        self.display_name.as_ref()
    }

    pub fn set_display_name(&mut self, display_name: Option<TextComponent>) -> io::Result<()> {
        self.display_name = display_name;
        let packet =
            PlayerInfoUpdatePacket::update_display_name(self.uuid, self.display_name.clone());
        self.broadcast_to_play_clients(|client| packet.clone().dispatch(client))
    }

    pub fn get_custom_name(&self) -> Option<TextComponent> {
        match self.metadata.get_value(&definitions::get_custom_name()) {
            MetadataValue::OptionalText(custom_name) => custom_name,
            _ => None,
        }
    }

    pub fn set_custom_name(&mut self, custom_name: Option<TextComponent>) {
        self.metadata.set(
            &definitions::get_custom_name(),
            MetadataValue::OptionalText(custom_name),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn is_custom_name_visible(&self) -> bool {
        match self.metadata.get_value(&definitions::custom_name_visible()) {
            MetadataValue::Boolean(custom_name_visible) => custom_name_visible,
            _ => false,
        }
    }

    pub fn set_custom_name_visible(&mut self, custom_name_visible: bool) {
        self.metadata.set(
            &definitions::custom_name_visible(),
            MetadataValue::Boolean(custom_name_visible),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn is_silent(&self) -> bool {
        match self.metadata.get_value(&definitions::is_silent()) {
            MetadataValue::Boolean(silent) => silent,
            _ => false,
        }
    }

    pub fn set_silent(&mut self, silent: bool) {
        self.metadata
            .set(&definitions::is_silent(), MetadataValue::Boolean(silent));
        self.refresh_dirty_metadata_to_viewers();
    }

    pub const fn get_protocol_version(&self) -> i32 {
        self.protocol_version
    }

    pub const fn get_address(&self) -> SocketAddr {
        self.addr
    }

    pub(crate) fn set_client(&mut self, client: &mut Client) {
        client.set_player_entity_id(self.get_entity_id());
        self.client = Some(client as *mut Client as usize);
    }

    pub fn get_client(&self) -> Option<&Client> {
        self.client
            .map(|client| unsafe { &*(client as *const Client) })
    }

    pub fn get_world(&self) -> Option<WorldHandle> {
        let client = self.get_client()?;
        let server = client.server_ptr?;
        let current_world = self.current_world?;
        Some(WorldHandle::new(server, current_world))
    }

    pub(crate) fn get_client_mut(&mut self) -> Option<&mut Client> {
        self.client
            .map(|client| unsafe { &mut *(client as *mut Client) })
    }

    pub fn set_game_mode(&mut self, game_mode: GameMode) -> bool {
        let player = self as *mut Player;
        let Some(client_ptr) = self.client else {
            self.apply_game_mode(game_mode);
            return true;
        };
        let client = unsafe { &mut *(client_ptr as *mut Client) };
        let final_game_mode = if let Some(server_ptr) = client.server_ptr {
            let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
            let mut event = PlayerGameModeChangeEvent::new(player, game_mode);
            event.dispatch(server, client);
            if event.is_cancelled() {
                return false;
            }
            event.new_game_mode()
        } else {
            game_mode
        };
        self.apply_game_mode(final_game_mode);
        if !self.has_entered_world() || client.state != ConnectionState::Play {
            return true;
        }
        let self_sync_succeeded = self.sync_game_mode_state(client).is_ok();
        let viewer_sync_succeeded = self.refresh_game_mode_to_viewers();
        self_sync_succeeded && viewer_sync_succeeded
    }

    pub fn get_game_mode(&self) -> GameMode {
        self.game_mode
    }

    pub const fn is_flying(&self) -> bool {
        self.flying
    }

    pub const fn can_fly(&self) -> bool {
        self.allow_flying
    }

    pub const fn has_instant_break(&self) -> bool {
        self.instant_break
    }

    pub const fn is_invulnerable(&self) -> bool {
        self.living.is_invulnerable()
    }

    pub fn is_sneaking(&self) -> bool {
        self.metadata.get_flag(&definitions::is_crouching())
    }

    pub fn is_sprinting(&self) -> bool {
        self.metadata.get_flag(&definitions::is_sprinting())
    }

    pub fn is_swimming(&self) -> bool {
        self.metadata.get_flag(&definitions::is_swimming())
    }

    pub fn is_invisible(&self) -> bool {
        self.metadata.get_flag(&definitions::is_invisible())
    }

    pub fn is_glowing(&self) -> bool {
        self.metadata.get_flag(&definitions::has_glowing_effect())
    }

    pub fn is_flying_with_elytra(&self) -> bool {
        self.metadata
            .get_flag(&definitions::is_flying_with_elytra())
    }

    pub fn get_air_ticks(&self) -> i32 {
        match self.metadata.get_value(&definitions::get_air_ticks()) {
            MetadataValue::VarInt(air_ticks) => air_ticks,
            _ => 300,
        }
    }

    pub fn is_hand_active(&self) -> bool {
        self.metadata
            .get_flag(&definitions::living_entity::is_hand_active())
    }

    pub fn get_active_hand(&self) -> PlayerHand {
        if self
            .metadata
            .get_flag(&definitions::living_entity::get_active_hand())
        {
            return PlayerHand::Off;
        }
        PlayerHand::Main
    }

    pub fn is_in_riptide_spin_attack(&self) -> bool {
        self.metadata
            .get_flag(&definitions::living_entity::is_riptide_spin_attack())
    }

    pub fn get_effect_particles(&self) -> Vec<Particle> {
        match self
            .metadata
            .get_value(&definitions::living_entity::potion_effect_particles())
        {
            MetadataValue::ParticleList(effect_particles) => effect_particles,
            _ => Vec::new(),
        }
    }

    pub fn is_potion_effect_ambient(&self) -> bool {
        match self
            .metadata
            .get_value(&definitions::living_entity::is_potion_effect_ambient())
        {
            MetadataValue::Boolean(potion_effect_ambient) => potion_effect_ambient,
            _ => false,
        }
    }

    pub fn get_arrow_count(&self) -> i32 {
        match self
            .metadata
            .get_value(&definitions::living_entity::number_of_arrows())
        {
            MetadataValue::VarInt(arrow_count) => arrow_count,
            _ => 0,
        }
    }

    pub fn get_bee_stinger_count(&self) -> i32 {
        match self
            .metadata
            .get_value(&definitions::living_entity::number_of_bee_stingers())
        {
            MetadataValue::VarInt(bee_stinger_count) => bee_stinger_count,
            _ => 0,
        }
    }

    pub fn get_bed_in_which_sleeping_position(&self) -> Option<Position> {
        match self
            .metadata
            .get_value(&definitions::living_entity::location_of_bed())
        {
            MetadataValue::OptionalPosition(bed_position) => bed_position,
            _ => None,
        }
    }

    pub const fn is_listed(&self) -> bool {
        self.listed
    }

    pub const fn get_latency(&self) -> i32 {
        self.latency
    }

    pub const fn get_flying_speed(&self) -> f32 {
        self.flying_speed
    }

    pub const fn get_field_view_modifier(&self) -> f32 {
        self.field_view_modifier
    }

    pub fn set_listed(&mut self, listed: bool) -> io::Result<()> {
        self.listed = listed;
        let packet = PlayerInfoUpdatePacket::update_listed(self.uuid, listed);
        self.dispatch_player_info_update(packet)
    }

    pub fn refresh_latency(&mut self, latency: i32) -> io::Result<()> {
        self.latency = latency;
        let packet = PlayerInfoUpdatePacket::update_latency(self.uuid, latency);
        self.dispatch_player_info_update(packet)
    }

    pub fn set_flying(&mut self, flying: bool) -> io::Result<()> {
        self.set_flying_state(flying);
        self.refresh_abilities()
    }

    pub fn set_flying_state(&mut self, flying: bool) {
        self.flying = flying;
    }

    pub fn refresh_flying(&mut self, flying: bool) {
        if self.flying != flying {
            if self.is_sneaking() && self.get_pose() == EntityPose::Standing {
                self.set_pose(EntityPose::Sneaking);
            } else if self.get_pose() == EntityPose::Sneaking {
                self.set_pose(EntityPose::Standing);
            }
        }
        self.flying = flying;
    }

    pub fn set_allow_flying(&mut self, allow_flying: bool) -> io::Result<()> {
        self.allow_flying = allow_flying;
        self.refresh_abilities()
    }

    pub fn set_instant_break(&mut self, instant_break: bool) -> io::Result<()> {
        self.instant_break = instant_break;
        self.refresh_abilities()
    }

    pub fn set_invulnerable(&mut self, invulnerable: bool) -> io::Result<()> {
        self.living.set_invulnerable(invulnerable);
        self.refresh_abilities()
    }

    pub fn set_sneaking(&mut self, sneaking: bool) {
        self.metadata
            .set_flag(&definitions::is_crouching(), sneaking);
        if !self.is_flying() {
            self.refresh_pose();
        }
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_on_fire(&mut self, on_fire: bool) {
        self.metadata.set_flag(&definitions::is_on_fire(), on_fire);
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn get_pose(&self) -> EntityPose {
        match self.metadata.get_value(&definitions::get_pose()) {
            MetadataValue::Pose(pose) => {
                EntityPose::from_protocol_id(pose).unwrap_or(EntityPose::Standing)
            }
            _ => EntityPose::Standing,
        }
    }

    pub fn set_pose(&mut self, pose: EntityPose) {
        self.metadata.set(
            &definitions::get_pose(),
            MetadataValue::Pose(pose.get_protocol_id()),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_sprinting(&mut self, sprinting: bool) -> bool {
        let old_sprint = self.is_sprinting();
        self.metadata
            .set_flag(&definitions::is_sprinting(), sprinting);
        self.refresh_dirty_metadata_to_viewers();
        old_sprint != sprinting
    }

    pub fn set_swimming(&mut self, swimming: bool) {
        self.metadata
            .set_flag(&definitions::is_swimming(), swimming);
        self.refresh_pose();
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_invisible(&mut self, invisible: bool) {
        self.metadata
            .set_flag(&definitions::is_invisible(), invisible);
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_glowing(&mut self, glowing: bool) {
        self.metadata
            .set_flag(&definitions::has_glowing_effect(), glowing);
        self.refresh_dirty_metadata_to_viewers();
    }

    pub(crate) fn set_flying_with_elytra(&mut self, flying_with_elytra: bool) -> bool {
        let old_flying_with_elytra = self
            .metadata
            .get_flag(&definitions::is_flying_with_elytra());
        self.metadata
            .set_flag(&definitions::is_flying_with_elytra(), flying_with_elytra);
        self.refresh_pose();
        self.refresh_dirty_metadata_to_viewers();
        old_flying_with_elytra != flying_with_elytra
    }

    pub fn set_air_ticks(&mut self, air_ticks: i32) {
        self.metadata.set(
            &definitions::get_air_ticks(),
            MetadataValue::VarInt(air_ticks),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_hand_active(&mut self, hand_active: bool) {
        self.metadata
            .set_flag(&definitions::living_entity::is_hand_active(), hand_active);
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_active_hand(&mut self, hand: PlayerHand) {
        self.metadata.set_flag(
            &definitions::living_entity::get_active_hand(),
            hand == PlayerHand::Off,
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_in_riptide_spin_attack(&mut self, in_riptide_spin_attack: bool) {
        self.metadata.set_flag(
            &definitions::living_entity::is_riptide_spin_attack(),
            in_riptide_spin_attack,
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_effect_particles(&mut self, effect_particles: Vec<Particle>) {
        self.metadata.set(
            &definitions::living_entity::potion_effect_particles(),
            MetadataValue::ParticleList(effect_particles),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_potion_effect_ambient(&mut self, potion_effect_ambient: bool) {
        self.metadata.set(
            &definitions::living_entity::is_potion_effect_ambient(),
            MetadataValue::Boolean(potion_effect_ambient),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_arrow_count(&mut self, arrow_count: i32) {
        let normalized_arrow_count = arrow_count.max(0);
        self.living.set_arrow_count(normalized_arrow_count);
        self.metadata.set(
            &definitions::living_entity::number_of_arrows(),
            MetadataValue::VarInt(normalized_arrow_count),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_bee_stinger_count(&mut self, bee_stinger_count: i32) {
        self.metadata.set(
            &definitions::living_entity::number_of_bee_stingers(),
            MetadataValue::VarInt(bee_stinger_count.max(0)),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn set_bed_in_which_sleeping_position(&mut self, bed_position: Option<Position>) {
        self.metadata.set(
            &definitions::living_entity::location_of_bed(),
            MetadataValue::OptionalPosition(bed_position),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn get_main_hand(&self) -> MainHand {
        match self
            .metadata
            .get_value(&definitions::avatar::get_main_hand())
        {
            MetadataValue::MainHand(main_hand) => main_hand,
            _ => MainHand::Right,
        }
    }

    pub fn set_main_hand(&mut self, main_hand: MainHand) {
        self.metadata.set(
            &definitions::avatar::get_main_hand(),
            MetadataValue::MainHand(main_hand),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn is_cape_enabled(&self) -> bool {
        self.metadata
            .get_flag(&definitions::avatar::is_cape_enabled())
    }

    pub fn set_cape_enabled(&mut self, cape_enabled: bool) {
        self.metadata
            .set_flag(&definitions::avatar::is_cape_enabled(), cape_enabled);
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn is_jacket_enabled(&self) -> bool {
        self.metadata
            .get_flag(&definitions::avatar::is_jacket_enabled())
    }

    pub fn set_jacket_enabled(&mut self, jacket_enabled: bool) {
        self.metadata
            .set_flag(&definitions::avatar::is_jacket_enabled(), jacket_enabled);
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn is_left_sleeve_enabled(&self) -> bool {
        self.metadata
            .get_flag(&definitions::avatar::is_left_sleeve_enabled())
    }

    pub fn set_left_sleeve_enabled(&mut self, left_sleeve_enabled: bool) {
        self.metadata.set_flag(
            &definitions::avatar::is_left_sleeve_enabled(),
            left_sleeve_enabled,
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn is_right_sleeve_enabled(&self) -> bool {
        self.metadata
            .get_flag(&definitions::avatar::is_right_sleeve_enabled())
    }

    pub fn set_right_sleeve_enabled(&mut self, right_sleeve_enabled: bool) {
        self.metadata.set_flag(
            &definitions::avatar::is_right_sleeve_enabled(),
            right_sleeve_enabled,
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn is_left_leg_enabled(&self) -> bool {
        self.metadata
            .get_flag(&definitions::avatar::is_left_pants_leg_enabled())
    }

    pub fn set_left_leg_enabled(&mut self, left_leg_enabled: bool) {
        self.metadata.set_flag(
            &definitions::avatar::is_left_pants_leg_enabled(),
            left_leg_enabled,
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn is_right_leg_enabled(&self) -> bool {
        self.metadata
            .get_flag(&definitions::avatar::is_right_pants_leg_enabled())
    }

    pub fn set_right_leg_enabled(&mut self, right_leg_enabled: bool) {
        self.metadata.set_flag(
            &definitions::avatar::is_right_pants_leg_enabled(),
            right_leg_enabled,
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn is_hat_enabled(&self) -> bool {
        self.metadata
            .get_flag(&definitions::avatar::is_hat_enabled())
    }

    pub fn set_hat_enabled(&mut self, hat_enabled: bool) {
        self.metadata
            .set_flag(&definitions::avatar::is_hat_enabled(), hat_enabled);
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn get_displayed_skin_parts(&self) -> i8 {
        match self
            .metadata
            .get_value(&definitions::avatar::displayed_model_parts_flags())
        {
            MetadataValue::Byte(displayed_skin_parts) => displayed_skin_parts,
            _ => 0,
        }
    }

    pub fn set_displayed_skin_parts(&mut self, displayed_skin_parts: i8) {
        self.metadata.set(
            &definitions::avatar::displayed_model_parts_flags(),
            MetadataValue::Byte(displayed_skin_parts),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn get_score(&self) -> i32 {
        match self.metadata.get_value(&definitions::player::get_score()) {
            MetadataValue::VarInt(score) => score,
            _ => 0,
        }
    }

    pub fn set_score(&mut self, score: i32) {
        self.metadata.set(
            &definitions::player::get_score(),
            MetadataValue::VarInt(score),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn get_left_shoulder_entity_data(&self) -> Option<i32> {
        match self
            .metadata
            .get_value(&definitions::player::get_left_shoulder_entity_data())
        {
            MetadataValue::OptionalVarInt(left_shoulder_entity_data) => left_shoulder_entity_data,
            _ => None,
        }
    }

    pub fn set_left_shoulder_entity_data(&mut self, left_shoulder_entity_data: Option<i32>) {
        self.metadata.set(
            &definitions::player::get_left_shoulder_entity_data(),
            MetadataValue::OptionalVarInt(left_shoulder_entity_data),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn get_right_shoulder_entity_data(&self) -> Option<i32> {
        match self
            .metadata
            .get_value(&definitions::player::get_right_shoulder_entity_data())
        {
            MetadataValue::OptionalVarInt(right_shoulder_entity_data) => right_shoulder_entity_data,
            _ => None,
        }
    }

    pub fn set_right_shoulder_entity_data(&mut self, right_shoulder_entity_data: Option<i32>) {
        self.metadata.set(
            &definitions::player::get_right_shoulder_entity_data(),
            MetadataValue::OptionalVarInt(right_shoulder_entity_data),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn leave_bed(&mut self) {
        self.metadata
            .set(&definitions::get_pose(), MetadataValue::Pose(0));
    }

    pub fn set_flying_speed(&mut self, flying_speed: f32) -> io::Result<()> {
        self.flying_speed = flying_speed;
        self.refresh_abilities()
    }

    pub fn set_field_view_modifier(&mut self, field_view_modifier: f32) -> io::Result<()> {
        self.field_view_modifier = field_view_modifier;
        self.refresh_abilities()
    }

    pub fn set_respawn_point(&mut self, respawn_point: PlayerSpawnPoint) {
        self.respawn_point = respawn_point;
    }

    pub fn get_respawn_point(&self) -> PlayerSpawnPoint {
        self.respawn_point
    }

    pub fn get_inventory(&mut self) -> &mut PlayerInventory {
        &mut self.inventory
    }

    pub fn get_inventory_ref(&self) -> &PlayerInventory {
        &self.inventory
    }

    pub fn open_inventory(&mut self, inventory: Inventory) {
        self.open_inventory = Some(inventory);
        let Some(client) = self.client else {
            return;
        };
        let client = unsafe { &mut *(client as *mut Client) };
        let _ = self.sync_open_inventory(client);
    }

    pub fn close_inventory(&mut self) {
        self.click_preprocessor.clear_cache();
        self.open_inventory = None;
    }

    pub(crate) fn close_inventory_window_with_client(
        &mut self,
        from_client: bool,
        window_id: i32,
        server: &mut crate::server::MinecraftServer,
        client: &mut Client,
    ) -> bool {
        if window_id == self.get_inventory_ref().window_id() {
            self.close_inventory();
            return self.sync_player_inventory_window_contents(client).is_ok();
        }
        let Some(open_inventory) = self.get_opened_inventory().cloned() else {
            return false;
        };
        if window_id != open_inventory.window_id() {
            return false;
        }
        let mut event = InventoryCloseEvent::new(self as *mut Player, open_inventory, from_client);
        event.dispatch(server, client);
        if !from_client {
            self.did_close_inventory = true;
        }
        let cursor_item = self.get_inventory_ref().cursor_item().clone();
        self.close_inventory();
        self.get_inventory().set_cursor_item(ItemStack::air());
        if !cursor_item.is_air() && !self.drop_item(cursor_item.clone(), server, client) {
            let _ = self.get_inventory().add_item_stack(cursor_item);
        }
        let player_inventory_window_is_synced =
            self.sync_player_inventory_window_contents(client).is_ok();
        if !from_client {
            let packet_result = ContainerClosePacket {
                container_id: event.get_inventory().id().into(),
            }
            .dispatch(client)
            .is_ok();
            self.did_close_inventory = false;
            return packet_result && player_inventory_window_is_synced;
        }
        self.did_close_inventory = false;
        player_inventory_window_is_synced
    }

    pub fn get_opened_inventory(&self) -> Option<&Inventory> {
        self.open_inventory.as_ref()
    }

    pub fn get_anvil_rename_text(&self) -> Option<&str> {
        self.anvil_rename_text.as_deref()
    }

    pub fn set_anvil_rename_text(&mut self, anvil_rename_text: impl Into<String>) {
        self.anvil_rename_text = Some(anvil_rename_text.into());
    }

    pub fn get_debug_subscriptions(&self) -> &BTreeSet<i32> {
        &self.debug_subscriptions
    }

    pub fn set_debug_subscriptions(&mut self, debug_subscriptions: BTreeSet<i32>) {
        self.debug_subscriptions = debug_subscriptions;
    }

    pub const fn get_vehicle(&self) -> Option<EntityId> {
        self.vehicle
    }

    pub(crate) fn set_vehicle(&mut self, vehicle: EntityId) {
        self.vehicle = Some(vehicle);
    }

    pub(crate) fn clear_vehicle(&mut self) {
        self.vehicle = None;
    }

    pub const fn get_velocity(&self) -> Velocity {
        self.velocity
    }

    pub(crate) fn set_velocity(&mut self, velocity: Velocity) {
        self.velocity = velocity;
    }

    pub fn take_knockback(&mut self, strength: f32, x: f64, z: f64) {
        let living_strength = strength
            * (1.0
                - self
                    .living
                    .get_attribute_value(Attribute::KNOCKBACK_RESISTANCE) as f32);
        self.velocity = knockback_velocity(self.velocity, self.on_ground, living_strength, x, z);
    }

    pub fn has_velocity(&self) -> bool {
        let velocity = self.velocity.0;
        if self.on_ground {
            return velocity.x != 0.0 || velocity.z != 0.0 || velocity.y > 0.0;
        }
        velocity.x != 0.0 || velocity.y != 0.0 || velocity.z != 0.0
    }

    pub fn get_velocity_packet(&self) -> EntityVelocityPacket {
        EntityVelocityPacket {
            entity_id: self.get_entity_id().get_value(),
            velocity: self.protocol_velocity(),
        }
    }

    fn protocol_velocity(&self) -> Velocity {
        Velocity(Vector3d {
            x: self.velocity.0.x / SERVER_TICKS_PER_SECOND,
            y: self.velocity.0.y / SERVER_TICKS_PER_SECOND,
            z: self.velocity.0.z / SERVER_TICKS_PER_SECOND,
        })
    }

    pub(crate) fn add_passenger(&mut self, passenger_id: EntityId) -> bool {
        self.passengers.insert(passenger_id)
    }

    pub(crate) fn remove_passenger(&mut self, passenger_id: EntityId) -> bool {
        self.passengers.remove(&passenger_id)
    }

    pub fn has_passenger(&self) -> bool {
        !self.passengers.is_empty()
    }

    pub fn get_passengers(&self) -> &BTreeSet<EntityId> {
        &self.passengers
    }

    pub(crate) fn get_passenger_packet(&self) -> SetPassengersPacket {
        SetPassengersPacket {
            vehicle_entity_id: self.get_entity_id().get_value(),
            passenger_entity_ids: IntList(
                self.passengers
                    .iter()
                    .map(|passenger_id| passenger_id.get_value())
                    .collect(),
            ),
        }
    }

    pub fn get_leashed_entities(&self) -> &BTreeSet<EntityId> {
        self.leash.get_leashed_entities()
    }

    pub const fn get_leash_holder(&self) -> Option<EntityId> {
        self.leash.get_holder()
    }

    pub(crate) fn set_leash_holder(&mut self, leash_holder: Option<EntityId>) {
        self.leash.set_holder(leash_holder);
    }

    pub(crate) fn add_leashed_entity(&mut self, entity_id: EntityId) -> bool {
        self.leash.add_leashed_entity(entity_id)
    }

    pub(crate) fn remove_leashed_entity(&mut self, entity_id: EntityId) -> bool {
        self.leash.remove_leashed_entity(entity_id)
    }

    pub(crate) fn get_attach_entity_packet(
        &self,
    ) -> spinel_core::network::clientbound::play::attach_entity::AttachEntityPacket {
        self.leash.get_packet(self.entity_id)
    }

    pub const fn get_synchronization_ticks(&self) -> u64 {
        self.synchronization.get_interval_ticks()
    }

    pub fn set_synchronization_ticks(&mut self, synchronization_ticks: u64) {
        self.synchronization
            .set_interval_ticks(synchronization_ticks);
    }

    pub fn synchronize_next_tick(&mut self) {
        self.synchronization.synchronize_next_tick();
    }

    pub(super) fn record_synchronization_position(&mut self, position: EntityPosition) {
        self.synchronization.record_position(position);
    }

    pub(crate) fn synchronize_entity_position_packet(
        &mut self,
    ) -> spinel_core::network::clientbound::play::entity_position_sync::EntityPositionSyncPacket
    {
        let position = self.get_position();
        self.synchronization
            .synchronize(self.entity_id, self.alive_ticks, position, self.on_ground)
    }

    pub(crate) fn get_scheduled_entity_position_sync_packet(
        &mut self,
    ) -> Option<
        spinel_core::network::clientbound::play::entity_position_sync::EntityPositionSyncPacket,
    > {
        self.synchronization
            .is_due(self.alive_ticks, self.vehicle.is_some())
            .then(|| self.synchronize_entity_position_packet())
    }

    pub const fn is_vanished(&self) -> bool {
        self.vanished
    }

    pub fn set_vanished(&mut self, vanished: bool) {
        self.vanished = vanished;
    }

    #[cfg(test)]
    pub(crate) fn mark_chunk_sent_to_client(&mut self, chunk: PlayerChunk) {
        self.client_sent_chunks.insert(chunk);
    }

    pub(crate) fn get_opened_inventory_mut(&mut self) -> Option<&mut Inventory> {
        self.open_inventory.as_mut()
    }

    pub fn get_click_preprocessor(&mut self) -> &mut ClickPreprocessor {
        &mut self.click_preprocessor
    }

    pub fn get_did_close_inventory(&self) -> bool {
        self.did_close_inventory
    }

    pub fn set_did_close_inventory(&mut self, did_close_inventory: bool) {
        self.did_close_inventory = did_close_inventory;
    }

    pub fn get_held_slot(&self) -> i32 {
        self.held_slot
    }

    pub fn set_held_slot(&mut self, held_slot: i32) -> bool {
        if !(0..=8).contains(&held_slot) {
            return false;
        }
        self.held_slot = held_slot;
        true
    }

    pub(crate) fn change_held_slot(
        &mut self,
        held_slot: i32,
        server: &mut crate::server::MinecraftServer,
        client: &mut Client,
    ) -> bool {
        if !(0..=8).contains(&held_slot) {
            return false;
        }
        let old_slot = self.get_held_slot();
        let mut event = PlayerChangeHeldSlotEvent::new(self as *mut Player, old_slot, held_slot);
        event.dispatch(server, client);
        if event.is_cancelled() {
            let _ = SetHeldSlotPacket {
                slot: old_slot as i8,
            }
            .dispatch(client);
            return false;
        }
        let new_slot = event.new_slot();
        if new_slot != held_slot {
            let _ = SetHeldSlotPacket {
                slot: new_slot as i8,
            }
            .dispatch(client);
        }
        if !self.set_held_slot_with_client(new_slot, client) {
            return false;
        }
        if self.get_item_use_hand() != Some(PlayerHand::Off) {
            self.refresh_active_hand(false, false, false);
            self.clear_item_use();
        }
        true
    }

    pub(crate) const fn has_entered_world(&self) -> bool {
        self.has_entered_world
    }

    pub(crate) fn mark_entered_world(&mut self) {
        self.has_entered_world = true;
    }

    pub const fn is_on_ground(&self) -> bool {
        self.on_ground
    }

    pub fn set_on_ground(&mut self, on_ground: bool) {
        self.on_ground = on_ground;
    }

    pub(crate) fn refresh_on_ground(&mut self, on_ground: bool) -> bool {
        self.on_ground = on_ground;
        let player_is_airborne = !self.on_ground;
        let player_is_not_flying_with_elytra = !self.is_flying_with_elytra();
        if player_is_airborne || player_is_not_flying_with_elytra {
            return false;
        }
        self.set_flying_with_elytra(false)
    }

    pub const fn get_aerodynamics(&self) -> EntityAerodynamics {
        self.aerodynamics
    }

    pub fn set_aerodynamics(&mut self, aerodynamics: EntityAerodynamics) {
        self.aerodynamics = aerodynamics;
    }

    pub const fn get_gravity_tick_count(&self) -> u64 {
        self.gravity_tick_count
    }

    pub(crate) fn tick_gravity_counter(&mut self) {
        self.gravity_tick_count = if self.on_ground {
            0
        } else {
            self.gravity_tick_count.saturating_add(1)
        };
    }

    pub(crate) fn movement_tick(&mut self, world: &WorldSnapshot) {
        self.tick_gravity_counter();
        if self.vehicle.is_some() {
            return;
        }
        let position = self.get_position();
        let velocity_per_tick = Velocity(Vector3d {
            x: self.velocity.0.x / SERVER_TICKS_PER_SECOND,
            y: self.velocity.0.y / SERVER_TICKS_PER_SECOND,
            z: self.velocity.0.z / SERVER_TICKS_PER_SECOND,
        });
        let physics = simulate_movement(
            position,
            velocity_per_tick,
            self.get_bounding_box(),
            world,
            self.aerodynamics,
            self.has_no_gravity(),
            self.has_physics,
            self.on_ground,
            self.is_flying(),
            self.previous_physics_result,
        );
        self.previous_physics_result = Some(physics);
        if !world.is_chunk_loaded(ChunkPosition::from(physics.get_new_position())) {
            return;
        }
        self.velocity = Velocity(Vector3d {
            x: physics.get_new_velocity_per_tick().0.x * SERVER_TICKS_PER_SECOND,
            y: physics.get_new_velocity_per_tick().0.y * SERVER_TICKS_PER_SECOND,
            z: physics.get_new_velocity_per_tick().0.z * SERVER_TICKS_PER_SECOND,
        });
    }

    pub const fn has_physics(&self) -> bool {
        self.has_physics
    }

    pub fn set_has_physics(&mut self, has_physics: bool) {
        self.has_physics = has_physics;
    }

    pub fn has_no_gravity(&self) -> bool {
        match self.metadata.get_value(&definitions::has_no_gravity()) {
            MetadataValue::Boolean(has_no_gravity) => has_no_gravity,
            _ => false,
        }
    }

    pub fn set_no_gravity(&mut self, has_no_gravity: bool) {
        self.metadata.set(
            &definitions::has_no_gravity(),
            MetadataValue::Boolean(has_no_gravity),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn ticks_frozen(&self) -> i32 {
        match self.metadata.get_value(&definitions::ticks_frozen()) {
            MetadataValue::VarInt(ticks_frozen) => ticks_frozen,
            _ => 0,
        }
    }

    pub fn set_ticks_frozen(&mut self, ticks_frozen: i32) {
        self.metadata.set(
            &definitions::ticks_frozen(),
            MetadataValue::VarInt(ticks_frozen),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn get_bounding_box(&self) -> EntityBoundingBox {
        self.get_pose()
            .get_bounding_box(EntityType::PLAYER.get_bounding_box())
            .unwrap_or_else(|| EntityType::PLAYER.get_bounding_box())
    }

    pub fn get_next_teleport_id(&mut self) -> i32 {
        self.last_sent_teleport_id += 1;
        self.last_sent_teleport_id
    }

    pub const fn get_last_sent_teleport_id(&self) -> i32 {
        self.last_sent_teleport_id
    }

    pub const fn get_last_received_teleport_id(&self) -> i32 {
        self.last_received_teleport_id
    }

    pub fn set_last_received_teleport_id(&mut self, received_teleport_id: i32) {
        if received_teleport_id < 0 {
            return;
        }
        self.last_received_teleport_id = received_teleport_id;
    }

    pub fn has_pending_teleport_confirmation(&self) -> bool {
        self.last_sent_teleport_id != self.last_received_teleport_id
    }

    pub fn synchronize_position_after_teleport(
        &mut self,
        position: EntityPosition,
        velocity: Vector3d,
        flags: TeleportFlags,
        should_confirm: bool,
    ) -> io::Result<()> {
        let teleport_id = if should_confirm {
            self.get_next_teleport_id()
        } else {
            -1
        };
        let packet = SyncPlayerPositionPacket {
            teleport_id,
            x: position.get_x(),
            y: position.get_y(),
            z: position.get_z(),
            velocity_x: velocity.x,
            velocity_y: velocity.y,
            velocity_z: velocity.z,
            yaw: position.get_yaw(),
            pitch: position.get_pitch(),
            flags,
        };
        let Some(client) = self.get_client_mut() else {
            return Ok(());
        };
        if client.state != ConnectionState::Play {
            return Ok(());
        }
        packet.dispatch(client)
    }

    pub fn teleport_with_chunks_and_flags(
        &mut self,
        position: EntityPosition,
        chunks: Option<Vec<i64>>,
        flags: TeleportFlags,
    ) -> io::Result<EntityTeleport> {
        self.teleport(position, chunks, flags, true)
    }

    pub fn teleport(
        &mut self,
        position: EntityPosition,
        chunks: Option<Vec<i64>>,
        flags: TeleportFlags,
        should_confirm: bool,
    ) -> io::Result<EntityTeleport> {
        self.teleport_with_velocity_chunks_and_flags(
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

    pub fn teleport_with_velocity_chunks_and_flags(
        &mut self,
        position: EntityPosition,
        velocity: Velocity,
        chunks: Option<Vec<i64>>,
        flags: TeleportFlags,
        should_confirm: bool,
    ) -> io::Result<EntityTeleport> {
        let teleport = EntityTeleport::resolve(
            self.get_position(),
            self.velocity,
            position,
            velocity,
            chunks,
            flags,
        );
        self.set_position(teleport.get_position());
        self.set_velocity(teleport.get_velocity());
        self.synchronize_position_after_teleport(position, velocity.0, flags, should_confirm)?;
        Ok(teleport)
    }

    pub(crate) fn apply_teleport(
        &mut self,
        teleport: &EntityTeleport,
        should_confirm: bool,
    ) -> io::Result<()> {
        self.set_position(teleport.get_position());
        self.set_velocity(teleport.get_velocity());
        self.synchronize_position_after_teleport(
            teleport.get_teleport_position(),
            teleport.get_teleport_velocity().0,
            teleport.get_flags(),
            should_confirm,
        )
    }

    pub(crate) fn teleport_destination(
        &self,
        position: EntityPosition,
        flags: TeleportFlags,
    ) -> EntityPosition {
        EntityTeleport::resolve_position(self.get_position(), position, flags)
    }

    pub fn get_item_in_hand(&self, hand: PlayerHand) -> ItemStack {
        let equipment_slot = match hand {
            PlayerHand::Main => EquipmentSlot::MainHand,
            PlayerHand::Off => EquipmentSlot::OffHand,
        };
        self.get_equipment(equipment_slot)
    }

    pub fn set_item_in_hand(&mut self, hand: PlayerHand, item_stack: ItemStack) -> bool {
        let equipment_slot = match hand {
            PlayerHand::Main => EquipmentSlot::MainHand,
            PlayerHand::Off => EquipmentSlot::OffHand,
        };
        self.set_equipment(equipment_slot, item_stack)
    }

    pub fn get_equipment(&self, equipment_slot: EquipmentSlot) -> ItemStack {
        self.get_inventory_ref()
            .get_equipment(equipment_slot, self.held_slot)
    }

    pub fn set_equipment(&mut self, equipment_slot: EquipmentSlot, item_stack: ItemStack) -> bool {
        let previous_item_stack = self.get_equipment(equipment_slot);
        if !self
            .inventory
            .set_equipment(equipment_slot, self.held_slot, item_stack)
        {
            return false;
        }
        let current_item_stack = self.get_equipment(equipment_slot);
        self.living
            .get_attributes_mut()
            .update_equipment_attributes(&previous_item_stack, &current_item_stack, equipment_slot);
        let slot = self
            .inventory
            .slot_for_equipment(equipment_slot, self.held_slot);
        let _ = self.sync_active_equipment_change(slot);
        true
    }

    fn sync_active_equipment_change(&self, slot: i32) -> bool {
        let Some(client) = self.client else {
            return false;
        };
        let attributes_packet = self.update_attributes_packet();
        let client = unsafe { &mut *(client as *mut Client) };
        let slot_is_synced = self.sync_player_inventory_slot(slot, client).is_ok();
        let attributes_are_synced = attributes_packet.dispatch(client).is_ok();
        slot_is_synced && attributes_are_synced
    }

    pub(crate) fn swap_item_hands(
        &mut self,
        server: &mut crate::server::MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let main_hand_item = self.get_item_in_hand(PlayerHand::Main);
        let off_hand_item = self.get_item_in_hand(PlayerHand::Off);
        let mut event =
            PlayerSwapItemEvent::new(self as *mut Player, off_hand_item, main_hand_item);
        event.dispatch(server, client);
        if event.is_cancelled() {
            return false;
        }
        let main_hand_item = event.main_hand_item().clone();
        let off_hand_item = event.off_hand_item().clone();
        self.set_item_in_hand(PlayerHand::Main, main_hand_item);
        self.set_item_in_hand(PlayerHand::Off, off_hand_item);
        let _ = self.sync_slot(self.held_slot, client);
        let _ = self.sync_slot(crate::inventory::slot_conversion::OFFHAND_SLOT, client);
        let _ = self.sync_main_hand_attributes(client);
        true
    }

    pub(crate) fn drop_main_hand_item(
        &mut self,
        all: bool,
        server: &mut crate::server::MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let hand_item = self.get_item_in_hand(PlayerHand::Main);
        if hand_item.is_air() {
            return false;
        }
        let dropped_item = if all {
            hand_item.clone()
        } else {
            hand_item.with_amount(1)
        };
        if !self.drop_item(dropped_item, server, client) {
            let _ = self.sync_inventory(client);
            return false;
        }
        let updated_item = if all {
            ItemStack::air()
        } else {
            hand_item.consume(1)
        };
        self.set_item_in_hand(PlayerHand::Main, updated_item);
        let slot_is_synced = self.sync_slot(self.held_slot, client).is_ok();
        let attributes_are_synced = self.sync_main_hand_attributes(client).is_ok();
        slot_is_synced && attributes_are_synced
    }

    pub(crate) fn drop_item(
        &mut self,
        item_stack: ItemStack,
        server: &mut crate::server::MinecraftServer,
        client: &mut Client,
    ) -> bool {
        if item_stack.is_air() {
            return false;
        }
        let mut event = ItemDropEvent::new(self as *mut Player, item_stack);
        event.dispatch(server, client);
        !event.is_cancelled()
    }

    pub(crate) fn tick(&mut self) -> Option<crate::entity::player::PlayerItemUseCompletion> {
        self.process_scheduler_tick_start();
        let current_tick = self.last_completed_client_tick;
        self.item_cooldowns
            .retain(|_, cooldown_expires_at| *cooldown_expires_at > current_tick);
        let item_use_completion = self.tick_item_use();
        self.process_scheduler_tick_end();
        item_use_completion
    }

    pub(crate) fn get_experience_pickup_is_ready(&self, current_tick: i64) -> bool {
        self.last_experience_pickup_tick
            .is_none_or(|last_pickup_tick| current_tick - last_pickup_tick >= 10)
    }

    pub(crate) fn refresh_experience_pickup_cooldown(&mut self, current_tick: i64) {
        self.last_experience_pickup_tick = Some(current_tick);
    }

    fn process_scheduler_tick_start(&mut self) {
        let mut scheduler = std::mem::take(&mut self.scheduler);
        scheduler.process_tick(self);
        self.scheduler = scheduler;
    }

    fn process_scheduler_tick_end(&mut self) {
        let mut scheduler = std::mem::take(&mut self.scheduler);
        scheduler.process_tick_end(self);
        self.scheduler = scheduler;
    }

    fn dispatch_player_death_event(
        &mut self,
        death_text: TextComponent,
        chat_message: TextComponent,
    ) -> (Option<TextComponent>, Option<TextComponent>) {
        let Some(client_ptr) = self.client else {
            return (Some(death_text), Some(chat_message));
        };
        let client = unsafe { &mut *(client_ptr as *mut Client) };
        let Some(server_ptr) = client.server_ptr else {
            return (Some(death_text), Some(chat_message));
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let mut event =
            PlayerDeathEvent::new(self as *mut Player, Some(death_text), Some(chat_message));
        event.dispatch(server, client);
        event.into_messages()
    }

    fn dispatch_player_respawn_event(&mut self) -> PlayerSpawnPoint {
        let respawn_point = self.get_respawn_point();
        let Some(client_ptr) = self.client else {
            return respawn_point;
        };
        let client = unsafe { &mut *(client_ptr as *mut Client) };
        let Some(server_ptr) = client.server_ptr else {
            return respawn_point;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let mut event = PlayerRespawnEvent::new(self as *mut Player, respawn_point);
        event.dispatch(server, client);
        event.respawn_position()
    }

    fn broadcast_death_message(&mut self, chat_message: Option<TextComponent>) -> io::Result<()> {
        let Some(chat_message) = chat_message else {
            return Ok(());
        };
        let Some(client_ptr) = self.client else {
            return Ok(());
        };
        let client = unsafe { &mut *(client_ptr as *mut Client) };
        let Some(server_ptr) = client.server_ptr else {
            return Ok(());
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        server
            .connection_manager
            .clients()
            .into_iter()
            .try_for_each(|client_arc| {
                let Ok(mut viewer_client) = client_arc.lock() else {
                    return Ok(());
                };
                if viewer_client.state != ConnectionState::Play {
                    return Ok(());
                }
                SystemChatPacket::new(chat_message.clone(), false).dispatch(&mut *viewer_client)
            })
    }

    pub(super) fn send_tick_rate(
        &self,
        client: &mut Client,
        ticks_per_second: u32,
    ) -> io::Result<()> {
        TickingStatePacket {
            tick_rate: ticks_per_second as f32,
            is_frozen: false,
        }
        .dispatch(client)?;
        TickingStepPacket::new(0).dispatch(client)
    }

    pub(crate) fn finish_client_tick(&mut self, server_tick: u64) {
        self.last_completed_client_tick = server_tick;
    }

    pub(crate) fn look(&mut self, yaw: f32, pitch: f32) {
        self.position = self.position.looking_at(yaw, pitch);
    }

    pub fn look_at_position(&mut self, target: Vector3d) -> io::Result<()> {
        self.face_position(FacePoint::Eyes, target)
    }

    pub fn look_at_entity(
        &mut self,
        entity_id: EntityId,
        target: EntityPosition,
    ) -> io::Result<()> {
        self.face_entity(
            FacePoint::Eyes,
            target.as_vector(),
            entity_id,
            FacePoint::Eyes,
        )
    }

    pub fn face_position(&mut self, face_point: FacePoint, target: Vector3d) -> io::Result<()> {
        self.send_packet(PlayerLookAtPacket::at_position(face_point, target))
    }

    pub fn face_entity(
        &mut self,
        face_point: FacePoint,
        target: Vector3d,
        entity_id: EntityId,
        target_point: FacePoint,
    ) -> io::Result<()> {
        self.send_packet(PlayerLookAtPacket::at_entity(
            face_point,
            target,
            entity_id.get_value(),
            target_point,
        ))
    }

    pub(crate) fn refresh_input(
        &mut self,
        forward: bool,
        backward: bool,
        left: bool,
        right: bool,
        jump: bool,
        shift: bool,
        sprint: bool,
    ) -> bool {
        let old_shift = self.metadata.get_flag(&definitions::is_crouching());
        self.inputs
            .refresh(forward, backward, left, right, jump, shift, sprint);
        self.metadata.set_flag(&definitions::is_crouching(), shift);
        self.refresh_pose();
        old_shift != shift
    }

    pub const fn get_inputs(&self) -> PlayerInputs {
        self.inputs
    }

    pub fn get_eye_height(&self) -> f64 {
        self.entity_type.get_eye_height()
    }

    pub const fn get_last_keep_alive(&self) -> i64 {
        self.last_keep_alive
    }

    pub fn refresh_keep_alive(&mut self, last_keep_alive: i64) {
        self.last_keep_alive = last_keep_alive;
    }

    pub const fn get_did_answer_keep_alive(&self) -> bool {
        self.answer_keep_alive
    }

    pub fn refresh_answer_keep_alive(&mut self, answer_keep_alive: bool) {
        self.answer_keep_alive = answer_keep_alive;
    }

    pub(crate) fn get_metadata_packet(&self) -> SetEntityDataPacket {
        SetEntityDataPacket::new(
            self.get_entity_id().get_value(),
            self.metadata.get_entries(),
        )
    }

    pub(crate) fn get_dirty_metadata_packet(&mut self) -> Option<SetEntityDataPacket> {
        let dirty_entries = self.metadata.drain_dirty_entries();
        if dirty_entries.is_empty() {
            return None;
        }
        Some(SetEntityDataPacket::new(
            self.get_entity_id().get_value(),
            dirty_entries,
        ))
    }

    pub fn get_position(&self) -> crate::entity::EntityPosition {
        crate::entity::EntityPosition::new(
            self.position.x,
            self.position.y,
            self.position.z,
            self.position.yaw,
            self.position.pitch,
        )
    }

    pub(crate) fn get_player_info_packet(&self) -> PlayerInfoUpdatePacket {
        let properties = self
            .skin
            .as_ref()
            .map(|skin| vec![skin.get_property()])
            .unwrap_or_default();
        PlayerInfoUpdatePacket::add_player_with_properties(
            self.uuid,
            self.username.clone(),
            self.listed,
            properties,
        )
    }

    pub(crate) fn get_game_mode_packet(&self) -> GameEventPacket {
        GameEventPacket::from(GameEvent::ChangeGameMode(self.game_mode))
    }

    pub(crate) fn get_abilities_packet(&self) -> PlayerAbilitiesPacket {
        let mut flags = 0;
        if self.living.is_invulnerable() {
            flags |= PlayerAbilitiesPacket::INVULNERABLE;
        }
        if self.flying {
            flags |= PlayerAbilitiesPacket::FLYING;
        }
        if self.allow_flying {
            flags |= PlayerAbilitiesPacket::ALLOW_FLYING;
        }
        if self.instant_break {
            flags |= PlayerAbilitiesPacket::INSTANT_BREAK;
        }
        PlayerAbilitiesPacket::new(flags, self.flying_speed, self.field_view_modifier)
    }

    pub(crate) fn spawn_packet(&self) -> SpawnEntityPacket {
        SpawnEntityPacket {
            entity_id: self.get_entity_id().get_value(),
            uuid: self.uuid,
            entity_type: self.entity_type.id(),
            x: self.position.x,
            y: self.position.y,
            z: self.position.z,
            velocity: Velocity(Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            pitch: EntityAngle(self.position.pitch),
            yaw: EntityAngle(self.position.yaw),
            head_yaw: EntityAngle(self.position.yaw),
            data: 0,
        }
    }

    pub(crate) fn get_visible_equipment_packet(&self) -> SetEquipmentPacket {
        SetEquipmentPacket::new(
            self.get_entity_id().get_value(),
            vec![
                self.equipment_entry(EquipmentSlot::MainHand),
                self.equipment_entry(EquipmentSlot::OffHand),
                self.equipment_entry(EquipmentSlot::Boots),
                self.equipment_entry(EquipmentSlot::Leggings),
                self.equipment_entry(EquipmentSlot::Chestplate),
                self.equipment_entry(EquipmentSlot::Helmet),
                self.equipment_entry(EquipmentSlot::Body),
            ],
        )
    }

    fn equipment_entry(&self, equipment_slot: EquipmentSlot) -> EntityEquipmentEntry {
        EntityEquipmentEntry {
            slot: equipment_slot.get_entity_equipment_slot(),
            item: Slot::from_item_stack(&self.get_equipment(equipment_slot)),
        }
    }

    pub(crate) fn get_head_look_packet(&self) -> EntityHeadLookPacket {
        EntityHeadLookPacket {
            entity_id: self.get_entity_id().get_value(),
            head_yaw: EntityAngle(self.position.yaw),
        }
    }

    pub(crate) fn get_animation_packet(&self, hand: PlayerHand) -> EntityAnimationPacket {
        let animation = match hand {
            PlayerHand::Main => EntityAnimation::SwingMainArm,
            PlayerHand::Off => EntityAnimation::SwingOffHand,
        };
        EntityAnimationPacket {
            entity_id: self.get_entity_id().get_value(),
            animation,
        }
    }

    pub(super) fn refresh_pose(&mut self) {
        let pose = if self
            .metadata
            .get_flag(&definitions::is_flying_with_elytra())
        {
            EntityPose::FallFlying
        } else if self.metadata.get_flag(&definitions::is_swimming()) {
            EntityPose::Swimming
        } else if self.metadata.get_flag(&definitions::is_crouching()) {
            EntityPose::Sneaking
        } else {
            EntityPose::Standing
        };
        self.metadata.set(
            &definitions::get_pose(),
            MetadataValue::Pose(pose.get_protocol_id()),
        );
    }

    fn apply_game_mode(&mut self, game_mode: GameMode) {
        self.game_mode = game_mode;
        self.allow_flying = game_mode.allows_flying();
        self.instant_break = game_mode.has_instant_break();
        self.living.set_invulnerable(game_mode.is_invulnerable());
        self.has_entity_collision = game_mode != GameMode::Spectator;
        self.prevents_block_placement = game_mode != GameMode::Spectator;
        if game_mode == GameMode::Spectator || !game_mode.allows_flying() {
            self.flying = game_mode.allows_flying();
        }
    }

    fn sync_game_mode_state(&self, client: &mut Client) -> io::Result<()> {
        self.get_game_mode_packet().dispatch(client)?;
        PlayerInfoUpdatePacket::update_game_mode(self.uuid, self.game_mode).dispatch(client)?;
        self.get_abilities_packet().dispatch(client)
    }

    pub(super) fn sync_health(&mut self) -> io::Result<()> {
        let packet =
            SetHealthPacket::new(self.living.get_health(), self.food, self.food_saturation);
        let Some(client) = self.get_client_mut() else {
            return Ok(());
        };
        if client.state != ConnectionState::Play {
            return Ok(());
        }
        packet.dispatch(client)
    }

    fn sync_experience(&mut self) -> io::Result<()> {
        let packet = SetExperiencePacket::new(
            self.experience,
            self.experience_level,
            self.total_experience,
        );
        let Some(client) = self.get_client_mut() else {
            return Ok(());
        };
        if client.state != ConnectionState::Play {
            return Ok(());
        }
        packet.dispatch(client)
    }

    fn dispatch_game_event(&mut self, game_event: GameEvent) -> io::Result<()> {
        let packet = GameEventPacket::from(game_event);
        let Some(client) = self.get_client_mut() else {
            return Ok(());
        };
        if client.state != ConnectionState::Play {
            return Ok(());
        }
        packet.dispatch(client)
    }

    fn refresh_abilities(&mut self) -> io::Result<()> {
        if !self.has_entered_world() {
            return Ok(());
        }
        let packet = self.get_abilities_packet();
        let Some(client) = self.get_client_mut() else {
            return Ok(());
        };
        if client.state != ConnectionState::Play {
            return Ok(());
        }
        packet.dispatch(client)
    }

    fn dispatch_player_info_update(&mut self, packet: PlayerInfoUpdatePacket) -> io::Result<()> {
        if !self.has_entered_world() {
            return Ok(());
        }
        let Some(client) = self.get_client_mut() else {
            return Ok(());
        };
        if client.state != ConnectionState::Play {
            return Ok(());
        }
        packet.dispatch(client)
    }

    fn refresh_dirty_metadata_to_viewers(&mut self) {
        if !self.has_entered_world() {
            return;
        }
        let Some(metadata_packet) = self.get_dirty_metadata_packet() else {
            return;
        };
        let metadata_entity_id = metadata_packet.entity_id;
        let metadata_entries = metadata_packet.entries.0;
        let _ = self.dispatch_to_viewer_clients(|viewer_client| {
            SetEntityDataPacket::new(metadata_entity_id, metadata_entries.clone())
                .dispatch(viewer_client)
        });
    }

    fn refresh_game_mode_to_viewers(&mut self) -> bool {
        let packet = PlayerInfoUpdatePacket::update_game_mode(self.uuid, self.game_mode);
        self.dispatch_to_other_play_clients(|viewer_client| packet.clone().dispatch(viewer_client))
            .is_ok()
    }

    fn dispatch_to_viewer_clients(
        &mut self,
        mut dispatch_packet: impl FnMut(&mut Client) -> io::Result<()>,
    ) -> io::Result<()> {
        let viewer_ids = self.get_viewers();
        let Some(client_ptr) = self.client else {
            return Ok(());
        };
        let client = unsafe { &mut *(client_ptr as *mut Client) };
        let Some(server_ptr) = client.server_ptr else {
            return Ok(());
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        server
            .connection_manager
            .clients()
            .into_iter()
            .try_for_each(|viewer_client| {
                let Ok(mut viewer_client) = viewer_client.lock() else {
                    return Ok(());
                };
                let client_is_entity_viewer = viewer_client
                    .player_entity_id()
                    .is_some_and(|viewer_id| viewer_ids.contains(&viewer_id));
                if !client_is_entity_viewer || viewer_client.state != ConnectionState::Play {
                    return Ok(());
                }
                dispatch_packet(&mut viewer_client)
            })
    }

    fn broadcast_to_play_clients(
        &mut self,
        mut dispatch_packet: impl FnMut(&mut Client) -> io::Result<()>,
    ) -> io::Result<()> {
        let Some(client_ptr) = self.client else {
            return Ok(());
        };
        let client = unsafe { &mut *(client_ptr as *mut Client) };
        let Some(server_ptr) = client.server_ptr else {
            if client.state != ConnectionState::Play {
                return Ok(());
            }
            return dispatch_packet(client);
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        server
            .connection_manager
            .clients()
            .into_iter()
            .try_for_each(|play_client| {
                let Ok(mut play_client) = play_client.lock() else {
                    return Ok(());
                };
                if play_client.state != ConnectionState::Play {
                    return Ok(());
                }
                dispatch_packet(&mut play_client)
            })
    }

    fn refresh_skin_for_self(
        &mut self,
        player_uuid: Uuid,
        add_player_packet: PlayerInfoUpdatePacket,
        viewer_snapshot: &PlayerViewerSnapshot,
    ) -> io::Result<()> {
        let Some(client_ptr) = self.client else {
            return Ok(());
        };
        let client = unsafe { &mut *(client_ptr as *mut Client) };
        if client.state != ConnectionState::Play {
            return Ok(());
        }
        let mut respawn_packet = RespawnPacket::new(
            self.get_game_mode(),
            self.world_name
                .clone()
                .unwrap_or_else(|| Identifier::minecraft("overworld")),
        );
        respawn_packet.common_player_spawn_info.last_death_location =
            self.get_death_location().map(|death_location| GlobalPos {
                dimension: death_location.get_dimension().clone(),
                position: Position {
                    x: death_location.get_position().get_x().floor() as i32,
                    y: death_location.get_position().get_y().floor() as i32,
                    z: death_location.get_position().get_z().floor() as i32,
                },
            });
        respawn_packet.common_player_spawn_info.portal_cooldown = self.get_portal_cooldown();

        PlayerInfoRemovePacket::new(player_uuid).dispatch(client)?;
        RemoveEntitiesPacket::new(vec![self.get_entity_id().get_value()]).dispatch(client)?;
        add_player_packet.dispatch(client)?;
        respawn_packet.dispatch(client)?;
        GameEventPacket::from(GameEvent::StartWaitingForLevelChunks).dispatch(client)?;
        ServerDifficultyPacket::normal(false).dispatch(client)?;
        SetHealthPacket::new(
            self.get_health(),
            self.get_food(),
            self.get_food_saturation(),
        )
        .dispatch(client)?;
        SetExperiencePacket::new(
            self.get_experience(),
            self.get_experience_level(),
            self.get_total_experience(),
        )
        .dispatch(client)?;
        EntityStatusPacket {
            entity_id: self.get_entity_id().get_value(),
            status: (24 + self.get_permission_level()) as i8,
        }
        .dispatch(client)?;
        self.get_abilities_packet().dispatch(client)?;
        self.sync_inventory(client)?;
        self.synchronize_position_after_teleport(
            self.get_position(),
            Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            TeleportFlags::absolute(),
            true,
        )?;
        viewer_snapshot.dispatch_shared_state(client)
    }

    fn dispatch_to_other_play_clients(
        &mut self,
        mut dispatch_packet: impl FnMut(&mut Client) -> io::Result<()>,
    ) -> io::Result<()> {
        let Some(client_ptr) = self.client else {
            return Ok(());
        };
        let client = unsafe { &mut *(client_ptr as *mut Client) };
        let Some(server_ptr) = client.server_ptr else {
            return Ok(());
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        server
            .connection_manager
            .clients()
            .into_iter()
            .try_for_each(|viewer_client| {
                let Ok(mut viewer_client) = viewer_client.lock() else {
                    return Ok(());
                };
                if viewer_client.addr == self.addr || viewer_client.state != ConnectionState::Play {
                    return Ok(());
                }
                dispatch_packet(&mut viewer_client)
            })
    }

    pub fn schedule_remove_after_ticks(&mut self, delay_ticks: u64) {
        let _ = delay_ticks;
    }

    pub fn schedule_remove_after_duration(&mut self, duration: std::time::Duration) {
        let _ = duration;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerHand {
    Main,
    Off,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerMessageType {
    Chat,
    System,
    ActionBar,
}

impl PlayerMessageType {
    const fn is_accepted_by_chat_mode(self, chat_mode: i32) -> bool {
        match (chat_mode, self) {
            (_, Self::ActionBar) => true,
            (0, Self::Chat | Self::System) => true,
            (1, Self::System) => true,
            _ => false,
        }
    }
}

impl PlayerHand {
    pub const fn get_protocol_id(self) -> i32 {
        match self {
            Self::Main => 0,
            Self::Off => 1,
        }
    }

    pub fn from_protocol_id(protocol_id: i32) -> Option<Self> {
        match protocol_id {
            0 => Some(Self::Main),
            1 => Some(Self::Off),
            _ => None,
        }
    }
}
