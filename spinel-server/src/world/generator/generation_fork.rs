use crate::world::{BlockPosition, Chunk, ChunkPosition, ChunkSection, SectionPosition};

#[derive(Clone)]
pub struct GenerationFork {
    pub start: BlockPosition,
    pub end: BlockPosition,
    pub section_positions: Vec<SectionPosition>,
    pub sections: Vec<ChunkSection>,
}

impl GenerationFork {
    pub fn new(
        start: BlockPosition,
        end: BlockPosition,
        section_positions: Vec<SectionPosition>,
        sections: Vec<ChunkSection>,
    ) -> Self {
        Self {
            start,
            end,
            section_positions,
            sections,
        }
    }

    pub(crate) fn target_positions(&self) -> Vec<ChunkPosition> {
        let min_x = self.start.x.div_euclid(16);
        let max_x = (self.end.x - 1).div_euclid(16);
        let min_z = self.start.z.div_euclid(16);
        let max_z = (self.end.z - 1).div_euclid(16);
        (min_x..=max_x)
            .flat_map(|x| (min_z..=max_z).map(move |z| ChunkPosition::new(x, z)))
            .collect()
    }

    pub(crate) fn apply_to(&self, chunk: &mut Chunk) {
        let chunk_x = chunk.x();
        let chunk_z = chunk.z();
        self.section_positions
            .iter()
            .zip(self.sections.iter())
            .filter(|(section_position, _)| {
                section_position.x == chunk_x && section_position.z == chunk_z
            })
            .for_each(|(section_position, fork_section)| {
                chunk.merge_section_from_fork(section_position.y, fork_section);
            });
    }
}
