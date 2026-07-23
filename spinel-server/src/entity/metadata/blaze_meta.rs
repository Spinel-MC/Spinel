use crate::entity::metadata::{LivingEntityMeta, MonsterMeta, definitions};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct BlazeMeta<'entity> {
    monster_meta: MonsterMeta<'entity>,
}

impl<'entity> BlazeMeta<'entity> {
    pub(crate) fn from_living_entity_meta(
        living_entity_meta: LivingEntityMeta<'entity>,
    ) -> Option<Self> {
        (living_entity_meta.get_entity_type() == EntityType::BLAZE).then(|| Self {
            monster_meta: MonsterMeta::from_living_entity_meta(living_entity_meta),
        })
    }

    pub fn is_on_fire(&self) -> bool {
        self.get_state()
            .get_metadata()
            .get_flag(&definitions::blaze::is_on_fire())
    }

    pub fn set_on_fire(&mut self, is_on_fire: bool) {
        self.get_entity_state_mut()
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
