use crate::entity::metadata::{EntityMeta, MonsterMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct ZoglinMeta<'entity> {
    monster_meta: MonsterMeta<'entity>,
}

impl<'entity> ZoglinMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::ZOGLIN).then(|| Self {
            monster_meta: MonsterMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn is_baby(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::zoglin::is_baby())
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
            &definitions::zoglin::is_baby(),
            MetadataValue::Boolean(is_baby),
        );
    }
}

impl<'entity> Deref for ZoglinMeta<'entity> {
    type Target = MonsterMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.monster_meta
    }
}

impl<'entity> DerefMut for ZoglinMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster_meta
    }
}
