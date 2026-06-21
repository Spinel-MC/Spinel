use crate::entity::metadata::{EntityMeta, MonsterMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct WardenMeta<'entity> {
    monster_meta: MonsterMeta<'entity>,
}

impl<'entity> WardenMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::WARDEN).then(|| Self {
            monster_meta: MonsterMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_anger_level(&self) -> i32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::warden::get_anger_level())
        {
            MetadataValue::VarInt(anger_level) => anger_level,
            _ => 0,
        }
    }

    pub fn set_anger_level(&mut self, anger_level: i32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::warden::get_anger_level(),
            MetadataValue::VarInt(anger_level),
        );
    }
}

impl<'entity> Deref for WardenMeta<'entity> {
    type Target = MonsterMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.monster_meta
    }
}

impl<'entity> DerefMut for WardenMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster_meta
    }
}
