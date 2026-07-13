use crate::command::CommandManager;
use crate::entity::player::QueuedPlayerPacket;
use crate::entity::{Player, PlayerHand};
use crate::events::server_started::ServerStartedEvent;
use crate::events::shutdown::ShutdownEvent;
use crate::events::signal::{ServerSignal, SignalEvent};
use crate::events::startup::StartupEvent;
use crate::network::client::instance::Client;
use crate::network::connection_manager::ConnectionManager;
use crate::registry_cache::RegistryCache;
use crate::scheduler::{Scheduler, Task, TaskSchedule};
use crate::server::packet_router::PacketRouter;
use crate::server::{Auth, EventHandler, GlobalEventHandler, GlobalPacketHandler, PacketHandler};
use crate::world::WorldManager;
use spinel_network::ConnectionState;
use spinel_network::types::ClientInformation;
use spinel_registry::Registries;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

const DEFAULT_TICKS_PER_SECOND: u32 = 20;

pub struct MinecraftServer {
    pub connection_manager: ConnectionManager,
    pub world_manager: WorldManager,
    pub registry_cache: RegistryCache,
    pub registries: Registries,
    pub command_manager: CommandManager,
    pub ticks_per_second: u32,
    pub current_tick: u64,
    auth: Auth,
    pub is_ticking: Arc<AtomicBool>,
    restart_requested: AtomicBool,
    stop_requested: AtomicBool,
    pub enforce_entity_interaction_range: bool,
    scheduler: Scheduler,
    packet_router: PacketRouter,
    pub(crate) global_event_handler: GlobalEventHandler,
    pub(crate) global_packet_handler: GlobalPacketHandler,
}

impl MinecraftServer {
    pub fn get_world_manager(&mut self) -> &mut WorldManager {
        &mut self.world_manager
    }

    pub fn init(auth: Auth) -> Self {
        let registries = Registries::new_vanilla();
        let registry_cache = RegistryCache::new(&registries);

        Self {
            connection_manager: ConnectionManager::new(),
            world_manager: WorldManager::new(),
            registry_cache,
            registries,
            command_manager: CommandManager::new(),
            ticks_per_second: DEFAULT_TICKS_PER_SECOND,
            current_tick: 0,
            auth,
            is_ticking: Arc::new(AtomicBool::new(false)),
            restart_requested: AtomicBool::new(false),
            stop_requested: AtomicBool::new(false),
            enforce_entity_interaction_range: true,
            scheduler: Scheduler::new(),
            packet_router: PacketRouter::new(),
            global_event_handler: GlobalEventHandler::new(),
            global_packet_handler: GlobalPacketHandler::new(),
        }
    }

    pub fn new() -> Self {
        Self::init(Auth::Offline)
    }

    pub fn auth(&self) -> &Auth {
        &self.auth
    }

    pub fn scheduler(&mut self) -> &mut Scheduler {
        &mut self.scheduler
    }

    pub fn get_global_event_handler(&mut self) -> &mut GlobalEventHandler {
        &mut self.global_event_handler
    }

    pub fn register_event_handler(&mut self, event_handler: impl EventHandler) {
        event_handler.register_event_handlers(&mut self.global_event_handler);
    }

    pub fn get_global_packet_handler(&mut self) -> &mut GlobalPacketHandler {
        &mut self.global_packet_handler
    }

    pub fn register_packet_handler(&mut self, packet_handler: impl PacketHandler) {
        packet_handler.register_packet_handlers(&mut self.global_packet_handler);
    }

    pub fn schedule_next_tick(&mut self, callback: impl FnMut() + Send + 'static) -> Task {
        self.scheduler
            .schedule_next_tick(callback, crate::scheduler::ExecutionType::TickStart)
    }

    pub fn schedule_end_of_tick(&mut self, callback: impl FnMut() + Send + 'static) -> Task {
        self.scheduler.schedule_end_of_tick(callback)
    }

    pub fn schedule_task(
        &mut self,
        task: impl FnMut() + Send + 'static,
        delay: TaskSchedule,
        repeat: TaskSchedule,
    ) -> Task {
        self.scheduler.schedule_task(
            task,
            delay,
            repeat,
            crate::scheduler::ExecutionType::TickStart,
        )
    }

    pub fn stop(&mut self) {
        self.stop_requested.store(false, Ordering::SeqCst);
        self.is_ticking.store(false, Ordering::SeqCst);
        self.on_shutdown();
    }

