use crate::entity::metadata::{AgeableWaterAnimalMeta, EntityMeta, definitions};
use spinel_network::types::Position;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct DolphinMeta<'entity> {
    ageable_water_animal_meta: AgeableWaterAnimalMeta<'entity>,
}

impl<'entity> DolphinMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::DOLPHIN).then(|| Self {
            ageable_water_animal_meta: AgeableWaterAnimalMeta::new(entity_meta),
        })
    }

    pub fn get_treasure_position(&self) -> Position {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::dolphin::get_treasure_position())
        {
            MetadataValue::Position(position) => position,
            _ => Position { x: 0, y: 0, z: 0 },
        }
    }

    pub fn set_treasure_position(&mut self, treasure_position: Position) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::dolphin::get_treasure_position(),
            MetadataValue::Position(treasure_position),
        );
    }

    pub fn has_fish(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::dolphin::has_fish())
        {
            MetadataValue::Boolean(has_fish) => has_fish,
            _ => false,
        }
    }

    pub fn set_has_fish(&mut self, has_fish: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::dolphin::has_fish(),
            MetadataValue::Boolean(has_fish),
        );
    }

    pub fn get_moisture_level(&self) -> i32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::dolphin::get_moisture_level())
        {
            MetadataValue::VarInt(moisture_level) => moisture_level,
            _ => 2400,
        }
    }

    pub fn set_moisture_level(&mut self, moisture_level: i32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::dolphin::get_moisture_level(),
            MetadataValue::VarInt(moisture_level),
        );
    }
}

impl<'entity> Deref for DolphinMeta<'entity> {
    type Target = AgeableWaterAnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.ageable_water_animal_meta
    }
}

impl<'entity> DerefMut for DolphinMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ageable_water_animal_meta
    }
}
