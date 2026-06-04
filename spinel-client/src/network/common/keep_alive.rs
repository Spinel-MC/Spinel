use crate::instance::MinecraftClient;
use crate::network::server::instance::Server;
use spinel_core::network::clientbound::configuration::keep_alive::KeepAlivePacket as ConfigurationKeepAlivePacket;
use spinel_core::network::clientbound::play::keep_alive::KeepAlivePacket as PlayKeepAlivePacket;
use spinel_core::network::serverbound::configuration::keep_alive::KeepAlivePacket as ConfigurationKeepAliveResponsePacket;
use spinel_core::network::serverbound::play::keep_alive::KeepAlivePacket as PlayKeepAliveResponsePacket;
use spinel_macros::packet_listener;

#[packet_listener(state: spinel_network::ConnectionState::Configuration)]
fn on_configuration_keep_alive(
    server: &mut Server,
    packet: ConfigurationKeepAlivePacket,
    _client: &mut MinecraftClient,
) -> bool {
    ConfigurationKeepAliveResponsePacket { id: packet.id }
        .dispatch(server)
        .is_ok()
}

#[packet_listener(state: spinel_network::ConnectionState::Play)]
fn on_play_keep_alive(
    server: &mut Server,
    packet: PlayKeepAlivePacket,
    _client: &mut MinecraftClient,
) -> bool {
    PlayKeepAliveResponsePacket { id: packet.id }
        .dispatch(server)
        .is_ok()
}