    pub fn stop_after_current_tick(&mut self) {
        self.stop_requested.store(true, Ordering::SeqCst);
    }

    pub fn restart(&mut self) {
        self.restart_requested.store(true, Ordering::SeqCst);
        self.stop();
    }

    pub fn restart_after_current_tick(&mut self) {
        self.restart_requested.store(true, Ordering::SeqCst);
        self.stop_after_current_tick();
    }

    pub(crate) fn process_lifecycle_request(&mut self) {
        if !self.stop_requested.swap(false, Ordering::SeqCst) {
            return;
        }
        self.stop();
    }

    pub fn restart_was_requested(&self) -> bool {
        self.restart_requested.load(Ordering::SeqCst)
    }

    pub(crate) fn clear_restart_request(&mut self) {
        self.restart_requested.store(false, Ordering::SeqCst);
    }

    pub fn on_startup(&mut self, bind_address: impl Into<String>, port: u16) -> bool {
        let mut startup_event = StartupEvent::new(bind_address, port);
        startup_event.dispatch(self);
        startup_event.cancelled
    }

    pub fn on_started(&mut self, bind_address: impl Into<String>, port: u16) {
        let mut server_started_event = ServerStartedEvent::new(bind_address, port);
        server_started_event.dispatch(self);
    }

    pub fn on_shutdown(&mut self) {
        let mut shutdown_event = ShutdownEvent::new();
        shutdown_event.dispatch(self);

        for client_arc in self.connection_manager.clients() {
            let Ok(mut client) = client_arc.lock() else {
                continue;
            };
            client.close_connection();
        }
    }

    pub fn on_signal(&mut self, signal: ServerSignal) -> SignalEvent {
        let mut signal_event = SignalEvent::new(signal);
        signal_event.dispatch(self);
        signal_event
    }

    pub fn has_listener_for(&self, packet_id: i32, state: &ConnectionState) -> bool {
        self.packet_router.has_listener_for(packet_id, state)
            || self
                .global_packet_handler
                .has_listener_for(*state, packet_id)
    }

    pub fn has_codec_for(&self, packet_id: i32, state: ConnectionState) -> bool {
        self.packet_router.has_codec_for(packet_id, state)
    }

    pub fn dispatch_packet(
        &mut self,
        packet_id: i32,
        client: &mut Client,
        payload: Vec<u8>,
    ) -> bool {
        let server_pointer = self as *mut Self;
        self.packet_router
            .dispatch_packet(server_pointer, packet_id, client, payload)
    }

    pub(crate) fn queue_player_packet(
        &mut self,
        packet_id: i32,
        client: &mut Client,
        payload: Vec<u8>,
    ) -> bool {
        let Some(player) = self.world_manager.player_pointer_for_client(client) else {
            return self.dispatch_packet(packet_id, client, payload);
        };
        client.server_ptr = Some(self as *mut Self as usize);
        unsafe { &mut *player }.add_packet_to_queue(QueuedPlayerPacket::new(
            client.state,
            packet_id,
            payload,
        ))
    }

    pub fn set_tick_rate(&mut self, ticks_per_second: u32) {
        self.ticks_per_second = ticks_per_second.max(1);
        self.sync_client_tick_rate();
    }

    pub(crate) fn sync_client_tick_rate(&mut self) {
        self.connection_manager
            .clients()
            .into_iter()
            .for_each(|client_arc| self.sync_client(client_arc));
    }

    fn sync_client(&mut self, client_arc: std::sync::Arc<std::sync::Mutex<Client>>) {
        let Ok(mut client) = client_arc.lock() else {
            return;
        };
        if client.state != ConnectionState::Play {
            return;
        }
        let _ = self.send_tick_rate(&mut client);
    }

    pub(crate) fn send_tick_rate(&self, client: &mut Client) -> std::io::Result<()> {
        spinel_core::network::clientbound::play::ticking_state::TickingStatePacket {
            tick_rate: self.ticks_per_second as f32,
            is_frozen: false,
        }
        .dispatch(client)?;
        spinel_core::network::clientbound::play::ticking_step::TickingStepPacket::new(0)
            .dispatch(client)
    }

    pub(crate) fn enter_player_world(&mut self, client: &mut Client) -> io::Result<()> {
        self.world_manager
            .enter_player(client, self.ticks_per_second, &self.registries)
    }

    pub(crate) fn move_player_in_world(
        &mut self,
        client: &mut Client,
        x: f64,
        y: f64,
        z: f64,
        on_ground: bool,
    ) -> io::Result<()> {
        self.world_manager
            .move_player(client, x, y, z, on_ground, &self.registries)
    }

