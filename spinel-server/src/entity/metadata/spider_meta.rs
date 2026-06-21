use crate::entity::metadata::{EntityMeta, MonsterMeta, definitions};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct SpiderMeta<'entity> {
    monster_meta: MonsterMeta<'entity>,
}

impl<'entity> SpiderMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::SPIDER).then(|| Self {
            monster_meta: MonsterMeta::from_entity_meta(entity_meta),
        })
    }

    pub(crate) fn from_cave_spider_entity_meta(entity_meta: EntityMeta<'entity>) -> Self {
        Self {
            monster_meta: MonsterMeta::from_entity_meta(entity_meta),
        }
    }

    pub fn is_climbing(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .flag(&definitions::spider::is_climbing())
    }

    pub fn set_climbing(&mut self, is_climbing: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::spider::is_climbing(), is_climbing);
    }
}

impl<'entity> Deref for SpiderMeta<'entity> {
    type Target = MonsterMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.monster_meta
    }
}

impl<'entity> DerefMut for SpiderMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster_meta
    }
}
