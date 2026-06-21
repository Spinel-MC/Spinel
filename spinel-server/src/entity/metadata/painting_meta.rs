use crate::entity::metadata::{EntityMeta, HangingMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct PaintingMeta<'entity> {
    hanging_meta: HangingMeta<'entity>,
}

impl<'entity> PaintingMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::PAINTING).then(|| Self {
            hanging_meta: HangingMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_variant(&self) -> i32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::painting::get_variant())
        {
            MetadataValue::PaintingVariant(variant) => variant,
            _ => 24,
        }
    }

    pub fn set_variant(&mut self, variant: i32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::painting::get_variant(),
            MetadataValue::PaintingVariant(variant),
        );
    }
}

impl<'entity> Deref for PaintingMeta<'entity> {
    type Target = HangingMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.hanging_meta
    }
}

impl<'entity> DerefMut for PaintingMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.hanging_meta
    }
}
