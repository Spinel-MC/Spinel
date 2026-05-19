use crate::events::shutdown::ShutdownEvent;
use crate::events::signal::{ServerSignal, SignalEvent};
use crate::events::startup::StartupEvent;
use crate::network::client::instance::Client;
use crate::network::connection_manager::ConnectionManager;
use crate::registry_cache::RegistryCache;
use crate::server::packet_router::PacketRouter;
use crate::world::WorldManager;
use spinel_network::ConnectionState;
use spinel_registry::Registries;
use spinel_utils::component::Component;
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
    pub ticks_per_second: u32,
    pub current_tick: u64,
    pub is_ticking: Arc<AtomicBool>,
    packet_router: PacketRouter,
}

impl MinecraftServer {
    pub fn new() -> Self {
        let registries = Registries::new_vanilla();
        let registry_cache = RegistryCache::new(&registries);

        Self {
            connection_manager: ConnectionManager::new(),
            world_manager: WorldManager::new(),
            registry_cache,
            registries,
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
        self.sync_client_tick_rate();
    }

    pub(crate) fn sync_client_tick_rate(&mut self) {
        self.connection_manager
            .get_all_clients()
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
    ) -> io::Result<()> {
        self.world_manager
            .move_player(client, x, y, z, &self.registries)
    }

    pub(crate) fn tick_connections(&mut self) {
        let timed_out_clients = self
            .connection_manager
            .get_all_clients()
            .into_iter()
            .filter_map(|client_arc| self.tick_client(client_arc))
            .collect();
        self.disconnect_timed_out_clients(timed_out_clients);
    }

    fn tick_client(
        &mut self,
        client_arc: std::sync::Arc<std::sync::Mutex<Client>>,
    ) -> Option<SocketAddr> {
        let Ok(mut client) = client_arc.lock() else {
            return None;
        };
        if client.state != ConnectionState::Play {
            return None;
        }
        if client.tick() {
            return None;
        }
        Some(client.addr)
    }

    fn disconnect_timed_out_clients(&mut self, client_addresses: Vec<SocketAddr>) {
        client_addresses.into_iter().for_each(|client_address| {
            let Some(client_arc) = self.connection_manager.get_client(&client_address) else {
                return;
            };
            let Ok(mut client) = client_arc.lock() else {
                return;
            };
            let _ = self.disconnect(&mut client, Component::text("Timed out"));
        });
    }
}

impl Default for MinecraftServer {
    fn default() -> Self {
        Self::new()
    }
}
