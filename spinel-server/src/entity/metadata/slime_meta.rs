use crate::entity::metadata::{EntityMeta, MobMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct SlimeMeta<'entity> {
    mob_meta: MobMeta<'entity>,
}

impl<'entity> SlimeMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::SLIME).then(|| Self {
            mob_meta: MobMeta::new(crate::entity::metadata::LivingEntityMeta::new(entity_meta)),
        })
    }

    pub(crate) fn from_entity_meta_unchecked(entity_meta: EntityMeta<'entity>) -> Self {
        Self {
            mob_meta: MobMeta::new(crate::entity::metadata::LivingEntityMeta::new(entity_meta)),
        }
    }

    pub fn get_size(&self) -> i32 {
        match self.get_entity().get_metadata().get_value(&definitions::slime::get_size()) {
            MetadataValue::VarInt(size) => size,
            _ => 1,
        }
    }

    pub fn set_size(&mut self, size: i32) {
        let box_size = f64::from(0.51000005_f32 * size as f32);
        self.get_entity_mut()
            .set_bounding_box_dimensions(box_size, box_size, box_size);
        self.get_entity_mut()
            .get_metadata_mut()
            .set(&definitions::slime::get_size(), MetadataValue::VarInt(size));
    }
}

impl<'entity> Deref for SlimeMeta<'entity> {
    type Target = MobMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.mob_meta
    }
}

impl<'entity> DerefMut for SlimeMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mob_meta
    }
}
