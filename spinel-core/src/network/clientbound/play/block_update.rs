use spinel_macros::packet;
use spinel_network::types::{Position, VarInt};

#[packet(id: "block_update", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct BlockUpdatePacket {
    pub block_position: Position,
    pub block_state: VarInt,
}

impl BlockUpdatePacket {
    pub fn new(block_position: Position, block_state: i32) -> Self {
        Self {
            block_position,
            block_state,
        }
    }
}
