use crate::ClientPacketListener;
use crate::events::disconnect::DisconnectEvent;
use crate::events::network::inbound_packet::InboundPacketEvent;
use crate::events::network::outbound_packet::OutboundPacketEvent;
use crate::hit_target::{BlockHitTarget, ClientHitTargetTracker, HitTarget, TrackedEntity};
use crate::network::server::instance::Server;
use crate::network::socket::connect_to_server;
use spinel_core::network::serverbound::play::chunk_batch_received::ChunkBatchReceivedPacket;
use spinel_core::network::serverbound::play::client_command::ClientCommandPacket;
use spinel_core::network::serverbound::play::interact::{InteractAction, InteractPacket};
use spinel_core::network::serverbound::play::move_player_pos::MovePlayerPosPacket;
use spinel_core::network::serverbound::play::player_action::PlayerActionPacket;
use spinel_core::network::serverbound::play::swing::SwingPacket;
use spinel_core::network::serverbound::play::use_item::UseItemPacket;
use spinel_network::packet_names::PacketNameRegistry;
use spinel_network::types::{Position, Vector3d, chunk::ChunkData};
use spinel_network::{ConnectionState, PacketSender};
use spinel_utils::component::text::TextComponent;
use std::collections::HashMap;
use std::io;
use std::io::Cursor;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ServerHandle(pub Arc<Mutex<Option<Server>>>);

impl PacketSender for ServerHandle {
    fn send_packet(&mut self, id: i32, payload: &[u8]) -> io::Result<()> {
        let mut server_lock = self
            .0
            .lock()
            .map_err(|_| io::Error::other("ServerHandle lock poisoned"))?;
        let server = server_lock
            .as_mut()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotConnected, "Server missing"))?;
        server.send_packet(id, payload)
    }
}

impl ServerHandle {
    pub fn disconnect_sync(&self) {
        if let Ok(mut lock) = self.0.lock() {
            if let Some(server) = lock.as_mut() {
                server.disconnect();
            }
            *lock = None;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClientPosition {
    x: f64,
    y: f64,
    z: f64,
    is_on_ground: bool,
}

impl ClientPosition {
    pub const fn new(x: f64, y: f64, z: f64, is_on_ground: bool) -> Self {
        Self {
            x,
            y,
            z,
            is_on_ground,
        }
    }

    pub const fn x(&self) -> f64 {
        self.x
    }

    pub const fn y(&self) -> f64 {
        self.y
    }

    pub const fn z(&self) -> f64 {
        self.z
    }

    pub const fn is_on_ground(&self) -> bool {
        self.is_on_ground
    }
}

#[derive(Clone)]
pub struct MinecraftClient {
    pub state: ConnectionState,
    pub server: ServerHandle,
    position: Arc<Mutex<ClientPosition>>,
    yaw: Arc<Mutex<f32>>,
    pitch: Arc<Mutex<f32>>,
    hit_target_tracker: Arc<Mutex<ClientHitTargetTracker>>,
    active_destroying_block: Arc<Mutex<Option<BlockHitTarget>>>,
    pub registries: Arc<Mutex<crate::registry_manager::ClientRegistries>>,
    assigned_packet_listeners:
        Arc<HashMap<(ConnectionState, i32), Vec<&'static ClientPacketListener>>>,
    generic_packet_listeners: Arc<HashMap<ConnectionState, Vec<&'static ClientPacketListener>>>,
}

impl PacketSender for MinecraftClient {
    fn send_packet(&mut self, id: i32, payload: &[u8]) -> io::Result<()> {
        let server_handle = self.server.0.clone();
        let mut server_lock = server_handle
            .lock()
            .map_err(|_| io::Error::other("MinecraftClient server lock poisoned"))?;
        let server = server_lock
            .as_mut()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotConnected, "Server missing"))?;
        server.state = self.state;
        let packet_name = PacketNameRegistry::get_serverbound_packet_name(server.state, id);
        let mut outbound_packet_event =
            OutboundPacketEvent::new(server.state, id, packet_name, payload.len());
        outbound_packet_event.dispatch(self, server);
        server.send_packet(id, payload)
    }
}

impl MinecraftClient {
    pub fn new() -> Self {
        let (assigned, generic) = crate::listeners::get_client_listeners();
        Self {
            state: ConnectionState::Handshaking,
            server: ServerHandle(Arc::new(Mutex::new(None))),
            position: Arc::new(Mutex::new(ClientPosition::new(0.0, 64.0, 0.0, true))),
            yaw: Arc::new(Mutex::new(0.0)),
            pitch: Arc::new(Mutex::new(0.0)),
            hit_target_tracker: Arc::new(Mutex::new(ClientHitTargetTracker::default())),
            active_destroying_block: Arc::new(Mutex::new(None)),
            registries: Arc::new(Mutex::new(crate::registry_manager::ClientRegistries::new())),
            assigned_packet_listeners: Arc::new(assigned),
            generic_packet_listeners: Arc::new(generic),
        }
    }

