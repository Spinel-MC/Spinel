use crate::{
    self as spinel,
    core::{network::clientbound::play::login_play::LoginPlayPacket, server::MinecraftServer},
};

use spinel_macros::packet_listener;
use spinel_network::{client::instance::ConnectionState, Client};

#[packet_listener(id: "finish_configuration", state: ConnectionState::Configuration)]
fn on_finish_configuration(client: &mut Client, _server: &mut MinecraftServer) -> bool {
    println!("Client acknowledged finish configuration. Transitioning to Play state.");

    client.state = ConnectionState::Play;

    let login_packet = LoginPlayPacket::new_default(41);
    login_packet.dispatch(client);

    true
}
