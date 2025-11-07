use crate::{self as spinel, core::server::MinecraftServer};
use spinel_macros::packet_listener;
use spinel_network::{Client, client::instance::ConnectionState};

#[packet_listener(id: 0x03, state: ConnectionState::Login, module: "login")]
fn on_login_acknowledge(client: &mut Client, _server: &mut MinecraftServer) -> bool {
    println!("Client acknowledged login. Transitioning to Configuration state.");

    client.state = ConnectionState::Configuration;
    true
}
