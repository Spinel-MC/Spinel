use crate::entity::metadata::{CamelMeta, EntityMeta};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct CamelHuskMeta<'entity> {
    camel_meta: CamelMeta<'entity>,
}

impl<'entity> CamelHuskMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::CAMEL_HUSK).then(|| Self {
            camel_meta: CamelMeta::from_camel_husk_entity_meta(entity_meta),
        })
    }
}

impl<'entity> Deref for CamelHuskMeta<'entity> {
    type Target = CamelMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.camel_meta
    }
}

impl<'entity> DerefMut for CamelHuskMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.camel_meta
    }
}
