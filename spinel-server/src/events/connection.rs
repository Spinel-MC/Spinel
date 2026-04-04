use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct ConnectionEvent {
    pub cancelled: bool,
}

impl ConnectionEvent {
    pub fn new() -> Self {
        Self {
            cancelled: false,
            connection_ptr: None,
        }
    }
}

impl Default for ConnectionEvent {
    fn default() -> Self {
        Self::new()
    }
}
