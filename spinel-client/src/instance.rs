use crate::ClientPacketListener;
use crate::network::server::instance::Server;
use crate::network::socket::connect_to_server;
use spinel_core::network::serverbound::play::chunk_batch_received::ChunkBatchReceivedPacket;
use spinel_core::network::serverbound::play::move_player_pos::MovePlayerPosPacket;
use spinel_network::{ConnectionState, PacketSender};
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
    position: ClientPosition,
    pub registries: Arc<Mutex<crate::registry_manager::ClientRegistries>>,
    assigned_packet_listeners:
        Arc<HashMap<(ConnectionState, i32), Vec<&'static ClientPacketListener>>>,
    generic_packet_listeners: Arc<HashMap<ConnectionState, Vec<&'static ClientPacketListener>>>,
}

impl PacketSender for MinecraftClient {
    fn send_packet(&mut self, id: i32, payload: &[u8]) -> io::Result<()> {
        let mut server_lock = self
            .server
            .0
            .lock()
            .map_err(|_| io::Error::other("MinecraftClient server lock poisoned"))?;
        let server = server_lock
            .as_mut()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotConnected, "Server missing"))?;
        server.state = self.state;
        server.send_packet(id, payload)
    }
}

impl MinecraftClient {
    pub fn new() -> Self {
        let (assigned, generic) = crate::listeners::get_client_listeners();
        Self {
            state: ConnectionState::Handshaking,
            server: ServerHandle(Arc::new(Mutex::new(None))),
            position: ClientPosition::new(0.0, 64.0, 0.0, true),
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
        self.server
            .0
            .lock()
            .ok()
            .and_then(|server_lock| server_lock.as_ref().map(|server| server.state))
    }

    pub fn refresh_state_from_server(&mut self) -> Option<ConnectionState> {
        let state = self.server_state()?;
        self.state = state;
        Some(state)
    }

    pub const fn position(&self) -> ClientPosition {
        self.position
    }

    pub fn move_to(&mut self, x: f64, y: f64, z: f64, is_on_ground: bool) -> io::Result<()> {
        let flags = Self::movement_flags(is_on_ground, false);
        MovePlayerPosPacket { x, y, z, flags }.dispatch(self)?;
        self.position = ClientPosition::new(x, y, z, is_on_ground);
        Ok(())
    }

    pub fn move_by(
        &mut self,
        delta_x: f64,
        delta_y: f64,
        delta_z: f64,
        is_on_ground: bool,
    ) -> io::Result<()> {
        self.move_to(
            self.position.x + delta_x,
            self.position.y + delta_y,
            self.position.z + delta_z,
            is_on_ground,
        )
    }

    pub fn acknowledge_chunk_batch(&mut self, desired_chunks_per_tick: f32) -> io::Result<()> {
        ChunkBatchReceivedPacket {
            desired_chunks_per_tick,
        }
        .dispatch(self)
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

    pub async fn connect(&mut self, address: &str, port: u16) {
        if let Err(e) = connect_to_server(self.clone(), address, port).await {
            eprintln!("Failed to connect: {}", e);
        }
    }

    pub async fn disconnect(&self) {
        self.server.disconnect_sync();
    }

    pub fn on_disconnect(&self) {
        println!("Client disconnected.");
    }

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
