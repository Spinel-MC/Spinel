use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "set_chunk_cache_center", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct SetChunkCacheCenterPacket {
    pub x: VarInt,
    pub z: VarInt,
}

impl SetChunkCacheCenterPacket {
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }
}
