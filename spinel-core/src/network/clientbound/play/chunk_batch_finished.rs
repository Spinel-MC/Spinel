use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "chunk_batch_finished", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct ChunkBatchFinishedPacket {
    pub batch_size: VarInt,
}

impl ChunkBatchFinishedPacket {
    pub const fn new(batch_size: i32) -> Self {
        Self { batch_size }
    }
}
