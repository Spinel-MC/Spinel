use spinel_macros::packet;

#[packet(id: 0xFE, state: ConnectionState::Handshaking)]
pub struct LegacyServerListPingPacket;
