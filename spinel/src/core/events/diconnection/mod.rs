use std::net::SocketAddr;

use spinel_macros::{event_dispatcher};
use crate as spinel;

#[event_dispatcher(event: "disconnection")]
pub struct DisconnectionEvent {
        pub socket_address: SocketAddr
}

impl DisconnectionEvent {
        pub fn new(socket_address: SocketAddr) -> Self {
                Self {
                    socket_address,
                }
        }
}