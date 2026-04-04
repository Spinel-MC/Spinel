use crate::instance::MinecraftClient;
use crate::network::server::instance::Server;
use spinel_core::network::clientbound::status::pong_response::PongResponsePacket;
use spinel_macros::packet_listener;

#[packet_listener()]
fn on_pong_response(
    client: &mut Server,
    packet: PongResponsePacket,
    _mc_client: &mut MinecraftClient,
) -> bool {
    println!("Received Pong Response: payload = {}", packet.timestamp);

    client.disconnect();

    true
}
