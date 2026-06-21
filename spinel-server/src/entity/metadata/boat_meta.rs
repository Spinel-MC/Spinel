use crate::entity::metadata::{AbstractVehicleMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct BoatMeta<'entity> {
    abstract_vehicle_meta: AbstractVehicleMeta<'entity>,
}

impl<'entity> BoatMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        is_boat_type(entity_meta.get_entity().get_entity_type()).then(|| Self {
            abstract_vehicle_meta: AbstractVehicleMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn is_left_paddle_turning(&self) -> bool {
        self.boolean(&definitions::boat::is_left_paddle_turning())
    }

    pub fn set_left_paddle_turning(&mut self, is_left_paddle_turning: bool) {
        self.set_boolean(
            &definitions::boat::is_left_paddle_turning(),
            is_left_paddle_turning,
        );
    }

    pub fn is_right_paddle_turning(&self) -> bool {
        self.boolean(&definitions::boat::is_right_paddle_turning())
    }

    pub fn set_right_paddle_turning(&mut self, is_right_paddle_turning: bool) {
        self.set_boolean(
            &definitions::boat::is_right_paddle_turning(),
            is_right_paddle_turning,
        );
    }

    pub fn get_splash_timer(&self) -> i32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::boat::get_splash_timer())
        {
            MetadataValue::VarInt(splash_timer) => splash_timer,
            _ => 0,
        }
    }

    pub fn set_splash_timer(&mut self, splash_timer: i32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::boat::get_splash_timer(),
            MetadataValue::VarInt(splash_timer),
        );
    }

    fn boolean(&self, definition: &crate::entity::metadata::MetadataDefinition) -> bool {
        match self.get_entity().get_metadata().get_value(definition) {
            MetadataValue::Boolean(value) => value,
            _ => false,
        }
    }

    fn set_boolean(
        &mut self,
        definition: &crate::entity::metadata::MetadataDefinition,
        value: bool,
    ) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set(definition, MetadataValue::Boolean(value));
    }
}

impl<'entity> Deref for BoatMeta<'entity> {
    type Target = AbstractVehicleMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_vehicle_meta
    }
}

impl<'entity> DerefMut for BoatMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_vehicle_meta
    }
}

fn is_boat_type(entity_type: EntityType) -> bool {
    matches!(
        entity_type,
        EntityType::ACACIA_BOAT
            | EntityType::ACACIA_CHEST_BOAT
            | EntityType::BIRCH_BOAT
            | EntityType::BIRCH_CHEST_BOAT
            | EntityType::CHERRY_BOAT
            | EntityType::CHERRY_CHEST_BOAT
            | EntityType::DARK_OAK_BOAT
            | EntityType::DARK_OAK_CHEST_BOAT
            | EntityType::JUNGLE_BOAT
            | EntityType::JUNGLE_CHEST_BOAT
            | EntityType::MANGROVE_BOAT
            | EntityType::MANGROVE_CHEST_BOAT
            | EntityType::OAK_BOAT
            | EntityType::OAK_CHEST_BOAT
            | EntityType::PALE_OAK_BOAT
            | EntityType::PALE_OAK_CHEST_BOAT
            | EntityType::SPRUCE_BOAT
            | EntityType::SPRUCE_CHEST_BOAT
    )
}
