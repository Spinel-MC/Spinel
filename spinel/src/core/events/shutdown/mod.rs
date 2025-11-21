use crate as spinel;
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct ShutdownEvent {}

impl ShutdownEvent {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ShutdownEvent {
    fn default() -> Self {
        Self::new()
    }
}
