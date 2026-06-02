use crate::entity::Player;
use spinel_macros::event_dispatcher;
use spinel_network::types::Identifier;

#[event_dispatcher(with_client: true)]
pub struct AdvancementTabEvent {
    player: *mut Player,
    tab_identifier: Option<Identifier>,
}

impl AdvancementTabEvent {
    pub fn new(player: *mut Player, tab_identifier: Option<Identifier>) -> Self {
        Self {
            player,
            tab_identifier,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn tab_identifier(&self) -> Option<&Identifier> {
        self.tab_identifier.as_ref()
    }

    pub const fn is_closed(&self) -> bool {
        self.tab_identifier.is_none()
    }
}
