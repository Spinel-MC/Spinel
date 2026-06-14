use spinel_macros::event_dispatcher;
use spinel_network::{ConnectionState, Recipient};

#[event_dispatcher(with_client: true)]
pub struct PacketEvent {
    pub recipient: Recipient,
    pub state: ConnectionState,
    pub id: i32,
    pub packet_name: String,
    pub payload_size: usize,
}

impl PacketEvent {
    pub fn new(
        recipient: Recipient,
        state: ConnectionState,
        id: i32,
        packet_name: String,
        payload_size: usize,
    ) -> Self {
        Self {
            recipient,
            state,
            id,
            packet_name,
            payload_size,
            connection_ptr: None,
        }
    }
}
