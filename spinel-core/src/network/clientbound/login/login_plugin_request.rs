use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "custom_query", state: ConnectionState::Login, recipient: Recipient::Client)]
pub struct LoginPluginRequestPacket {
    pub message_id: VarInt,
    pub channel: String,
    pub data: Vec<u8>,
}
