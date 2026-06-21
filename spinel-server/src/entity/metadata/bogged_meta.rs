use crate::entity::metadata::{AbstractSkeletonMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct BoggedMeta<'entity> {
    abstract_skeleton_meta: AbstractSkeletonMeta<'entity>,
}

impl<'entity> BoggedMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::BOGGED).then(|| Self {
            abstract_skeleton_meta: AbstractSkeletonMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn is_sheared(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::bogged::is_sheared())
        {
            MetadataValue::Boolean(is_sheared) => is_sheared,
            _ => false,
        }
    }

    pub fn set_sheared(&mut self, is_sheared: bool) {
        self.get_entity_mut().get_metadata_mut().set(
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
