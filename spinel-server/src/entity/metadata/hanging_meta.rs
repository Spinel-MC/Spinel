use crate::entity::metadata::{EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use std::ops::{Deref, DerefMut};

pub struct HangingMeta<'entity> {
    entity_meta: EntityMeta<'entity>,
}

impl<'entity> HangingMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Self {
        Self { entity_meta }
    }

    pub fn get_direction(&self) -> i32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::hanging::get_direction())
        {
            MetadataValue::Direction(direction) => direction,
            _ => 3,
        }
    }

    pub fn set_direction(&mut self, direction: i32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::hanging::get_direction(),
            MetadataValue::Direction(direction),
        );
    }
}

impl<'entity> Deref for HangingMeta<'entity> {
    type Target = EntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.entity_meta
    }
}

impl<'entity> DerefMut for HangingMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_meta
    }
}
