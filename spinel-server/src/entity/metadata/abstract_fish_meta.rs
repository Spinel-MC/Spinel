use crate::entity::metadata::{EntityMeta, WaterAnimalMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use std::ops::{Deref, DerefMut};

pub struct AbstractFishMeta<'entity> {
    water_animal_meta: WaterAnimalMeta<'entity>,
}

impl<'entity> AbstractFishMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Self {
        Self {
            water_animal_meta: WaterAnimalMeta::new(entity_meta),
        }
    }

    pub fn is_from_bucket(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::abstract_fish::from_bucket())
        {
            MetadataValue::Boolean(is_from_bucket) => is_from_bucket,
            _ => false,
        }
    }

    pub fn set_from_bucket(&mut self, is_from_bucket: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::abstract_fish::from_bucket(),
            MetadataValue::Boolean(is_from_bucket),
        );
    }
}

impl<'entity> Deref for AbstractFishMeta<'entity> {
    type Target = WaterAnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.water_animal_meta
    }
}

impl<'entity> DerefMut for AbstractFishMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.water_animal_meta
    }
}
