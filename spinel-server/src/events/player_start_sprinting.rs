use crate::entity::Player;
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerStartSprintingEvent {
    player: *mut Player,
}

impl PlayerStartSprintingEvent {
    pub fn new(player: *mut Player) -> Self {
        Self {
            player,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }
}
