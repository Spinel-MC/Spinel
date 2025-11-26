use crate as spinel;
use spinel_macros::packet_dispatcher;
use spinel_network::types::var_int::VarInt;

#[packet_dispatcher(id: "transfer", state: ConnectionState::Configuration)]
pub struct TransferPacket {
    pub host: String,
    pub port: VarInt,
}
