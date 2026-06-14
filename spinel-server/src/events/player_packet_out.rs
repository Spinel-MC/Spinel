use crate::entity::Player;
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerPacketOutEvent {
    player: *mut Player,
    packet_id: i32,
    packet_name: String,
    payload_size: usize,
    cancelled: bool,
}

impl PlayerPacketOutEvent {
    pub fn new(
        player: *mut Player,
        packet_id: i32,
        packet_name: impl Into<String>,
        payload_size: usize,
    ) -> Self {
        Self {
            player,
            packet_id,
            packet_name: packet_name.into(),
            payload_size,
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

    pub const fn payload_size(&self) -> usize {
        self.payload_size
    }

    pub const fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
