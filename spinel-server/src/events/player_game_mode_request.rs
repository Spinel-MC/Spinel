use crate::entity::Player;
use spinel_core::entity::game_mode::GameMode;
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerGameModeRequestEvent {
    player: *mut Player,
    requested_game_mode: GameMode,
}

impl PlayerGameModeRequestEvent {
    pub fn new(player: *mut Player, requested_game_mode: GameMode) -> Self {
        Self {
            player,
            requested_game_mode,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn requested_game_mode(&self) -> GameMode {
        self.requested_game_mode
    }
}
