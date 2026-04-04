use crate::entity::player::Player;
use crate::network::client::instance::Client;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;
use uuid::Uuid;

pub struct ConnectionManager {
    pub clients: HashMap<SocketAddr, Arc<Mutex<Client>>>,
    pub players: HashMap<Uuid, Player>,
    pub addr_to_uuid: HashMap<SocketAddr, Uuid>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            players: HashMap::new(),
            addr_to_uuid: HashMap::new(),
        }
    }

    pub fn add_connection(&mut self, addr: SocketAddr, client: Arc<Mutex<Client>>) {
        self.clients.insert(addr, client);
    }

    pub fn register_connection(&mut self, _addr: SocketAddr, _stream: std::net::TcpStream) {}

    pub fn remove_connection(&mut self, addr: &SocketAddr) {
        self.clients.remove(addr);
        if let Some(uuid) = self.addr_to_uuid.remove(addr) {
            self.players.remove(&uuid);
        }
    }

    pub fn get_client(&self, addr: &SocketAddr) -> Option<Arc<Mutex<Client>>> {
        self.clients.get(addr).cloned()
    }

    pub fn get_client_by_uuid(&self, uuid: &Uuid) -> Option<Arc<Mutex<Client>>> {
        self.players
            .get(uuid)
            .and_then(|player| self.clients.get(&player.addr).cloned())
    }

    pub fn get_all_clients(&self) -> Vec<Arc<Mutex<Client>>> {
        self.clients.values().cloned().collect()
    }

    pub fn add_player(&mut self, uuid: Uuid, player: Player) {
        self.addr_to_uuid.insert(player.addr, uuid);
        self.players.insert(uuid, player);
    }

    pub fn register_player(&mut self, _addr: SocketAddr, player: Player) {
        let uuid = player.uuid;
        self.add_player(uuid, player);
    }

    pub fn get_player(&self, uuid: &Uuid) -> Option<&Player> {
        self.players.get(uuid)
    }

    pub fn get_player_mut(&mut self, uuid: &Uuid) -> Option<&mut Player> {
        self.players.get_mut(uuid)
    }

    pub fn get_player_by_addr(&self, addr: &SocketAddr) -> Option<&Player> {
        self.addr_to_uuid
            .get(addr)
            .and_then(|uuid| self.players.get(uuid))
    }

    pub fn get_player_by_addr_mut(&mut self, addr: &SocketAddr) -> Option<&mut Player> {
        self.addr_to_uuid
            .get(addr)
            .and_then(|uuid| self.players.get_mut(uuid))
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}
