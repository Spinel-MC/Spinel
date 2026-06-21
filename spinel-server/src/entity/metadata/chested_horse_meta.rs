use crate::entity::metadata::{AbstractHorseMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use std::ops::{Deref, DerefMut};

pub struct ChestedHorseMeta<'entity> {
    abstract_horse_meta: AbstractHorseMeta<'entity>,
}

impl<'entity> ChestedHorseMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Self {
        Self {
            abstract_horse_meta: AbstractHorseMeta::from_entity_meta(entity_meta),
        }
    }

    pub fn has_chest(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::chested_horse::has_chest())
        {
            MetadataValue::Boolean(has_chest) => has_chest,
            _ => false,
        }
    }

    pub fn set_has_chest(&mut self, has_chest: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::chested_horse::has_chest(),
            MetadataValue::Boolean(has_chest),
        );
    }
}

impl<'entity> Deref for ChestedHorseMeta<'entity> {
    type Target = AbstractHorseMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_horse_meta
    }
}

impl<'entity> DerefMut for ChestedHorseMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_horse_meta
    }
}
