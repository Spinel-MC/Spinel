use crate::events::connection::ConnectionEvent;
use crate::events::disconnection::DisconnectionEvent;
use crate::events::player_disconnect::PlayerDisconnectEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::clientbound::configuration::disconnect::ConfigurationDisconnectPacket;
use spinel_core::network::clientbound::login::disconnect::LoginDisconnectPacket;
use spinel_core::network::clientbound::play::disconnect::PlayDisconnectPacket;
use spinel_network::ConnectionState;
use spinel_utils::component::text::TextComponent;
use std::io;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Instant;

enum ConnectionTickOutcome {
    Active,
    Closed(SocketAddr),
    TimedOut(SocketAddr),
}

impl MinecraftServer {
    pub(crate) fn tick_connections(&mut self) {
        let connection_tick_outcomes = self
            .connection_manager
            .clients()
            .into_iter()
            .map(Self::tick_connection)
            .collect::<Vec<_>>();

        connection_tick_outcomes
            .iter()
            .filter_map(|outcome| match outcome {
                ConnectionTickOutcome::Closed(address) => Some(*address),
                _ => None,
            })
            .for_each(|address| self.handle_connection_closed(address));

        connection_tick_outcomes
            .into_iter()
            .filter_map(|outcome| match outcome {
                ConnectionTickOutcome::TimedOut(address) => Some(address),
                _ => None,
            })
            .for_each(|address| self.kick_timed_out_connection(address));
    }

    pub fn on_connection(&mut self, client: Arc<Mutex<Client>>) -> bool {
        let Some(client_address) = self.assign_server_pointer(&client) else {
            return true;
        };

        let Some(connection_event) = self.create_connection_event(&client) else {
            return true;
        };

        if connection_event.cancelled {
            return true;
        }

        self.connection_manager
            .add_connection(client_address, client);
        false
    }

    pub fn handle_connection_closed(&mut self, address: SocketAddr) {
        if !self.connection_manager.has_connection(&address) {
            return;
        }

        let client_arc = self.connection_manager.client(&address);
        if let Some(client_arc) = client_arc
            && let Ok(mut client) = client_arc.lock()
        {
            self.handle_connection_closed_with_client(address, &mut client);
            return;
        }

        self.remove_closed_connection(address);
    }

    pub(crate) fn handle_connection_closed_with_client(
        &mut self,
        address: SocketAddr,
        client: &mut Client,
    ) {
        if !self.connection_manager.has_connection(&address) {
            return;
        }

        self.dispatch_player_disconnect_event_with_client(address, client);
        self.remove_closed_connection(address);
    }

    pub fn kick(
        &mut self,
        client: &mut Client,
        reason: impl Into<TextComponent>,
    ) -> io::Result<()> {
        let reason = reason.into();
        let reason_text = reason.to_plain_string();
        client.discard_queued_outbound_packets();
        let kick_result = match client.state {
            ConnectionState::Login => LoginDisconnectPacket::new(reason).dispatch(client),
            ConnectionState::Configuration => {
                ConfigurationDisconnectPacket::new(reason).dispatch(client)
            }
            ConnectionState::Play => PlayDisconnectPacket::new(reason).dispatch(client),
            _ => Ok(()),
        };
        let client_address = client.addr;
        println!("Kicked client {client_address}: {reason_text}");
        client.close_after_disconnect_packet();
        self.handle_connection_closed_with_client(client_address, client);
        kick_result
    }

    fn remove_closed_connection(&mut self, address: SocketAddr) {
        DisconnectionEvent::new(address).dispatch(self);
        if let Err(error) = self.world_manager.remove_entity_by_addr(&address) {
            eprintln!("Failed to despawn disconnected player {address}: {error}");
        }
        self.connection_manager.remove_connection(&address);
        println!("Removed connection {address}");
    }

    fn dispatch_player_disconnect_event_with_client(
        &mut self,
        address: SocketAddr,
        client: &mut Client,
    ) {
        let Some(player) = self
            .world_manager
            .player_pointer_for_client_address(&address)
        else {
            return;
        };
        PlayerDisconnectEvent::new(player).dispatch(self, client);
    }

    fn assign_server_pointer(&mut self, client: &Arc<Mutex<Client>>) -> Option<SocketAddr> {
        let Ok(mut connection) = client.lock() else {
            return None;
        };

        connection.server_ptr = Some(self as *mut MinecraftServer as usize);
        connection.enable_outbound_packet_queue();
        Some(connection.addr)
    }

    fn create_connection_event(&mut self, client: &Arc<Mutex<Client>>) -> Option<ConnectionEvent> {
        let Ok(mut connection) = client.lock() else {
            return None;
        };

        let mut connection_event = ConnectionEvent::new();
        connection_event.dispatch(self, &mut connection);
        Some(connection_event)
    }

    fn tick_connection(client: Arc<Mutex<Client>>) -> ConnectionTickOutcome {
        let Ok(mut client) = client.lock() else {
            return ConnectionTickOutcome::Active;
        };
        if !client.is_online() {
            client.dispatch_outbound_write_errors();
            return ConnectionTickOutcome::Closed(client.addr);
        }
        if client.login_plugin_requests_have_timed_out(Instant::now()) {
            return ConnectionTickOutcome::TimedOut(client.addr);
        }
        if client.state != ConnectionState::Play {
            return ConnectionTickOutcome::Active;
        }
        let _ = client.flush_outbound_packets();
        if !client.is_online() {
            client.dispatch_outbound_write_errors();
            return ConnectionTickOutcome::Closed(client.addr);
        }
        if client.tick() {
            let _ = client.flush_outbound_packets();
            return ConnectionTickOutcome::Active;
        }
        ConnectionTickOutcome::TimedOut(client.addr)
    }

    fn kick_timed_out_connection(&mut self, address: SocketAddr) {
        let Some(client) = self.connection_manager.client(&address) else {
            return;
        };
        let Ok(mut client) = client.lock() else {
            return;
        };
        let _ = self.kick(&mut client, TextComponent::literal("Timed out"));
    }
}