    pub fn set_state(&mut self, state: ConnectionState) {
        self.state = state;
        if let Ok(mut lock) = self.server.0.lock() {
            if let Some(server) = lock.as_mut() {
                server.state = state;
            }
        }
    }

    pub fn server_state(&self) -> Option<ConnectionState> {
        self.server.0.lock().ok().and_then(|server_lock| {
            server_lock
                .as_ref()
                .filter(|server| server.is_connected())
                .map(|server| server.state)
        })
    }

    pub fn refresh_state_from_server(&mut self) -> Option<ConnectionState> {
        let state = self.server_state()?;
        self.state = state;
        Some(state)
    }

    pub fn position(&self) -> ClientPosition {
        self.position
            .lock()
            .map(|position| *position)
            .unwrap_or(ClientPosition::new(0.0, 64.0, 0.0, true))
    }

    pub fn yaw(&self) -> f32 {
        self.yaw.lock().map(|yaw| *yaw).unwrap_or(0.0)
    }

    pub fn pitch(&self) -> f32 {
        self.pitch.lock().map(|pitch| *pitch).unwrap_or(0.0)
    }

    pub fn set_rotation(&mut self, yaw: f32, pitch: f32) {
        if let Ok(mut current_yaw) = self.yaw.lock() {
            *current_yaw = yaw;
        }
        if let Ok(mut current_pitch) = self.pitch.lock() {
            *current_pitch = pitch;
        }
    }

    pub fn synchronize_position(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        is_on_ground: bool,
    ) {
        if let Ok(mut position) = self.position.lock() {
            *position = ClientPosition::new(x, y, z, is_on_ground);
        }
        self.set_rotation(yaw, pitch);
    }

    pub fn move_to(&mut self, x: f64, y: f64, z: f64, is_on_ground: bool) -> io::Result<()> {
        let flags = Self::movement_flags(is_on_ground, false);
        MovePlayerPosPacket { x, y, z, flags }.dispatch(self)?;
        if let Ok(mut position) = self.position.lock() {
            *position = ClientPosition::new(x, y, z, is_on_ground);
        }
        Ok(())
    }

    pub fn move_by(
        &mut self,
        delta_x: f64,
        delta_y: f64,
        delta_z: f64,
        is_on_ground: bool,
    ) -> io::Result<()> {
        let position = self.position();
        self.move_to(
            position.x() + delta_x,
            position.y() + delta_y,
            position.z() + delta_z,
            is_on_ground,
        )
    }

    pub fn acknowledge_chunk_batch(&mut self, desired_chunks_per_tick: f32) -> io::Result<()> {
        ChunkBatchReceivedPacket {
            desired_chunks_per_tick,
        }
        .dispatch(self)
    }

    pub fn respawn(&mut self) -> io::Result<()> {
        ClientCommandPacket {
            action: ClientCommandPacket::PERFORM_RESPAWN,
        }
        .dispatch(self)
    }

    pub fn right_click(&mut self) -> io::Result<()> {
        UseItemPacket {
            hand: 0,
            sequence: 0,
            y_rot: 0.0,
            x_rot: 0.0,
        }
        .dispatch(self)
    }

    pub fn swing(&mut self) -> io::Result<()> {
        SwingPacket { hand: 0 }.dispatch(self)
    }

