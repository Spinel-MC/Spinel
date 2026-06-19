use spinel_macros::packet;
use spinel_network::types::{Position, VarInt};

#[packet(id: "jigsaw_generate", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct JigsawGeneratePacket {
    pub position: Position,
    pub levels: VarInt,
    pub keep_jigsaws: bool,
}
