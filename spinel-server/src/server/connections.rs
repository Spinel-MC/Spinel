use crate::events::connection::ConnectionEvent;
use crate::events::disconnection::DisconnectionEvent;
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

        self.connection_manager.add_connection(client_address, client);
        false
    }

    pub fn on_disconnect(&mut self, address: SocketAddr) {
        if !self.connection_manager.has_connection(&address) {
            return;
        }

        DisconnectionEvent::new(address).dispatch(self);
        self.connection_manager.remove_connection(&address);
    }

    fn assign_server_pointer(&mut self, client: &Arc<Mutex<Client>>) -> Option<SocketAddr> {
        let Ok(mut connection) = client.lock() else {
            return None;
        };

        connection.server_ptr = Some(self as *mut MinecraftServer as usize);
        Some(connection.addr)
    }

    fn create_connection_event(
        &mut self,
        client: &Arc<Mutex<Client>>,
    ) -> Option<ConnectionEvent> {
        let Ok(mut connection) = client.lock() else {
            return None;
        };

        let mut connection_event = ConnectionEvent::new();
        connection_event.dispatch(self, &mut connection);
        Some(connection_event)
    }
}