    pub fn left_click(&mut self) -> io::Result<()> {
        match self.hit_target() {
            HitTarget::Entity { entity_id } => {
                self.clear_destroying_block();
                self.attack_entity(entity_id)
            }
            HitTarget::Block(block) => self.start_destroying_block(block.position, block.face),
            HitTarget::Miss => {
                self.clear_destroying_block();
                self.swing()
            }
        }
    }

    pub fn continue_left_click(&mut self) -> io::Result<()> {
        if self.active_destroying_block().is_some() {
            return self.swing();
        }
        Ok(())
    }

    pub fn release_left_click(&mut self) -> io::Result<()> {
        let Some(block) = self.active_destroying_block() else {
            return Ok(());
        };

        self.clear_destroying_block();
        self.cancel_destroying_block(block.position, block.face)
    }

    pub fn active_destroying_block(&self) -> Option<BlockHitTarget> {
        self.active_destroying_block
            .lock()
            .map(|active_destroying_block| *active_destroying_block)
            .unwrap_or(None)
    }

    pub fn clear_destroying_block(&mut self) {
        if let Ok(mut active_destroying_block) = self.active_destroying_block.lock() {
            *active_destroying_block = None;
        }
    }

    pub fn hit_target(&self) -> HitTarget {
        self.hit_target_tracker
            .lock()
            .map(|tracker| tracker.hit_target(self.eye_position(), self.view_direction()))
            .unwrap_or(HitTarget::Miss)
    }

    pub fn track_entity(&mut self, entity: TrackedEntity) {
        if let Ok(mut tracker) = self.hit_target_tracker.lock() {
            tracker.track_entity(entity);
        }
    }

    pub fn remove_tracked_entities(&mut self, entity_ids: impl IntoIterator<Item = i32>) {
        if let Ok(mut tracker) = self.hit_target_tracker.lock() {
            tracker.remove_entities(entity_ids);
        }
    }

    pub fn move_tracked_entity_by_protocol_delta(
        &mut self,
        entity_id: i32,
        delta_x: i16,
        delta_y: i16,
        delta_z: i16,
    ) {
        if let Ok(mut tracker) = self.hit_target_tracker.lock() {
            tracker.move_entity_by_protocol_delta(entity_id, delta_x, delta_y, delta_z);
        }
    }

    pub fn set_tracked_entity_position(&mut self, entity_id: i32, position: Vector3d) {
        if let Ok(mut tracker) = self.hit_target_tracker.lock() {
            tracker.set_entity_position(entity_id, position);
        }
    }

    pub fn set_tracked_block_state(&mut self, position: Position, block_state: i32) {
        if let Ok(mut tracker) = self.hit_target_tracker.lock() {
            tracker.set_block_state(position, block_state);
        }
    }

    pub fn track_chunk(&mut self, chunk_x: i32, chunk_z: i32, chunk_data: ChunkData) {
        if let Ok(mut tracker) = self.hit_target_tracker.lock() {
            tracker.track_chunk(chunk_x, chunk_z, chunk_data);
        }
    }

    pub fn remove_tracked_chunk(&mut self, chunk_x: i32, chunk_z: i32) {
        if let Ok(mut tracker) = self.hit_target_tracker.lock() {
            tracker.remove_chunk(chunk_x, chunk_z);
        }
    }

    pub fn attack_entity(&mut self, entity_id: i32) -> io::Result<()> {
        InteractPacket {
            entity_id,
            action: InteractAction::Attack,
            using_secondary_action: false,
        }
        .dispatch(self)?;
        self.swing()
    }

    pub fn start_destroying_block(
        &mut self,
        block_position: Position,
        block_face: i8,
    ) -> io::Result<()> {
        if let Ok(mut active_destroying_block) = self.active_destroying_block.lock() {
            *active_destroying_block = Some(BlockHitTarget {
                position: block_position,
                face: block_face,
            });
        }
        PlayerActionPacket {
            status: PlayerActionPacket::START_DESTROY_BLOCK,
            block_position,
            block_face,
            sequence: 0,
        }
        .dispatch(self)?;
        self.swing()
    }

    pub fn finish_destroying_block(
        &mut self,
        block_position: Position,
        block_face: i8,
    ) -> io::Result<()> {
        PlayerActionPacket {
            status: PlayerActionPacket::STOP_DESTROY_BLOCK,
            block_position,
            block_face,
            sequence: 0,
        }
        .dispatch(self)
    }

