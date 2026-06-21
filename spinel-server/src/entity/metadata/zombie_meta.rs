use crate::entity::metadata::{EntityMeta, MonsterMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct ZombieMeta<'entity> {
    monster_meta: MonsterMeta<'entity>,
}

impl<'entity> ZombieMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::ZOMBIE).then(|| Self {
            monster_meta: MonsterMeta::from_entity_meta(entity_meta),
        })
    }

    pub(crate) fn from_subtype_entity_meta(entity_meta: EntityMeta<'entity>) -> Self {
        Self {
            monster_meta: MonsterMeta::from_entity_meta(entity_meta),
        }
    }

    pub fn is_baby(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::zombie::is_baby())
        {
            MetadataValue::Boolean(is_baby) => is_baby,
            _ => false,
        }
    }

    pub fn set_baby(&mut self, is_baby: bool) {
        if self.is_baby() == is_baby {
            return;
        }

        let bounding_box = self.get_entity().get_bounding_box();
        let scale = if is_baby { 0.5 } else { 2.0 };
        self.get_entity_mut().set_bounding_box_dimensions(
            bounding_box.get_width() * scale,
            bounding_box.get_height() * scale,
            bounding_box.depth() * scale,
        );
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::zombie::is_baby(),
            MetadataValue::Boolean(is_baby),
        );
    }

    pub fn is_becoming_drowned(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::zombie::is_becoming_drowned())
        {
            MetadataValue::Boolean(is_becoming_drowned) => is_becoming_drowned,
            _ => false,
        }
    }

    pub fn set_becoming_drowned(&mut self, is_becoming_drowned: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::zombie::is_becoming_drowned(),
            MetadataValue::Boolean(is_becoming_drowned),
        );
    }
}

impl<'entity> Deref for ZombieMeta<'entity> {
    type Target = MonsterMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.monster_meta
    }
}

impl<'entity> DerefMut for ZombieMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster_meta
    }
}
