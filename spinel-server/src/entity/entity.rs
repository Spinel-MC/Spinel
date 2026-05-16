use crate::entity::player::Player;
use crate::entity::tick_context::EntityTickContext;
use std::net::SocketAddr;

pub enum Entity {
    Player(Player),
}

impl Entity {
    pub(crate) fn tick(&mut self, context: &EntityTickContext<'_>) -> Option<SocketAddr> {
        match self {
            Self::Player(player) => player.tick(context),
        }
    }
}