    pub fn cancel_destroying_block(
        &mut self,
        block_position: Position,
        block_face: i8,
    ) -> io::Result<()> {
        PlayerActionPacket {
            status: PlayerActionPacket::ABORT_DESTROY_BLOCK,
            block_position,
            block_face,
            sequence: 0,
        }
        .dispatch(self)
    }

    fn eye_position(&self) -> Vector3d {
        let position = self.position();
        Vector3d {
            x: position.x(),
            y: position.y() + 1.62,
            z: position.z(),
        }
    }

    fn view_direction(&self) -> Vector3d {
        let yaw = self.yaw().to_radians() as f64;
        let pitch = self.pitch().to_radians() as f64;
        let pitch_cosine = pitch.cos();
        Vector3d {
            x: -yaw.sin() * pitch_cosine,
            y: -pitch.sin(),
            z: yaw.cos() * pitch_cosine,
        }
    }

    const fn movement_flags(is_on_ground: bool, has_horizontal_collision: bool) -> u8 {
        let mut flags = 0;
        if is_on_ground {
            flags |= MovePlayerPosPacket::FLAG_ON_GROUND;
        }
        if has_horizontal_collision {
            flags |= MovePlayerPosPacket::FLAG_HORIZONTAL_COLLISION;
        }
        flags
    }

    pub async fn connect(&mut self, address: &str, port: u16) -> io::Result<()> {
        connect_to_server(self.clone(), address, port).await
    }

    pub async fn disconnect(&self) {
        self.server.disconnect_sync();
    }

    pub async fn disconnect_for_reason(&mut self, reason: impl Into<TextComponent>) {
        self.disconnect_for_reason_sync(reason.into());
    }

    pub fn disconnect_for_reason_sync(&mut self, reason: TextComponent) {
        let server_handle = self.server.0.clone();
        let Ok(mut server_lock) = server_handle.lock() else {
            return;
        };

        let Some(server) = server_lock.as_mut() else {
            return;
        };

        let mut disconnect_event = DisconnectEvent::new(server.state, reason);
        disconnect_event.dispatch(self, server);
        server.disconnect();
        *server_lock = None;
    }

    pub fn on_disconnect(&self) {}

    pub fn has_listener_for(&self, packet_id: i32, state: &ConnectionState) -> bool {
        let key = (*state, packet_id);
        self.assigned_packet_listeners.contains_key(&key)
            || self.generic_packet_listeners.contains_key(state)
    }

    pub fn dispatch_packet(
        &self,
        packet_id: i32,
        server_conn: &mut Server,
        payload: Vec<u8>,
    ) -> bool {
        let key = (server_conn.state, packet_id);

        let mut client_handle = self.clone();
        let client_ptr = &mut client_handle as *mut MinecraftClient as *mut ();

        let specific = self
            .assigned_packet_listeners
            .get(&key)
            .cloned()
            .unwrap_or_default();
        let generic = self
            .generic_packet_listeners
            .get(&server_conn.state)
            .cloned()
            .unwrap_or_default();

        if specific.is_empty() && generic.is_empty() {
            return false;
        }

        let packet_name =
            PacketNameRegistry::get_clientbound_packet_name(server_conn.state, packet_id);
        let mut inbound_packet_event =
            InboundPacketEvent::new(server_conn.state, packet_id, packet_name, payload.len());
        let mut event_client_handle = self.clone();
        inbound_packet_event.dispatch(&mut event_client_handle, server_conn);

        server_conn.payload_cursor = Some(Cursor::new(payload));

        for listener in specific {
            let Some(payload_cursor) = server_conn.payload_cursor.as_mut() else {
                return false;
            };
            payload_cursor.set_position(0);
            (listener.handler)(server_conn, client_ptr);
        }
        for listener in generic {
            let Some(payload_cursor) = server_conn.payload_cursor.as_mut() else {
                return false;
            };
            payload_cursor.set_position(0);
            (listener.handler)(server_conn, client_ptr);
        }
        server_conn.payload_cursor = None;
        true
    }
}

impl Default for MinecraftClient {
    fn default() -> Self {
        Self::new()
    }
}
