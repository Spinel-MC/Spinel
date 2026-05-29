use crate::data_components::vanilla_components::MAX_STACK_SIZE;
use crate::{DataComponentMap, Identifier, vanilla_world_blocks::Block};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Material {
    id: i32,
    key: Identifier,
    block: Option<Block>,
}

impl Material {
    pub const fn new(id: i32, key: Identifier, block: Option<Block>) -> Self {
        Self { id, key, block }
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

    pub fn max_stack_size(&self) -> i32 {
        let empty_prototype = DataComponentMap::new();
        self.prototype()
            .get_or(&empty_prototype, MAX_STACK_SIZE, 64)
    }
}
