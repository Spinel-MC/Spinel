use crate::entity::Player;
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerPacketEvent {
    player: *mut Player,
    packet_id: i32,
    packet_name: String,
    payload: Vec<u8>,
    cancelled: bool,
}

impl PlayerPacketEvent {
    pub fn new(
        player: *mut Player,
        packet_id: i32,
        packet_name: impl Into<String>,
        payload: Vec<u8>,
    ) -> Self {
        Self {
            player,
            packet_id,
            packet_name: packet_name.into(),
            payload,
            cancelled: false,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub const fn packet_id(&self) -> i32 {
        self.packet_id
    }

    pub fn packet_name(&self) -> &str {
        &self.packet_name
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub const fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }

    pub fn into_payload(self) -> Vec<u8> {
        self.payload
    }
}
