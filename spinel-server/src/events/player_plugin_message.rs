use crate::entity::Player;
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerPluginMessageEvent {
    player: *mut Player,
    channel: String,
    data: Vec<u8>,
}

impl PlayerPluginMessageEvent {
    pub fn new(player: *mut Player, channel: String, data: Vec<u8>) -> Self {
        Self {
            player,
            channel,
            data,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn channel(&self) -> &str {
        &self.channel
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn data_string(&self) -> String {
        String::from_utf8_lossy(&self.data).into_owned()
    }
}
