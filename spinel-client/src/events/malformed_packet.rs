use spinel_macros::event_dispatcher;

#[event_dispatcher(with_server: true)]
pub struct MalformedPacketEvent {
    pub packet_id: i32,
    pub error: String,
}

impl MalformedPacketEvent {
    pub fn new(packet_id: i32, error: String) -> Self {
        Self {
            packet_id,
            error,
            connection_ptr: None,
        }
    }
}
