use spinel_macros::packet;

#[packet(id: "finish_configuration", generate_fields: true, state: ConnectionState::Configuration, recipient: Recipient::Client)]
pub struct FinishConfigurationPacket;

impl FinishConfigurationPacket {
    pub fn new() -> Self {
        Self {}
    }
}
