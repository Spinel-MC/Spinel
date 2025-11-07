use crate as spinel;
use spinel_macros::event_dispatcher;

#[event_dispatcher(event: "connection", with_client: true)]
pub struct ConnectionEvent {
    pub cancelled: bool,
}

impl ConnectionEvent {
    pub fn new() -> Self {
        Self {
            cancelled: false,
            client_ptr: None,
        }
    }
}

impl Default for ConnectionEvent {
    fn default() -> Self {
        Self::new()
    }
}
