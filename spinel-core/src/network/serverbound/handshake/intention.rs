use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(
    id: "intention",
    state: ConnectionState::Handshaking,
    recipient: Recipient::Server,
)]
pub struct IntentionPacket {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub intention: VarInt,
}
