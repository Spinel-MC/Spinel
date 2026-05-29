use crate::entity::Player;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;

pub enum CommandSender<'a> {
    Player(&'a mut Client),
    Server,
    Console,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandSenderKind {
    Player,
    Server,
    Console,
}

impl CommandSender<'_> {
    pub const fn kind(&self) -> CommandSenderKind {
        match self {
            Self::Player(_) => CommandSenderKind::Player,
            Self::Server => CommandSenderKind::Server,
            Self::Console => CommandSenderKind::Console,
        }
    }

    pub fn player_client_mut(&mut self) -> Option<&mut Client> {
        match self {
            Self::Player(client) => Some(*client),
            Self::Server | Self::Console => None,
        }
    }

    pub fn player<'a>(&mut self, server: &'a mut MinecraftServer) -> Option<&'a mut Player> {
        let client = self.player_client_mut()?;
        server.world_manager.player_mut_for_client(client)
    }
}
