use crate::core::events::connection::ConnectionEvent;
use crate::core::events::diconnection::DisconnectionEvent;
use crate::core::events::shutdown::ShutdownEvent;
use crate::core::events::startup::StartupEvent;
use crate::core::network::clientbound::configuration::disconnect::ConfigurationDisconnectPacket;
use crate::core::network::clientbound::login::disconnect::LoginDisconnectPacket;
use crate::core::network::clientbound::play::disconnect::PlayDisconnectPacket;
use spinel_events::{RegisteredEvent, RegisteredModule, RegisteredModuleDependency};
use spinel_network::server::start_tcp_listener;
use spinel_network::{
    Client, ConnectionManager, ConnectionState, PacketListener, Player, ServerContext,
};
use spinel_utils::component::text::TextComponent;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Cursor;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::select;
use tokio::signal;
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct MinecraftServer {
    pub connection_manager: ConnectionManager,
    packet_listeners: HashMap<(ConnectionState, i32), Vec<&'static PacketListener>>,
    generic_listeners: HashMap<ConnectionState, Vec<&'static PacketListener>>,
}

fn resolve_ambiguous_module(
    unqualified_name: &str,
    all_modules_map: &HashMap<String, Vec<String>>,
) -> String {
    if unqualified_name.contains(':') {
        return unqualified_name.to_string();
    }

    match all_modules_map.get(unqualified_name) {
        None => {
            format!("unknown:{}", unqualified_name)
        }
        Some(candidates) => {
            if candidates.len() == 1 {
                candidates[0].clone()
            } else {
                let minecraft_candidate = format!("minecraft:{}", unqualified_name);
                if candidates.iter().any(|c| c == &minecraft_candidate) {
                    minecraft_candidate
                } else {
                    panic!(
                        "Module name '{}' is ambiguous. Candidates found: {:?}. Please specify a namespace (e.g., 'namespace:{}').",
                        unqualified_name, candidates, unqualified_name
                    );
                }
            }
        }
    }
}

