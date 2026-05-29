use crate::entity::Player;
use spinel_macros::event_dispatcher;
use spinel_network::types::Identifier;

#[event_dispatcher(with_client: true)]
pub struct PlayerSpawnEvent {
    player: *mut Player,
    world_name: Identifier,
    first_spawn: bool,
}

impl PlayerSpawnEvent {
    pub fn new(player: *mut Player, world_name: Identifier, first_spawn: bool) -> Self {
        Self {
            player,
            world_name,
            first_spawn,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn world_name(&self) -> &Identifier {
        &self.world_name
    }

    pub fn first_spawn(&self) -> bool {
        self.first_spawn
    }
}
