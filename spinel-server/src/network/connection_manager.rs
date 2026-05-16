use crate::network::client::instance::Client;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

pub struct ConnectionManager {
    pub clients: HashMap<SocketAddr, Arc<Mutex<Client>>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub fn add_connection(&mut self, addr: SocketAddr, client: Arc<Mutex<Client>>) {
        self.clients.insert(addr, client);
    }

    pub fn register_connection(&mut self, _addr: SocketAddr, _stream: std::net::TcpStream) {}

    pub fn remove_connection(&mut self, addr: &SocketAddr) {
        self.clients.remove(addr);
    }

    pub fn get_client(&self, addr: &SocketAddr) -> Option<Arc<Mutex<Client>>> {
        self.clients.get(addr).cloned()
    }

    pub fn has_connection(&self, addr: &SocketAddr) -> bool {
        self.clients.contains_key(addr)
    }

    pub fn get_all_clients(&self) -> Vec<Arc<Mutex<Client>>> {
        self.clients.values().cloned().collect()
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}
