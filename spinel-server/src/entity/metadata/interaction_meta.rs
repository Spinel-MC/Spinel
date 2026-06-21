use crate::entity::metadata::{EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct InteractionMeta<'entity> {
    entity_meta: EntityMeta<'entity>,
}

impl<'entity> InteractionMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::INTERACTION)
            .then_some(Self { entity_meta })
    }

    pub fn get_width(&self) -> f32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::interaction::get_width())
        {
            MetadataValue::Float(width) => width,
            _ => 1.0,
        }
    }

    pub fn set_width(&mut self, width: f32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::interaction::get_width(),
            MetadataValue::Float(width),
        );
    }

    pub fn get_height(&self) -> f32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::interaction::get_height())
        {
            MetadataValue::Float(height) => height,
            _ => 1.0,
        }
    }

    pub fn set_height(&mut self, height: f32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::interaction::get_height(),
            MetadataValue::Float(height),
        );
    }

    pub fn get_response(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::interaction::responsive())
        {
            MetadataValue::Boolean(response) => response,
            _ => false,
        }
    }

    pub fn set_response(&mut self, response: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::interaction::responsive(),
            MetadataValue::Boolean(response),
        );
    }
}

impl<'entity> Deref for InteractionMeta<'entity> {
    type Target = EntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.entity_meta
    }
}

impl<'entity> DerefMut for InteractionMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_meta
    }
}
