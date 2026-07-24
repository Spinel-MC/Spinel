use crate::entity::metadata::{LivingEntityMeta, MonsterMeta, definitions};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct VexMeta<'entity> {
    monster_meta: MonsterMeta<'entity>,
}

impl<'entity> VexMeta<'entity> {
    pub(crate) fn from_living_entity_meta(
        living_entity_meta: LivingEntityMeta<'entity>,
    ) -> Option<Self> {
        (living_entity_meta.get_entity_type() == EntityType::VEX).then(|| Self {
            monster_meta: MonsterMeta::from_living_entity_meta(living_entity_meta),
        })
    }

    pub fn is_attacking(&self) -> bool {
        self.get_state()
            .get_metadata()
            .get_flag(&definitions::vex::is_attacking())
    }

    pub fn set_attacking(&mut self, is_attacking: bool) {
        self.get_entity_state_mut()
            .get_metadata_mut()
            .set_flag(&definitions::vex::is_attacking(), is_attacking);
    }
}

impl<'entity> Deref for VexMeta<'entity> {
    type Target = MonsterMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.monster_meta
    }
}

impl<'entity> DerefMut for VexMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster_meta
    }
}
