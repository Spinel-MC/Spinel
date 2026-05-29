use crate::world::{
    Biome, Block, BlockPosition, BlockSize, ChunkSection, RelativeBlockPosition, SectionPosition,
};
use spinel_registry::RegistryKey;

#[derive(Clone, Copy)]
pub(crate) enum UnitWriteMode {
    Chunk,
    Fork,
}

pub struct UnitModifier<'a> {
    section_positions: &'a [SectionPosition],
    sections: &'a mut [ChunkSection],
    size: BlockSize,
    absolute_start: BlockPosition,
    absolute_end: BlockPosition,
    write_mode: UnitWriteMode,
}

impl<'a> UnitModifier<'a> {
    pub(crate) fn new(
        section_positions: &'a [SectionPosition],
        sections: &'a mut [ChunkSection],
        size: BlockSize,
        absolute_start: BlockPosition,
        absolute_end: BlockPosition,
        write_mode: UnitWriteMode,
    ) -> Self {
        Self {
            section_positions,
            sections,
            size,
            absolute_start,
            absolute_end,
            write_mode,
        }
    }

    pub fn set_block(&mut self, position: BlockPosition, block: Block) {
        let _ = self.try_set_block(position, block);
    }

    pub fn try_set_block(
        &mut self,
        position: BlockPosition,
        block: Block,
    ) -> Result<(), UnitWriteError> {
        self.require_contains(position)?;
        self.set_contained_block(position, block)
    }

    pub fn set_relative(&mut self, position: RelativeBlockPosition, block: Block) {
        let _ = self.try_set_relative(position, block);
    }

    pub fn try_set_relative(
        &mut self,
        position: RelativeBlockPosition,
        block: Block,
    ) -> Result<(), UnitWriteError> {
        self.require_contains_relative(position)?;
        let absolute_position = BlockPosition::new(
            self.absolute_start.x + position.x,
            self.absolute_start.y + position.y,
            self.absolute_start.z + position.z,
        );
        self.set_contained_block(absolute_position, block)
    }

    pub fn set_all(&mut self, mut supplier: impl FnMut(i32, i32, i32) -> Block) {
        let _ = self.try_set_all(&mut supplier);
    }

    pub fn try_set_all(
        &mut self,
        supplier: &mut impl FnMut(i32, i32, i32) -> Block,
    ) -> Result<(), UnitWriteError> {
        for section_index in 0..self.section_positions.len() {
            let section_position = self.section_positions[section_index];
            self.set_all_in_section(section_index, section_position, supplier);
        }
        Ok(())
    }

    pub fn set_all_relative(&mut self, mut supplier: impl FnMut(i32, i32, i32) -> Block) {
        let _ = self.try_set_all_relative(&mut supplier);
    }

    pub fn try_set_all_relative(
        &mut self,
        supplier: &mut impl FnMut(i32, i32, i32) -> Block,
    ) -> Result<(), UnitWriteError> {
        for section_index in 0..self.section_positions.len() {
            let section_position = self.section_positions[section_index];
            self.set_all_relative_in_section(section_index, section_position, supplier);
        }
        Ok(())
    }

    pub fn fill(&mut self, block: Block) {
        self.sections
            .iter_mut()
            .for_each(|section| section.fill_block(block));
    }

    pub fn fill_area(&mut self, start: BlockPosition, end: BlockPosition, block: Block) {
        let _ = self.try_fill_area(start, end, block);
    }

    pub fn fill_height(&mut self, min_height: i32, max_height: i32, block: Block) {
        let start = BlockPosition::new(self.absolute_start.x, min_height, self.absolute_start.z);
        let end = BlockPosition::new(self.absolute_end.x, max_height, self.absolute_end.z);
        self.fill_area(start, end, block);
    }

    pub fn try_fill_area(
        &mut self,
        start: BlockPosition,
        end: BlockPosition,
        block: Block,
    ) -> Result<(), UnitWriteError> {
        self.require_contains(start)?;
        self.require_contains(BlockPosition::new(end.x - 1, end.y - 1, end.z - 1))?;
        for section_index in 0..self.section_positions.len() {
            let section_position = self.section_positions[section_index];
            self.fill_section_area(section_index, section_position, start, end, block)?;
        }
        Ok(())
    }

    pub fn set_biome(&mut self, position: BlockPosition, biome: RegistryKey<Biome>) {
        let _ = self.try_set_biome(position, biome);
    }

    pub fn try_set_biome(
        &mut self,
        position: BlockPosition,
        biome: RegistryKey<Biome>,
    ) -> Result<(), UnitWriteError> {
        self.require_can_write_biomes()?;
        self.require_contains(position)?;
        self.set_contained_biome(position, biome)
    }

    pub fn set_relative_biome(
        &mut self,
        position: RelativeBlockPosition,
        biome: RegistryKey<Biome>,
    ) {
        let _ = self.try_set_relative_biome(position, biome);
    }