    pub(crate) fn move_player_with_view_in_world(
        &mut self,
        client: &mut Client,
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    ) -> io::Result<()> {
        self.world_manager.move_player_with_view(
            client,
            x,
            y,
            z,
            yaw,
            pitch,
            on_ground,
            &self.registries,
        )
    }

    pub(crate) fn look_player_in_world(
        &mut self,
        client: &Client,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    ) -> io::Result<()> {
        self.world_manager
            .look_player(client, yaw, pitch, on_ground)
    }

    pub(crate) fn refresh_player_status_in_world(
        &mut self,
        client: &mut Client,
        on_ground: bool,
    ) -> io::Result<()> {
        self.world_manager.refresh_player_status(client, on_ground)
    }

    pub(crate) fn animate_player_hand_in_world(
        &mut self,
        client: &Client,
        hand: PlayerHand,
    ) -> io::Result<()> {
        self.world_manager.animate_player_hand(client, hand)
    }

    pub(crate) fn refresh_player_input_in_world(
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
        self.world_manager
            .refresh_player_input(client, forward, backward, left, right, jump, shift, sprint)
    }

    pub(crate) fn steer_client_boat_in_world(
        &mut self,
        client: &Client,
        vehicle_id: crate::entity::EntityId,
        left_paddle_turning: bool,
        right_paddle_turning: bool,
    ) -> bool {
        self.world_manager.steer_client_boat(
            client,
            vehicle_id,
            left_paddle_turning,
            right_paddle_turning,
        )
    }

    pub(crate) fn set_player_sprinting_in_world(
        &mut self,
        client: &Client,
        sprinting: bool,
    ) -> io::Result<()> {
        self.world_manager.set_player_sprinting(client, sprinting)
    }

    pub(crate) fn start_player_flying_with_elytra_in_world(
        &mut self,
        client: &Client,
    ) -> io::Result<()> {
        self.world_manager.start_player_flying_with_elytra(client)
    }

    pub(crate) fn set_player_held_slot_in_world(
        &mut self,
        client: &mut Client,
        held_slot: i32,
    ) -> io::Result<bool> {
        let server = self as *mut Self;
        self.world_manager
            .set_player_held_slot(client, held_slot, server)
    }

    pub(crate) fn refresh_player_visible_equipment_in_world(
        &mut self,
        client: &Client,
    ) -> io::Result<()> {
        self.world_manager.refresh_player_visible_equipment(client)
    }

    pub(crate) fn refresh_player_metadata_in_world(&mut self, client: &Client) -> io::Result<()> {
        self.world_manager.refresh_player_metadata(client)
    }

    pub(crate) fn refresh_player_settings_in_world(
        &mut self,
        client: &mut Client,
        settings: ClientInformation,
    ) -> io::Result<()> {
        self.world_manager.refresh_player_settings(client, settings)
    }

    pub(crate) fn world_uuid_for_client(&self, client: &Client) -> Option<uuid::Uuid> {
        self.world_manager.world_uuid_for_client(client)
    }

    pub(crate) fn place_block_in_world(
        &mut self,
        client: &Client,
        placement: crate::world::BlockHandlerPlacement,
        do_block_updates: bool,
    ) -> io::Result<bool> {
        self.world_manager
            .place_block_for_client(client, placement, do_block_updates)
    }

    pub(crate) fn break_block_in_world(
        &mut self,
        client: &Client,
        player_id: crate::entity::EntityId,
        position: crate::world::BlockPosition,
        block_face: crate::events::player_block_interact::BlockFace,
        do_block_updates: bool,
    ) -> io::Result<bool> {
        self.world_manager.break_block_for_client(
            client,
            player_id,
            position,
            block_face,
            do_block_updates,
        )
    }

    pub(crate) fn interact_block_handler_in_world(
        &mut self,
        client: &Client,
        player_id: crate::entity::EntityId,
        hand: crate::entity::PlayerHand,
        position: crate::world::BlockPosition,
        block_face: crate::events::player_block_interact::BlockFace,
        cursor_position: (f32, f32, f32),
    ) -> io::Result<bool> {
        self.world_manager.interact_block_handler_for_client(
            client,
            player_id,
            hand,
            position,
            block_face,
            cursor_position,
        )
    }

    pub(crate) fn loaded_block_in_world(
        &self,
        client: &Client,
        position: crate::world::BlockPosition,
    ) -> Option<crate::world::Block> {
        self.world_manager.loaded_block_for_client(client, position)
    }

