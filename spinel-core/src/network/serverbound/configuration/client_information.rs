use spinel_macros::packet;

#[packet(
    id: "client_information",
    state: ConnectionState::Configuration,
    recipient: Recipient::Server,
    generate_fields: true
)]
pub struct ClientInformationPacket;

impl Default for ClientInformationPacket {
    fn default() -> Self {
        Self {
            information: spinel_network::types::ClientInformation::default(),
        }
    }
}
