use crate as spinel;
use spinel_macros::packet_dispatcher;
use spinel_network::types::var_int::VarInt;

#[packet_dispatcher(id: "login_compression", state: ConnectionState::Login)]
pub struct SetCompressionPacket {
    pub threshold: VarInt,
}

impl SetCompressionPacket {
    pub fn new(threshold: i32) -> Self {
        Self {
            threshold: VarInt(threshold),
        }
    }
}
