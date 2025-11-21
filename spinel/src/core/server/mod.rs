use crate::core::events::connection::ConnectionEvent;
use crate::core::events::diconnection::DisconnectionEvent;
use crate::core::events::shutdown::ShutdownEvent;
use crate::core::events::startup::StartupEvent;
use crate::core::network::clientbound::configuration::disconnect::ConfigurationDisconnectPacket;
use crate::core::network::clientbound::login::disconnect::LoginDisconnectPacket;
use crate::core::network::clientbound::play::disconnect::PlayDisconnectPacket;
use crate::core::server::listeners::get_listeners;
use spinel_network::server::start_tcp_listener;
use spinel_network::{
    Client, ConnectionManager, ConnectionState, PacketListener, Player, ServerContext,
};
use spinel_utils::component::text::TextComponent;
use std::collections::HashMap;
use std::io::Cursor;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::select;
use tokio::signal;
use tokio::sync::Mutex;
use uuid::Uuid;

mod listeners;
mod modules;
pub mod registry_cache;

use registry_cache::RegistryCache;

// Manages the Minecraft server instance.
pub struct MinecraftServer {
    pub connection_manager: ConnectionManager,
    pub registry_cache: RegistryCache,
    pub registry: spinel_registry::Registry,
    assigned_packet_listeners: HashMap<(ConnectionState, i32), Vec<&'static PacketListener>>,
    generic_packet_listeners: HashMap<ConnectionState, Vec<&'static PacketListener>>,
}

impl MinecraftServer {
    // Creates a new server instance.
    pub fn new() -> Self {
        let (assigned_packet_listeners, generic_packet_listeners) = get_listeners();

        // Initialize registry
        let registry = spinel_registry::Registry::new_vanilla();

        // Initialize registry cache at startup (not on player join)
        let registry_cache = RegistryCache::new(&registry);

        MinecraftServer {
            connection_manager: ConnectionManager::new(),
            registry_cache,
            registry,
            assigned_packet_listeners,
            generic_packet_listeners,
        }
    }

    // Starts the server.
    pub async fn start(self, address: &str, port: u16) {
        let server_arc = Arc::new(Mutex::new(self));
        let shutdown_server_arc = server_arc.clone();
        let address_owned = address.to_string();

        let mut server_guard = server_arc.lock().await;
        // Check if startup event was cancelled.
        if server_guard.on_startup() {
            eprintln!("Server startup event was cancelled.");
            return;
        }
        drop(server_guard);

        let server_handle =
            tokio::spawn(async move { start_tcp_listener(server_arc, &address_owned, port).await });

        // Wait for shutdown signal or listener failure.
        select! {
            _ = signal::ctrl_c() => {
                println!("Shutdown signal received. Stopping the server.");
                let mut server = shutdown_server_arc.lock().await;
                server.on_shutdown();
            }
            result = server_handle => {
                match result {
                    Ok(Ok(_)) => println!("Server listener task completed normally."),
                    Ok(Err(e)) => eprintln!("Server listener task failed: {}", e),
                    Err(e) => eprintln!("Server listener task panicked: {:?}", e),
                }
            }
        }
    }

    // Shuts down the server.
    pub fn stop(&mut self) {
        self.on_shutdown();
    }

    // Sends a message to all players.
    pub fn broadcast(&self, component_like: impl Into<TextComponent>) {
        let final_component: TextComponent = component_like.into();
        println!("{}\x1b[0m", final_component.to_ansi_string())
    }

    // Disconnects a client.
    pub fn disconnect(&mut self, client: &mut Client, reason: impl Into<TextComponent>) {
        match client.state {
            ConnectionState::Login => LoginDisconnectPacket::new(reason).dispatch(client),
            ConnectionState::Configuration => {
                ConfigurationDisconnectPacket::new(reason).dispatch(client)
            }
            ConnectionState::Play => PlayDisconnectPacket::new(reason).dispatch(client),
            _ => (),
        }

        self.connection_manager.remove_connection(&client.addr);
    }

