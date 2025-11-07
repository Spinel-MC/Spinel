use crate as spinel;
use spinel_macros::packet_dispatcher;

#[packet_dispatcher(id: 0x01)]
pub struct PongResponsePacket {
    pub timestamp: i64,
}

impl PongResponsePacket {
    pub fn new(timestamp: i64) -> Self {
        Self { timestamp }
    }
}
