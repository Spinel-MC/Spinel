use crate::entity::GenericEntity;
use crate::entity::metadata::definitions;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::{
    DataComponentType, DataComponentValue, DynamicRegistry, EntityType, Identifier, Registries,
    RegistryKey, cat_variant, chicken_variant, cow_variant, frog_variant, pig_variant,
    wolf_sound_variant, wolf_variant, zombie_nautilus_variant,
};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnregisteredEntityVariantError {
    registry: Identifier,
    variant: Identifier,
}

impl UnregisteredEntityVariantError {
    fn new<T>(registry: &DynamicRegistry<T>, variant: &RegistryKey<T>) -> Self {
        Self {
            registry: registry.key().clone(),
            variant: variant.key().clone(),
        }
    }

    pub const fn get_registry(&self) -> &Identifier {
        &self.registry
    }

    pub const fn get_variant(&self) -> &Identifier {
        &self.variant
    }
}

impl Display for UnregisteredEntityVariantError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "entity variant {} is not registered in {}",
            self.variant, self.registry
        )
    }
}

impl std::error::Error for UnregisteredEntityVariantError {}

impl GenericEntity {
    pub(crate) fn get_cat_variant_metadata(
        &self,
        registries: &Registries,
    ) -> Option<RegistryKey<cat_variant::CatVariant>> {
        let MetadataValue::CatVariant(variant_id) = self
            .get_metadata()
            .get_value(&definitions::cat::get_variant())
        else {
            return None;
        };
        registries.cat_variant().key_by_id(variant_id).cloned()
    }

    pub(crate) fn set_cat_variant_metadata(
        &mut self,
        registries: &Registries,
        variant: RegistryKey<cat_variant::CatVariant>,
    ) -> Result<(), UnregisteredEntityVariantError> {
        let variant_id = registered_variant_id(registries.cat_variant(), &variant)?;
        self.get_metadata_mut().set(
            &definitions::cat::get_variant(),
            MetadataValue::CatVariant(variant_id),
        );
        Ok(())
    }

    pub(crate) fn get_chicken_variant_metadata(
        &self,
        registries: &Registries,
    ) -> Option<RegistryKey<chicken_variant::ChickenVariant>> {
        let MetadataValue::ChickenVariant(variant_id) = self
            .get_metadata()
            .get_value(&definitions::chicken::get_variant())
        else {
            return None;
        };
        registries.chicken_variant().key_by_id(variant_id).cloned()
    }

    pub(crate) fn set_chicken_variant_metadata(
        &mut self,
        registries: &Registries,
        variant: RegistryKey<chicken_variant::ChickenVariant>,
    ) -> Result<(), UnregisteredEntityVariantError> {
        let variant_id = registered_variant_id(registries.chicken_variant(), &variant)?;
        self.get_metadata_mut().set(
            &definitions::chicken::get_variant(),
            MetadataValue::ChickenVariant(variant_id),
        );
        Ok(())
    }

    pub(crate) fn get_cow_variant_metadata(
        &self,
        registries: &Registries,
    ) -> Option<RegistryKey<cow_variant::CowVariant>> {
        let MetadataValue::CowVariant(variant_id) = self
            .get_metadata()
            .get_value(&definitions::cow::get_variant())
        else {
            return None;
        };
        registries.cow_variant().key_by_id(variant_id).cloned()
    }

    pub(crate) fn set_cow_variant_metadata(
        &mut self,
        registries: &Registries,
        variant: RegistryKey<cow_variant::CowVariant>,
    ) -> Result<(), UnregisteredEntityVariantError> {
        let variant_id = registered_variant_id(registries.cow_variant(), &variant)?;
        self.get_metadata_mut().set(
            &definitions::cow::get_variant(),
            MetadataValue::CowVariant(variant_id),
        );
        Ok(())
    }

    pub(crate) fn get_frog_variant_metadata(
        &self,
        registries: &Registries,
    ) -> Option<RegistryKey<frog_variant::FrogVariant>> {
        let MetadataValue::FrogVariant(variant_id) = self
            .get_metadata()
            .get_value(&definitions::frog::get_variant())
        else {
            return None;
        };
        registries.frog_variant().key_by_id(variant_id).cloned()
    }

    pub(crate) fn set_frog_variant_metadata(
        &mut self,
        registries: &Registries,
        variant: RegistryKey<frog_variant::FrogVariant>,
    ) -> Result<(), UnregisteredEntityVariantError> {
        let variant_id = registered_variant_id(registries.frog_variant(), &variant)?;
        self.get_metadata_mut().set(
            &definitions::frog::get_variant(),
            MetadataValue::FrogVariant(variant_id),
        );
        Ok(())
    }

    pub(crate) fn get_pig_variant_metadata(
        &self,
        registries: &Registries,
    ) -> Option<RegistryKey<pig_variant::PigVariant>> {
        let MetadataValue::PigVariant(variant_id) = self
            .get_metadata()
            .get_value(&definitions::pig::get_variant())
        else {
            return None;
        };
        registries.pig_variant().key_by_id(variant_id).cloned()
    }

