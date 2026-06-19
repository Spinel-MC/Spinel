use crate::entity::metadata::{AnimalMeta, ArmadilloState, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct ArmadilloMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> ArmadilloMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.entity().entity_type() == EntityType::ARMADILLO).then(|| Self {
            animal_meta: AnimalMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn state(&self) -> ArmadilloState {
        match self
            .entity()
            .metadata()
            .value(&definitions::armadillo::state())
        {
            MetadataValue::ArmadilloState(state) => {
                ArmadilloState::from_protocol_id(state).unwrap_or_default()
            }
            _ => ArmadilloState::default(),
        }
    }

    pub fn set_state(&mut self, state: ArmadilloState) {
        self.entity_mut().metadata_mut().set(
            &definitions::armadillo::state(),
            MetadataValue::ArmadilloState(state.protocol_id()),
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
