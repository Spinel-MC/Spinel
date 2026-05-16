use spinel_macros::packet;

#[packet(id: "keep_alive", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct KeepAlivePacket {
    pub id: i64,
}
