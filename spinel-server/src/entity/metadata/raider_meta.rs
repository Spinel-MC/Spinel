use crate::entity::metadata::{EntityMeta, MonsterMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use std::ops::{Deref, DerefMut};

pub struct RaiderMeta<'entity> {
    monster_meta: MonsterMeta<'entity>,
}

impl<'entity> RaiderMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Self {
        Self {
            monster_meta: MonsterMeta::from_entity_meta(entity_meta),
        }
    }

    pub fn is_celebrating(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::raider::is_celebrating())
        {
            MetadataValue::Boolean(is_celebrating) => is_celebrating,
            _ => false,
        }
    }

    pub fn set_celebrating(&mut self, is_celebrating: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::raider::is_celebrating(),
            MetadataValue::Boolean(is_celebrating),
        );
    }
}

impl<'entity> Deref for RaiderMeta<'entity> {
    type Target = MonsterMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.monster_meta
    }
}

impl<'entity> DerefMut for RaiderMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster_meta
    }
}
