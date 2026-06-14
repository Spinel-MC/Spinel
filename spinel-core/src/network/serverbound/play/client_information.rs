use spinel_macros::packet;
use spinel_network::types::ClientInformation;

#[packet(id: "client_information", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct ClientInformationPacket {
    pub settings: ClientInformation,
}

impl Default for ClientInformationPacket {
    fn default() -> Self {
        Self {
            settings: ClientInformation::default(),
        }
    }
}
