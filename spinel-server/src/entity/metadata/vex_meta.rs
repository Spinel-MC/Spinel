use crate::entity::metadata::{EntityMeta, MonsterMeta, definitions};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct VexMeta<'entity> {
    monster_meta: MonsterMeta<'entity>,
}

impl<'entity> VexMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.entity().entity_type() == EntityType::VEX).then(|| Self {
            monster_meta: MonsterMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn is_attacking(&self) -> bool {
        self.entity()
            .metadata()
            .flag(&definitions::vex::is_attacking())
    }

    pub fn set_attacking(&mut self, is_attacking: bool) {
        self.entity_mut()
            .metadata_mut()
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
