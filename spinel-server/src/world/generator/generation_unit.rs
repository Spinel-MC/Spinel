use crate::world::generator::{GenerationFork, UnitModifier};
use crate::world::{BlockPosition, BlockSize, ChunkSection, SectionPosition};

pub struct GenerationUnit {
    size: BlockSize,
    absolute_start: BlockPosition,
    absolute_end: BlockPosition,
    sections: Vec<ChunkSection>,
    forks: Vec<GenerationFork>,
}

impl GenerationUnit {
    pub fn new(
        size: BlockSize,
        absolute_start: BlockPosition,
        sections: Vec<ChunkSection>,
    ) -> Self {
        Self {
            size,
            absolute_start,
            absolute_end: BlockPosition::new(
                absolute_start.x + size.x,
                absolute_start.y + size.y,
                absolute_start.z + size.z,
            ),
            sections,
            forks: Vec::new(),
        }
    }

    pub fn modifier(&mut self) -> UnitModifier<'_> {
        UnitModifier::new(
            &mut self.sections,
            self.size,
            self.absolute_start,
            self.absolute_end,
        )
    }

    pub fn size(&self) -> BlockSize {
        self.size
    }

    pub fn absolute_start(&self) -> BlockPosition {
        self.absolute_start
    }

    pub fn absolute_end(&self) -> BlockPosition {
        self.absolute_end
    }

    pub fn fork(&mut self, start: BlockPosition, end: BlockPosition) -> GenerationUnit {
        let size = BlockSize::new(end.x - start.x, end.y - start.y, end.z - start.z);
        let sections = Self::sections_for(size, start.y);
        self.forks
            .push(GenerationFork::new(start, end, sections.clone()));
        GenerationUnit::new(size, start, sections)
    }

    pub fn subdivide(&self) -> Vec<GenerationUnit> {
        self.sections
            .iter()
            .map(|section| Self::section_unit(section, self.absolute_start))
            .collect()
    }

    pub fn sections(&self) -> Vec<SectionPosition> {
        self.sections
            .iter()
            .map(|section| {
                SectionPosition::new(
                    self.absolute_start.x >> 4,
                    section.y,
                    self.absolute_start.z >> 4,
                )
            })
            .collect()
    }

    pub(crate) fn into_generation(self) -> (Vec<ChunkSection>, Vec<GenerationFork>) {
        (self.sections, self.forks)
    }

    fn section_unit(section: &ChunkSection, start: BlockPosition) -> GenerationUnit {
        let section_start = BlockPosition::new(start.x, section.y << 4, start.z);
        GenerationUnit::new(
            BlockSize::new(16, 16, 16),
            section_start,
            vec![section.clone()],
        )
    }

    fn sections_for(size: BlockSize, min_y: i32) -> Vec<ChunkSection> {
        (0..(size.y >> 4))
            .map(|section_offset| ChunkSection::new((min_y >> 4) + section_offset))
            .collect()
    }
}
