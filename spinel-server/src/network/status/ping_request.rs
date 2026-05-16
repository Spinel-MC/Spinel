use crate::events::ping::PingEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use ::spinel_core::network::clientbound::status::pong_response::PongResponsePacket;
use ::spinel_core::network::serverbound::status::ping_request::PingRequestPacket;
use ::spinel_macros::packet_listener;

#[packet_listener(module: "ping")]
fn on_ping_request(
    client: &mut Client,
    packet: PingRequestPacket,
    server: &mut MinecraftServer,
) -> bool {
    let mut event = PingEvent::new(packet.timestamp);

    event.dispatch(server, client);

    if event.cancelled {
        return true;
    }

    PongResponsePacket::new(packet.timestamp)
        .dispatch(client)
        .is_ok()
}
