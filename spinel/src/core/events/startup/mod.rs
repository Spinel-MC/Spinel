use crate as spinel;
use spinel_macros::event_dispatcher;

#[event_dispatcher(event: "startup")]
pub struct StartupEvent {
    pub cancelled: bool,
}

impl StartupEvent {
    pub fn new() -> Self {
        Self { cancelled: true }
    }
}

impl Default for StartupEvent {
    fn default() -> Self {
        Self::new()
    }
}