    pub(crate) fn set_pig_variant_metadata(
        &mut self,
        registries: &Registries,
        variant: RegistryKey<pig_variant::PigVariant>,
    ) -> Result<(), UnregisteredEntityVariantError> {
        let variant_id = registered_variant_id(registries.pig_variant(), &variant)?;
        self.get_metadata_mut().set(
            &definitions::pig::get_variant(),
            MetadataValue::PigVariant(variant_id),
        );
        Ok(())
    }

    pub(crate) fn get_wolf_variant_metadata(
        &self,
        registries: &Registries,
    ) -> Option<RegistryKey<wolf_variant::WolfVariant>> {
        let MetadataValue::WolfVariant(variant_id) = self
            .get_metadata()
            .get_value(&definitions::wolf::get_variant())
        else {
            return None;
        };
        registries.wolf_variant().key_by_id(variant_id).cloned()
    }

    pub(crate) fn set_wolf_variant_metadata(
        &mut self,
        registries: &Registries,
        variant: RegistryKey<wolf_variant::WolfVariant>,
    ) -> Result<(), UnregisteredEntityVariantError> {
        let variant_id = registered_variant_id(registries.wolf_variant(), &variant)?;
        self.get_metadata_mut().set(
            &definitions::wolf::get_variant(),
            MetadataValue::WolfVariant(variant_id),
        );
        Ok(())
    }

    pub(crate) fn get_wolf_sound_variant_metadata(
        &self,
        registries: &Registries,
    ) -> Option<RegistryKey<wolf_sound_variant::WolfSoundVariant>> {
        let MetadataValue::WolfSoundVariant(variant_id) = self
            .get_metadata()
            .get_value(&definitions::wolf::get_sound_variant())
        else {
            return None;
        };
        registries
            .wolf_sound_variant()
            .key_by_id(variant_id)
            .cloned()
    }

    pub(crate) fn set_wolf_sound_variant_metadata(
        &mut self,
        registries: &Registries,
        variant: RegistryKey<wolf_sound_variant::WolfSoundVariant>,
    ) -> Result<(), UnregisteredEntityVariantError> {
        let variant_id = registered_variant_id(registries.wolf_sound_variant(), &variant)?;
        self.get_metadata_mut().set(
            &definitions::wolf::get_sound_variant(),
            MetadataValue::WolfSoundVariant(variant_id),
        );
        Ok(())
    }

    pub(crate) fn get_zombie_nautilus_variant_metadata(
        &self,
        registries: &Registries,
    ) -> Option<RegistryKey<zombie_nautilus_variant::ZombieNautilusVariant>> {
        let MetadataValue::ZombieNautilusVariant(variant_id) = self
            .get_metadata()
            .get_value(&definitions::zombie_nautilus::get_variant())
        else {
            return None;
        };
        registries
            .zombie_nautilus_variant()
            .key_by_id(variant_id)
            .cloned()
    }

    pub(crate) fn set_zombie_nautilus_variant_metadata(
        &mut self,
        registries: &Registries,
        variant: RegistryKey<zombie_nautilus_variant::ZombieNautilusVariant>,
    ) -> Result<(), UnregisteredEntityVariantError> {
        let variant_id = registered_variant_id(registries.zombie_nautilus_variant(), &variant)?;
        self.get_metadata_mut().set(
            &definitions::zombie_nautilus::get_variant(),
            MetadataValue::ZombieNautilusVariant(variant_id),
        );
        Ok(())
    }

    pub fn component_with_registries<T>(
        &self,
        registries: &Registries,
        component: DataComponentType<T>,
    ) -> Option<T>
    where
        T: DataComponentValue,
    {
        use spinel_registry::data_components::vanilla_components::{
            CAT_VARIANT, CHICKEN_VARIANT, COW_VARIANT, FROG_VARIANT, PIG_VARIANT,
            WOLF_SOUND_VARIANT, WOLF_VARIANT, ZOMBIE_NAUTILUS_VARIANT,
        };

        let component_id = component.id();
        if component_id == CAT_VARIANT.id() && self.get_entity_type() == EntityType::CAT {
            return component_from_variant(self.get_cat_variant_metadata(registries)?);
        }
        if component_id == CHICKEN_VARIANT.id() && self.get_entity_type() == EntityType::CHICKEN {
            return component_from_variant(self.get_chicken_variant_metadata(registries)?);
        }
        if component_id == COW_VARIANT.id() && self.get_entity_type() == EntityType::COW {
            return component_from_variant(self.get_cow_variant_metadata(registries)?);
        }
        if component_id == FROG_VARIANT.id() && self.get_entity_type() == EntityType::FROG {
            return component_from_variant(self.get_frog_variant_metadata(registries)?);
        }
        if component_id == PIG_VARIANT.id() && self.get_entity_type() == EntityType::PIG {
            return component_from_variant(self.get_pig_variant_metadata(registries)?);
        }
        if component_id == WOLF_VARIANT.id() && self.get_entity_type() == EntityType::WOLF {
            return component_from_variant(self.get_wolf_variant_metadata(registries)?);
        }
        if component_id == WOLF_SOUND_VARIANT.id() && self.get_entity_type() == EntityType::WOLF {
            return component_from_variant(self.get_wolf_sound_variant_metadata(registries)?);
        }
        if component_id == ZOMBIE_NAUTILUS_VARIANT.id()
            && self.get_entity_type() == EntityType::ZOMBIE_NAUTILUS
        {
            return component_from_variant(self.get_zombie_nautilus_variant_metadata(registries)?);
        }
        self.component(component)
    }

