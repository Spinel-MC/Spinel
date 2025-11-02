use spinel_macros::{event_dispatcher};
use crate as spinel;

#[event_dispatcher(event: "shutdown")]
pub struct ShutdownEvent {
}

impl ShutdownEvent {
        pub fn new() -> Self {
                Self {
                }
        }
}

impl Default for ShutdownEvent {
    fn default() -> Self {
        Self::new()
    }
}