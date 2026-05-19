use spinel_macros::packet;
use spinel_network::types::{chunk::ChunkData, light::LightData};

#[packet(id: "level_chunk_with_light", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct ChunkDataAndUpdateLightPacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub chunk_data: ChunkData,
    pub light_data: LightData,
}

impl ChunkDataAndUpdateLightPacket {
    pub fn with_light_data(
        chunk_x: i32,
        chunk_z: i32,
        chunk_data: ChunkData,
        light_data: LightData,
    ) -> Self {
        Self {
            chunk_x,
            chunk_z,
            chunk_data,
            light_data,
        }
    }

    pub fn new(chunk_x: i32, chunk_z: i32, chunk_data: ChunkData) -> Self {
        let light_data = LightData {
            sky_light_mask: Self::full_light_mask(),
            block_light_mask: Self::full_light_mask(),
            empty_sky_light_mask: vec![0],
            empty_block_light_mask: vec![0],
            sky_light_arrays: Self::filled_light_arrays(),
            block_light_arrays: Self::filled_light_arrays(),
        };

        Self::with_light_data(chunk_x, chunk_z, chunk_data, light_data)
    }

    fn full_light_mask() -> Vec<i64> {
        vec![(1i64 << 26) - 1]
    }

    fn filled_light_arrays() -> Vec<Vec<u8>> {
        (0..26).map(|_| vec![255; 2048]).collect()
    }
}