    pub fn try_set_relative_biome(
        &mut self,
        position: RelativeBlockPosition,
        biome: RegistryKey<Biome>,
    ) -> Result<(), UnitWriteError> {
        self.require_can_write_biomes()?;
        self.require_contains_relative(position)?;
        let absolute_position = BlockPosition::new(
            self.absolute_start.x + position.x,
            self.absolute_start.y + position.y,
            self.absolute_start.z + position.z,
        );
        self.set_contained_biome(absolute_position, biome)
    }

    pub fn fill_biome(&mut self, biome: RegistryKey<Biome>) {
        let _ = self.try_fill_biome(biome);
    }

    pub fn try_fill_biome(&mut self, biome: RegistryKey<Biome>) -> Result<(), UnitWriteError> {
        self.require_can_write_biomes()?;
        self.sections
            .iter_mut()
            .for_each(|section| section.fill_biome(biome.clone()));
        Ok(())
    }

    pub fn fill_biome_area(
        &mut self,
        start: BlockPosition,
        end: BlockPosition,
        biome: RegistryKey<Biome>,
    ) {
        let _ = self.try_fill_biome_area(start, end, biome);
    }

    pub fn try_fill_biome_area(
        &mut self,
        start: BlockPosition,
        end: BlockPosition,
        biome: RegistryKey<Biome>,
    ) -> Result<(), UnitWriteError> {
        self.require_can_write_biomes()?;
        self.require_contains(start)?;
        self.require_contains(BlockPosition::new(end.x - 1, end.y - 1, end.z - 1))?;
        for block_x in start.x..end.x {
            for block_y in start.y..end.y {
                for block_z in start.z..end.z {
                    self.set_contained_biome(
                        BlockPosition::new(block_x, block_y, block_z),
                        biome.clone(),
                    )?;
                }
            }
        }
        Ok(())
    }

    pub fn fill_section_biome(&mut self, section_y: i32, biome: RegistryKey<Biome>) {
        let _ = self.try_fill_section_biome(section_y, biome);
    }

    pub fn try_fill_section_biome(
        &mut self,
        section_y: i32,
        biome: RegistryKey<Biome>,
    ) -> Result<(), UnitWriteError> {
        self.require_can_write_biomes()?;
        let section_position = SectionPosition::new(
            self.absolute_start.x.div_euclid(16),
            section_y,
            self.absolute_start.z.div_euclid(16),
        );
        let Some(section_index) = self
            .section_positions
            .iter()
            .position(|stored_position| *stored_position == section_position)
        else {
            return Err(UnitWriteError::MissingSection(BlockPosition::new(
                self.absolute_start.x,
                section_y << 4,
                self.absolute_start.z,
            )));
        };
        self.sections[section_index].fill_biome(biome);
        Ok(())
    }

    fn set_contained_block(
        &mut self,
        position: BlockPosition,
        block: Block,
    ) -> Result<(), UnitWriteError> {
        let Some((section_index, local_x, local_y, local_z)) =
            self.section_index_and_local_position(position)
        else {
            return Err(UnitWriteError::MissingSection(position));
        };
        self.sections[section_index].set_block(local_x, local_y, local_z, block);
        Ok(())
    }

    fn set_contained_biome(
        &mut self,
        position: BlockPosition,
        biome: RegistryKey<Biome>,
    ) -> Result<(), UnitWriteError> {
        let Some((section_index, local_x, local_y, local_z)) =
            self.section_index_and_local_position(position)
        else {
            return Err(UnitWriteError::MissingSection(position));
        };
        self.sections[section_index].set_biome(local_x >> 2, local_y >> 2, local_z >> 2, biome);
        Ok(())
    }

    fn set_all_in_section(
        &mut self,
        section_index: usize,
        section_position: SectionPosition,
        supplier: &mut impl FnMut(i32, i32, i32) -> Block,
    ) {
        let section_start = BlockPosition::new(
            section_position.x << 4,
            section_position.y << 4,
            section_position.z << 4,
        );
        let section_end = BlockPosition::new(
            section_start.x + 16,
            section_start.y + 16,
            section_start.z + 16,
        );
        let fill_start = BlockPosition::new(
            self.absolute_start.x.max(section_start.x),
            self.absolute_start.y.max(section_start.y),
            self.absolute_start.z.max(section_start.z),
        );
        let fill_end = BlockPosition::new(
            self.absolute_end.x.min(section_end.x),
            self.absolute_end.y.min(section_end.y),
            self.absolute_end.z.min(section_end.z),
        );
        for block_x in fill_start.x..fill_end.x {
            for block_y in fill_start.y..fill_end.y {
                for block_z in fill_start.z..fill_end.z {
                    self.sections[section_index].set_block(
                        block_x - section_start.x,
                        block_y - section_start.y,
                        block_z - section_start.z,
                        supplier(block_x, block_y, block_z),
                    );
                }
            }
        }
    }

