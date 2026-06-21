use crate::entity::dynamic_variant::UnregisteredEntityVariantError;
use crate::entity::metadata::{EntityMeta, TameableAnimalMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::Registries;
use spinel_registry::{EntityType, RegistryKey, cat_variant};
use spinel_utils::color::DyeColor;
use std::ops::{Deref, DerefMut};

pub struct CatMeta<'entity> {
    tameable_animal_meta: TameableAnimalMeta<'entity>,
}

impl<'entity> CatMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::CAT).then(|| Self {
            tameable_animal_meta: TameableAnimalMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_variant(&self, registries: &Registries) -> Option<RegistryKey<cat_variant::CatVariant>> {
        self.get_entity().g(registries)
    }

    pub fn set_variant(
        &mut self,
        registries: &Registries,
        variant: RegistryKey<cat_variant::CatVariant>,
    ) -> Result<(), UnregisteredEntityVariantError> {
        self.get_entity_mut()
            .set_cat_variant_metadata(registries, variant)
    }

    pub fn is_lying(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::cat::is_lying())
        {
            MetadataValue::Boolean(is_lying) => is_lying,
            _ => false,
        }
    }

    pub fn set_lying(&mut self, is_lying: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::cat::is_lying(),
            MetadataValue::Boolean(is_lying),
        );
    }

    pub fn is_relaxed(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::cat::is_relaxed())
        {
            MetadataValue::Boolean(is_relaxed) => is_relaxed,
            _ => false,
        }
    }

    pub fn set_relaxed(&mut self, is_relaxed: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::cat::is_relaxed(),
            MetadataValue::Boolean(is_relaxed),
        );
    }

    pub fn get_collar_color(&self) -> DyeColor {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::cat::get_collar_color())
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
            &definitions::cat::get_collar_color(),
            MetadataValue::VarInt(color_id),
        );
    }
}

impl<'entity> Deref for CatMeta<'entity> {
    type Target = TameableAnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.tameable_animal_meta
    }
}

impl<'entity> DerefMut for CatMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tameable_animal_meta
    }
}
