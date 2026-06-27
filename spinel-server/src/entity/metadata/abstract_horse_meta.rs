use crate::entity::metadata::{AnimalMeta, EntityMeta, definitions};
use std::ops::{Deref, DerefMut};

pub struct AbstractHorseMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> AbstractHorseMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Self {
        Self {
            animal_meta: AnimalMeta::from_entity_meta(entity_meta),
        }
    }

    pub fn is_tamed(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::abstract_horse::is_tame())
    }

    pub fn set_tamed(&mut self, is_tamed: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::abstract_horse::is_tame(), is_tamed);
    }

    pub fn has_bred(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::abstract_horse::has_bred())
    }

    pub fn set_has_bred(&mut self, has_bred: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::abstract_horse::has_bred(), has_bred);
    }

    pub fn is_eating(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::abstract_horse::is_eating())
    }

    pub fn set_eating(&mut self, is_eating: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::abstract_horse::is_eating(), is_eating);
    }

    pub fn is_rearing(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::abstract_horse::is_rearing())
    }

    pub fn set_rearing(&mut self, is_rearing: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::abstract_horse::is_rearing(), is_rearing);
    }

    pub fn is_mouth_open(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::abstract_horse::is_mouth_open())
    }

    pub fn set_mouth_open(&mut self, is_mouth_open: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::abstract_horse::is_mouth_open(), is_mouth_open);
    }
}

impl<'entity> Deref for AbstractHorseMeta<'entity> {
    type Target = AnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.animal_meta
    }
}

impl<'entity> DerefMut for AbstractHorseMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal_meta
    }
}
