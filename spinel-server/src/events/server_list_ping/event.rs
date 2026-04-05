use spinel_macros::event_dispatcher;

use crate::events::server_list_ping::{ping_type::ServerListPingType, response_data::ServerListPingEventResponseData};

#[event_dispatcher(with_client: true)]
pub struct ServerListPingEvent {
    pub response_data: ServerListPingEventResponseData,
    pub server_list_ping_type: ServerListPingType,
    pub hide_players: bool,
    pub cancelled: bool,
}

impl ServerListPingEvent {
    pub fn new(server_list_ping_type: ServerListPingType) -> Self {
        Self {
            response_data: ServerListPingEventResponseData::new(),
            server_list_ping_type,
            cancelled: false,
            hide_players: false,
            connection_ptr: None,
        }
    }

    pub fn hide_players(&mut self) {
        self.hide_players = true;
    }
}

