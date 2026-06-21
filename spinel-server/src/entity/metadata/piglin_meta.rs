use crate::entity::metadata::{BasePiglinMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct PiglinMeta<'entity> {
    base_piglin_meta: BasePiglinMeta<'entity>,
}

impl<'entity> PiglinMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::PIGLIN).then(|| Self {
            base_piglin_meta: BasePiglinMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn is_baby(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::piglin::is_baby())
        {
            MetadataValue::Boolean(is_baby) => is_baby,
            _ => false,
        }
    }

    pub fn set_baby(&mut self, is_baby: bool) {
        if self.is_baby() == is_baby {
            return;
        }

        let bounding_box = self.get_entity().get_bounding_box();
        let scale = if is_baby { 0.5 } else { 2.0 };
        self.get_entity_mut().set_bounding_box_dimensions(
            bounding_box.get_width() * scale,
            bounding_box.get_height() * scale,
            bounding_box.depth() * scale,
        );
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::piglin::is_baby(),
            MetadataValue::Boolean(is_baby),
        );
    }

    pub fn is_charging_crossbow(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::piglin::is_charging_crossbow())
        {
            MetadataValue::Boolean(is_charging_crossbow) => is_charging_crossbow,
            _ => false,
        }
    }

    pub fn set_charging_crossbow(&mut self, is_charging_crossbow: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::piglin::is_charging_crossbow(),
            MetadataValue::Boolean(is_charging_crossbow),
        );
    }

    pub fn is_dancing(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::piglin::is_dancing())
        {
            MetadataValue::Boolean(is_dancing) => is_dancing,
            _ => false,
        }
    }

    pub fn set_dancing(&mut self, is_dancing: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::piglin::is_dancing(),
            MetadataValue::Boolean(is_dancing),
        );
    }
}

impl<'entity> Deref for PiglinMeta<'entity> {
    type Target = BasePiglinMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.base_piglin_meta
    }
}

impl<'entity> DerefMut for PiglinMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base_piglin_meta
    }
}
