use spinel_macros::packet_dispatcher;
use spinel_network::types::var_int::VarInt;
use crate as spinel;

#[packet_dispatcher(id: 0x03, state: ConnectionState::Login)]
pub struct SetCompressionPacket {
    pub threshold: VarInt,
}

impl SetCompressionPacket {
    pub fn new(threshold: i32) -> Self {
        Self { threshold: VarInt(threshold) }
    }
}