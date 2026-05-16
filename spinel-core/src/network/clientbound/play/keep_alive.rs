use spinel_macros::packet;

#[packet(id: "keep_alive", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct KeepAlivePacket {
    pub id: i64,
}
