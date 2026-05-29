use crate::entity::Player;
use crate::server::MinecraftServer;
use spinel_core::network::clientbound::configuration::disconnect::ConfigurationDisconnectPacket;
use spinel_core::network::clientbound::login::disconnect::LoginDisconnectPacket;
use spinel_core::network::clientbound::play::disconnect::PlayDisconnectPacket;
use spinel_network::ConnectionState;
use spinel_utils::component::text::TextComponent;
use std::io;

impl Player {
    pub fn kick(&mut self, reason: impl Into<TextComponent>) -> io::Result<()> {
        let disconnect_reason = reason.into();
        let Some(client) = self.client_mut() else {
            return Ok(());
        };
        let client_address = client.addr;
        let server_ptr = client.server_ptr;
        let disconnect_result = match client.state {
            ConnectionState::Login => {
                LoginDisconnectPacket::new(disconnect_reason).dispatch(client)
            }
            ConnectionState::Configuration => {
                ConfigurationDisconnectPacket::new(disconnect_reason).dispatch(client)
            }
            ConnectionState::Play => PlayDisconnectPacket::new(disconnect_reason).dispatch(client),
            _ => Ok(()),
        };
        client.finish_disconnect_packet();
        if let Some(server_ptr) = server_ptr {
            let server = unsafe { &mut *(server_ptr as *mut MinecraftServer) };
            server.on_disconnect_with_client(client_address, client);
        }
        disconnect_result
    }
}
