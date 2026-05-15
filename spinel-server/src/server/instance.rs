use crate::events::signal::{ServerSignal, SignalEvent};
use crate::events::shutdown::ShutdownEvent;
use crate::events::startup::StartupEvent;
use crate::network::client::instance::Client;
use crate::network::connection_manager::ConnectionManager;
use crate::network::socket::start_tcp_listener;
use crate::registry_cache::RegistryCache;
use crate::server::packet_router::PacketRouter;
use spinel_network::ConnectionState;
use std::sync::{Arc, Mutex};

pub struct MinecraftServer {
    pub connection_manager: ConnectionManager,
    pub registry_cache: RegistryCache,
    pub registry: ::spinel_registry::Registry,
    packet_router: PacketRouter,
}

impl MinecraftServer {
    pub fn new() -> Self {
        let registry = ::spinel_registry::Registry::new_vanilla();
        let registry_cache = RegistryCache::new(&registry);

        Self {
            connection_manager: ConnectionManager::new(),
            registry_cache,
            registry,
            packet_router: PacketRouter::new(),
        }
    }

    pub async fn start(self, address: &str, port: u16) {
        let server_arc = Arc::new(Mutex::new(self));
        Self::start_shared(server_arc, address, port).await;
    }

    pub async fn start_shared(server_arc: Arc<Mutex<Self>>, address: &str, port: u16) {
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

    pub fn on_startup(&mut self) -> bool {
        let mut startup_event = StartupEvent::new();
        startup_event.dispatch(self);
        startup_event.cancelled
    }

    pub fn on_shutdown(&mut self) {
        let mut shutdown_event = ShutdownEvent::new();
        shutdown_event.dispatch(self);

        for client_arc in self.connection_manager.get_all_clients() {
            let Ok(mut client) = client_arc.lock() else {
                continue;
            };
            client.disconnect();
        }
    }

    pub fn on_signal(&mut self, signal: ServerSignal) -> SignalEvent {
        let mut signal_event = SignalEvent::new(signal);
        signal_event.dispatch(self);
        signal_event
    }

    pub fn has_listener_for(&self, packet_id: i32, state: &ConnectionState) -> bool {
        self.packet_router.has_listener_for(packet_id, state)
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
