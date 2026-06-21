use crate::entity::metadata::{AbstractMinecartMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct FurnaceMinecartMeta<'entity> {
    abstract_minecart_meta: AbstractMinecartMeta<'entity>,
}

impl<'entity> FurnaceMinecartMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::FURNACE_MINECART).then(|| Self {
            abstract_minecart_meta: AbstractMinecartMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn has_fuel(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::furnace_minecart::has_fuel())
        {
            MetadataValue::Boolean(has_fuel) => has_fuel,
            _ => false,
        }
    }

    pub fn set_has_fuel(&mut self, has_fuel: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::furnace_minecart::has_fuel(),
            MetadataValue::Boolean(has_fuel),
        );
    }
}

impl<'entity> Deref for FurnaceMinecartMeta<'entity> {
    type Target = AbstractMinecartMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_minecart_meta
    }
}

impl<'entity> DerefMut for FurnaceMinecartMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_minecart_meta
    }
}
