use crate::ClientPacketListener;
use crate::network::server::instance::Server;
use crate::network::socket::connect_to_server;
use spinel_network::{ConnectionState, PacketSender};
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ServerHandle(pub Arc<Mutex<Option<Server>>>);

impl PacketSender for ServerHandle {
    fn send_packet(&mut self, id: i32, payload: &[u8]) {
        if let Ok(mut lock) = self.0.lock() {
            if let Some(server) = lock.as_mut() {
                server.send_packet(id, payload);
            }
        }
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

#[derive(Clone)]
pub struct MinecraftClient {
    pub state: ConnectionState,
    pub server: ServerHandle,
    pub registries: Arc<Mutex<crate::registry_manager::ClientRegistries>>,
    assigned_packet_listeners:
        Arc<HashMap<(ConnectionState, i32), Vec<&'static ClientPacketListener>>>,
    generic_packet_listeners: Arc<HashMap<ConnectionState, Vec<&'static ClientPacketListener>>>,
}

impl PacketSender for MinecraftClient {
    fn send_packet(&mut self, id: i32, payload: &[u8]) {
        if let Ok(mut lock) = self.server.0.lock() {
            if let Some(server) = lock.as_mut() {
                server.state = self.state;
                server.send_packet(id, payload);
            }
        }
    }
}

impl MinecraftClient {
    pub fn new() -> Self {
        let (assigned, generic) = crate::listeners::get_client_listeners();
        Self {
            state: ConnectionState::Handshaking,
            server: ServerHandle(Arc::new(Mutex::new(None))),
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
            server_conn.payload_cursor.as_mut().unwrap().set_position(0);
            (listener.handler)(server_conn, client_ptr);
        }
        for listener in generic {
            server_conn.payload_cursor.as_mut().unwrap().set_position(0);
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
