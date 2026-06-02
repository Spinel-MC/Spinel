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

#[cfg(test)]
mod tests {
    use super::ClientInformationPacket;
    use spinel_network::DataType;
    use spinel_network::types::ClientInformation;

    #[test]
    fn play_client_information_decodes_minestom_settings_shape() {
        let settings = ClientInformation {
            locale: "en_GB".to_string(),
            view_distance: 12,
            chat_mode: 1,
            chat_colors: false,
            displayed_skin_parts: 3,
            main_hand: 0,
            enable_text_filtering: true,
            allow_server_listings: false,
            particle_status: 2,
        };
        let mut payload = Vec::new();
        settings.encode(&mut payload).unwrap();

        let packet = ClientInformationPacket::decode(&mut payload.as_slice()).unwrap();

        assert_eq!(packet.settings, settings);
        assert_eq!(ClientInformationPacket::get_id(), 0x0d);
    }
}
