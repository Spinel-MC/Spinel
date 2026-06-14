use crate::entity::{Player, PlayerSpawnPoint};
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerRespawnEvent {
    player: *mut Player,
    respawn_position: PlayerSpawnPoint,
}

impl PlayerRespawnEvent {
    pub fn new(player: *mut Player, respawn_position: PlayerSpawnPoint) -> Self {
        Self {
            player,
            respawn_position,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn respawn_position(&self) -> PlayerSpawnPoint {
        self.respawn_position
    }

    pub fn set_respawn_position(&mut self, respawn_position: PlayerSpawnPoint) {
        self.respawn_position = respawn_position;
    }
}
