use spinel::client::MinecraftClient;
use spinel::core::network::clientbound::login::disconnect::LoginDisconnectPacket;
use spinel::core::network::clientbound::login::encryption_request::EncryptionRequestPacket;
use spinel::core::network::clientbound::login::login_success::LoginSuccessPacket;
use spinel::core::network::clientbound::login::set_compression::SetCompressionPacket;
use spinel::core::network::serverbound::configuration::client_information::ClientInformationPacket;
use spinel::core::network::serverbound::login::login_acknowledge::LoginAcknowledgedPacket;
use spinel::macros::packet_listener;
use spinel::network::{ConnectionState, Server};

use crate::dispatch::report_dispatch_result;

#[packet_listener(state: ConnectionState::Login)]
fn on_login_disconnect(
    _server: &mut Server,
    _packet: LoginDisconnectPacket,
    _client: &mut MinecraftClient,
) -> bool {
    println!("Received Login Disconnect!");
    true
}

#[packet_listener(state: ConnectionState::Login)]
fn on_encryption_request(
    _server: &mut Server,
    _packet: EncryptionRequestPacket,
    _client: &mut MinecraftClient,
) -> bool {
    println!("Received Encryption Request (Hello)!");
    true
}

#[packet_listener(state: ConnectionState::Login)]
fn on_set_compression(
    server: &mut Server,
    packet: SetCompressionPacket,
    _client: &mut MinecraftClient,
) -> bool {
    println!("Setting compression threshold to: {}", packet.threshold);
    server.set_compression(packet.threshold);
    true
}

#[packet_listener(state: ConnectionState::Login)]
fn on_login_success(
    server: &mut Server,
    _packet: LoginSuccessPacket,
    client: &mut MinecraftClient,
) -> bool {
    println!("Login Success!");
    server.state = ConnectionState::Login;
    client.state = ConnectionState::Login;

    report_dispatch_result(
        LoginAcknowledgedPacket {}.dispatch(server),
        "login acknowledged packet",
    );

    server.state = ConnectionState::Configuration;
    client.state = ConnectionState::Configuration;
    println!("Sent Login Acknowledged. Transitioning to Configuration.");
    println!("Sending Client Information...");

    report_dispatch_result(
        ClientInformationPacket::default().dispatch(server),
        "client information packet",
    );
    true
}
