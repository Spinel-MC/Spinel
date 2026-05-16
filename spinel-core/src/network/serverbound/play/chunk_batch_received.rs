use spinel_macros::packet;

#[packet(id: "chunk_batch_received", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct ChunkBatchReceivedPacket {
    pub desired_chunks_per_tick: f32,
}
