use crate::entity::metadata::{EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use std::ops::{Deref, DerefMut};

pub struct AbstractVehicleMeta<'entity> {
    entity_meta: EntityMeta<'entity>,
}

impl<'entity> AbstractVehicleMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Self {
        Self { entity_meta }
    }

    pub fn get_shaking_ticks(&self) -> i32 {
        self.var_int(&definitions::abstract_vehicle::shaking_power(), 0)
    }

    pub fn set_shaking_ticks(&mut self, shaking_ticks: i32) {
        self.set_var_int(
            &definitions::abstract_vehicle::shaking_power(),
            shaking_ticks,
        );
    }

    pub fn get_shaking_direction(&self) -> i32 {
        self.var_int(&definitions::abstract_vehicle::get_shaking_direction(), 1)
    }

    pub fn set_shaking_direction(&mut self, shaking_direction: i32) {
        self.set_var_int(
            &definitions::abstract_vehicle::get_shaking_direction(),
            shaking_direction,
        );
    }

    pub fn get_shaking_multiplier(&self) -> f32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::abstract_vehicle::get_shaking_multiplier())
        {
            MetadataValue::Float(shaking_multiplier) => shaking_multiplier,
            _ => 0.0,
        }
    }

    pub fn set_shaking_multiplier(&mut self, shaking_multiplier: f32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::abstract_vehicle::get_shaking_multiplier(),
            MetadataValue::Float(shaking_multiplier),
        );
    }

    fn var_int(
        &self,
        definition: &crate::entity::metadata::MetadataDefinition,
        default_value: i32,
    ) -> i32 {
        match self.get_entity().get_metadata().get_value(definition) {
            MetadataValue::VarInt(value) => value,
            _ => default_value,
        }
    }

    fn set_var_int(
        &mut self,
        definition: &crate::entity::metadata::MetadataDefinition,
        value: i32,
    ) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set(definition, MetadataValue::VarInt(value));
    }
}

impl<'entity> Deref for AbstractVehicleMeta<'entity> {
    type Target = EntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.entity_meta
    }
}

impl<'entity> DerefMut for AbstractVehicleMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_meta
    }
}
