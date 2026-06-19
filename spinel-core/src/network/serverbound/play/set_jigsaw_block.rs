use spinel_macros::packet;
use spinel_network::types::{Position, VarInt};
use spinel_registry::Identifier;

#[packet(
    id: "set_jigsaw_block",
    state: ConnectionState::Play,
    recipient: Recipient::Server
)]
pub struct SetJigsawBlockPacket {
    pub position: Position,
    pub name: Identifier,
    pub target: Identifier,
    pub pool: Identifier,
    pub final_state: String,
    pub joint: String,
    pub selection_priority: VarInt,
    pub placement_priority: VarInt,
}
