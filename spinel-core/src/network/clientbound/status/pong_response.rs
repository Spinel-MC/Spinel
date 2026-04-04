use spinel_macros::packet;

#[packet(id: "pong_response", state: ConnectionState::Status, recipient: Recipient::Client)]
pub struct PongResponsePacket {
    pub timestamp: i64,
}

impl PongResponsePacket {
    pub fn new(timestamp: i64) -> Self {
        Self { timestamp }
    }
}
