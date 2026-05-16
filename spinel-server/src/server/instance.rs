use crate::events::shutdown::ShutdownEvent;
use crate::events::signal::{ServerSignal, SignalEvent};
use crate::events::startup::StartupEvent;
use crate::network::client::instance::Client;
use crate::network::connection_manager::ConnectionManager;
use crate::registry_cache::RegistryCache;
use crate::server::packet_router::PacketRouter;
use crate::world::WorldManager;
use spinel_network::ConnectionState;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

const DEFAULT_TICKS_PER_SECOND: u32 = 20;

pub struct MinecraftServer {
    pub connection_manager: ConnectionManager,
    pub world_manager: WorldManager,
    pub registry_cache: RegistryCache,
    pub registry: ::spinel_registry::Registry,
    pub ticks_per_second: u32,
    pub current_tick: u64,
    pub is_ticking: Arc<AtomicBool>,
    packet_router: PacketRouter,
}

impl MinecraftServer {
    pub fn new() -> Self {
        let registry = ::spinel_registry::Registry::new_vanilla();
        let registry_cache = RegistryCache::new(&registry);

        Self {
            connection_manager: ConnectionManager::new(),
            world_manager: WorldManager::new(),
            registry_cache,
            registry,
            ticks_per_second: DEFAULT_TICKS_PER_SECOND,
            current_tick: 0,
            is_ticking: Arc::new(AtomicBool::new(false)),
            packet_router: PacketRouter::new(),
        }
    }

    pub fn stop(&mut self) {
        self.is_ticking.store(false, Ordering::SeqCst);
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

    pub fn set_tick_rate(&mut self, ticks_per_second: u32) {
        self.ticks_per_second = ticks_per_second.max(1);
        self.world_manager
            .sync_ticks(&self.connection_manager, self.ticks_per_second);
    }
}

impl Default for MinecraftServer {
    fn default() -> Self {
        Self::new()
    }
}
