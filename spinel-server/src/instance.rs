use crate::events::connection::ConnectionEvent;
use crate::events::disconnection::DisconnectionEvent;
use crate::events::shutdown::ShutdownEvent;
use crate::events::startup::StartupEvent;
use crate::listeners::get_listeners;
use crate::network::connection_manager::ConnectionManager;
use crate::network::socket::start_tcp_listener;
use crate::registry_cache::RegistryCache;
use ::spinel_core::network::clientbound::configuration::disconnect::ConfigurationDisconnectPacket;
use ::spinel_core::network::clientbound::login::disconnect::LoginDisconnectPacket;
use ::spinel_core::network::clientbound::play::disconnect::PlayDisconnectPacket;

use crate::ServerPacketListener;
use crate::network::client::instance::Client;
use ::spinel_network::ConnectionState;

use ::spinel_utils::component::text::TextComponent;
use ::std::collections::HashMap;
use ::std::io::Cursor;
use ::std::net::SocketAddr;
use ::std::sync::{Arc, Mutex};
use ::tokio::select;
use ::tokio::signal;

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
        let shutdown_server_arc = server_arc.clone();
        let address_owned = address.to_string();

        let mut server_guard = server_arc.lock().unwrap();
        if server_guard.on_startup() {
            eprintln!("Server startup event was cancelled.");
            return;
        }
        drop(server_guard);

        let server_handle =
            ::tokio::spawn(
                async move { start_tcp_listener(server_arc, &address_owned, port).await },
            );

        select! {
            _ = signal::ctrl_c() => {
                println!("Shutdown signal received. Stopping the server.");
                let mut server = shutdown_server_arc.lock().unwrap();
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

    pub fn stop(&mut self) {
        self.on_shutdown();
    }

    pub fn disconnect(&mut self, client: &mut Client, reason: impl Into<TextComponent>) {
        match client.state {
            ConnectionState::Login => LoginDisconnectPacket::new(reason).dispatch(client),
            ConnectionState::Configuration => {
                ConfigurationDisconnectPacket::new(reason).dispatch(client)
            }
            ConnectionState::Play => PlayDisconnectPacket::new(reason).dispatch(client),
            _ => (),
        }
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
            let mut client = client_arc.lock().unwrap();
            client.disconnect();
        }
    }

    pub fn on_connection(&mut self, client: Arc<Mutex<Client>>) -> bool {
        let addr = {
            let c = client.lock().unwrap();
            c.addr
        };

        let mut event = ConnectionEvent::new();

        {
            let mut c = client.lock().unwrap();
            event.dispatch(self, &mut *c);
        }

        let cancelled = event.cancelled;

        if !cancelled {
            self.connection_manager.add_connection(addr, client);
        }

        cancelled
    }

    pub fn on_disconnect(&mut self, addr: SocketAddr) {
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

        client.payload_cursor = Some(Cursor::new(payload));

        for listener in specific {
            client.payload_cursor.as_mut().unwrap().set_position(0);
            (listener.handler)(client, server_ptr);
        }
        for listener in generic {
            client.payload_cursor.as_mut().unwrap().set_position(0);
            (listener.handler)(client, server_ptr);
        }
        client.payload_cursor = None;
        true
    }
}

impl Default for MinecraftServer {
    fn default() -> Self {
        Self::new()
    }
}