impl MinecraftServer {
    pub fn new() -> Self {
        let mut listeners: HashMap<(ConnectionState, i32), Vec<&'static PacketListener>> =
            HashMap::new();
        let mut generic_listeners: HashMap<ConnectionState, Vec<&'static PacketListener>> =
            HashMap::new();

        let mut all_module_names: HashSet<&'static str> = HashSet::new();
        for module in inventory::iter::<&'static RegisteredModule>() {
            all_module_names.insert(module.name);
        }
        for dep in inventory::iter::<&'static RegisteredModuleDependency>() {
            all_module_names.insert(dep.subject_module);
            all_module_names.insert(dep.dependent_on);
        }

        let mut all_known_modules: HashMap<String, Vec<String>> = HashMap::new();
        for qualified_name in all_module_names {
            let unqualified_name = qualified_name
                .split(':')
                .last()
                .unwrap_or(qualified_name)
                .to_string();
            all_known_modules
                .entry(unqualified_name)
                .or_default()
                .push(qualified_name.to_string());
        }

        let explicitly_imported_raw: HashSet<&'static str> =
            inventory::iter::<&'static RegisteredModule>()
                .map(|m| m.name)
                .collect();

        let mut explicitly_activated_qualified = HashSet::new();
        for raw_name in explicitly_imported_raw {
            let resolved = resolve_ambiguous_module(raw_name, &all_known_modules);
            explicitly_activated_qualified.insert(resolved);
        }

        let mut dependency_graph: HashMap<String, HashSet<String>> = HashMap::new();
        for dep in inventory::iter::<&'static RegisteredModuleDependency>() {
            let resolved_subject = resolve_ambiguous_module(dep.subject_module, &all_known_modules);
            let resolved_dependency =
                resolve_ambiguous_module(dep.dependent_on, &all_known_modules);
            dependency_graph
                .entry(resolved_subject)
                .or_default()
                .insert(resolved_dependency);
        }

        let mut resolved_modules: HashSet<String> = explicitly_activated_qualified.clone();
        let mut queue: VecDeque<String> = explicitly_activated_qualified.into_iter().collect();

        while let Some(current_module) = queue.pop_front() {
            if let Some(dependencies) = dependency_graph.get(&current_module) {
                for dep in dependencies {
                    if resolved_modules.insert(dep.clone()) {
                        queue.push_back(dep.clone());
                    }
                }
            }
        }

        let registered_independent_events: HashSet<&'static str> =
            inventory::iter::<&'static RegisteredEvent>()
                .filter(|e| e.is_independent)
                .map(|e| e.name)
                .collect();

        for listener in inventory::iter::<&'static PacketListener>() {
            let module_ok = listener.modules.is_empty()
                || listener.modules.iter().any(|unqualified_name| {
                    let resolved_name =
                        resolve_ambiguous_module(unqualified_name, &all_known_modules);
                    resolved_modules.contains(&resolved_name)
                });

            let event_ok = listener.events.is_empty()
                || listener
                    .events
                    .iter()
                    .any(|event_name| registered_independent_events.contains(event_name));

            if module_ok && event_ok {
                if listener.id == -1 {
                    generic_listeners
                        .entry(listener.state)
                        .or_default()
                        .push(listener);
                } else {
                    let key = (listener.state, listener.id);
                    listeners.entry(key).or_default().push(listener);
                }
            }
        }

        for listener_vec in listeners.values_mut() {
            listener_vec.sort_by_key(|l| Reverse(l.priority.to_order()));
        }
        for listener_vec in generic_listeners.values_mut() {
            listener_vec.sort_by_key(|l| Reverse(l.priority.to_order()));
        }

        MinecraftServer {
            connection_manager: ConnectionManager::new(),
            packet_listeners: listeners,
            generic_listeners,
        }
    }

    pub async fn start(self, address: &str, port: u16) {
        let server_arc = Arc::new(Mutex::new(self));
        let shutdown_server_arc = server_arc.clone();
        let address_owned = address.to_string();

        let mut server_guard = server_arc.lock().await;
        if !server_guard.on_startup() {
            eprintln!("Server startup event was cancelled.");
            return;
        }
        drop(server_guard);

        let server_handle =
            tokio::spawn(async move { start_tcp_listener(server_arc, &address_owned, port).await });

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

    pub fn stop(&mut self) {
        self.on_shutdown();
    }

    pub fn broadcast(&self, component_like: impl Into<TextComponent>) {
        let final_component: TextComponent = component_like.into();
        println!("{}\x1b[0m", final_component.to_ansi_string())
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

        self.connection_manager.remove_connection(&client.addr);
    }

    pub fn get_player(&self, uuid: &Uuid) -> Option<&Player> {
        self.connection_manager.get_player(uuid)
    }

    pub fn get_player_mut(&mut self, uuid: &Uuid) -> Option<&mut Player> {
        self.connection_manager.get_player_mut(uuid)
    }

    pub fn get_player_for_client<'a>(&'a self, client: &'a Client) -> Option<&'a Player> {
        self.connection_manager.get_player_by_addr(&client.addr)
    }

    pub fn get_player_for_client_mut<'a>(
        &'a mut self,
        client: &'a Client,
    ) -> Option<&'a mut Player> {
        self.connection_manager.get_player_by_addr_mut(&client.addr)
    }

    pub fn get_client_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut Client> {
        self.connection_manager.get_client_by_uuid(uuid)
    }

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
    fn connection_manager_mut(&mut self) -> &mut ConnectionManager {
        &mut self.connection_manager
    }

    fn on_connection(&mut self, client: &mut Client) -> bool {
        let mut event = ConnectionEvent::new();
        event.dispatch(self, client);
        event.cancelled
    }

    fn on_disconnect(&mut self, addr: SocketAddr) {
        DisconnectionEvent::new(addr).dispatch(self);
        self.connection_manager.remove_connection(&addr);
    }

    fn has_listener_for(&self, packet_id: i32, state: &ConnectionState) -> bool {
        let key = (*state, packet_id);
        self.packet_listeners.contains_key(&key) || self.generic_listeners.contains_key(state)
    }

    fn dispatch_packet(&mut self, packet_id: i32, client: &mut Client, payload: Vec<u8>) -> bool {
        let key = (client.state, packet_id);
        let server_ptr = self as *mut Self as *mut ();

        let specific_listeners = self.packet_listeners.get(&key).cloned().unwrap_or_default();
        let generic_listeners = self
            .generic_listeners
            .get(&client.state)
            .cloned()
            .unwrap_or_default();

        if specific_listeners.is_empty() && generic_listeners.is_empty() {
            return false;
        }

        client.payload_cursor = Some(Cursor::new(payload));

        for listener in specific_listeners {
            client.payload_cursor.as_mut().unwrap().set_position(0);
            (listener.handler)(client, server_ptr);
        }

        for listener in generic_listeners {
            client.payload_cursor.as_mut().unwrap().set_position(0);
            (listener.handler)(client, server_ptr);
        }

        client.payload_cursor = None;

        true
    }

    fn on_startup(&mut self) -> bool {
        let mut event = StartupEvent::new();
        event.dispatch(self);
        event.cancelled
    }

    fn on_shutdown(&mut self) {
        let mut event = ShutdownEvent::new();
        event.dispatch(self);

        for client in self.connection_manager.get_all_connected_clients() {
            client.disconnect();
        }
    }
}
