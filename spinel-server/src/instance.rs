use crate::ServerPacketListener;
use crate::events::connection::ConnectionEvent;
use crate::events::disconnection::DisconnectionEvent;
use crate::events::network::packet_io::{PacketFlowDirection, PacketIoEvent};
use crate::events::signal::{ServerSignal, SignalEvent};
use crate::events::shutdown::ShutdownEvent;
use crate::events::startup::StartupEvent;
use crate::listeners::get_listeners;
use crate::network::client::instance::Client;
use crate::network::connection_manager::ConnectionManager;
use crate::network::socket::start_tcp_listener;
use crate::registry_cache::RegistryCache;
use ::spinel_network::ConnectionState;
use ::spinel_utils::component::text::TextComponent;
use ::std::collections::HashMap;
use ::std::io::Cursor;
use ::std::net::SocketAddr;
use ::std::sync::{Arc, Mutex};
use spinel_core::network::clientbound::configuration::disconnect::ConfigurationDisconnectPacket;
use spinel_core::network::clientbound::login::disconnect::LoginDisconnectPacket;
use spinel_core::network::clientbound::play::disconnect::PlayDisconnectPacket;
use std::io;

pub struct MinecraftServer {
    pub connection_manager: ConnectionManager,
    pub registry_cache: RegistryCache,
    pub registry: ::spinel_registry::Registry,
    assigned_packet_listeners: HashMap<(ConnectionState, i32), Vec<&'static ServerPacketListener>>,
    generic_packet_listeners: HashMap<ConnectionState, Vec<&'static ServerPacketListener>>,
}

impl MinecraftServer {
    pub fn new() -> Self {
        let (assigned_packet_listeners, generic_packet_listeners) = get_listeners();
        let registry = ::spinel_registry::Registry::new_vanilla();
        let registry_cache = RegistryCache::new(&registry);

        MinecraftServer {
            connection_manager: ConnectionManager::new(),
            registry_cache,
            registry,
            assigned_packet_listeners,
            generic_packet_listeners,
        }
    }

    pub async fn start(self, address: &str, port: u16) {
        let server_arc = Arc::new(Mutex::new(self));
        Self::start_shared(server_arc, address, port).await;
    }

    pub async fn start_shared(
        server_arc: Arc<Mutex<Self>>,
        address: &str,
        port: u16,
    ) {
        if Self::startup_cancelled(&server_arc) {
            eprintln!("Server startup event was cancelled.");
            return;
        }

        match start_tcp_listener(server_arc, address, port).await {
            Ok(()) => println!("Server listener task completed normally."),
            Err(error) => eprintln!("Server listener task failed: {}", error),
        }
    }

    pub fn stop(&mut self) {
        self.on_shutdown();
    }

    pub fn disconnect(
        &mut self,
        client: &mut Client,
        reason: impl Into<TextComponent>,
    ) -> io::Result<()> {
        let reason = reason.into();

        let disconnect_result = match client.state {
            ConnectionState::Login => LoginDisconnectPacket::new(reason).dispatch(client),
            ConnectionState::Configuration => {
                ConfigurationDisconnectPacket::new(reason).dispatch(client)
            }
            ConnectionState::Play => PlayDisconnectPacket::new(reason).dispatch(client),
            _ => Ok(()),
        };

        self.force_disconnect(client);
        disconnect_result
    }

    pub fn on_startup(&mut self) -> bool {
        let mut event = StartupEvent::new();
        event.dispatch(self);
        event.cancelled
    }

    pub fn on_shutdown(&mut self) {
        let mut event = ShutdownEvent::new();
        event.dispatch(self);

        for client_arc in self.connection_manager.get_all_clients() {
            let Ok(mut client) = client_arc.lock() else {
                continue;
            };
            client.disconnect();
        }
    }

    pub fn on_signal(&mut self, signal: ServerSignal) -> SignalEvent {
        let mut event = SignalEvent::new(signal);
        event.dispatch(self);
        event
    }

    pub fn on_connection(&mut self, client: Arc<Mutex<Client>>) -> bool {
        let addr = {
            let Ok(mut connection) = client.lock() else {
                return true;
            };
            connection.server_ptr = Some(self as *mut Self as usize);
            connection.addr
        };
        let mut event = ConnectionEvent::new();
        {
            let Ok(mut connection) = client.lock() else {
                return true;
            };
            event.dispatch(self, &mut *connection);
        }

        let cancelled = event.cancelled;

        if !cancelled {
            self.connection_manager.add_connection(addr, client);
        }

        cancelled
    }

    pub fn on_disconnect(&mut self, addr: SocketAddr) {
        if !self.connection_manager.has_connection(&addr) {
            return;
        }

        DisconnectionEvent::new(addr).dispatch(self);
        self.connection_manager.remove_connection(&addr);
    }

    pub fn has_listener_for(&self, packet_id: i32, state: &ConnectionState) -> bool {
        let key = (*state, packet_id);
        self.assigned_packet_listeners.contains_key(&key)
            || self.generic_packet_listeners.contains_key(state)
    }

    pub fn dispatch_packet(
        &mut self,
        packet_id: i32,
        client: &mut Client,
        payload: Vec<u8>,
    ) -> bool {
        let key = (client.state, packet_id);
        let server_ptr = self as *mut Self as *mut ();
        client.server_ptr = Some(self as *mut Self as usize);

        let specific = self
            .assigned_packet_listeners
            .get(&key)
            .cloned()
            .unwrap_or_default();
        let generic = self
            .generic_packet_listeners
            .get(&client.state)
            .cloned()
            .unwrap_or_default();

        if specific.is_empty() && generic.is_empty() {
            return false;
        }
        let packet_name =
            ::spinel_network::packet_names::PacketNameRegistry::get_serverbound_packet_name(
                client.state,
                packet_id,
            );
        let mut packet_io_event = PacketIoEvent::new(
            PacketFlowDirection::Serverbound,
            client.state,
            packet_id,
            packet_name,
            payload.len(),
        );
        packet_io_event.dispatch(self, client);
        client.payload_cursor = Some(Cursor::new(payload));
        for listener in specific {
            if let Some(payload_cursor) = client.payload_cursor.as_mut() {
                payload_cursor.set_position(0);
            }
            (listener.handler)(client, server_ptr);
        }
        for listener in generic {
            if let Some(payload_cursor) = client.payload_cursor.as_mut() {
                payload_cursor.set_position(0);
            }
            (listener.handler)(client, server_ptr);
        }
        client.payload_cursor = None;
        true
    }

    fn force_disconnect(&mut self, client: &mut Client) {
        let addr = client.addr;
        client.disconnect();
        self.on_disconnect(addr);
    }

    fn startup_cancelled(server_arc: &Arc<Mutex<Self>>) -> bool {
        let Ok(mut server_guard) = server_arc.lock() else {
            return true;
        };
        server_guard.on_startup()
    }
}

impl Default for MinecraftServer {
    fn default() -> Self {
        Self::new()
    }
}
