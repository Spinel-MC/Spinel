use std::{
    collections::HashMap,
    net::{SocketAddr, TcpStream},
};
use uuid::Uuid;

use crate::client::instance::Client;
use crate::client::player::Player;

#[derive(Default)]
pub struct ConnectionManager {
    clients: HashMap<SocketAddr, Client>,
    players: HashMap<Uuid, Player>,
    addr_to_uuid: HashMap<SocketAddr, Uuid>,
    stream_handles: HashMap<SocketAddr, TcpStream>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_connection(&mut self, address: SocketAddr, stream: TcpStream) {
        self.stream_handles.insert(address, stream);
    }

    pub fn register_player(&mut self, addr: SocketAddr, player: Player) {
        let uuid = player.uuid;
        self.addr_to_uuid.insert(addr, uuid);
        self.players.insert(uuid, player);
    }

    pub fn remove_connection(&mut self, address: &SocketAddr) -> Option<Client> {
        self.stream_handles.remove(address);
        if let Some(uuid) = self.addr_to_uuid.remove(address) {
            self.players.remove(&uuid);
        }
        self.clients.remove(address)
    }

    pub fn get_client_mut(&mut self, address: &SocketAddr) -> Option<&mut Client> {
        self.clients.get_mut(address)
    }

    pub fn get_player(&self, uuid: &Uuid) -> Option<&Player> {
        self.players.get(uuid)
    }

    pub fn get_player_mut(&mut self, uuid: &Uuid) -> Option<&mut Player> {
        self.players.get_mut(uuid)
    }

    pub fn get_player_by_addr(&self, address: &SocketAddr) -> Option<&Player> {
        self.addr_to_uuid
            .get(address)
            .and_then(|uuid| self.players.get(uuid))
    }

    pub fn get_player_by_addr_mut(&mut self, address: &SocketAddr) -> Option<&mut Player> {
        self.addr_to_uuid
            .get(address)
            .and_then(|uuid| self.players.get_mut(uuid))
    }

    pub fn get_client_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut Client> {
        self.players
            .get(uuid)
            .and_then(|p| self.clients.get_mut(&p.addr))
    }

    pub fn take_client(&mut self, address: &SocketAddr) -> Option<Client> {
        self.clients.remove(address)
    }

    pub fn insert_client(&mut self, address: SocketAddr, client: Client) {
        self.clients.insert(address, client);
    }

    pub fn get_connected_addrs(&self) -> Vec<SocketAddr> {
        self.clients.keys().cloned().collect()
    }

    pub fn get_all_connected_clients(&mut self) -> Vec<&mut Client> {
        let mut connected_clients = Vec::new();

        for client in &mut self.clients {
            connected_clients.push(client.1);
        }
        connected_clients
    }
}
