use crate::entity::metadata::{MetadataHolder, definitions};
use crate::entity::player::ChunkUpdateLimitChecker;
use crate::entity::player::chunks::PlayerChunk;
use crate::entity::player::input::PlayerInputs;
use crate::entity::player::position::PlayerPosition;
use crate::entity::player::skin::PlayerSkin;
use crate::entity::{EntityId, EntityPosition, EquipmentSlot, PlayerSpawnPoint};
use crate::events::inventory_close::InventoryCloseEvent;
use crate::events::item_drop::ItemDropEvent;
use crate::events::player_change_held_slot::PlayerChangeHeldSlotEvent;
use crate::events::player_death::PlayerDeathEvent;
use crate::events::player_game_mode_change::PlayerGameModeChangeEvent;
use crate::events::player_swap_item::PlayerSwapItemEvent;
use crate::inventory::{ClickPreprocessor, Inventory, PlayerInventory};
use crate::network::client::instance::Client;
use crate::scheduler::{ContextScheduler, Task, TaskSchedule};
use crate::world::WorldHandle;
use spinel_core::entity::game_mode::GameMode;
use spinel_core::network::clientbound::play::chunk_data::ChunkDataAndUpdateLightPacket;
use spinel_core::network::clientbound::play::container_close::ContainerClosePacket;
use spinel_core::network::clientbound::play::entity_animation::{
    EntityAnimation, EntityAnimationPacket,
};
use spinel_core::network::clientbound::play::entity_head_look::EntityHeadLookPacket;
use spinel_core::network::clientbound::play::game_event::{GameEvent, GameEventPacket};
use spinel_core::network::clientbound::play::player_abilities::PlayerAbilitiesPacket;
use spinel_core::network::clientbound::play::player_combat_kill::PlayerCombatKillPacket;
use spinel_core::network::clientbound::play::player_info_update::PlayerInfoUpdatePacket;
use spinel_core::network::clientbound::play::recipe_book_add::RecipeBookAddPacket;
use spinel_core::network::clientbound::play::respawn::RespawnPacket;
use spinel_core::network::clientbound::play::server_difficulty::ServerDifficultyPacket;
use spinel_core::network::clientbound::play::set_entity_data::SetEntityDataPacket;
use spinel_core::network::clientbound::play::set_equipment::{
    EntityEquipmentEntry, SetEquipmentPacket,
};
use spinel_core::network::clientbound::play::set_experience::SetExperiencePacket;
use spinel_core::network::clientbound::play::set_health::SetHealthPacket;
use spinel_core::network::clientbound::play::set_held_slot::SetHeldSlotPacket;
use spinel_core::network::clientbound::play::spawn_entity::{EntityAngle, SpawnEntityPacket};
use spinel_core::network::clientbound::play::start_configuration::StartConfigurationPacket;
use spinel_core::network::clientbound::play::sync_player_pos::SyncPlayerPositionPacket;
use spinel_core::network::clientbound::play::system_chat::SystemChatPacket;
use spinel_core::network::clientbound::play::ticking_state::TickingStatePacket;
use spinel_core::network::clientbound::play::ticking_step::TickingStepPacket;
use spinel_core::network::clientbound::play::update_recipes::UpdateRecipesPacket;
use spinel_nbt::{TagHandler, Taggable};
use spinel_network::ConnectionState;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::{Identifier, Slot, TeleportFlags, Vector3d, Velocity};
use spinel_registry::{EntityType, ItemStack};
use spinel_utils::component::color::{NamedTextColor, TextColor};
use spinel_utils::component::text::TextComponent;
use std::collections::{BTreeMap, HashSet, VecDeque};
use std::io;
use std::net::SocketAddr;
use uuid::Uuid;

const MIN_CHUNKS_PER_TICK: f32 = 0.01;
const MAX_CHUNKS_PER_TICK: f32 = 64.0;
const CHUNKS_PER_TICK_MULTIPLIER: f32 = 1.0;
const DEFAULT_CLIENT_CHUNK_VIEW_DISTANCE: i32 = 8;
const PLAYER_CHUNK_UPDATE_LIMITER_HISTORY_SIZE: usize = 8;

