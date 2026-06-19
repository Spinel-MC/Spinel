use spinel_macros::packet;

#[packet(id: "pong", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct PongPacket {
    pub id: i32,
}
