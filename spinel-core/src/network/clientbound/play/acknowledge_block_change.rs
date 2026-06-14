use spinel_macros::packet;
use spinel_network::VarInt;

#[packet(id: "block_changed_ack", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct AcknowledgeBlockChangePacket {
    pub sequence: VarInt,
}
