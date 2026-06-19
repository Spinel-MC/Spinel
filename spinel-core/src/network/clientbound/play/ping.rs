use spinel_macros::packet;

#[packet(id: "ping", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct PingPacket {
    pub id: i32,
}
