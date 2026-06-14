use spinel_macros::packet;

#[packet(
    id: 0xFE,
    state: ConnectionState::Handshaking,
    recipient: Recipient::Server
)]
pub struct LegacyServerListPingPacket;
