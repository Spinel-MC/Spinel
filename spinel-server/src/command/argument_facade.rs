use crate::command::CommandArgument;

#[derive(Clone, Copy)]
#[allow(non_snake_case)]
pub enum ArgumentType {
    GameMode(&'static str),
    Entity(&'static str),
    Entities(&'static str),
    Player(&'static str),
    Players(&'static str),
}

impl ArgumentType {
    pub fn only_players(self, only_players: bool) -> Self {
        match (self, only_players) {
            (Self::Entity(id) | Self::Entities(id), true) => Self::Players(id),
            (Self::Player(id) | Self::Players(id), false) => Self::Entities(id),
            _ => self,
        }
    }

    pub fn single_entity(self, single_entity: bool) -> Self {
        match (self, single_entity) {
            (Self::Entity(id) | Self::Entities(id), true) => Self::Entity(id),
            (Self::Players(id) | Self::Player(id), true) => Self::Player(id),
            (Self::Entity(id) | Self::Entities(id), false) => Self::Entities(id),
            (Self::Player(id) | Self::Players(id), false) => Self::Players(id),
            _ => self,
        }
    }
}

impl From<ArgumentType> for CommandArgument {
    fn from(argument_type: ArgumentType) -> Self {
        match argument_type {
            ArgumentType::GameMode(id) => CommandArgument::game_mode(id),
            ArgumentType::Entity(id) => entity_argument(id, true, false),
            ArgumentType::Entities(id) => entity_argument(id, false, false),
            ArgumentType::Player(id) => entity_argument(id, true, true),
            ArgumentType::Players(id) => entity_argument(id, false, true),
        }
    }
}

impl From<&ArgumentType> for CommandArgument {
    fn from(argument_type: &ArgumentType) -> Self {
        (*argument_type).into()
    }
}

fn entity_argument(id: &'static str, single_entity: bool, only_players: bool) -> CommandArgument {
    let mut argument = CommandArgument::entity(id);
    argument.set_entity_selector_flags(entity_selector_flags(single_entity, only_players));
    argument
}

fn entity_selector_flags(single_entity: bool, only_players: bool) -> u8 {
    u8::from(single_entity) | (u8::from(only_players) << 1)
}
