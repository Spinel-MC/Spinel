use crate::entity::metadata::{EntityMeta, definitions};
use spinel_network::types::Slot;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::{EntityType, ItemStack};
use std::ops::{Deref, DerefMut};

pub struct OminousItemSpawnerMeta<'entity> {
    entity_meta: EntityMeta<'entity>,
}

impl<'entity> OminousItemSpawnerMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::OMINOUS_ITEM_SPAWNER)
            .then_some(Self { entity_meta })
    }

    pub fn get_item(&self) -> ItemStack {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::ominous_item_spawner::get_item())
        {
            MetadataValue::Slot(item) => item.to_item_stack(),
            _ => ItemStack::air(),
        }
    }

    pub fn set_item(&mut self, item: ItemStack) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::ominous_item_spawner::get_item(),
            MetadataValue::Slot(Slot::from_item_stack(&item)),
        );
    }
}

impl<'entity> Deref for OminousItemSpawnerMeta<'entity> {
    type Target = EntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.entity_meta
    }
}

impl<'entity> DerefMut for OminousItemSpawnerMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_meta
    }
}
