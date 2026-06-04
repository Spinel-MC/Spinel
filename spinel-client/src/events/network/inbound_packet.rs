use spinel_macros::event_dispatcher;
use spinel_network::ConnectionState;

#[event_dispatcher(with_server: true)]
pub struct InboundPacketEvent {
    pub state: ConnectionState,
    pub id: i32,
    pub packet_name: String,
    pub payload_size: usize,
}

impl InboundPacketEvent {
    pub fn new(state: ConnectionState, id: i32, packet_name: String, payload_size: usize) -> Self {
        Self {
            state,
            id,
            packet_name,
            payload_size,
            connection_ptr: None,
        }
    }
}
