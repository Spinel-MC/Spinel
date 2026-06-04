use spinel::client::MinecraftClient;
use spinel::core::network::clientbound::login::encryption_request::EncryptionRequestPacket;
use spinel::core::network::clientbound::login::login_success::LoginSuccessPacket;
use spinel::core::network::clientbound::login::set_compression::SetCompressionPacket;
use spinel::core::network::serverbound::configuration::client_information::ClientInformationPacket;
use spinel::core::network::serverbound::login::login_acknowledge::LoginAcknowledgedPacket;
use spinel::macros::packet_listener;
use spinel::network::{ConnectionState, Server};

use crate::dispatch::report_dispatch_result;

#[packet_listener(state: ConnectionState::Login)]
fn on_encryption_request(
    _server: &mut Server,
    _packet: EncryptionRequestPacket,
    _client: &mut MinecraftClient,
) -> bool {
    true
}

#[packet_listener(state: ConnectionState::Login)]
fn on_set_compression(
    server: &mut Server,
    packet: SetCompressionPacket,
    _client: &mut MinecraftClient,
) -> bool {
    server.set_compression(packet.threshold);
    true
}

#[packet_listener(state: ConnectionState::Login)]
fn on_login_success(
    server: &mut Server,
    _packet: LoginSuccessPacket,
    client: &mut MinecraftClient,
) -> bool {
    server.state = ConnectionState::Login;
    client.state = ConnectionState::Login;

    report_dispatch_result(
        LoginAcknowledgedPacket {}.dispatch(server),
        "login acknowledged packet",
    );

    server.state = ConnectionState::Configuration;
    client.state = ConnectionState::Configuration;

    report_dispatch_result(
        ClientInformationPacket::default().dispatch(server),
        "client information packet",
    );
    true
}
