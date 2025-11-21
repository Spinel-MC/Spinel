use crate::client::instance::Client;
use crate::client::player::Player;
use std::collections::HashMap;
use std::net::SocketAddr;
use uuid::Uuid;

pub struct ConnectionManager {
    clients: HashMap<SocketAddr, Client>,
    players: HashMap<Uuid, Player>,
    addr_to_uuid: HashMap<SocketAddr, Uuid>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            players: HashMap::new(),
            addr_to_uuid: HashMap::new(),
        }
    }

    pub fn add_connection(&mut self, addr: SocketAddr, client: Client) {
        self.clients.insert(addr, client);
    }

    pub fn register_connection(&mut self, addr: SocketAddr, stream: std::net::TcpStream) {
        let client = Client::new(stream, addr);
        self.add_connection(addr, client);
    }

    pub fn remove_connection(&mut self, addr: &SocketAddr) {
        self.clients.remove(addr);
        if let Some(uuid) = self.addr_to_uuid.remove(addr) {
            self.players.remove(&uuid);
        }
    }

    pub fn get_client(&self, addr: &SocketAddr) -> Option<&Client> {
        self.clients.get(addr)
    }

    pub fn get_client_mut(&mut self, addr: &SocketAddr) -> Option<&mut Client> {
        self.clients.get_mut(addr)
    }

    pub fn get_client_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut Client> {
        self.players
            .get(uuid)
            .and_then(|player| self.clients.get_mut(&player.addr))
    }

    pub fn get_all_connected_clients(&mut self) -> Vec<&mut Client> {
        self.clients.values_mut().collect()
    }

    pub fn add_player(&mut self, uuid: Uuid, player: Player) {
        self.addr_to_uuid.insert(player.addr, uuid);
        self.players.insert(uuid, player);
    }

    pub fn register_player(&mut self, addr: SocketAddr, player: Player) {
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
