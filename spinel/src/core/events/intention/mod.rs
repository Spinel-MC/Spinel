use crate as spinel;
use spinel_macros::event_dispatcher;

#[event_dispatcher(event: "intention", with_client: true)]
pub struct IntentionEvent {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub intent: i32,
    pub cancelled: bool,
}

impl IntentionEvent {
    pub fn new(
        protocol_version: i32,
        server_address: String,
        server_port: u16,
        intent: i32,
    ) -> Self {
        IntentionEvent {
            protocol_version,
            server_address,
            server_port,
            intent,
            cancelled: false,
            client_ptr: None,
        }
    }
}
