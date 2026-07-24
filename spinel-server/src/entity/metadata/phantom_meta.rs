use crate::entity::metadata::{FlyingMeta, LivingEntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct PhantomMeta<'entity> {
    flying_meta: FlyingMeta<'entity>,
}

impl<'entity> PhantomMeta<'entity> {
    pub(crate) fn from_living_entity_meta(
        living_entity_meta: LivingEntityMeta<'entity>,
    ) -> Option<Self> {
        (living_entity_meta.get_entity_type() == EntityType::PHANTOM).then(|| Self {
            flying_meta: FlyingMeta::from_living_entity_meta(living_entity_meta),
        })
    }

    pub fn get_size(&self) -> i32 {
        match self
            .get_state()
            .get_metadata()
            .get_value(&definitions::phantom::get_size())
        {
            MetadataValue::VarInt(size) => size,
            _ => 0,
        }
    }

    pub fn set_size(&mut self, size: i32) {
        self.get_entity_state_mut().get_metadata_mut().set(
            &definitions::phantom::get_size(),
            MetadataValue::VarInt(size),
        );
    }
}

impl<'entity> Deref for PhantomMeta<'entity> {
    type Target = FlyingMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.flying_meta
    }
}

impl<'entity> DerefMut for PhantomMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.flying_meta
    }
}
