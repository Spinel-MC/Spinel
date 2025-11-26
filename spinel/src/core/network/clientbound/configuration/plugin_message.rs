use crate as spinel;
use spinel_macros::packet_dispatcher;

#[packet_dispatcher(id: "custom_payload", state: ConnectionState::Configuration)]
pub struct PluginMessagePacket {
    pub channel: String,
    pub data: Vec<u8>,
}
