use spinel_macros::packet;
use spinel_network::types::{VarInt, light::LightData};

#[packet(id: "light_update", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct LightUpdatePacket {
    pub chunk_x: VarInt,
    pub chunk_z: VarInt,
    pub light_data: LightData,
}

impl LightUpdatePacket {
    pub fn new(chunk_x: i32, chunk_z: i32, light_data: LightData) -> Self {
        Self {
            chunk_x,
            chunk_z,
            light_data,
        }
    }
}
