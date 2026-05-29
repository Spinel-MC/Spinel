use crate::entity::Player;
use spinel_core::entity::game_mode::GameMode;
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerGameModeChangeEvent {
    player: *mut Player,
    new_game_mode: GameMode,
    cancelled: bool,
}

impl PlayerGameModeChangeEvent {
    pub fn new(player: *mut Player, new_game_mode: GameMode) -> Self {
        Self {
            player,
            new_game_mode,
            cancelled: false,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn new_game_mode(&self) -> GameMode {
        self.new_game_mode
    }

    pub fn set_new_game_mode(&mut self, new_game_mode: GameMode) {
        self.new_game_mode = new_game_mode;
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
