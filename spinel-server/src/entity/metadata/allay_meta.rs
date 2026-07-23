use crate::entity::metadata::{LivingEntityMeta, MobMeta, PathfinderMobMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct AllayMeta<'entity> {
    pathfinder_mob_meta: PathfinderMobMeta<'entity>,
}

impl<'entity> AllayMeta<'entity> {
    pub(crate) fn from_living_entity_meta(
        living_entity_meta: LivingEntityMeta<'entity>,
    ) -> Option<Self> {
        (living_entity_meta.get_entity_type() == EntityType::ALLAY).then(|| Self {
            pathfinder_mob_meta: PathfinderMobMeta::new(MobMeta::new(living_entity_meta)),
        })
    }

    pub fn is_dancing(&self) -> bool {
        match self
            .get_entity_state()
            .get_metadata()
            .get_value(&definitions::allay::is_dancing())
        {
            MetadataValue::Boolean(is_dancing) => is_dancing,
            _ => false,
        }
    }

    pub fn set_dancing(&mut self, is_dancing: bool) {
        self.get_entity_state_mut().get_metadata_mut().set(
            &definitions::allay::is_dancing(),
            MetadataValue::Boolean(is_dancing),
        );
    }

    pub fn can_duplicate(&self) -> bool {
        match self
            .get_entity_state()
            .get_metadata()
            .get_value(&definitions::allay::can_duplicate())
        {
            MetadataValue::Boolean(can_duplicate) => can_duplicate,
            _ => true,
        }
    }

    pub fn set_can_duplicate(&mut self, can_duplicate: bool) {
        self.get_entity_state_mut().get_metadata_mut().set(
            &definitions::allay::can_duplicate(),
            MetadataValue::Boolean(can_duplicate),
        );
    }
}

impl<'entity> Deref for AllayMeta<'entity> {
    type Target = PathfinderMobMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.pathfinder_mob_meta
    }
}

impl<'entity> DerefMut for AllayMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pathfinder_mob_meta
    }
}
