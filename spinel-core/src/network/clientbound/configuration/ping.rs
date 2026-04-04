use spinel_macros::packet;

#[packet(id: "ping", generate_fields: true, state: ConnectionState::Configuration, recipient: Recipient::Client)]
pub struct PingPacket;
