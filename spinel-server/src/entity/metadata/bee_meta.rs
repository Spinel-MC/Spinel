use crate::entity::metadata::{AnimalMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct BeeMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> BeeMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::BEE).then(|| Self {
            animal_meta: AnimalMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn is_angry(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::bee::is_angry())
    }

    pub fn set_angry(&mut self, is_angry: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::bee::is_angry(), is_angry);
    }

    pub fn has_stung(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::bee::has_stung())
    }

    pub fn set_has_stung(&mut self, has_stung: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::bee::has_stung(), has_stung);
    }

    pub fn has_nectar(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::bee::has_nectar())
    }

    pub fn set_has_nectar(&mut self, has_nectar: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::bee::has_nectar(), has_nectar);
    }

    pub fn get_anger_ticks(&self) -> i64 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::bee::anger_time_ticks())
        {
            MetadataValue::Long(anger_ticks) => anger_ticks,
            _ => -1,
        }
    }

    pub fn set_anger_ticks(&mut self, anger_ticks: i64) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::bee::anger_time_ticks(),
            MetadataValue::Long(anger_ticks),
        );
    }
}

impl<'entity> Deref for BeeMeta<'entity> {
    type Target = AnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.animal_meta
    }
}

impl<'entity> DerefMut for BeeMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal_meta
    }
}
