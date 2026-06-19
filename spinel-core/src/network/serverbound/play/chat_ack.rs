use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "chat_ack", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct ChatAckPacket {
    pub offset: VarInt,
}
