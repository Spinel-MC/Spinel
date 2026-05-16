use crate::entity::player::Player;

pub enum Entity {
    Player(Player),
}

impl Entity {
    pub(crate) fn tick(&mut self) {
        match self {
            Self::Player(player) => player.tick(),
        }
    }
}
