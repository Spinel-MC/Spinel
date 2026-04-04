use crate::instance::MinecraftServer;
use crate::network::client::instance::Client;
use ::spinel_core::network::clientbound::play::login_play::LoginPlayPacket;
use ::spinel_core::network::serverbound::configuration::finish_configuration::FinishConfigurationPacket;
use ::spinel_macros::packet_listener;
use ::spinel_network::ConnectionState;

#[packet_listener(id: "finish_configuration", state: ConnectionState::Configuration)]
fn on_finish_configuration(
    client: &mut Client,
    _packet: FinishConfigurationPacket,
    _server: &mut MinecraftServer,
) -> bool {
    println!("Client acknowledged finish configuration. Transitioning to Play state.");

    client.state = ConnectionState::Play;

    let login_packet = LoginPlayPacket::new_default(41);
    login_packet.dispatch(client);

    true
}
