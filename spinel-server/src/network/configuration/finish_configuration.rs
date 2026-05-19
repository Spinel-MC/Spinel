use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use ::spinel_core::network::serverbound::configuration::finish_configuration::FinishConfigurationPacket;
use ::spinel_macros::packet_listener;
use ::spinel_network::ConnectionState;
use std::io;

#[packet_listener(id: "finish_configuration", state: ConnectionState::Configuration)]
fn on_finish_configuration(
    client: &mut Client,
    _packet: FinishConfigurationPacket,
    server: &mut MinecraftServer,
) -> bool {
    println!("Client acknowledged finish configuration. Transitioning to Play state.");
    client.state = ConnectionState::Play;
    dispatch_play_packets(client, server).is_ok()
}

fn dispatch_play_packets(client: &mut Client, server: &mut MinecraftServer) -> io::Result<()> {
    server.enter_player_world(client)
}
