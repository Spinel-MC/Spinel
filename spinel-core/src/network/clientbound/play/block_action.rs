use spinel_macros::packet;
use spinel_network::types::{Position, VarInt};

#[packet(id: "block_event", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct BlockActionPacket {
    pub block_position: Position,
    pub action_id: u8,
    pub action_param: u8,
    pub block_type: VarInt,
}

impl BlockActionPacket {
    pub fn new(block_position: Position, action_id: u8, action_param: u8, block_type: i32) -> Self {
        Self {
            block_position,
            action_id,
            action_param,
            block_type,
        }
    }
}
