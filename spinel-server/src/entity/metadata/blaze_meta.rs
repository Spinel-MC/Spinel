use crate::entity::metadata::{EntityMeta, MonsterMeta, definitions};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct BlazeMeta<'entity> {
    monster_meta: MonsterMeta<'entity>,
}

impl<'entity> BlazeMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::BLAZE).then(|| Self {
            monster_meta: MonsterMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn is_on_fire(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .flag(&definitions::blaze::is_on_fire())
    }

    pub fn set_on_fire(&mut self, is_on_fire: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::blaze::is_on_fire(), is_on_fire);
    }
}

impl<'entity> Deref for BlazeMeta<'entity> {
    type Target = MonsterMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.monster_meta
    }
}

impl<'entity> DerefMut for BlazeMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster_meta
    }
}
