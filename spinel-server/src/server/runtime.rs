use crate::network::socket::start_tcp_listener;
use crate::server::MinecraftServer;
use spinel_utils::component::Component;
use std::net::SocketAddr;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const MAX_TICK_CATCH_UP: u32 = 5;

impl MinecraftServer {
    pub async fn start(self, address: &str, port: u16) {
        let server_arc = Arc::new(Mutex::new(self));
        Self::start_shared(server_arc, address, port).await;
    }

    async fn start_shared(server_arc: Arc<Mutex<Self>>, address: &str, port: u16) {
        if Self::startup_cancelled(&server_arc) {
            eprintln!("Server startup event was cancelled.");
            return;
        }

        if !Self::start_loop(&server_arc) {
            return;
        }

        match start_tcp_listener(server_arc.clone(), address, port).await {
            Ok(()) => println!("Server listener task completed normally."),
            Err(error) => eprintln!("Server listener task failed: {}", error),
        }

        Self::stop_loop(&server_arc);
    }

    fn startup_cancelled(server_arc: &Arc<Mutex<Self>>) -> bool {
        let Ok(mut server) = server_arc.lock() else {
            return true;
        };
        server.on_startup()
    }

    fn start_loop(server_arc: &Arc<Mutex<Self>>) -> bool {
        let Ok(server) = server_arc.lock() else {
            return false;
        };
        server.is_ticking.store(true, Ordering::SeqCst);
        tokio::task::spawn_blocking({
            let server_arc = server_arc.clone();
            move || Self::run(server_arc)
        });
        true
    }

    fn stop_loop(server_arc: &Arc<Mutex<Self>>) {
        if let Ok(server) = server_arc.lock() {
            server.is_ticking.store(false, Ordering::SeqCst);
        }
    }

    fn run(server_arc: Arc<Mutex<Self>>) {
        let mut next_tick_at = Instant::now();

        while Self::tick_server(&server_arc) {
            let Some(tick_duration) = Self::tick_duration(&server_arc) else {
                return;
            };
            next_tick_at += tick_duration;
            Self::wait(next_tick_at);
            if Instant::now() > next_tick_at + tick_duration * MAX_TICK_CATCH_UP {
                next_tick_at = Instant::now();
            }
        }
    }

    fn tick_duration(server_arc: &Arc<Mutex<Self>>) -> Option<Duration> {
        let Ok(server) = server_arc.lock() else {
            return None;
        };
        Some(Duration::from_nanos(
            1_000_000_000 / u64::from(server.ticks_per_second.max(1)),
        ))
    }

    fn tick_server(server_arc: &Arc<Mutex<Self>>) -> bool {
        let Ok(mut server) = server_arc.lock() else {
            return false;
        };
        if !server.is_ticking.load(Ordering::SeqCst) {
            return false;
        }

        server.tick();
        true
    }

    fn wait(next_tick_at: Instant) {
        let sleep_threshold_millis = if cfg!(target_os = "windows") { 17 } else { 2 };
        while Instant::now() < next_tick_at {
            let remaining = next_tick_at.saturating_duration_since(Instant::now());
            let remaining_millis = remaining.as_millis() as u64;

            if remaining_millis >= sleep_threshold_millis {
                std::thread::sleep(Duration::from_millis(remaining_millis / 2));
                continue;
            }

            std::hint::spin_loop();
        }
    }

    fn tick(&mut self) {
        self.current_tick += 1;
        let timed_out_players = self.world_manager.tick(&self.connection_manager);
        self.disconnect_timed_out(timed_out_players);
    }

    fn disconnect_timed_out(&mut self, player_addresses: Vec<SocketAddr>) {
        for player_address in player_addresses {
            let Some(client_arc) = self.connection_manager.get_client(&player_address) else {
                continue;
            };
            let Ok(mut client) = client_arc.lock() else {
                continue;
            };
            let _ = self.disconnect(&mut client, Component::text("Timed out"));
        }
    }
}
