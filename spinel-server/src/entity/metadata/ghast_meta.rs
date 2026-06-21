use crate::entity::metadata::{EntityMeta, FlyingMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct GhastMeta<'entity> {
    flying_meta: FlyingMeta<'entity>,
}

impl<'entity> GhastMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::GHAST).then(|| Self {
            flying_meta: FlyingMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn is_attacking(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::ghast::is_attacking())
        {
            MetadataValue::Boolean(is_attacking) => is_attacking,
            _ => false,
        }
    }

    pub fn set_attacking(&mut self, is_attacking: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::ghast::is_attacking(),
            MetadataValue::Boolean(is_attacking),
        );
    }
}

impl<'entity> Deref for GhastMeta<'entity> {
    type Target = FlyingMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.flying_meta
    }
}

impl<'entity> DerefMut for GhastMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.flying_meta
    }
}
