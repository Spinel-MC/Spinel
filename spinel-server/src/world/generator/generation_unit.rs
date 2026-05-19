use crate::world::generator::{DynamicFork, GenerationFork, UnitModifier, UnitWriteMode};
use crate::world::{BlockPosition, BlockSize, ChunkSection, SectionPosition};

pub struct GenerationUnit {
    size: BlockSize,
    absolute_start: BlockPosition,
    absolute_end: BlockPosition,
    section_positions: Vec<SectionPosition>,
    sections: Vec<ChunkSection>,
    forks: Vec<GenerationFork>,
    write_mode: UnitWriteMode,
}

impl GenerationUnit {
    pub fn new(
        size: BlockSize,
        absolute_start: BlockPosition,
        sections: Vec<ChunkSection>,
    ) -> Self {
        let absolute_end = BlockPosition::new(
            absolute_start.x + size.x,
            absolute_start.y + size.y,
            absolute_start.z + size.z,
        );
        let section_positions = Self::section_positions_between(absolute_start, absolute_end);
        Self::from_sections(
            size,
            absolute_start,
            absolute_end,
            section_positions,
            sections,
            UnitWriteMode::Chunk,
        )
    }

    pub fn modifier(&mut self) -> UnitModifier<'_> {
        UnitModifier::new(
            &self.section_positions,
            &mut self.sections,
            self.size,
            self.absolute_start,
            self.absolute_end,
            self.write_mode,
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

    pub fn fork(
        &mut self,
        start: BlockPosition,
        end: BlockPosition,
        generate_fork: impl FnOnce(&mut GenerationUnit),
    ) {
        let aligned_start = align_start(start);
        let aligned_end = align_end(end);
        let mut fork = Self::new_fork(aligned_start, aligned_end);
        generate_fork(&mut fork);
        let (section_positions, sections, forks) = fork.into_fork_generation();
        self.forks.push(GenerationFork::new(
            aligned_start,
            aligned_end,
            section_positions,
            sections,
        ));
        self.forks.extend(forks);
    }

    pub fn fork_blocks(&mut self, generate_fork: impl FnOnce(&mut DynamicFork)) {
        let mut dynamic_fork = DynamicFork::new();
        generate_fork(&mut dynamic_fork);
        let Some(block_writes) = dynamic_fork.into_block_writes() else {
            return;
        };
        self.fork(block_writes.start, block_writes.end, |fork| {
            block_writes
                .block_writes
                .into_iter()
                .for_each(|(position, block)| fork.modifier().set_block(position, block));
        });
    }

    pub fn subdivide(&self) -> Vec<GenerationUnit> {
        self.section_positions
            .iter()
            .zip(self.sections.iter())
            .map(|(section_position, section)| {
                Self::section_unit(*section_position, section, self.write_mode)
            })
            .collect()
    }

    pub fn sections(&self) -> Vec<SectionPosition> {
        self.section_positions.clone()
    }

    pub(crate) fn into_generation(self) -> (Vec<ChunkSection>, Vec<GenerationFork>) {
        (self.sections, self.forks)
    }

    fn new_fork(start: BlockPosition, end: BlockPosition) -> Self {
        let size = BlockSize::new(end.x - start.x, end.y - start.y, end.z - start.z);
        let section_positions = Self::section_positions_between(start, end);
        let sections = section_positions
            .iter()
            .map(|section_position| ChunkSection::new_sparse(section_position.y))
            .collect();
        Self::from_sections(
            size,
            start,
            end,
            section_positions,
            sections,
            UnitWriteMode::Fork,
        )
    }

    fn section_unit(
        section_position: SectionPosition,
        section: &ChunkSection,
        write_mode: UnitWriteMode,
    ) -> GenerationUnit {
        let section_start = BlockPosition::new(
            section_position.x << 4,
            section_position.y << 4,
            section_position.z << 4,
        );
        Self::from_sections(
            BlockSize::new(16, 16, 16),
            section_start,
            BlockPosition::new(
                section_start.x + 16,
                section_start.y + 16,
                section_start.z + 16,
            ),
            vec![section_position],
            vec![section.clone()],
            write_mode,
        )
    }

    fn into_fork_generation(
        self,
    ) -> (Vec<SectionPosition>, Vec<ChunkSection>, Vec<GenerationFork>) {
        (self.section_positions, self.sections, self.forks)
    }

    fn from_sections(
        size: BlockSize,
        absolute_start: BlockPosition,
        absolute_end: BlockPosition,
        section_positions: Vec<SectionPosition>,
        sections: Vec<ChunkSection>,
        write_mode: UnitWriteMode,
    ) -> Self {
        Self {
            size,
            absolute_start,
            absolute_end,
            section_positions,
            sections,
            forks: Vec::new(),
            write_mode,
        }
    }

    fn section_positions_between(start: BlockPosition, end: BlockPosition) -> Vec<SectionPosition> {
        let min_section = SectionPosition::new(
            start.x.div_euclid(16),
            start.y.div_euclid(16),
            start.z.div_euclid(16),
        );
        let max_section = SectionPosition::new(
            (end.x - 1).div_euclid(16),
            (end.y - 1).div_euclid(16),
            (end.z - 1).div_euclid(16),
        );
        (min_section.x..=max_section.x)
            .flat_map(|section_x| {
                (min_section.z..=max_section.z).flat_map(move |section_z| {
                    (min_section.y..=max_section.y)
                        .map(move |section_y| SectionPosition::new(section_x, section_y, section_z))
                })
            })
            .collect()
    }
}

fn align_start(position: BlockPosition) -> BlockPosition {
    BlockPosition::new(
        position.x.div_euclid(16) * 16,
        position.y.div_euclid(16) * 16,
        position.z.div_euclid(16) * 16,
    )
}

fn align_end(position: BlockPosition) -> BlockPosition {
    BlockPosition::new(
        ceil_section(position.x),
        ceil_section(position.y),
        ceil_section(position.z),
    )
}

fn ceil_section(value: i32) -> i32 {
    let section = value.div_euclid(16);
    if value.rem_euclid(16) == 0 {
        section * 16
    } else {
        (section + 1) * 16
    }
}
