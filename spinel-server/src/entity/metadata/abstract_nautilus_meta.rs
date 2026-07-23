use crate::entity::metadata::{LivingEntityMeta, TameableAnimalMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use std::ops::{Deref, DerefMut};

pub struct AbstractNautilusMeta<'entity> {
    tameable_animal_meta: TameableAnimalMeta<'entity>,
}

impl<'entity> AbstractNautilusMeta<'entity> {
    pub(crate) fn from_living_entity_meta(living_entity_meta: LivingEntityMeta<'entity>) -> Self {
        Self {
            tameable_animal_meta: TameableAnimalMeta::from_living_entity_meta(living_entity_meta),
        }
    }

    pub fn is_dashing(&self) -> bool {
        match self
            .get_state()
            .get_metadata()
            .get_value(&definitions::abstract_nautilus::is_dashing())
        {
            MetadataValue::Boolean(is_dashing) => is_dashing,
            _ => false,
        }
    }

    pub fn set_dashing(&mut self, is_dashing: bool) {
        self.get_entity_state_mut().get_metadata_mut().set(
            &definitions::abstract_nautilus::is_dashing(),
            MetadataValue::Boolean(is_dashing),
        );
    }
}

impl<'entity> Deref for AbstractNautilusMeta<'entity> {
    type Target = TameableAnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.tameable_animal_meta
    }
}

impl<'entity> DerefMut for AbstractNautilusMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tameable_animal_meta
    }
}
