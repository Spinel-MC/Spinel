use spinel_macros::event_dispatcher;
use spinel_network::ConnectionState;

#[event_dispatcher(with_client: true)]
pub struct OutboundEventPacket {
    pub state: ConnectionState,
    pub id: i32,
    pub packet_name: String,
    pub payload_size: usize,
}

impl OutboundEventPacket {
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
