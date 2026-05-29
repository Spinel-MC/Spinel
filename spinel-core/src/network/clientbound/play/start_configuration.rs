use spinel_macros::packet;

#[packet(id: "start_configuration", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct StartConfigurationPacket;
