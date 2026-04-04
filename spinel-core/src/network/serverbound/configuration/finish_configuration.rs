use spinel_macros::packet;

#[packet(
    id: "finish_configuration",
    state: ConnectionState::Configuration,
    recipient: Recipient::Server
)]
pub struct FinishConfigurationPacket;
