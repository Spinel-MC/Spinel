use crate::network::client::instance::Client;
use spinel_core::network::clientbound::configuration::disconnect::ConfigurationDisconnectPacket;
use spinel_core::network::clientbound::login::disconnect::LoginDisconnectPacket;
use spinel_core::network::clientbound::play::disconnect::PlayDisconnectPacket;
use spinel_network::ConnectionState;
use spinel_utils::component::text::TextComponent;
use std::io;

impl Client {
    pub fn kick(&mut self, reason: impl Into<TextComponent>) -> io::Result<()> {
        let reason = reason.into();
        self.discard_queued_outbound_packets();
        let kick_result = match self.state {
            ConnectionState::Login => LoginDisconnectPacket::new(reason).dispatch(self),
            ConnectionState::Configuration => {
                ConfigurationDisconnectPacket::new(reason).dispatch(self)
            }
            ConnectionState::Play => PlayDisconnectPacket::new(reason).dispatch(self),
            _ => Ok(()),
        };
        self.close_after_disconnect_packet();
        kick_result
    }
}
