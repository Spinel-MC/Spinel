use crate as spinel;
use spinel_macros::event_dispatcher;

#[event_dispatcher(event: "status", with_client: true)]
pub struct PingEvent {
        pub timestamp: i64,
        pub cancelled: bool,
}

impl PingEvent {
        pub fn new(timestamp: i64) -> Self {
                PingEvent { timestamp, cancelled: false, client_ptr: None }
        }
}