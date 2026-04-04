use spinel_macros::packet;
use spinel_network::types::Array;

#[packet(
    id: "custom_payload",
    state: ConnectionState::Configuration,
    recipient: Recipient::Server
)]
pub struct PluginMessagePacket {
    pub channel: String,
    pub data: Array<u8>,
}
