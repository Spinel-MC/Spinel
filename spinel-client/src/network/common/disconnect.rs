use crate::events::disconnect::DisconnectEvent;
use crate::instance::MinecraftClient;
use crate::network::server::instance::Server;
use spinel_core::network::clientbound::configuration::disconnect::ConfigurationDisconnectPacket;
use spinel_core::network::clientbound::login::disconnect::LoginDisconnectPacket;
use spinel_core::network::clientbound::play::disconnect::PlayDisconnectPacket;
use spinel_macros::packet_listener;

#[packet_listener(state: spinel_network::ConnectionState::Login)]
fn on_login_disconnect(
    server: &mut Server,
    packet: LoginDisconnectPacket,
    client: &mut MinecraftClient,
) -> bool {
    dispatch_disconnect_event(server, client, packet.reason)
}

#[packet_listener(state: spinel_network::ConnectionState::Configuration)]
fn on_configuration_disconnect(
    server: &mut Server,
    packet: ConfigurationDisconnectPacket,
    client: &mut MinecraftClient,
) -> bool {
    dispatch_disconnect_event(server, client, packet.reason)
}

#[packet_listener(state: spinel_network::ConnectionState::Play)]
fn on_play_disconnect(
    server: &mut Server,
    packet: PlayDisconnectPacket,
    client: &mut MinecraftClient,
) -> bool {
    dispatch_disconnect_event(server, client, packet.reason)
}

fn dispatch_disconnect_event(
    server: &mut Server,
    client: &mut MinecraftClient,
    reason: spinel_utils::component::text::TextComponent,
) -> bool {
    let mut disconnect_event = DisconnectEvent::new(server.state, reason);
    disconnect_event.dispatch(client, server);
    server.disconnect();
    true
}
