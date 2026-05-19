use crate::world::ChunkSection;
use spinel_network::types::light::LightData;

pub(crate) struct ChunkLighting {
    section_count: usize,
    sky_light_arrays: Vec<Vec<u8>>,
    block_light_arrays: Vec<Vec<u8>>,
}

impl ChunkLighting {
    pub(crate) fn from_sections(sections: &[ChunkSection]) -> Self {
        Self {
            section_count: sections.len(),
            sky_light_arrays: sections
                .iter()
                .map(|section| section.sky_light().to_vec())
                .collect(),
            block_light_arrays: sections
                .iter()
                .map(|section| section.block_light().to_vec())
                .collect(),
        }
    }

    pub(crate) fn data(&self) -> LightData {
        LightData {
            sky_light_mask: vec![self.section_mask()],
            block_light_mask: vec![self.section_mask()],
            empty_sky_light_mask: vec![self.boundary_mask()],
            empty_block_light_mask: vec![self.boundary_mask()],
            sky_light_arrays: self.sky_light_arrays.clone(),
            block_light_arrays: self.block_light_arrays.clone(),
        }
    }

    fn section_mask(&self) -> i64 {
        (((1i64 << self.section_count) - 1) << 1) as i64
    }

    fn boundary_mask(&self) -> i64 {
        (1i64 << 0) | (1i64 << (self.section_count + 1))
    }
}
