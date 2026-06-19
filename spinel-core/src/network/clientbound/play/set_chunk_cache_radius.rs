use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(
    id: "set_chunk_cache_radius",
    state: ConnectionState::Play,
    recipient: Recipient::Client
)]
pub struct SetChunkCacheRadiusPacket {
    pub radius: VarInt,
}
