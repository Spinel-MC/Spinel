use crate::world::{Block, BlockPosition};

pub struct DynamicFork {
    block_writes: Vec<(BlockPosition, Block)>,
}

impl DynamicFork {
    pub(crate) fn new() -> Self {
        Self {
            block_writes: Vec::new(),
        }
    }

    pub fn set_block(&mut self, position: BlockPosition, block: Block) {
        self.block_writes.push((position, block));
    }

    pub(crate) fn into_block_writes(self) -> Option<DynamicForkBlockWrites> {
        let first_position = self.block_writes.first().map(|(position, _)| *position)?;
        let bounds = self.block_writes.iter().fold(
            DynamicForkBounds::new(first_position),
            |bounds, (position, _)| bounds.include(*position),
        );
        Some(DynamicForkBlockWrites {
            start: bounds.start(),
            end: bounds.end(),
            block_writes: self.block_writes,
        })
    }
}

pub(crate) struct DynamicForkBlockWrites {
    pub(crate) start: BlockPosition,
    pub(crate) end: BlockPosition,
    pub(crate) block_writes: Vec<(BlockPosition, Block)>,
}

struct DynamicForkBounds {
    minimum: BlockPosition,
    maximum: BlockPosition,
}

impl DynamicForkBounds {
    fn new(position: BlockPosition) -> Self {
        Self {
            minimum: position,
            maximum: position,
        }
    }

    fn include(self, position: BlockPosition) -> Self {
        Self {
            minimum: BlockPosition::new(
                self.minimum.x.min(position.x),
                self.minimum.y.min(position.y),
                self.minimum.z.min(position.z),
            ),
            maximum: BlockPosition::new(
                self.maximum.x.max(position.x),
                self.maximum.y.max(position.y),
                self.maximum.z.max(position.z),
            ),
        }
    }

    fn start(&self) -> BlockPosition {
        BlockPosition::new(
            floor_section(self.minimum.x),
            floor_section(self.minimum.y),
            floor_section(self.minimum.z),
        )
    }

    fn end(&self) -> BlockPosition {
        BlockPosition::new(
            ceil_section(self.maximum.x + 1),
            ceil_section(self.maximum.y + 1),
            ceil_section(self.maximum.z + 1),
        )
    }
}

fn floor_section(value: i32) -> i32 {
    value.div_euclid(16) * 16
}

fn ceil_section(value: i32) -> i32 {
    let section = value.div_euclid(16);
    if value.rem_euclid(16) == 0 {
        section * 16
    } else {
        (section + 1) * 16
    }
}
