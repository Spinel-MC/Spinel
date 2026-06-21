use crate::entity::dynamic_variant::UnregisteredEntityVariantError;
use crate::entity::metadata::{EntityMeta, TameableAnimalMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::Registries;
use spinel_registry::{EntityType, RegistryKey, wolf_sound_variant, wolf_variant};
use spinel_utils::color::DyeColor;
use std::ops::{Deref, DerefMut};

pub struct WolfMeta<'entity> {
    tameable_animal_meta: TameableAnimalMeta<'entity>,
}

impl<'entity> WolfMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::WOLF).then(|| Self {
            tameable_animal_meta: TameableAnimalMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_variant(
        &self,
        registries: &Registries,
    ) -> Option<RegistryKey<wolf_variant::WolfVariant>> {
        self.get_entity().g(registries)
    }

    pub fn set_variant(
        &mut self,
        registries: &Registries,
        variant: RegistryKey<wolf_variant::WolfVariant>,
    ) -> Result<(), UnregisteredEntityVariantError> {
        self.get_entity_mut()
            .set_wolf_variant_metadata(registries, variant)
    }

    pub fn get_sound_variant(
        &self,
        registries: &Registries,
    ) -> Option<RegistryKey<wolf_sound_variant::WolfSoundVariant>> {
        self.get_entity().g(registries)
    }

    pub fn set_sound_variant(
        &mut self,
        registries: &Registries,
        sound_variant: RegistryKey<wolf_sound_variant::WolfSoundVariant>,
    ) -> Result<(), UnregisteredEntityVariantError> {
        self.get_entity_mut()
            .set_wolf_sound_variant_metadata(registries, sound_variant)
    }

    pub fn is_begging(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::wolf::is_begging())
        {
            MetadataValue::Boolean(is_begging) => is_begging,
            _ => false,
        }
    }

    pub fn set_begging(&mut self, is_begging: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::wolf::is_begging(),
            MetadataValue::Boolean(is_begging),
        );
    }

    pub fn get_collar_color(&self) -> DyeColor {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::wolf::get_collar_color())
        {
            MetadataValue::VarInt(color_id) => DyeColor::ALL
                .get(color_id as usize)
                .copied()
                .unwrap_or(DyeColor::Red),
            _ => DyeColor::Red,
        }
    }

    pub fn set_collar_color(&mut self, collar_color: DyeColor) {
        let color_id = DyeColor::ALL
            .iter()
            .position(|candidate| candidate == &collar_color)
            .unwrap_or(14) as i32;
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::wolf::get_collar_color(),
            MetadataValue::VarInt(color_id),
        );
    }

    pub fn get_anger_time(&self) -> i64 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::wolf::get_anger_time())
        {
            MetadataValue::Long(anger_time) => anger_time,
            _ => -1,
        }
    }

    pub fn set_anger_time(&mut self, anger_time: i64) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::wolf::get_anger_time(),
            MetadataValue::Long(anger_time),
        );
    }
}

impl<'entity> Deref for WolfMeta<'entity> {
    type Target = TameableAnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.tameable_animal_meta
    }
}

impl<'entity> DerefMut for WolfMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tameable_animal_meta
    }
}
