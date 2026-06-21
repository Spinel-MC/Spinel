use crate::entity::metadata::{AnimalMeta, ArmadilloState, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct ArmadilloMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> ArmadilloMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::ARMADILLO).then(|| Self {
            animal_meta: AnimalMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_state(&self) -> ArmadilloState {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::armadillo::get_state())
        {
            MetadataValue::ArmadilloState(state) => {
                ArmadilloState::from_protocol_id(state).unwrap_or_default()
            }
            _ => ArmadilloState::default(),
        }
    }

    pub fn set_state(&mut self, state: ArmadilloState) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::armadillo::get_state(),
            MetadataValue::ArmadilloState(state.get_protocol_id()),
        );
    }
}

impl<'entity> Deref for ArmadilloMeta<'entity> {
    type Target = AnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.animal_meta
    }
}

impl<'entity> DerefMut for ArmadilloMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal_meta
    }
}