    pub(crate) fn loaded_block_state_in_world(
        &self,
        client: &Client,
        position: crate::world::BlockPosition,
    ) -> Option<crate::world::BlockState> {
        self.world_manager
            .loaded_block_state_for_client(client, position)
    }

    pub(crate) fn client_block_entity_nbt_in_world(
        &self,
        client: &Client,
        position: crate::world::BlockPosition,
    ) -> Option<spinel_nbt::NbtCompound> {
        self.world_manager
            .client_block_entity_nbt_for_client(client, position)
    }

    pub(crate) fn block_is_self_replaceable_in_world(
        &self,
        client: &Client,
        replacement: crate::world::BlockReplacement,
    ) -> bool {
        self.world_manager
            .block_is_self_replaceable_for_client(client, replacement)
    }

    pub(crate) fn block_position_is_loaded_in_world(
        &self,
        client: &Client,
        position: crate::world::BlockPosition,
    ) -> bool {
        self.world_manager
            .block_position_is_loaded_for_client(client, position)
    }

    pub(crate) fn block_position_is_inside_world_border(
        &self,
        client: &Client,
        position: crate::world::BlockPosition,
    ) -> bool {
        self.world_manager
            .block_position_is_inside_world_border_for_client(client, position)
    }

    pub(crate) fn block_placement_collision_entity(
        &self,
        client: &Client,
        position: crate::world::BlockPosition,
    ) -> Option<crate::entity::EntityId> {
        self.world_manager
            .block_placement_collision_entity_for_client(client, position)
    }

    pub(crate) fn chunk_is_read_only_in_world(
        &self,
        client: &Client,
        position: crate::world::BlockPosition,
    ) -> bool {
        self.world_manager
            .chunk_is_read_only_for_client(client, position)
    }

    pub(crate) fn refresh_chunk_in_world(
        &mut self,
        client: &Client,
        position: crate::world::BlockPosition,
    ) -> bool {
        self.world_manager
            .refresh_chunk_for_client(client, position)
    }

    pub(crate) fn refresh_block_in_world(
        &mut self,
        client: &mut Client,
        position: crate::world::BlockPosition,
    ) -> io::Result<()> {
        self.world_manager
            .refresh_block_for_client(client, position)
    }

    pub(crate) fn refresh_block_entity_in_world(
        &mut self,
        client: &mut Client,
        position: crate::world::BlockPosition,
    ) -> io::Result<()> {
        self.world_manager
            .refresh_block_entity_for_client(client, position)
    }

    pub(crate) fn process_queued_player_packets(&mut self) {
        self.connection_manager
            .clients()
            .into_iter()
            .for_each(|client_arc| self.process_queued_player_packets_for_client(client_arc));
    }

    pub(crate) fn flush_outbound_packets(&mut self) {
        self.connection_manager
            .clients()
            .into_iter()
            .for_each(|client_arc| {
                let Ok(mut client) = client_arc.lock() else {
                    return;
                };
                let _ = client.flush_outbound_packets();
            });
    }

    fn process_queued_player_packets_for_client(
        &mut self,
        client_arc: std::sync::Arc<std::sync::Mutex<Client>>,
    ) {
        let Ok(client) = client_arc.lock() else {
            return;
        };
        if client.state != ConnectionState::Play {
            return;
        }
        let client_address = client.addr;
        drop(client);
        let mut interpreted_packets = 0;
        while interpreted_packets < Player::PLAYER_PACKET_PER_TICK {
            let Some((client, queued_packet)) =
                self.pop_next_queued_player_packet_for_client_address(&client_address)
            else {
                return;
            };
            let client = unsafe { &mut *client };
            client.state = queued_packet.state;
            self.dispatch_packet(queued_packet.packet_id, client, queued_packet.payload);
            interpreted_packets += 1;
        }
    }

    fn pop_next_queued_player_packet_for_client_address(
        &mut self,
        client_address: &SocketAddr,
    ) -> Option<(*mut Client, QueuedPlayerPacket)> {
        let player = self
            .world_manager
            .player_pointer_for_client_address(client_address)?;
        let player = unsafe { &mut *player };
        let queued_packet = player.pop_next_packet_from_queue()?;
        player
            .get_client_mut()
            .map(|client| (client as *mut Client, queued_packet))
    }
}

impl Default for MinecraftServer {
    fn default() -> Self {
        Self::new()
    }
}
