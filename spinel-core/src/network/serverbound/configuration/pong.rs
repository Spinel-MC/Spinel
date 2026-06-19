use spinel_macros::packet;

#[packet(id: "pong", state: ConnectionState::Configuration, recipient: Recipient::Server)]
pub struct PongPacket {
    pub id: i32,
}
