use spinel_macros::packet;
use spinel_network::types::{Position, VarInt};

#[packet(id: "block_destruction", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct BlockDestructionPacket {
    pub id: VarInt,
    pub pos: Position,
    pub progress: u8,
}
