use crate::entity::metadata::{AbstractGolemMeta, LivingEntityMeta, definitions};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct SnowGolemMeta<'entity> {
    abstract_golem_meta: AbstractGolemMeta<'entity>,
}

impl<'entity> SnowGolemMeta<'entity> {
    pub(crate) fn from_living_entity_meta(
        living_entity_meta: LivingEntityMeta<'entity>,
    ) -> Option<Self> {
        (living_entity_meta.get_entity_type() == EntityType::SNOW_GOLEM).then(|| Self {
            abstract_golem_meta: AbstractGolemMeta::from_living_entity_meta(living_entity_meta),
        })
    }

    pub fn has_pumpkin_hat(&self) -> bool {
        self.get_state()
            .get_metadata()
            .get_flag(&definitions::snow_golem::has_pumpkin_hat())
    }

    pub fn set_has_pumpkin_hat(&mut self, has_pumpkin_hat: bool) {
        self.get_entity_state_mut()
            .get_metadata_mut()
            .set_flag(&definitions::snow_golem::has_pumpkin_hat(), has_pumpkin_hat);
    }
}

impl<'entity> Deref for SnowGolemMeta<'entity> {
    type Target = AbstractGolemMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_golem_meta
    }
}

impl<'entity> DerefMut for SnowGolemMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_golem_meta
    }
}
