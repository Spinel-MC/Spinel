use crate::entity::Player;
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct OutgoingTransferEvent {
    player: *mut Player,
    host: String,
    port: i32,
    cancelled: bool,
}

impl OutgoingTransferEvent {
    pub fn new(player: *mut Player, host: impl Into<String>, port: i32) -> Self {
        Self {
            player,
            host: host.into(),
            port,
            cancelled: false,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn set_host(&mut self, host: impl Into<String>) {
        self.host = host.into();
    }

    pub const fn port(&self) -> i32 {
        self.port
    }

    pub fn set_port(&mut self, port: i32) {
        self.port = port;
    }

    pub const fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }

    pub fn into_destination(self) -> (String, i32) {
        (self.host, self.port)
    }
}