    fn set_all_relative_in_section(
        &mut self,
        section_index: usize,
        section_position: SectionPosition,
        supplier: &mut impl FnMut(i32, i32, i32) -> Block,
    ) {
        let section_start = BlockPosition::new(
            section_position.x << 4,
            section_position.y << 4,
            section_position.z << 4,
        );
        let section_end = BlockPosition::new(
            section_start.x + 16,
            section_start.y + 16,
            section_start.z + 16,
        );
        let fill_start = BlockPosition::new(
            self.absolute_start.x.max(section_start.x),
            self.absolute_start.y.max(section_start.y),
            self.absolute_start.z.max(section_start.z),
        );
        let fill_end = BlockPosition::new(
            self.absolute_end.x.min(section_end.x),
            self.absolute_end.y.min(section_end.y),
            self.absolute_end.z.min(section_end.z),
        );
        for block_x in fill_start.x..fill_end.x {
            for block_y in fill_start.y..fill_end.y {
                for block_z in fill_start.z..fill_end.z {
                    let relative_x = block_x - self.absolute_start.x;
                    let relative_y = block_y - self.absolute_start.y;
                    let relative_z = block_z - self.absolute_start.z;
                    self.sections[section_index].set_block(
                        block_x - section_start.x,
                        block_y - section_start.y,
                        block_z - section_start.z,
                        supplier(relative_x, relative_y, relative_z),
                    );
                }
            }
        }
    }

    fn fill_section_area(
        &mut self,
        section_index: usize,
        section_position: SectionPosition,
        start: BlockPosition,
        end: BlockPosition,
        block: Block,
    ) -> Result<(), UnitWriteError> {
        let section_start = BlockPosition::new(
            section_position.x << 4,
            section_position.y << 4,
            section_position.z << 4,
        );
        let section_end = BlockPosition::new(
            section_start.x + 16,
            section_start.y + 16,
            section_start.z + 16,
        );
        let fill_start = BlockPosition::new(
            start.x.max(section_start.x),
            start.y.max(section_start.y),
            start.z.max(section_start.z),
        );
        let fill_end = BlockPosition::new(
            end.x.min(section_end.x),
            end.y.min(section_end.y),
            end.z.min(section_end.z),
        );
        if fill_start.x >= fill_end.x || fill_start.y >= fill_end.y || fill_start.z >= fill_end.z {
            return Ok(());
        }
        if fill_start == section_start && fill_end == section_end {
            self.sections[section_index].fill_block(block);
            return Ok(());
        }
        for block_x in fill_start.x..fill_end.x {
            for block_y in fill_start.y..fill_end.y {
                for block_z in fill_start.z..fill_end.z {
                    self.sections[section_index].set_block(
                        block_x - section_start.x,
                        block_y - section_start.y,
                        block_z - section_start.z,
                        block,
                    );
                }
            }
        }
        Ok(())
    }

    fn section_index_and_local_position(
        &self,
        position: BlockPosition,
    ) -> Option<(usize, i32, i32, i32)> {
        let section_position = SectionPosition::new(
            position.x.div_euclid(16),
            position.y.div_euclid(16),
            position.z.div_euclid(16),
        );
        let section_index = self
            .section_positions
            .iter()
            .position(|stored_position| *stored_position == section_position)?;
        Some((
            section_index,
            position.x.rem_euclid(16),
            position.y.rem_euclid(16),
            position.z.rem_euclid(16),
        ))
    }

    fn can_write_biomes(&self) -> bool {
        matches!(self.write_mode, UnitWriteMode::Chunk)
    }

    fn require_can_write_biomes(&self) -> Result<(), UnitWriteError> {
        self.can_write_biomes()
            .then_some(())
            .ok_or(UnitWriteError::BiomeWriteInFork)
    }

    fn require_contains(&self, position: BlockPosition) -> Result<(), UnitWriteError> {
        self.contains(position)
            .then_some(())
            .ok_or(UnitWriteError::OutOfBounds {
                position,
                start: self.absolute_start,
                end: self.absolute_end,
            })
    }

    fn require_contains_relative(
        &self,
        position: RelativeBlockPosition,
    ) -> Result<(), UnitWriteError> {
        self.contains_relative(position)
            .then_some(())
            .ok_or(UnitWriteError::RelativeOutOfBounds {
                position,
                size: self.size,
            })
    }

    fn contains(&self, position: BlockPosition) -> bool {
        position.x >= self.absolute_start.x
            && position.y >= self.absolute_start.y
            && position.z >= self.absolute_start.z
            && position.x < self.absolute_end.x
            && position.y < self.absolute_end.y
            && position.z < self.absolute_end.z
    }

    fn contains_relative(&self, position: RelativeBlockPosition) -> bool {
        position.x >= 0
            && position.y >= 0
            && position.z >= 0
            && position.x < self.size.x
            && position.y < self.size.y
            && position.z < self.size.z
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitWriteError {
    OutOfBounds {
        position: BlockPosition,
        start: BlockPosition,
        end: BlockPosition,
    },
    RelativeOutOfBounds {
        position: RelativeBlockPosition,
        size: BlockSize,
    },
    MissingSection(BlockPosition),
    BiomeWriteInFork,
}
