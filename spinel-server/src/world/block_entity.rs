use crate::world::BlockPosition;
use spinel_nbt::NbtCompound;
use spinel_registry::block_entity_type::BlockEntityType;

#[derive(Debug, Clone, PartialEq)]
pub struct BlockEntity {
    position: BlockPosition,
    block_entity_type: BlockEntityType,
    nbt: NbtCompound,
}

impl BlockEntity {
    pub fn new(
        position: BlockPosition,
        block_entity_type: BlockEntityType,
        nbt: NbtCompound,
    ) -> Self {
        Self {
            position,
            block_entity_type,
            nbt,
        }
    }

    pub fn position(&self) -> BlockPosition {
        self.position
    }

    pub fn block_entity_type(&self) -> BlockEntityType {
        self.block_entity_type
    }

    pub fn nbt(&self) -> &NbtCompound {
        &self.nbt
    }

    pub(crate) fn to_network(&self) -> spinel_network::types::chunk::BlockEntity {
        spinel_network::types::chunk::BlockEntity::new(
            self.position.x,
            self.position.y,
            self.position.z,
            self.block_entity_type,
            self.nbt.clone(),
        )
    }
}
