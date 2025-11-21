use std::net::SocketAddr;

use crate as spinel;
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct DisconnectionEvent {
    pub socket_address: SocketAddr,
}

impl DisconnectionEvent {
    pub fn new(socket_address: SocketAddr) -> Self {
        Self { socket_address }
    }
}
