use crate::entity::metadata::{AbstractSkeletonMeta, LivingEntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct BoggedMeta<'entity> {
    abstract_skeleton_meta: AbstractSkeletonMeta<'entity>,
}

impl<'entity> BoggedMeta<'entity> {
    pub(crate) fn from_living_entity_meta(
        living_entity_meta: LivingEntityMeta<'entity>,
    ) -> Option<Self> {
        (living_entity_meta.get_entity_type() == EntityType::BOGGED).then(|| Self {
            abstract_skeleton_meta: AbstractSkeletonMeta::from_living_entity_meta(
                living_entity_meta,
            ),
        })
    }

    pub fn is_sheared(&self) -> bool {
        match self
            .get_state()
            .get_metadata()
            .get_value(&definitions::bogged::is_sheared())
        {
            MetadataValue::Boolean(is_sheared) => is_sheared,
            _ => false,
        }
    }

    pub fn set_sheared(&mut self, is_sheared: bool) {
        self.get_entity_state_mut().get_metadata_mut().set(
            &definitions::bogged::is_sheared(),
            MetadataValue::Boolean(is_sheared),
        );
    }
}

impl<'entity> Deref for BoggedMeta<'entity> {
    type Target = AbstractSkeletonMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_skeleton_meta
    }
}

impl<'entity> DerefMut for BoggedMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_skeleton_meta
    }
}
