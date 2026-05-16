use crate::world::{ChunkPosition, ChunkSection};
use spinel_network::types::chunk::{ChunkData, HeightmapEntry};

const WORLD_MIN_SECTION: i32 = -4;
const WORLD_SECTION_COUNT: i32 = 24;

pub struct Chunk {
    position: ChunkPosition,
    sections: Vec<ChunkSection>,
    has_generated: bool,
}

impl Chunk {
    pub fn new(position: ChunkPosition) -> Self {
        Self {
            position,
            sections: (0..WORLD_SECTION_COUNT)
                .map(|section_offset| ChunkSection::new(WORLD_MIN_SECTION + section_offset))
                .collect(),
            has_generated: false,
        }
    }

    pub fn x(&self) -> i32 {
        self.position.x
    }

    pub fn z(&self) -> i32 {
        self.position.z
    }

    pub fn sections(&self) -> &[ChunkSection] {
        &self.sections
    }

    pub fn data(&self) -> ChunkData {
        ChunkData {
            heightmaps: Self::heightmaps(),
            sections: self
                .sections
                .iter()
                .map(ChunkSection::to_network_section)
                .collect(),
            block_entities: Vec::new(),
        }
    }

    pub(crate) fn sections_mut(&mut self) -> &mut [ChunkSection] {
        &mut self.sections
    }

    pub(crate) fn replace_sections(&mut self, sections: Vec<ChunkSection>) {
        self.sections = sections;
    }

    pub(crate) fn should_generate(&self) -> bool {
        !self.has_generated
    }

    pub(crate) fn mark_generated(&mut self) {
        self.has_generated = true;
    }

    fn heightmaps() -> Vec<HeightmapEntry> {
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
}
