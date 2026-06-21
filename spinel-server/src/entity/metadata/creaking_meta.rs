use crate::entity::metadata::{EntityMeta, MonsterMeta, definitions};
use spinel_network::types::Position;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct CreakingMeta<'entity> {
    monster_meta: MonsterMeta<'entity>,
}

impl<'entity> CreakingMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::CREAKING).then(|| Self {
            monster_meta: MonsterMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn can_move(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::creaking::can_move())
        {
            MetadataValue::Boolean(can_move) => can_move,
            _ => true,
        }
    }

    pub fn set_can_move(&mut self, can_move: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::creaking::can_move(),
            MetadataValue::Boolean(can_move),
        );
    }

    pub fn is_active(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::creaking::is_active())
        {
            MetadataValue::Boolean(is_active) => is_active,
            _ => false,
        }
    }

    pub fn set_active(&mut self, is_active: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::creaking::is_active(),
            MetadataValue::Boolean(is_active),
        );
    }

    pub fn is_tearing_down(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::creaking::is_tearing_down())
        {
            MetadataValue::Boolean(is_tearing_down) => is_tearing_down,
            _ => false,
        }
    }

    pub fn set_tearing_down(&mut self, is_tearing_down: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::creaking::is_tearing_down(),
            MetadataValue::Boolean(is_tearing_down),
        );
    }

    pub fn get_home_position(&self) -> Option<Position> {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::creaking::home_position())
        {
            MetadataValue::OptionalPosition(home_position) => home_position,
            _ => None,
        }
    }

    pub fn set_home_position(&mut self, home_position: Option<Position>) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::creaking::home_position(),
            MetadataValue::OptionalPosition(home_position),
        );
    }
}

impl<'entity> Deref for CreakingMeta<'entity> {
    type Target = MonsterMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.monster_meta
    }
}

impl<'entity> DerefMut for CreakingMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster_meta
    }
}
