use crate::world::{BlockPosition, Chunk, ChunkPosition, ChunkSection};

#[derive(Clone)]
pub struct GenerationFork {
    pub start: BlockPosition,
    pub end: BlockPosition,
    pub sections: Vec<ChunkSection>,
}

impl GenerationFork {
    pub fn new(start: BlockPosition, end: BlockPosition, sections: Vec<ChunkSection>) -> Self {
        Self {
            start,
            end,
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
        self.sections.iter().for_each(|fork_section| {
            if let Some(chunk_section) = chunk
                .sections_mut()
                .iter_mut()
                .find(|chunk_section| chunk_section.y == fork_section.y)
            {
                *chunk_section = fork_section.clone();
            }
        });
    }
}
