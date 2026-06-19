use spinel_macros::packet;
use spinel_network::types::{Position, VarInt};

#[packet(
    id: "block_entity_tag_query",
    state: ConnectionState::Play,
    recipient: Recipient::Server
)]
pub struct BlockEntityTagQueryPacket {
    pub transaction_id: VarInt,
    pub pos: Position,
}
