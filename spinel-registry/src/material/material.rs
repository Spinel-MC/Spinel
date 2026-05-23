use crate::data_components::vanilla_components::MAX_STACK_SIZE;
use crate::{DataComponentMap, Identifier, vanilla_world_blocks::Block};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Material {
    id: i32,
    key: Identifier,
    block: Option<Block>,
    max_stack_size: i32,
}

impl Material {
    pub const fn new(id: i32, key: Identifier, block: Option<Block>, max_stack_size: i32) -> Self {
        Self {
            id,
            key,
            block,
            max_stack_size,
        }
    }

    pub const fn id(&self) -> i32 {
        self.id
    }

    pub const fn key(&self) -> &Identifier {
        &self.key
    }

    pub const fn is_block(&self) -> bool {
        self.block.is_some()
    }

    pub const fn block(&self) -> Option<Block> {
        self.block
    }

    pub const fn max_stack_size(&self) -> i32 {
        self.max_stack_size
    }

    pub fn prototype(&self) -> DataComponentMap {
        let mut components = DataComponentMap::new();
        components.set(MAX_STACK_SIZE, self.max_stack_size);
        components
    }
}
