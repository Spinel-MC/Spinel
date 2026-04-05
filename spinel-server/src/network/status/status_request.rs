
use crate::events::server_list_ping::event::ServerListPingEvent;
use crate::events::server_list_ping::ping_type::ServerListPingType;
use crate::network::client::instance::Client;
use ::spinel_macros::packet_listener;

use crate::instance::MinecraftServer;
use ::spinel_core::network::clientbound::status::status_response::StatusResponsePacket;
use ::spinel_core::network::serverbound::status::status_request::StatusRequestPacket;

#[packet_listener(module: "status")]
fn on_status_request(
    client: &mut Client,
    _packet: StatusRequestPacket,
    server: &mut MinecraftServer,
) -> bool {
    let mut event = ServerListPingEvent::new(ServerListPingType::Modern);

    event.dispatch(server, client);

    if event.cancelled {
        return true;
    }

    let response_packet = StatusResponsePacket::new(
        event
            .response_data
            .to_status_response_json(event.hide_players),
    );

    response_packet.dispatch(client);

    true
}
