use spinel_macros::packet;
use spinel_network::types::{
    chunk::{ChunkData, ChunkSection, HeightmapEntry},
    light::LightData,
};

#[packet(id: "level_chunk_with_light", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct ChunkDataAndUpdateLightPacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub chunk_data: ChunkData,
    pub light_data: LightData,
}

impl ChunkDataAndUpdateLightPacket {
    pub fn new_stub(chunk_x: i32, chunk_z: i32) -> Self {
        let light_data = LightData {
            sky_light_mask: Self::full_light_mask(),
            block_light_mask: Self::full_light_mask(),
            empty_sky_light_mask: vec![0],
            empty_block_light_mask: vec![0],
            sky_light_arrays: Self::filled_light_arrays(),
            block_light_arrays: Self::filled_light_arrays(),
        };

        Self {
            chunk_x,
            chunk_z,
            chunk_data: Self::stub_chunk_data(),
            light_data,
        }
    }

    fn stub_chunk_data() -> ChunkData {
        ChunkData {
            heightmaps: Self::stub_heightmaps(),
            sections: (0..24).map(|_| ChunkSection::empty()).collect(),
            block_entities: vec![],
        }
    }

    fn stub_heightmaps() -> Vec<HeightmapEntry> {
        let packed_heightmap_entries = vec![0; 37];
        vec![
            HeightmapEntry {
                kind: 1,
                data: packed_heightmap_entries.clone(),
            },
            HeightmapEntry {
                kind: 4,
                data: packed_heightmap_entries.clone(),
            },
            HeightmapEntry {
                kind: 5,
                data: packed_heightmap_entries,
            },
        ]
    }

    fn full_light_mask() -> Vec<i64> {
        vec![(1i64 << 26) - 1]
    }

    fn filled_light_arrays() -> Vec<Vec<u8>> {
        (0..26).map(|_| vec![255; 2048]).collect()
    }
}