    pub fn set_component_with_registries<T>(
        &mut self,
        registries: &Registries,
        component: DataComponentType<T>,
        value: T,
    ) -> Result<(), UnregisteredEntityVariantError>
    where
        T: DataComponentValue,
    {
        use spinel_registry::data_components::vanilla_components::{
            CAT_VARIANT, CHICKEN_VARIANT, COW_VARIANT, FROG_VARIANT, PIG_VARIANT,
            WOLF_SOUND_VARIANT, WOLF_VARIANT, ZOMBIE_NAUTILUS_VARIANT,
        };

        let component_id = component.id();
        let component_nbt = value.to_component_nbt();
        if component_id == CAT_VARIANT.id() && self.get_entity_type() == EntityType::CAT {
            let Some(variant) =
                RegistryKey::<cat_variant::CatVariant>::from_component_nbt(&component_nbt)
            else {
                return Ok(());
            };
            return self.set_cat_variant_metadata(registries, variant);
        }
        if component_id == CHICKEN_VARIANT.id() && self.get_entity_type() == EntityType::CHICKEN {
            let Some(variant) =
                RegistryKey::<chicken_variant::ChickenVariant>::from_component_nbt(&component_nbt)
            else {
                return Ok(());
            };
            return self.set_chicken_variant_metadata(registries, variant);
        }
        if component_id == COW_VARIANT.id() && self.get_entity_type() == EntityType::COW {
            let Some(variant) =
                RegistryKey::<cow_variant::CowVariant>::from_component_nbt(&component_nbt)
            else {
                return Ok(());
            };
            return self.set_cow_variant_metadata(registries, variant);
        }
        if component_id == FROG_VARIANT.id() && self.get_entity_type() == EntityType::FROG {
            let Some(variant) =
                RegistryKey::<frog_variant::FrogVariant>::from_component_nbt(&component_nbt)
            else {
                return Ok(());
            };
            return self.set_frog_variant_metadata(registries, variant);
        }
        if component_id == PIG_VARIANT.id() && self.get_entity_type() == EntityType::PIG {
            let Some(variant) =
                RegistryKey::<pig_variant::PigVariant>::from_component_nbt(&component_nbt)
            else {
                return Ok(());
            };
            return self.set_pig_variant_metadata(registries, variant);
        }
        if component_id == WOLF_VARIANT.id() && self.get_entity_type() == EntityType::WOLF {
            let Some(variant) =
                RegistryKey::<wolf_variant::WolfVariant>::from_component_nbt(&component_nbt)
            else {
                return Ok(());
            };
            return self.set_wolf_variant_metadata(registries, variant);
        }
        if component_id == WOLF_SOUND_VARIANT.id() && self.get_entity_type() == EntityType::WOLF {
            let Some(variant) =
                RegistryKey::<wolf_sound_variant::WolfSoundVariant>::from_component_nbt(
                    &component_nbt,
                )
            else {
                return Ok(());
            };
            return self.set_wolf_sound_variant_metadata(registries, variant);
        }
        if component_id == ZOMBIE_NAUTILUS_VARIANT.id()
            && self.get_entity_type() == EntityType::ZOMBIE_NAUTILUS
        {
            let Some(variant) =
                RegistryKey::<zombie_nautilus_variant::ZombieNautilusVariant>::from_component_nbt(
                    &component_nbt,
                )
            else {
                return Ok(());
            };
            return self.set_zombie_nautilus_variant_metadata(registries, variant);
        }
        self.set_component(component, value);
        Ok(())
    }
}

fn registered_variant_id<T>(
    registry: &DynamicRegistry<T>,
    variant: &RegistryKey<T>,
) -> Result<i32, UnregisteredEntityVariantError> {
    registry
        .id_of(variant)
        .ok_or_else(|| UnregisteredEntityVariantError::new(registry, variant))
}

fn component_from_variant<T, V>(variant: RegistryKey<V>) -> Option<T>
where
    T: DataComponentValue,
{
    T::from_component_nbt(&variant.to_component_nbt())
}
