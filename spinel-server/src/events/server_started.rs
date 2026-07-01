use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct ServerStartedEvent {
    pub bind_address: String,
    pub port: u16,
}

impl ServerStartedEvent {
    pub fn new(bind_address: impl Into<String>, port: u16) -> Self {
        Self {
            bind_address: bind_address.into(),
            port,
        }
    }
}

impl Default for ServerStartedEvent {
    fn default() -> Self {
        Self::new(String::new(), 0)
    }
}
