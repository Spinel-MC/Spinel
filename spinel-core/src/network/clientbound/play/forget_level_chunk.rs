use spinel_macros::packet;
use spinel_network::types::ChunkPos;

#[packet(id: "forget_level_chunk", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct ForgetLevelChunkPacket {
    pub pos: ChunkPos,
}

impl ForgetLevelChunkPacket {
    pub const fn new(pos: ChunkPos) -> Self {
        Self { pos }
    }
}
