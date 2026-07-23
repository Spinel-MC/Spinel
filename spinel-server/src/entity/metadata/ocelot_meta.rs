use crate::entity::metadata::{AnimalMeta, LivingEntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct OcelotMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> OcelotMeta<'entity> {
    pub(crate) fn from_living_entity_meta(
        living_entity_meta: LivingEntityMeta<'entity>,
    ) -> Option<Self> {
        (living_entity_meta.get_entity_type() == EntityType::OCELOT).then(|| Self {
            animal_meta: AnimalMeta::from_living_entity_meta(living_entity_meta),
        })
    }

    pub fn is_trusting(&self) -> bool {
        match self
            .get_state()
            .get_metadata()
            .get_value(&definitions::ocelot::is_trusting())
        {
            MetadataValue::Boolean(is_trusting) => is_trusting,
            _ => false,
        }
    }

    pub fn set_trusting(&mut self, is_trusting: bool) {
        self.get_entity_state_mut().get_metadata_mut().set(
            &definitions::ocelot::is_trusting(),
            MetadataValue::Boolean(is_trusting),
        );
    }
}

impl<'entity> Deref for OcelotMeta<'entity> {
    type Target = AnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.animal_meta
    }
}

impl<'entity> DerefMut for OcelotMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal_meta
    }
}
