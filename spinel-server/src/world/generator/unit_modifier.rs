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
        for block_x in self.absolute_start.x..self.absolute_end.x {
            for block_y in self.absolute_start.y..self.absolute_end.y {
                for block_z in self.absolute_start.z..self.absolute_end.z {
                    let position = BlockPosition::new(block_x, block_y, block_z);
                    self.set_contained_block(position, supplier(block_x, block_y, block_z))?;
                }
            }
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
        for relative_x in 0..self.size.x {
            for relative_y in 0..self.size.y {
                for relative_z in 0..self.size.z {
                    self.try_set_relative(
                        RelativeBlockPosition::new(relative_x, relative_y, relative_z),
                        supplier(relative_x, relative_y, relative_z),
                    )?;
                }
            }
        }
        Ok(())
    }

    pub fn fill(&mut self, block: Block) {
        self.fill_area(self.absolute_start, self.absolute_end, block);
    }

    pub fn fill_area(&mut self, start: BlockPosition, end: BlockPosition, block: Block) {
        (start.x..end.x).for_each(|block_x| {
            (start.y..end.y).for_each(|block_y| {
                (start.z..end.z).for_each(|block_z| {
                    self.set_block(BlockPosition::new(block_x, block_y, block_z), block)
                });
            });
        });
    }

    pub fn fill_height(&mut self, min_height: i32, max_height: i32, block: Block) {
        let start = BlockPosition::new(self.absolute_start.x, min_height, self.absolute_start.z);
        let end = BlockPosition::new(self.absolute_end.x, max_height, self.absolute_end.z);
        self.fill_area(start, end, block);
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
