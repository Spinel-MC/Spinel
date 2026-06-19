use spinel_macros::packet;

#[packet(id: "pong_response", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct PongResponsePacket {
    pub time: i64,
}
