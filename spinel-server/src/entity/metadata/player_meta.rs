use crate::entity::metadata::{AvatarMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct PlayerMeta<'entity> {
    avatar_meta: AvatarMeta<'entity>,
}

impl<'entity> PlayerMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::PLAYER).then(|| Self {
            avatar_meta: AvatarMeta::from_player_entity_meta(entity_meta),
        })
    }

    pub fn get_additional_hearts(&self) -> f32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::player::get_additional_hearts())
        {
            MetadataValue::Float(additional_hearts) => additional_hearts,
            _ => 0.0,
        }
    }

    pub fn set_additional_hearts(&mut self, additional_hearts: f32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::player::get_additional_hearts(),
            MetadataValue::Float(additional_hearts),
        );
    }

    pub fn get_score(&self) -> i32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::player::get_score())
        {
            MetadataValue::VarInt(score) => score,
            _ => 0,
        }
    }

    pub fn set_score(&mut self, score: i32) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set(&definitions::player::get_score(), MetadataValue::VarInt(score));
    }

    pub fn get_left_shoulder_entity_data(&self) -> Option<i32> {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::player::get_left_shoulder_entity_data())
        {
            MetadataValue::OptionalVarInt(left_shoulder_entity_data) => left_shoulder_entity_data,
            _ => None,
        }
    }

    pub fn set_left_shoulder_entity_data(&mut self, left_shoulder_entity_data: Option<i32>) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::player::get_left_shoulder_entity_data(),
            MetadataValue::OptionalVarInt(left_shoulder_entity_data),
        );
    }

    pub fn get_right_shoulder_entity_data(&self) -> Option<i32> {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::player::get_right_shoulder_entity_data())
        {
            MetadataValue::OptionalVarInt(right_shoulder_entity_data) => right_shoulder_entity_data,
            _ => None,
        }
    }

    pub fn set_right_shoulder_entity_data(&mut self, right_shoulder_entity_data: Option<i32>) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::player::get_right_shoulder_entity_data(),
            MetadataValue::OptionalVarInt(right_shoulder_entity_data),
        );
    }
}

impl<'entity> Deref for PlayerMeta<'entity> {
    type Target = AvatarMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.avatar_meta
    }
}

impl<'entity> DerefMut for PlayerMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.avatar_meta
    }
}
