use crate::entity::player::chunks::PlayerChunk;
use spinel_core::network::clientbound::play::chunk_data::ChunkDataAndUpdateLightPacket;

pub(crate) struct QueuedPlayerChunk {
    pub(crate) chunk: PlayerChunk,
    pub(in crate::entity::player) packet: Option<ChunkDataAndUpdateLightPacket>,
}

impl QueuedPlayerChunk {
    pub(in crate::entity::player) fn new(packet: ChunkDataAndUpdateLightPacket) -> Self {
        Self {
            chunk: PlayerChunk::new(packet.chunk_x, packet.chunk_z),
            packet: Some(packet),
        }
    }

    pub(in crate::entity::player) fn from_chunk(chunk: PlayerChunk) -> Self {
        Self {
            chunk,
            packet: None,
        }
    }
}
