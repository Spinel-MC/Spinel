use crate::events::connection::ConnectionEvent;
use crate::events::disconnection::DisconnectionEvent;
use crate::events::player_disconnect::PlayerDisconnectEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

impl MinecraftServer {
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

    pub fn on_disconnect(&mut self, address: SocketAddr) {
        if !self.connection_manager.has_connection(&address) {
            return;
        }

        let client_arc = self.connection_manager.client(&address);
        if let Some(client_arc) = client_arc
            && let Ok(mut client) = client_arc.lock()
        {
            self.on_disconnect_with_client(address, &mut client);
            return;
        }

        self.remove_disconnected_client(address);
    }

    pub(crate) fn on_disconnect_with_client(&mut self, address: SocketAddr, client: &mut Client) {
        if !self.connection_manager.has_connection(&address) {
            return;
        }

        self.dispatch_player_disconnect_event_with_client(address, client);
        self.remove_disconnected_client(address);
    }

    fn remove_disconnected_client(&mut self, address: SocketAddr) {
        DisconnectionEvent::new(address).dispatch(self);
        if let Err(error) = self.world_manager.remove_entity_by_addr(&address) {
            eprintln!("Failed to despawn disconnected player {address}: {error}");
        }
        self.connection_manager.remove_connection(&address);
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
}
