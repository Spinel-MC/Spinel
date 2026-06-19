use spinel_macros::packet;
use spinel_network::types::PackedMessageSignature;

#[packet(id: "delete_chat", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct DeleteChatPacket {
    pub message_signature: PackedMessageSignature,
}
