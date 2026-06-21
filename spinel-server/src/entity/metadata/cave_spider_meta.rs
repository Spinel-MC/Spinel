use crate::entity::metadata::{EntityMeta, SpiderMeta};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct CaveSpiderMeta<'entity> {
    spider_meta: SpiderMeta<'entity>,
}

impl<'entity> CaveSpiderMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::CAVE_SPIDER).then(|| Self {
            spider_meta: SpiderMeta::from_cave_spider_entity_meta(entity_meta),
        })
    }
}

impl<'entity> Deref for CaveSpiderMeta<'entity> {
    type Target = SpiderMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.spider_meta
    }
}

impl<'entity> DerefMut for CaveSpiderMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.spider_meta
    }
}
