use crate::entity::Player;
use spinel_macros::event_dispatcher;
use uuid::Uuid;

#[event_dispatcher(is_async: true, with_client: true)]
pub struct AsyncPlayerConfigurationEvent {
    player: Player,
    is_first_config: bool,
    is_hardcore: bool,
    spawning_world: Option<Uuid>,
}

impl AsyncPlayerConfigurationEvent {
    pub fn new(player: Player, is_first_config: bool) -> Self {
        Self {
            player,
            is_first_config,
            is_hardcore: false,
            spawning_world: None,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn into_player(self) -> Player {
        self.player
    }

    pub fn is_first_config(&self) -> bool {
        self.is_first_config
    }

    pub fn is_hardcore(&self) -> bool {
        self.is_hardcore
    }

    pub fn set_hardcore(&mut self, is_hardcore: bool) {
        self.is_hardcore = is_hardcore;
    }

    pub fn spawning_world(&self) -> Option<Uuid> {
        self.spawning_world
    }

    pub fn set_spawning_world(&mut self, spawning_world: Uuid) {
        self.spawning_world = Some(spawning_world);
    }
}
