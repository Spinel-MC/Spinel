use spinel_macros::packet;

#[packet(id: "chunk_batch_start", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct ChunkBatchStartPacket;
