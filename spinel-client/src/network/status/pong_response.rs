use crate::events::disconnect::DisconnectEvent;
use crate::instance::MinecraftClient;
use crate::network::server::instance::Server;
use spinel_core::network::clientbound::status::pong_response::PongResponsePacket;
use spinel_macros::packet_listener;

#[packet_listener()]
fn on_pong_response(
    server: &mut Server,
    _packet: PongResponsePacket,
    client: &mut MinecraftClient,
) -> bool {
    let mut disconnect_event =
        DisconnectEvent::new(server.state, "status pong response complete".into());
    disconnect_event.dispatch(client, server);
    server.disconnect();

    true
}