    // Gets an immutable player reference by UUID.
    pub fn get_player(&self, uuid: &Uuid) -> Option<&Player> {
        self.connection_manager.get_player(uuid)
    }

    // Gets a mutable player reference by UUID.
    pub fn get_player_mut(&mut self, uuid: &Uuid) -> Option<&mut Player> {
        self.connection_manager.get_player_mut(uuid)
    }

    // Gets an immutable player reference for a given client.
    pub fn get_player_for_client<'a>(&'a self, client: &'a Client) -> Option<&'a Player> {
        self.connection_manager.get_player_by_addr(&client.addr)
    }

    // Gets a mutable player reference for a given client.
    pub fn get_player_for_client_mut<'a>(
        &'a mut self,
        client: &'a Client,
    ) -> Option<&'a mut Player> {
        self.connection_manager.get_player_by_addr_mut(&client.addr)
    }

    // Gets a mutable client reference by UUID.
    pub fn get_client_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut Client> {
        self.connection_manager.get_client_by_uuid(uuid)
    }

    // Gets a mutable client reference for a given player.
    pub fn get_client_for_player<'a>(&'a mut self, player: &'a Player) -> Option<&'a mut Client> {
        self.connection_manager.get_client_mut(&player.addr)
    }
}

impl Default for MinecraftServer {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerContext for MinecraftServer {
    // Provides mutable access to the connection manager.
    fn connection_manager_mut(&mut self) -> &mut ConnectionManager {
        &mut self.connection_manager
    }

    // Handles a new client connection.
    fn on_connection(&mut self, client: &mut Client) -> bool {
        let mut event = ConnectionEvent::new();
        event.dispatch(self, client);
        event.cancelled
    }

    // Handles a client disconnection.
    fn on_disconnect(&mut self, addr: SocketAddr) {
        DisconnectionEvent::new(addr).dispatch(self);
        self.connection_manager.remove_connection(&addr);
    }

    // Checks if a listener exists for a given packet ID and state.
    fn has_listener_for(&self, packet_id: i32, state: &ConnectionState) -> bool {
        let key = (*state, packet_id);
        self.assigned_packet_listeners.contains_key(&key)
            || self.generic_packet_listeners.contains_key(state)
    }

    // Dispatches a raw packet payload to all matching listeners.
    fn dispatch_packet(&mut self, packet_id: i32, client: &mut Client, payload: Vec<u8>) -> bool {
        let key = (client.state, packet_id);
        let server_ptr = self as *mut Self as *mut ();

        // Get listeners specific to the packet ID.
        let specific_listeners = self
            .assigned_packet_listeners
            .get(&key)
            .cloned()
            .unwrap_or_default();
        // Get generic listeners specific to the connection state.
        let generic_listeners = self
            .generic_packet_listeners
            .get(&client.state)
            .cloned()
            .unwrap_or_default();

        // Check if any listeners are present.
        if specific_listeners.is_empty() && generic_listeners.is_empty() {
            return false;
        }

        client.payload_cursor = Some(Cursor::new(payload));

        // Execute specific packet listeners.
        for listener in specific_listeners {
            client.payload_cursor.as_mut().unwrap().set_position(0);
            (listener.handler)(client, server_ptr);
        }

        // Execute generic state listeners.
        for listener in generic_listeners {
            client.payload_cursor.as_mut().unwrap().set_position(0);
            (listener.handler)(client, server_ptr);
        }

        // Clear the packet payload cursor.
        client.payload_cursor = None;

        true
    }

    // Handles the server startup event.
    fn on_startup(&mut self) -> bool {
        let mut event = StartupEvent::new();
        event.dispatch(self);
        event.cancelled
    }

    // Handles the server shutdown event.
    fn on_shutdown(&mut self) {
        let mut event = ShutdownEvent::new();
        event.dispatch(self);

        // Disconnect all clients gracefully.
        for client in self.connection_manager.get_all_connected_clients() {
            client.disconnect();
        }
    }
}
