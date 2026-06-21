use crate::entity::metadata::{AbstractGolemMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::{BlockFaceDirection, EntityType};
use spinel_utils::color::DyeColor;
use std::ops::{Deref, DerefMut};

pub struct ShulkerMeta<'entity> {
    abstract_golem_meta: AbstractGolemMeta<'entity>,
}

impl<'entity> ShulkerMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::SHULKER).then(|| Self {
            abstract_golem_meta: AbstractGolemMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_attach_face(&self) -> BlockFaceDirection {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::shulker::get_attach_face())
        {
            MetadataValue::Direction(direction_id) => {
                BlockFaceDirection::from_protocol_id(direction_id)
                    .unwrap_or(BlockFaceDirection::Down)
            }
            _ => BlockFaceDirection::Down,
        }
    }

    pub fn set_attach_face(&mut self, attach_face: BlockFaceDirection) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::shulker::get_attach_face(),
            MetadataValue::Direction(attach_face.protocol_id()),
        );
    }

    pub fn get_shield_height(&self) -> i8 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::shulker::get_shield_height())
        {
            MetadataValue::Byte(shield_height) => shield_height,
            _ => 0,
        }
    }

    pub fn set_shield_height(&mut self, shield_height: i8) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::shulker::get_shield_height(),
            MetadataValue::Byte(shield_height),
        );
    }

    pub fn get_color(&self) -> DyeColor {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::shulker::get_color())
        {
            MetadataValue::Byte(16) => DyeColor::Purple,
            MetadataValue::Byte(color_id) => DyeColor::ALL
                .get(color_id as usize)
                .copied()
                .unwrap_or(DyeColor::Purple),
            _ => DyeColor::Purple,
        }
    }

    pub fn set_color(&mut self, color: DyeColor) {
        let color_id = DyeColor::ALL
            .iter()
            .position(|candidate| candidate == &color)
            .unwrap_or(10) as i8;
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::shulker::get_color(),
            MetadataValue::Byte(color_id),
        );
    }
}

impl<'entity> Deref for ShulkerMeta<'entity> {
    type Target = AbstractGolemMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_golem_meta
    }
}

impl<'entity> DerefMut for ShulkerMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_golem_meta
    }
}
