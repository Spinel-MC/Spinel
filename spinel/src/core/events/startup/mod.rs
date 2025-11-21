use crate as spinel;
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct StartupEvent {
    pub cancelled: bool,
}

impl StartupEvent {
    pub fn new() -> Self {
        Self { cancelled: false }
    }
}

impl Default for StartupEvent {
    fn default() -> Self {
        Self::new()
    }
}
