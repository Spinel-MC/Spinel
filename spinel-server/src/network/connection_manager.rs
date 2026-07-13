use crate::entity::Player;
use crate::network::client::instance::Client;
use spinel_network::types::game_profile::GameProfile;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

pub struct ConnectionManager {
    pub clients: HashMap<SocketAddr, Arc<Mutex<Client>>>,
    player_provider: Arc<dyn PlayerProvider>,
}

pub trait PlayerProvider: Send + Sync {
    fn create_player(&self, connection: &mut Client, game_profile: &GameProfile) -> Player;
}

pub trait IntoPlayerProvider {
    fn into_player_provider(self) -> Arc<dyn PlayerProvider>;
}

struct DefaultPlayerProvider;

impl<F> PlayerProvider for F
where
    F: Fn(&mut Client, &GameProfile) -> Player + Send + Sync,
{
    fn create_player(&self, connection: &mut Client, game_profile: &GameProfile) -> Player {
        self(connection, game_profile)
    }
}

impl<F> IntoPlayerProvider for F
where
    F: PlayerProvider + 'static,
{
    fn into_player_provider(self) -> Arc<dyn PlayerProvider> {
        Arc::new(self)
    }
}

impl<F> IntoPlayerProvider for Option<F>
where
    F: PlayerProvider + 'static,
{
    fn into_player_provider(self) -> Arc<dyn PlayerProvider> {
        match self {
            Some(player_provider) => Arc::new(player_provider),
            None => Arc::new(DefaultPlayerProvider),
        }
    }
}

impl PlayerProvider for DefaultPlayerProvider {
    fn create_player(&self, connection: &mut Client, game_profile: &GameProfile) -> Player {
        let protocol_version = connection
            .login_metadata
            .as_ref()
            .map(|login_metadata| login_metadata.protocol_version)
            .unwrap_or_default();
        Player::new(
            game_profile.uuid,
            game_profile.username.clone(),
            protocol_version,
            connection.addr,
        )
    }
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            player_provider: Arc::new(DefaultPlayerProvider),
        }
    }

    pub fn set_player_provider(&mut self, player_provider: impl IntoPlayerProvider) {
        self.player_provider = player_provider.into_player_provider();
    }

    pub fn create_player(&self, connection: &mut Client, game_profile: &GameProfile) -> Player {
        self.player_provider.create_player(connection, game_profile)
    }

    pub fn add_connection(&mut self, addr: SocketAddr, client: Arc<Mutex<Client>>) {
        self.clients.insert(addr, client);
    }

    pub fn register_connection(&mut self, _addr: SocketAddr, _stream: std::net::TcpStream) {}

    pub fn remove_connection(&mut self, addr: &SocketAddr) {
        self.clients.remove(addr);
    }

    pub fn client(&self, addr: &SocketAddr) -> Option<Arc<Mutex<Client>>> {
        self.clients.get(addr).cloned()
    }

    pub fn has_connection(&self, addr: &SocketAddr) -> bool {
        self.clients.contains_key(addr)
    }

    pub fn clients(&self) -> Vec<Arc<Mutex<Client>>> {
        self.clients.values().cloned().collect()
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}
