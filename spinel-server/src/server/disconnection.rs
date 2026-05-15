use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::clientbound::configuration::disconnect::ConfigurationDisconnectPacket;
use spinel_core::network::clientbound::login::disconnect::LoginDisconnectPacket;
use spinel_core::network::clientbound::play::disconnect::PlayDisconnectPacket;
use spinel_network::ConnectionState;
use spinel_utils::component::text::TextComponent;
use std::io;

impl MinecraftServer {
    pub fn disconnect(
        &mut self,
        client: &mut Client,
        reason: impl Into<TextComponent>,
    ) -> io::Result<()> {
        let disconnect_reason = reason.into();
        let disconnect_result = self.send_disconnect_packet(client, disconnect_reason);
        self.close_client_connection(client);
        disconnect_result
    }

    fn close_client_connection(&mut self, client: &mut Client) {
        let client_address = client.addr;
        client.disconnect();
        self.on_disconnect(client_address);
    }

    fn send_disconnect_packet(
        &mut self,
        client: &mut Client,
        disconnect_reason: TextComponent,
    ) -> io::Result<()> {
        match client.state {
            ConnectionState::Login => LoginDisconnectPacket::new(disconnect_reason).dispatch(client),
            ConnectionState::Configuration => {
                ConfigurationDisconnectPacket::new(disconnect_reason).dispatch(client)
            }
            ConnectionState::Play => PlayDisconnectPacket::new(disconnect_reason).dispatch(client),
            _ => Ok(()),
        }
    }
}
