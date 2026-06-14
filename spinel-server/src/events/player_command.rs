use crate::entity::Player;
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerCommandEvent {
    player: *mut Player,
    command: String,
    cancelled: bool,
}

impl PlayerCommandEvent {
    pub fn new(player: *mut Player, command: impl Into<String>) -> Self {
        Self {
            player,
            command: command.into(),
            cancelled: false,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn set_command(&mut self, command: impl Into<String>) {
        self.command = command.into();
    }

    pub const fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }

    pub fn into_command(self) -> String {
        self.command
    }
}