pub struct Player {
    entity_id: EntityId,
    pub uuid: Uuid,
    pub username: String,
    pub protocol_version: i32,
    pub addr: SocketAddr,
    skin: Option<PlayerSkin>,
    display_name: Option<TextComponent>,
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
    world_name: Option<Identifier>,
    hardcore: bool,
    is_dead: bool,
    can_pickup_item: bool,
    respawn_point: PlayerSpawnPoint,
    inventory: PlayerInventory,
    open_inventory: Option<Inventory>,
    pub(super) click_preprocessor: ClickPreprocessor,
    held_slot: i32,
    inputs: PlayerInputs,
    flying: bool,
    allow_flying: bool,
    instant_break: bool,
    invulnerable: bool,
    flying_speed: f32,
    field_view_modifier: f32,
    pub(super) metadata: MetadataHolder,
    tag_handler: TagHandler,
    has_entered_world: bool,
    on_ground: bool,
    last_sent_teleport_id: i32,
    last_received_teleport_id: i32,
    pub(super) last_completed_client_tick: u64,
    did_close_inventory: bool,
    pub(super) client: Option<usize>,
    pub(super) item_cooldowns: BTreeMap<String, u64>,
    pub(super) alive_ticks: u64,
    pub(super) item_use_hand: Option<PlayerHand>,
    pub(super) start_item_use_time: u64,
    pub(super) item_use_time: u64,
    packet_queue: VecDeque<QueuedPlayerPacket>,
    pub(super) chunk_queue: VecDeque<QueuedPlayerChunk>,
    pub(super) queued_client_chunks: HashSet<PlayerChunk>,
    pub(super) client_sent_chunks: HashSet<PlayerChunk>,
    pub(super) needs_chunk_position_sync: bool,
    pub(super) max_chunk_batch_lead: i32,
    pub(super) chunk_batch_lead: i32,
    pub(super) target_chunks_per_tick: f32,
    pub(super) pending_chunk_count: f32,
    scheduler: ContextScheduler<Player>,
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
        Self {
            entity_id: EntityId::next(),
            uuid,
            username,
            protocol_version,
            addr,
            skin: None,
            display_name: None,
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
            world_name: None,
            hardcore: false,
            is_dead: false,
            can_pickup_item: true,
            respawn_point,
            inventory: PlayerInventory::new(),
            open_inventory: None,
            click_preprocessor: ClickPreprocessor::default(),
            held_slot: 0,
            inputs: PlayerInputs::default(),
            flying: false,
            allow_flying: false,
            instant_break: false,
            invulnerable: false,
            flying_speed: 0.05,
            field_view_modifier: 0.1,
            metadata: MetadataHolder::default(),
            tag_handler: TagHandler::new_handler(),
            has_entered_world: false,
            on_ground: false,
            last_sent_teleport_id: 0,
            last_received_teleport_id: 0,
            last_completed_client_tick: 0,
            did_close_inventory: false,
            client: None,
            item_cooldowns: BTreeMap::new(),
            alive_ticks: 0,
            item_use_hand: None,
            start_item_use_time: 0,
            item_use_time: 0,
            packet_queue: VecDeque::new(),
            chunk_queue: VecDeque::new(),
            queued_client_chunks: HashSet::new(),
            client_sent_chunks: HashSet::new(),
            needs_chunk_position_sync: true,
            max_chunk_batch_lead: 1,
            chunk_batch_lead: 0,
            target_chunks_per_tick: 9.0,
            pending_chunk_count: 0.0,
            scheduler: ContextScheduler::new(),
        }
    }

    pub fn set_pending_options(&mut self, spawning_world: Uuid, hardcore: bool) {
        self.pending_spawning_world = Some(spawning_world);
        self.hardcore = hardcore;
    }

    pub const fn pending_spawning_world(&self) -> Option<Uuid> {
        self.pending_spawning_world
    }

    pub fn scheduler(&mut self) -> &mut ContextScheduler<Player> {
        &mut self.scheduler
    }

    pub fn schedule_next_tick(
        &mut self,
        callback: impl FnMut(&mut Player) -> TaskSchedule + Send + 'static,
    ) -> Task {
        self.scheduler.schedule_next_tick(callback)
    }

    pub const fn current_world(&self) -> Option<Uuid> {
        self.current_world
    }

    pub(crate) fn set_world(&mut self, world: Uuid) {
        self.current_world = Some(world);
    }

    pub const fn is_hardcore(&self) -> bool {
        self.hardcore
    }

    pub fn unsafe_init(
        &mut self,
        client: &mut Client,
        ticks_per_second: u32,
        world_name: Identifier,
        chunk_packets: Vec<
            spinel_core::network::clientbound::play::chunk_data::ChunkDataAndUpdateLightPacket,
        >,
        time_packet: spinel_core::network::clientbound::play::set_time::SetTimePacket,
    ) -> io::Result<()> {
        self.pending_spawning_world = None;
        self.is_dead = false;
        self.world_name = Some(world_name.clone());
        self.enter_world(
            client,
            ticks_per_second,
            world_name,
            chunk_packets,
            time_packet,
        )
    }

    pub(crate) fn unsafe_init_with_chunk_positions(
        &mut self,
        client: &mut Client,
        ticks_per_second: u32,
        world_name: Identifier,
        chunks: Vec<PlayerChunk>,
        time_packet: spinel_core::network::clientbound::play::set_time::SetTimePacket,
    ) -> io::Result<()> {
        self.pending_spawning_world = None;
        self.is_dead = false;
        self.world_name = Some(world_name.clone());
        self.enter_world_with_chunk_positions(
            client,
            ticks_per_second,
            world_name,
            chunks,
            time_packet,
        )
    }

    pub(super) fn prepare_instance_spawn(&mut self, world_name: Identifier) {
        self.pending_spawning_world = None;
        self.is_dead = false;
        self.world_name = Some(world_name);
    }

    pub fn start_configuration_phase(&mut self) -> io::Result<()> {
        let Some(client) = self.client_mut() else {
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

    pub const fn is_dead(&self) -> bool {
        self.is_dead
    }

    pub fn kill(&mut self) -> io::Result<()> {
        if self.is_dead {
            return Ok(());
        }
        self.is_dead = true;
        let default_death_text = TextComponent::literal("Killed by poor programming.");
        let default_chat_message = TextComponent::literal(format!(
            "{} was killed by poor programming.",
            self.username()
        ));
        let (death_text, chat_message) =
            self.dispatch_player_death_event(default_death_text, default_chat_message);
        let entity_id = self.entity_id.value();
        if let Some(client) = self.client_mut() {
            if client.state == ConnectionState::Play
                && let Some(death_text) = death_text
            {
                PlayerCombatKillPacket::new(entity_id, death_text).dispatch(client)?;
            }
        }
        self.broadcast_death_message(chat_message)?;
        Ok(())
    }

    pub fn respawn(&mut self) -> io::Result<bool> {
        if !self.is_dead {
            return Ok(false);
        }
        self.is_dead = false;
        self.refresh_pose();
        let respawn_dimension = self
            .world_name
            .clone()
            .unwrap_or_else(|| Identifier::minecraft("overworld"));
        let game_mode = self.game_mode;
        if let Some(client) = self.client_mut()
            && client.state == ConnectionState::Play
        {
            RespawnPacket::new(game_mode, respawn_dimension).dispatch(client)?;
            GameEventPacket::from(GameEvent::StartWaitingForLevelChunks).dispatch(client)?;
            ServerDifficultyPacket::normal(false).dispatch(client)?;
            SetHealthPacket::new(20.0, 20, 5.0).dispatch(client)?;
            SetExperiencePacket::new(0.0, 0, 0).dispatch(client)?;
            self.refresh_abilities()?;
        }
        Ok(true)
    }

    pub fn is_online(&self) -> bool {
        self.client().is_some_and(Client::is_online)
    }

    pub const fn can_pickup_item(&self) -> bool {
        self.can_pickup_item
    }

    pub fn set_can_pickup_item(&mut self, can_pickup_item: bool) {
        self.can_pickup_item = can_pickup_item;
    }

    pub fn refresh_recipes(&mut self) -> io::Result<()> {
        let Some(client) = self.client_mut() else {
            return Ok(());
        };
        UpdateRecipesPacket.dispatch(client)?;
        RecipeBookAddPacket::reset_empty().dispatch(client)
    }

    pub fn add_packet_to_queue(&mut self, packet: QueuedPlayerPacket) -> bool {
        if self.packet_queue.len() >= Self::PLAYER_PACKET_QUEUE_SIZE {
            let _ = self.kick(Self::too_many_packets_text());
            return false;
        }
        self.packet_queue.push_back(packet);
        true
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

    pub fn queued_packet_count(&self) -> usize {
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

    pub const fn chunk_batch_lead(&self) -> i32 {
        self.chunk_batch_lead
    }

    pub const fn max_chunk_batch_lead(&self) -> i32 {
        self.max_chunk_batch_lead
    }

    pub const fn target_chunks_per_tick(&self) -> f32 {
        self.target_chunks_per_tick
    }

    pub const fn pending_chunk_count(&self) -> f32 {
        self.pending_chunk_count
    }

    pub const fn client_chunk_view_distance(&self) -> i32 {
        self.client_chunk_view_distance
    }

    pub fn set_client_chunk_view_distance(&mut self, client_chunk_view_distance: i32) {
        self.client_chunk_view_distance = client_chunk_view_distance.max(0);
    }

    pub fn effective_chunk_view_distance(&self, world_view_distance: i32) -> i32 {
        self.client_chunk_view_distance
            .min(world_view_distance)
            .max(0)
            + 1
    }

    pub const fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub const fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn skin(&self) -> Option<&PlayerSkin> {
        self.skin.as_ref()
    }

    pub fn set_skin(&mut self, skin: Option<PlayerSkin>) -> io::Result<()> {
        self.skin = skin;
        self.dispatch_player_info_update(self.player_info_packet())
    }

    pub fn display_name(&self) -> Option<&TextComponent> {
        self.display_name.as_ref()
    }

    pub fn set_display_name(&mut self, display_name: Option<TextComponent>) -> io::Result<()> {
        self.display_name = display_name;
        let packet =
            PlayerInfoUpdatePacket::update_display_name(self.uuid, self.display_name.clone());
        self.dispatch_player_info_update(packet)
    }

    pub const fn protocol_version(&self) -> i32 {
        self.protocol_version
    }

    pub const fn address(&self) -> SocketAddr {
        self.addr
    }

    pub(crate) fn set_client(&mut self, client: &mut Client) {
        self.client = Some(client as *mut Client as usize);
    }

    pub fn client(&self) -> Option<&Client> {
        self.client
            .map(|client| unsafe { &*(client as *const Client) })
    }

    pub fn world(&self) -> Option<WorldHandle> {
        let client = self.client()?;
        let server = client.server_ptr?;
        let current_world = self.current_world?;
        Some(WorldHandle::new(server, current_world))
    }

    pub(crate) fn client_mut(&mut self) -> Option<&mut Client> {
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
        self.sync_game_mode_state(client).is_ok()
    }

    pub fn game_mode(&self) -> GameMode {
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
        self.invulnerable
    }

    pub fn is_sneaking(&self) -> bool {
        self.metadata.flag(&definitions::is_crouching())
    }

    pub fn is_sprinting(&self) -> bool {
        self.metadata.flag(&definitions::is_sprinting())
    }

    pub const fn is_listed(&self) -> bool {
        self.listed
    }

    pub const fn latency(&self) -> i32 {
        self.latency
    }

    pub const fn flying_speed(&self) -> f32 {
        self.flying_speed
    }

    pub const fn field_view_modifier(&self) -> f32 {
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

    pub fn set_allow_flying(&mut self, allow_flying: bool) -> io::Result<()> {
        self.allow_flying = allow_flying;
        self.refresh_abilities()
    }

    pub fn set_instant_break(&mut self, instant_break: bool) -> io::Result<()> {
        self.instant_break = instant_break;
        self.refresh_abilities()
    }

    pub fn set_invulnerable(&mut self, invulnerable: bool) -> io::Result<()> {
        self.invulnerable = invulnerable;
        self.refresh_abilities()
    }

    pub fn set_sneaking(&mut self, sneaking: bool) {
        self.metadata
            .set_flag(&definitions::is_crouching(), sneaking);
        if !self.is_flying() {
            self.refresh_pose();
        }
    }

    pub fn set_sprinting(&mut self, sprinting: bool) -> bool {
        let old_sprint = self.is_sprinting();
        self.metadata
            .set_flag(&definitions::is_sprinting(), sprinting);
        old_sprint != sprinting
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

    pub fn respawn_point(&self) -> PlayerSpawnPoint {
        self.respawn_point
    }

    pub fn inventory(&mut self) -> &mut PlayerInventory {
        &mut self.inventory
    }

    pub fn inventory_ref(&self) -> &PlayerInventory {
        &self.inventory
    }

    pub fn open_inventory(&mut self, inventory: Inventory) {
        self.open_inventory = Some(inventory);
    }

    pub fn close_inventory(&mut self) {
        self.click_preprocessor.clear_cache();
        self.open_inventory = None;
    }

    pub(crate) fn close_inventory_with_client(
        &mut self,
        from_client: bool,
        server: &mut crate::server::MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let window_id = self
            .opened_inventory()
            .map(|inventory| inventory.window_id())
            .unwrap_or_else(|| self.inventory_ref().window_id());
        self.close_inventory_window_with_client(from_client, window_id, server, client)
    }

    pub(crate) fn close_inventory_window_with_client(
        &mut self,
        from_client: bool,
        window_id: i32,
        server: &mut crate::server::MinecraftServer,
        client: &mut Client,
    ) -> bool {
        if window_id == self.inventory_ref().window_id() {
            self.close_inventory();
            return self.sync_player_inventory_window_contents(client).is_ok();
        }
        let Some(open_inventory) = self.opened_inventory().cloned() else {
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
        let cursor_item = self.inventory_ref().cursor_item().clone();
        self.close_inventory();
        self.inventory().set_cursor_item(ItemStack::air());
        if !cursor_item.is_air() && !self.drop_item(cursor_item.clone(), server, client) {
            let _ = self.inventory().add_item_stack(cursor_item);
        }
        let player_inventory_window_is_synced =
            self.sync_player_inventory_window_contents(client).is_ok();
        if !from_client {
            let packet_result = ContainerClosePacket {
                container_id: event.inventory().id().into(),
            }
            .dispatch(client)
            .is_ok();
            self.did_close_inventory = false;
            return packet_result && player_inventory_window_is_synced;
        }
        self.did_close_inventory = false;
        player_inventory_window_is_synced
    }

    pub fn opened_inventory(&self) -> Option<&Inventory> {
        self.open_inventory.as_ref()
    }

    pub(crate) fn opened_inventory_mut(&mut self) -> Option<&mut Inventory> {
        self.open_inventory.as_mut()
    }

    pub(crate) fn click_preprocessor(&mut self) -> &mut ClickPreprocessor {
        &mut self.click_preprocessor
    }

    pub fn did_close_inventory(&self) -> bool {
        self.did_close_inventory
    }

    pub fn set_did_close_inventory(&mut self, did_close_inventory: bool) {
        self.did_close_inventory = did_close_inventory;
    }

    pub fn held_slot(&self) -> i32 {
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
        let old_slot = self.held_slot();
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
        if self.item_use_hand() != Some(PlayerHand::Off) {
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

    pub fn next_teleport_id(&mut self) -> i32 {
        self.last_sent_teleport_id += 1;
        self.last_sent_teleport_id
    }

    pub const fn last_sent_teleport_id(&self) -> i32 {
        self.last_sent_teleport_id
    }

    pub const fn last_received_teleport_id(&self) -> i32 {
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
            self.next_teleport_id()
        } else {
            -1
        };
        let packet = SyncPlayerPositionPacket {
            teleport_id,
            x: position.x(),
            y: position.y(),
            z: position.z(),
            velocity_x: velocity.x,
            velocity_y: velocity.y,
            velocity_z: velocity.z,
            yaw: position.yaw(),
            pitch: position.pitch(),
            flags,
        };
        let Some(client) = self.client_mut() else {
            return Ok(());
        };
        if client.state != ConnectionState::Play {
            return Ok(());
        }
        packet.dispatch(client)
    }

    pub fn item_in_hand(&self, hand: PlayerHand) -> ItemStack {
        let equipment_slot = match hand {
            PlayerHand::Main => EquipmentSlot::MainHand,
            PlayerHand::Off => EquipmentSlot::OffHand,
        };
        self.equipment(equipment_slot)
    }

    pub fn set_item_in_hand(&mut self, hand: PlayerHand, item_stack: ItemStack) -> bool {
        let equipment_slot = match hand {
            PlayerHand::Main => EquipmentSlot::MainHand,
            PlayerHand::Off => EquipmentSlot::OffHand,
        };
        self.set_equipment(equipment_slot, item_stack)
    }

    pub fn equipment(&self, equipment_slot: EquipmentSlot) -> ItemStack {
        self.inventory_ref()
            .equipment(equipment_slot, self.held_slot)
    }

    pub fn set_equipment(&mut self, equipment_slot: EquipmentSlot, item_stack: ItemStack) -> bool {
        self.inventory
            .set_equipment(equipment_slot, self.held_slot, item_stack)
    }

    pub(crate) fn swap_item_hands(
        &mut self,
        server: &mut crate::server::MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let main_hand_item = self.item_in_hand(PlayerHand::Main);
        let off_hand_item = self.item_in_hand(PlayerHand::Off);
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
        let hand_item = self.item_in_hand(PlayerHand::Main);
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
        self.process_queued_packets_from_connection();
        let current_tick = self.last_completed_client_tick;
        self.item_cooldowns
            .retain(|_, cooldown_expires_at| *cooldown_expires_at > current_tick);
        let item_use_completion = self.tick_item_use();
        self.process_scheduler_tick_end();
        item_use_completion
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

    fn process_queued_packets_from_connection(&mut self) {
        let Some(client_ptr) = self.client else {
            return;
        };
        let client = unsafe { &mut *(client_ptr as *mut Client) };
        let Some(server_ptr) = client.server_ptr else {
            return;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        self.interpret_packet_queue(server, client);
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
        let old_shift = self.metadata.flag(&definitions::is_crouching());
        self.inputs
            .refresh(forward, backward, left, right, jump, shift, sprint);
        self.metadata.set_flag(&definitions::is_crouching(), shift);
        self.refresh_pose();
        old_shift != shift
    }

    pub(crate) fn set_flying_with_elytra(&mut self, flying_with_elytra: bool) -> bool {
        let old_flying_with_elytra = self.metadata.flag(&definitions::is_flying_with_elytra());
        self.metadata
            .set_flag(&definitions::is_flying_with_elytra(), flying_with_elytra);
        self.refresh_pose();
        old_flying_with_elytra != flying_with_elytra
    }

    pub(crate) fn metadata_packet(&self) -> SetEntityDataPacket {
        SetEntityDataPacket::new(self.entity_id().value(), self.metadata.entries())
    }

    pub(crate) fn dirty_metadata_packet(&mut self) -> Option<SetEntityDataPacket> {
        let dirty_entries = self.metadata.drain_dirty_entries();
        if dirty_entries.is_empty() {
            return None;
        }
        Some(SetEntityDataPacket::new(
            self.entity_id().value(),
            dirty_entries,
        ))
    }

    pub fn position(&self) -> crate::entity::EntityPosition {
        crate::entity::EntityPosition::new(
            self.position.x,
            self.position.y,
            self.position.z,
            self.position.yaw,
            self.position.pitch,
        )
    }

    pub(crate) fn player_info_packet(&self) -> PlayerInfoUpdatePacket {
        let properties = self
            .skin
            .as_ref()
            .map(|skin| vec![skin.property()])
            .unwrap_or_default();
        PlayerInfoUpdatePacket::add_player_with_properties(
            self.uuid,
            self.username.clone(),
            self.listed,
            properties,
        )
    }

    pub(crate) fn game_mode_packet(&self) -> GameEventPacket {
        GameEventPacket::from(GameEvent::ChangeGameMode(self.game_mode))
    }

    pub(crate) fn abilities_packet(&self) -> PlayerAbilitiesPacket {
        let mut flags = 0;
        if self.invulnerable {
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
            entity_id: self.entity_id().value(),
            uuid: self.uuid,
            entity_type: EntityType::PLAYER.id(),
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

    pub(crate) fn visible_equipment_packet(&self) -> SetEquipmentPacket {
        SetEquipmentPacket::new(
            self.entity_id().value(),
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
            slot: equipment_slot.entity_equipment_slot(),
            item: Slot::from_item_stack(&self.equipment(equipment_slot)),
        }
    }

    pub(crate) fn head_look_packet(&self) -> EntityHeadLookPacket {
        EntityHeadLookPacket {
            entity_id: self.entity_id().value(),
            head_yaw: EntityAngle(self.position.yaw),
        }
    }

    pub(crate) fn animation_packet(&self, hand: PlayerHand) -> EntityAnimationPacket {
        let animation = match hand {
            PlayerHand::Main => EntityAnimation::SwingMainArm,
            PlayerHand::Off => EntityAnimation::SwingOffHand,
        };
        EntityAnimationPacket {
            entity_id: self.entity_id().value(),
            animation,
        }
    }

    pub(super) fn refresh_pose(&mut self) {
        let pose = if self.metadata.flag(&definitions::is_flying_with_elytra()) {
            1
        } else if self.metadata.flag(&definitions::is_swimming()) {
            3
        } else if self.metadata.flag(&definitions::is_crouching()) {
            5
        } else {
            0
        };
        self.metadata
            .set(&definitions::pose(), MetadataValue::Pose(pose));
    }

    fn apply_game_mode(&mut self, game_mode: GameMode) {
        self.game_mode = game_mode;
        self.allow_flying = game_mode.allows_flying();
        self.instant_break = game_mode.has_instant_break();
        self.invulnerable = game_mode.is_invulnerable();
        if game_mode == GameMode::Spectator || !game_mode.allows_flying() {
            self.flying = game_mode.allows_flying();
        }
    }

    fn sync_game_mode_state(&self, client: &mut Client) -> io::Result<()> {
        self.game_mode_packet().dispatch(client)?;
        PlayerInfoUpdatePacket::update_game_mode(self.uuid, self.game_mode).dispatch(client)?;
        self.abilities_packet().dispatch(client)
    }

    fn refresh_abilities(&mut self) -> io::Result<()> {
        if !self.has_entered_world() {
            return Ok(());
        }
        let packet = self.abilities_packet();
        let Some(client) = self.client_mut() else {
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
        let Some(client) = self.client_mut() else {
            return Ok(());
        };
        if client.state != ConnectionState::Play {
            return Ok(());
        }
        packet.dispatch(client)
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

impl PlayerHand {
    pub fn from_protocol_id(protocol_id: i32) -> Option<Self> {
        match protocol_id {
            0 => Some(Self::Main),
            1 => Some(Self::Off),
            _ => None,
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
