use crate as spinel;
use spinel_macros::packet_dispatcher;
use spinel_network::types::var_int::VarInt;

#[packet_dispatcher(id: "custom_query", state: ConnectionState::Login)]
pub struct LoginPluginRequestPacket {
    pub message_id: VarInt,
    pub channel: String,
    pub data: Vec<u8>,
}
