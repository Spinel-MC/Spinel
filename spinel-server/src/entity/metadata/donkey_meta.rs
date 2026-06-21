use crate::entity::metadata::{ChestedHorseMeta, EntityMeta};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct DonkeyMeta<'entity> {
    chested_horse_meta: ChestedHorseMeta<'entity>,
}

impl<'entity> DonkeyMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::DONKEY).then(|| Self {
            chested_horse_meta: ChestedHorseMeta::from_entity_meta(entity_meta),
        })
    }
}

impl<'entity> Deref for DonkeyMeta<'entity> {
    type Target = ChestedHorseMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.chested_horse_meta
    }
}

impl<'entity> DerefMut for DonkeyMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.chested_horse_meta
    }
}
