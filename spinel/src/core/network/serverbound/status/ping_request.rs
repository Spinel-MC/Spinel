use crate::{
    self as spinel,
    core::{
        events::ping::PingEvent, network::clientbound::status::pong_response::PongResponsePacket,
        server::MinecraftServer,
    },
};
use spinel_macros::packet_listener;
use spinel_network::{Client, ConnectionState};

#[packet_listener(id: "ping_request", state: ConnectionState::Status, fields:(timestamp: Long), module: "ping")]
fn on_ping_request(client: &mut Client, packet: Packet, server: &mut MinecraftServer) -> bool {
    let mut event = PingEvent::new(packet.timestamp);

    event.dispatch(server, client);

    if event.cancelled {
        return true;
    }

    let response_packet = PongResponsePacket::new(packet.timestamp);

    response_packet.dispatch(client);

    true
}
