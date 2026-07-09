use spinel::client::MinecraftClient;
use spinel::core::network::clientbound::status::pong_response::PongResponsePacket;
use spinel::core::network::clientbound::status::status_response::StatusResponsePacket;
use spinel::macros::fn_packet_listener;
use spinel::network::Server;

#[fn_packet_listener(state: spinel::network::ConnectionState::Status)]
fn on_status_response(
    _server: &mut Server,
    _packet: StatusResponsePacket,
    _client: &mut MinecraftClient,
) -> bool {
    true
}

#[fn_packet_listener(state: spinel::network::ConnectionState::Status)]
fn on_pong(
    _server: &mut Server,
    _packet: PongResponsePacket,
    _client: &mut MinecraftClient,
) -> bool {
    true
}
