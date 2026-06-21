use crate::entity::metadata::{AbstractHorseMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct CamelMeta<'entity> {
    abstract_horse_meta: AbstractHorseMeta<'entity>,
}

impl<'entity> CamelMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::CAMEL).then(|| Self {
            abstract_horse_meta: AbstractHorseMeta::from_entity_meta(entity_meta),
        })
    }

    pub(crate) fn from_camel_husk_entity_meta(entity_meta: EntityMeta<'entity>) -> Self {
        Self {
            abstract_horse_meta: AbstractHorseMeta::from_entity_meta(entity_meta),
        }
    }

    pub fn is_dashing(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::camel::is_dashing())
        {
            MetadataValue::Boolean(is_dashing) => is_dashing,
            _ => false,
        }
    }

    pub fn set_dashing(&mut self, is_dashing: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::camel::is_dashing(),
            MetadataValue::Boolean(is_dashing),
        );
    }

    pub fn get_last_pose_change_tick(&self) -> i64 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::camel::get_last_pose_change_tick())
        {
            MetadataValue::Long(last_pose_change_tick) => last_pose_change_tick,
            _ => 0,
        }
    }

    pub fn set_last_pose_change_tick(&mut self, last_pose_change_tick: i64) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::camel::get_last_pose_change_tick(),
            MetadataValue::Long(last_pose_change_tick),
        );
    }
}

impl<'entity> Deref for CamelMeta<'entity> {
    type Target = AbstractHorseMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_horse_meta
    }
}

impl<'entity> DerefMut for CamelMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_horse_meta
    }
}
