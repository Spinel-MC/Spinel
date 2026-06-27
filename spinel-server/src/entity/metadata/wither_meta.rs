use crate::entity::Entity;
use crate::entity::metadata::{EntityMeta, MonsterMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct WitherMeta<'entity> {
    monster_meta: MonsterMeta<'entity>,
}

impl<'entity> WitherMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::WITHER).then(|| Self {
            monster_meta: MonsterMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_center_head_entity_id(&self) -> i32 {
        self.head_entity_id(&definitions::wither::center_head_target())
    }

    pub fn set_center_head(&mut self, center_head: Option<&Entity>) {
        self.set_center_head_entity_id(
            center_head.map_or(0, |entity| entity.get_entity_id().get_value()),
        );
    }

    pub fn get_left_head_entity_id(&self) -> i32 {
        self.head_entity_id(&definitions::wither::left_head_target())
    }

    pub fn set_left_head(&mut self, left_head: Option<&Entity>) {
        self.set_left_head_entity_id(
            left_head.map_or(0, |entity| entity.get_entity_id().get_value()),
        );
    }

    pub fn get_right_head_entity_id(&self) -> i32 {
        self.head_entity_id(&definitions::wither::right_head_target())
    }

    pub fn set_right_head(&mut self, right_head: Option<&Entity>) {
        self.set_right_head_entity_id(
            right_head.map_or(0, |entity| entity.get_entity_id().get_value()),
        );
    }

    pub fn get_invulnerable_time(&self) -> i32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::wither::get_invulnerable_time())
        {
            MetadataValue::VarInt(invulnerable_time) => invulnerable_time,
            _ => 0,
        }
    }

    pub fn set_invulnerable_time(&mut self, invulnerable_time: i32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::wither::get_invulnerable_time(),
            MetadataValue::VarInt(invulnerable_time),
        );
    }

    pub(crate) fn set_center_head_entity_id(&mut self, entity_id: i32) {
        self.set_head_entity_id(&definitions::wither::center_head_target(), entity_id);
    }

    pub(crate) fn set_left_head_entity_id(&mut self, entity_id: i32) {
        self.set_head_entity_id(&definitions::wither::left_head_target(), entity_id);
    }

    pub(crate) fn set_right_head_entity_id(&mut self, entity_id: i32) {
        self.set_head_entity_id(&definitions::wither::right_head_target(), entity_id);
    }

    fn head_entity_id(&self, definition: &crate::entity::metadata::MetadataDefinition) -> i32 {
        match self.get_entity().get_metadata().get_value(definition) {
            MetadataValue::VarInt(entity_id) => entity_id,
            _ => 0,
        }
    }

    fn set_head_entity_id(
        &mut self,
        definition: &crate::entity::metadata::MetadataDefinition,
        entity_id: i32,
    ) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set(definition, MetadataValue::VarInt(entity_id));
    }
}

impl<'entity> Deref for WitherMeta<'entity> {
    type Target = MonsterMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.monster_meta
    }
}

impl<'entity> DerefMut for WitherMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster_meta
    }
}
