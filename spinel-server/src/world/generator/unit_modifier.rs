use crate::world::{Biome, Block, BlockPosition, BlockSize, ChunkSection, RelativeBlockPosition};

pub struct UnitModifier<'a> {
    sections: &'a mut [ChunkSection],
    size: BlockSize,
    absolute_start: BlockPosition,
    absolute_end: BlockPosition,
}

impl<'a> UnitModifier<'a> {
    pub(crate) fn new(
        sections: &'a mut [ChunkSection],
        size: BlockSize,
        absolute_start: BlockPosition,
        absolute_end: BlockPosition,
    ) -> Self {
        Self {
            sections,
            size,
            absolute_start,
            absolute_end,
        }
    }

    pub fn set_block(&mut self, position: BlockPosition, block: Block) {
        if !self.contains(position) {
            return;
        }
        let relative = RelativeBlockPosition::new(
            position.x - self.absolute_start.x,
            position.y - self.absolute_start.y,
            position.z - self.absolute_start.z,
        );
        self.set_relative(relative, block);
    }

    pub fn set_relative(&mut self, position: RelativeBlockPosition, block: Block) {
        if !self.contains_relative(position) {
            return;
        }
        let section_index = (position.y >> 4) as usize;
        let section_y = position.y & 15;
        self.sections[section_index].set_block(position.x, section_y, position.z, block);
    }

    pub fn fill(&mut self, block: Block) {
        self.fill_area(self.absolute_start, self.absolute_end, block);
    }

    pub fn fill_area(&mut self, start: BlockPosition, end: BlockPosition, block: Block) {
        (start.x..end.x).for_each(|x| {
            (start.y..end.y).for_each(|y| {
                (start.z..end.z).for_each(|z| self.set_block(BlockPosition::new(x, y, z), block));
            });
        });
    }

    pub fn fill_height(&mut self, min_height: i32, max_height: i32, block: Block) {
        let start = BlockPosition::new(self.absolute_start.x, min_height, self.absolute_start.z);
        let end = BlockPosition::new(self.absolute_end.x, max_height, self.absolute_end.z);
        self.fill_area(start, end, block);
    }

    pub fn set_biome(&mut self, position: BlockPosition, biome: Biome) {
        if !self.contains(position) {
            return;
        }
        self.sections
            .iter_mut()
            .for_each(|section| section.set_biome(biome));
    }

    pub fn fill_biome(&mut self, biome: Biome) {
        self.sections
            .iter_mut()
            .for_each(|section| section.set_biome(biome));
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
