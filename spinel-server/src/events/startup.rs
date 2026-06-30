use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct StartupEvent {
    pub cancelled: bool,
    pub bind_address: String,
    pub port: u16,
}

impl StartupEvent {
    pub fn new(bind_address: impl Into<String>, port: u16) -> Self {
        Self {
            cancelled: false,
            bind_address: bind_address.into(),
            port,
        }
    }
}

impl Default for StartupEvent {
    fn default() -> Self {
        Self::new(String::new(), 0)
    }
}
