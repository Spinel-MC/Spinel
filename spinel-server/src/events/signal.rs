use spinel_macros::event_dispatcher;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServerSignal {
    CtrlC,
}

#[event_dispatcher]
pub struct SignalEvent {
    pub signal: ServerSignal,
}

impl SignalEvent {
    pub fn new(signal: ServerSignal) -> Self {
        Self { signal }
    }
}
