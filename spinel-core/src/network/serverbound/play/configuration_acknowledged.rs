use spinel_macros::packet;

#[packet(
    id: "configuration_acknowledged",
    state: ConnectionState::Play,
    recipient: Recipient::Server
)]
pub struct ConfigurationAcknowledgedPacket;
