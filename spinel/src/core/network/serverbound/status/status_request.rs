use spinel_macros::packet_listener;
use spinel_network::Client;

use crate::{
    self as spinel,
    core::{
        events::server_list_ping::{
            server_list_ping_type::ServerListPingType, ServerListPingEvent,
        },
        network::clientbound::status::status_response::StatusResponsePacket,
        server::MinecraftServer,
    },
};

#[packet_listener(id: "status_request", state: "Status", module: "status")]
fn on_status_request(client: &mut Client, server: &mut MinecraftServer) -> bool {
    let mut event = ServerListPingEvent::new(ServerListPingType::Modern);

    event.dispatch(server, client);

    if event.cancelled {
        return true;
    }

    let response_packet = StatusResponsePacket::new(event.response_data, event.hide_players);

    response_packet.dispatch(client);

    true
}
