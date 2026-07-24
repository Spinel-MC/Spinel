use crate::entity::metadata::{LivingEntityMeta, MonsterMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use std::ops::{Deref, DerefMut};

pub struct BasePiglinMeta<'entity> {
    monster_meta: MonsterMeta<'entity>,
}

impl<'entity> BasePiglinMeta<'entity> {
    pub(crate) fn from_living_entity_meta(living_entity_meta: LivingEntityMeta<'entity>) -> Self {
        Self {
            monster_meta: MonsterMeta::from_living_entity_meta(living_entity_meta),
        }
    }

    pub fn is_immune_to_zombification(&self) -> bool {
        match self
            .get_state()
            .get_metadata()
            .get_value(&definitions::base_piglin::is_immune_to_zombification())
        {
            MetadataValue::Boolean(is_immune_to_zombification) => is_immune_to_zombification,
            _ => false,
        }
    }

    pub fn set_immune_to_zombification(&mut self, is_immune_to_zombification: bool) {
        self.get_entity_state_mut().get_metadata_mut().set(
            &definitions::base_piglin::is_immune_to_zombification(),
            MetadataValue::Boolean(is_immune_to_zombification),
        );
    }
}

impl<'entity> Deref for BasePiglinMeta<'entity> {
    type Target = MonsterMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.monster_meta
    }
}

impl<'entity> DerefMut for BasePiglinMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster_meta
    }
}
