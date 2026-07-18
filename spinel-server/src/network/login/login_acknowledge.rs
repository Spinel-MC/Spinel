use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::clientbound::configuration::{
    known_packs::KnownPacksPacket, plugin_message::CustomPayloadPacket,
};
use spinel_core::network::serverbound::login::login_acknowledge::LoginAcknowledgedPacket;
use spinel_macros::fn_packet_listener;
use spinel_network::{ConnectionState, DataType, RawBytes};
use spinel_utils::constants::{MINECRAFT_VERSION, SERVER_BRAND};
use std::io;

struct ConfigurationTransition<'a> {
    client: &'a mut Client,
}

impl<'a> ConfigurationTransition<'a> {
    fn apply(self) -> io::Result<()> {
        self.client.state = ConnectionState::Configuration;

        let mut brand_payload = Vec::new();
        SERVER_BRAND.to_string().encode(&mut brand_payload)?;

        CustomPayloadPacket {
            channel: "minecraft:brand".to_string(),
            data: RawBytes::from(brand_payload),
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

#[fn_packet_listener(id: "login_acknowledged", state: ConnectionState::Login)]
fn on_login_acknowledged(
    client: &mut Client,
    _packet: LoginAcknowledgedPacket,
    _server: &mut MinecraftServer,
) -> bool {
    ConfigurationTransition { client }.apply().is_ok()
}
