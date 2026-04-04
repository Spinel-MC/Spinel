use spinel_macros::packet;

#[packet(id: "keep_alive", generate_fields: true, state: ConnectionState::Configuration, recipient: Recipient::Client)]
pub struct KeepAlivePacket;
