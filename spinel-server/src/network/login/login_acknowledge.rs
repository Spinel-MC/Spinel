use crate::instance::MinecraftServer;
use crate::network::client::instance::Client;
use ::spinel_core::network::clientbound::configuration::{
    known_packs::KnownPacksPacket, plugin_message::PluginMessagePacket,
};
use ::spinel_core::network::serverbound::login::login_acknowledge::LoginAcknowledgedPacket;
use ::spinel_macros::packet_listener;
use ::spinel_network::ConnectionState;
use ::spinel_utils::constants::{MINECRAFT_VERSION, SERVER_BRAND};

#[packet_listener(id: "login_acknowledged", state: ConnectionState::Login, module: "login")]
fn on_login_acknowledge(
    client: &mut Client,
    _packet: LoginAcknowledgedPacket,
    _server: &mut MinecraftServer,
) -> bool {
    client.state = ConnectionState::Configuration;

    send_brand_packet(client);

    send_known_packs_packet(client);

    true
}

fn send_brand_packet(client: &mut Client) {
    let channel = "minecraft:brand".to_string();
    let brand_data = SERVER_BRAND.as_bytes().to_vec();

    let packet = PluginMessagePacket {
        channel,
        data: brand_data,
    };
    packet.dispatch(client);
}

fn send_known_packs_packet(client: &mut Client) {
    let known_packs = vec![(
        "minecraft".to_string(),
        "core".to_string(),
        MINECRAFT_VERSION.to_string(),
    )];

    let packet = KnownPacksPacket::new(known_packs);
    packet.dispatch(client);
}
