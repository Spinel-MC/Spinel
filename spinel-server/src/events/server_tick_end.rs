use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct ServerTickEndEvent {
    pub current_tick: u64,
}

impl ServerTickEndEvent {
    pub fn new(current_tick: u64) -> Self {
        Self { current_tick }
    }
}

impl Default for ServerTickEndEvent {
    fn default() -> Self {
        Self::new(0)
    }
}
