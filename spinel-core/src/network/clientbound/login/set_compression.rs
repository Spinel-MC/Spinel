use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "login_compression", state: ConnectionState::Login, recipient: Recipient::Client)]
pub struct SetCompressionPacket {
    pub threshold: VarInt,
}

impl SetCompressionPacket {
    pub fn new(threshold: i32) -> Self {
        Self { threshold }
    }
}
