use spinel_macros::event_dispatcher;
use spinel_network::ConnectionState;

#[event_dispatcher(with_client: true)]
pub struct OutboundPacketErrorEvent {
    pub state: ConnectionState,
    pub packet_id: i32,
    pub packet_name: String,
    pub message: String,
}

impl OutboundPacketErrorEvent {
    pub fn new(
        state: ConnectionState,
        packet_id: i32,
        packet_name: String,
        message: String,
    ) -> Self {
        Self {
            state,
            packet_id,
            packet_name,
            message,
            connection_ptr: None,
        }
    }
}
