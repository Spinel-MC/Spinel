use crate::world::{BlockHandlerRegistry, BlockInstance, BlockState};
use spinel_nbt::NbtCompound;
use spinel_registry::Identifier;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StoredBlockInstance {
    block_state: BlockState,
    nbt: Option<NbtCompound>,
    handler_key: Option<Identifier>,
}

impl StoredBlockInstance {
    pub fn new(
        block_state: BlockState,
        nbt: Option<NbtCompound>,
        handler_key: Option<Identifier>,
    ) -> Self {
        Self {
            block_state,
            nbt,
            handler_key,
        }
    }

    pub const fn block_state(&self) -> BlockState {
        self.block_state
    }

    pub fn nbt(&self) -> Option<&NbtCompound> {
        self.nbt.as_ref()
    }

    pub fn handler_key(&self) -> Option<&Identifier> {
        self.handler_key.as_ref()
    }

    pub fn restore(self, handlers: &mut BlockHandlerRegistry) -> BlockInstance {
        let handler = self
            .handler_key
            .as_ref()
            .map(|handler_key| handlers.handler_or_dummy(handler_key));
        BlockInstance::new(self.block_state)
            .with_nbt(self.nbt)
            .with_handler(handler)
    }
}

impl From<&BlockInstance> for StoredBlockInstance {
    fn from(block_instance: &BlockInstance) -> Self {
        Self::new(
            block_instance.block_state(),
            block_instance.nbt().cloned(),
            block_instance
                .handler()
                .map(|handler| handler.key().clone()),
        )
    }
}
