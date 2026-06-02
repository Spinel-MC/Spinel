use crate::entity::Player;
use spinel_macros::event_dispatcher;
use spinel_network::types::Identifier;

#[event_dispatcher(with_client: true)]
pub struct PlayerConfigCustomClickEvent {
    player: *mut Player,
    key: Identifier,
    payload: Option<Vec<u8>>,
}

impl PlayerConfigCustomClickEvent {
    pub fn new(player: *mut Player, key: Identifier, payload: Option<Vec<u8>>) -> Self {
        Self {
            player,
            key,
            payload,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn key(&self) -> &Identifier {
        &self.key
    }

    pub fn payload(&self) -> Option<&[u8]> {
        self.payload.as_deref()
    }
}
