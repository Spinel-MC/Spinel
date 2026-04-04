use spinel_macros::packet;

#[packet(
    id: "ping_request",
    state: ConnectionState::Status,
    recipient: Recipient::Server
)]
pub struct PingRequestPacket {
    pub timestamp: i64,
}
