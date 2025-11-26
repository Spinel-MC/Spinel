use crate as spinel;
use crate::core::events::server_list_ping::ServerListPingEventResponseData;
use spinel_macros::packet_dispatcher;

#[packet_dispatcher(id: "status_response", state: ConnectionState::Status)]
pub struct StatusResponsePacket {
    pub json_response: String,
}

impl StatusResponsePacket {
    pub fn new(response_data: ServerListPingEventResponseData, hide_players: bool) -> Self {
        Self {
            json_response: response_data.to_status_response_json(hide_players),
        }
    }
}
