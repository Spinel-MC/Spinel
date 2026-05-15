use crate::server::MinecraftServer;
use crate::network::client::instance::Client;
use ::spinel_core::network::clientbound::configuration::{
    known_packs::KnownPacksPacket, plugin_message::CustomPayloadPacket,
};
use ::spinel_core::network::serverbound::login::login_acknowledge::LoginAcknowledgedPacket;
use ::spinel_macros::packet_listener;
use ::spinel_network::ConnectionState;
use ::spinel_utils::constants::{MINECRAFT_VERSION, SERVER_BRAND};
use std::io;

struct ConfigurationTransition<'a> {
    client: &'a mut Client,
}

impl<'a> ConfigurationTransition<'a> {
    fn apply(self) -> io::Result<()> {
        self.client.state = ConnectionState::Configuration;
        CustomPayloadPacket {
            channel: "minecraft:brand".to_string(),
            data: SERVER_BRAND.as_bytes().to_vec(),
        }
        .dispatch(self.client)?;
        KnownPacksPacket::new(vec![(
            "minecraft".to_string(),
            "core".to_string(),
            MINECRAFT_VERSION.to_string(),
        )])
        .dispatch(self.client)
    }
}

#[packet_listener(id: "login_acknowledged", state: ConnectionState::Login, module: "login")]
fn on_login_acknowledged(
    client: &mut Client,
    _packet: LoginAcknowledgedPacket,
    _server: &mut MinecraftServer,
) -> bool {
    ConfigurationTransition { client }.apply().is_ok()
}
