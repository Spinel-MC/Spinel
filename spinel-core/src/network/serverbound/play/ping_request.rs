use spinel_macros::packet;

#[packet(id: "ping_request", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct PingRequestPacket {
    pub time: i64,
}
