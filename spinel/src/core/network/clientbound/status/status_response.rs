use spinel_macros::packet_dispatcher;
use crate::core::events::server_list_ping::ServerListPingEventResponseData;
use crate as spinel;



#[packet_dispatcher(id:0x00)]
pub struct StatusResponsePacket {
        pub json_response: String,
}

impl StatusResponsePacket {
        pub fn new(response_data: ServerListPingEventResponseData, hide_players: bool) -> Self {
                Self {
                        json_response: response_data.to_status_response_json(hide_players)
                }
        }
}
