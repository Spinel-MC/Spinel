use crate as spinel;
use spinel_macros::packet_dispatcher;
use spinel_nbt::{Nbt, NbtCompound};
use spinel_network::types::{
    chunk::{ChunkData, ChunkSection, PalettedContainer},
    light::LightData,
};

#[packet_dispatcher(id: "level_chunk_with_light", state: ConnectionState::Play)]
pub struct ChunkDataAndUpdateLightPacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub chunk_data: ChunkData,
    pub light_data: LightData,
}

impl ChunkDataAndUpdateLightPacket {
    pub fn new_stub(chunk_x: i32, chunk_z: i32) -> Self {
        let stone_id = 1;
        let block_states = PalettedContainer {
            bits_per_entry: 0,
            palette: Some(vec![stone_id]),
            data: vec![],
        };

        let plains_id = 1;
        let biomes = PalettedContainer {
            bits_per_entry: 0,
            palette: Some(vec![plains_id]),
            data: vec![],
        };

        let chunk_section = ChunkSection {
            block_count: 16 * 16 * 16,
            block_states,
            biomes,
        };

        let mut sections = vec![chunk_section];
        for _ in 0..23 {
            sections.push(ChunkSection::empty());
        }

        let mut heightmaps = NbtCompound::new();
        let heightmap_data = vec![0; 37];
        heightmaps.insert(
            "MOTION_BLOCKING".to_string(),
            Nbt::LongArray(heightmap_data.clone().into_boxed_slice()),
        );
        heightmaps.insert(
            "WORLD_SURFACE".to_string(),
            Nbt::LongArray(heightmap_data.into_boxed_slice()),
        );

        let chunk_data = ChunkData {
            heightmaps,
            sections,
            block_entities: vec![],
        };

        let light_data = LightData {
            sky_light_mask: vec![],
            block_light_mask: vec![],
            empty_sky_light_mask: vec![],
            empty_block_light_mask: vec![],
            sky_light_arrays: vec![],
            block_light_arrays: vec![],
        };

        Self {
            chunk_x,
            chunk_z,
            chunk_data,
            light_data,
        }
    }
}
