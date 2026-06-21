use crate::entity::metadata::{AbstractVehicleMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::BlockState;
use std::ops::{Deref, DerefMut};

pub struct AbstractMinecartMeta<'entity> {
    abstract_vehicle_meta: AbstractVehicleMeta<'entity>,
}

impl<'entity> AbstractMinecartMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Self {
        Self {
            abstract_vehicle_meta: AbstractVehicleMeta::from_entity_meta(entity_meta),
        }
    }

    pub fn get_custom_block_state(&self) -> Option<BlockState> {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::abstract_minecart::get_custom_block_state())
        {
            MetadataValue::OptionalBlockState(0) => None,
            MetadataValue::OptionalBlockState(block_state_id) => {
                BlockState::from_state_id(block_state_id)
            }
            _ => None,
        }
    }

    pub fn set_custom_block_state(&mut self, custom_block_state: Option<BlockState>) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::abstract_minecart::get_custom_block_state(),
            MetadataValue::OptionalBlockState(custom_block_state.map_or(0, BlockState::state_id)),
        );
    }

    pub fn get_custom_block_y_position(&self) -> i32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::abstract_minecart::custom_block_y_position())
        {
            MetadataValue::VarInt(custom_block_y_position) => custom_block_y_position,
            _ => 6,
        }
    }

    pub fn set_custom_block_y_position(&mut self, custom_block_y_position: i32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::abstract_minecart::custom_block_y_position(),
            MetadataValue::VarInt(custom_block_y_position),
        );
    }
}

impl<'entity> Deref for AbstractMinecartMeta<'entity> {
    type Target = AbstractVehicleMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_vehicle_meta
    }
}

impl<'entity> DerefMut for AbstractMinecartMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_vehicle_meta
    }
}
