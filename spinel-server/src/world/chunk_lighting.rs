use crate::world::ChunkSection;
use spinel_network::types::light::LightData;

pub(crate) struct ChunkLighting {
    sky_light_arrays: Vec<Vec<u8>>,
    block_light_arrays: Vec<Vec<u8>>,
}

impl ChunkLighting {
    pub(crate) fn empty_data(section_count: usize) -> LightData {
        let empty_light_mask =
            (1..=section_count).fold(0i64, |mask, section_index| mask | (1i64 << section_index));
        LightData {
            sky_light_mask: Vec::new(),
            block_light_mask: Vec::new(),
            empty_sky_light_mask: Self::network_mask(empty_light_mask),
            empty_block_light_mask: Self::network_mask(empty_light_mask),
            sky_light_arrays: Vec::new(),
            block_light_arrays: Vec::new(),
        }
    }

    pub(crate) fn from_sections(sections: &[ChunkSection]) -> Self {
        Self {
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
        let mut sky_light_mask = 0i64;
        let mut block_light_mask = 0i64;
        let mut empty_sky_light_mask = 0i64;
        let mut empty_block_light_mask = 0i64;
        let mut sky_light_arrays = Vec::new();
        let mut block_light_arrays = Vec::new();
        self.sky_light_arrays
            .iter()
            .zip(&self.block_light_arrays)
            .enumerate()
            .for_each(|(section_index, (sky_light, block_light))| {
                let mask = 1i64 << (section_index + 1);
                Self::append_section(
                    sky_light,
                    mask,
                    &mut sky_light_mask,
                    &mut empty_sky_light_mask,
                    &mut sky_light_arrays,
                );
                Self::append_section(
                    block_light,
                    mask,
                    &mut block_light_mask,
                    &mut empty_block_light_mask,
                    &mut block_light_arrays,
                );
            });
        LightData {
            sky_light_mask: Self::network_mask(sky_light_mask),
            block_light_mask: Self::network_mask(block_light_mask),
            empty_sky_light_mask: Self::network_mask(empty_sky_light_mask),
            empty_block_light_mask: Self::network_mask(empty_block_light_mask),
            sky_light_arrays,
            block_light_arrays,
        }
    }

    pub(crate) fn partial_data(sections: &mut [ChunkSection]) -> Option<LightData> {
        let mut sky_light_mask = 0i64;
        let mut block_light_mask = 0i64;
        let mut empty_sky_light_mask = 0i64;
        let mut empty_block_light_mask = 0i64;
        let mut sky_light_arrays = Vec::new();
        let mut block_light_arrays = Vec::new();
        sections
            .iter_mut()
            .enumerate()
            .for_each(|(section_index, section)| {
                let mask = 1i64 << (section_index + 1);
                if section.sky_light_needs_send() {
                    if section.sky_light_is_invalidated() {
                        section.relight_sky_light();
                    }
                    Self::append_section(
                        section.sky_light(),
                        mask,
                        &mut sky_light_mask,
                        &mut empty_sky_light_mask,
                        &mut sky_light_arrays,
                    );
                    section.consume_sky_light_send();
                }
                if section.block_light_needs_send() {
                    if section.block_light_is_invalidated() {
                        section.relight_block_light();
                    }
                    Self::append_section(
                        section.block_light(),
                        mask,
                        &mut block_light_mask,
                        &mut empty_block_light_mask,
                        &mut block_light_arrays,
                    );
                    section.consume_block_light_send();
                }
            });
        let has_updates = sky_light_mask != 0
            || block_light_mask != 0
            || empty_sky_light_mask != 0
            || empty_block_light_mask != 0;
        has_updates.then_some(LightData {
            sky_light_mask: Self::network_mask(sky_light_mask),
            block_light_mask: Self::network_mask(block_light_mask),
            empty_sky_light_mask: Self::network_mask(empty_sky_light_mask),
            empty_block_light_mask: Self::network_mask(empty_block_light_mask),
            sky_light_arrays,
            block_light_arrays,
        })
    }

    fn append_section(
        light: &[u8],
        mask: i64,
        light_mask: &mut i64,
        empty_light_mask: &mut i64,
        light_arrays: &mut Vec<Vec<u8>>,
    ) {
        if light.iter().all(|light_level| *light_level == 0) {
            *empty_light_mask |= mask;
            return;
        }
        *light_mask |= mask;
        light_arrays.push(light.to_vec());
    }

    fn network_mask(mask: i64) -> Vec<i64> {
        if mask == 0 {
            return Vec::new();
        }
        vec![mask]
    }
}
