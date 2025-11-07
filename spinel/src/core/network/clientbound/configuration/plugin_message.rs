use crate as spinel;
use spinel_macros::packet_dispatcher;

#[packet_dispatcher(id: 0x01)]
pub struct PluginMessagePacket {
    pub channel: String,
    pub data: Vec<u8>,
}
