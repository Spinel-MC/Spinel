use crate::entity::metadata::{AbstractVillagerMeta, EntityMeta, VillagerData, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct VillagerMeta<'entity> {
    abstract_villager_meta: AbstractVillagerMeta<'entity>,
}

impl<'entity> VillagerMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::VILLAGER).then(|| Self {
            abstract_villager_meta: AbstractVillagerMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_villager_data(&self) -> VillagerData {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::villager::data())
        {
            MetadataValue::VillagerData(villager_type_id, profession_id, level_id) => {
                VillagerData::from_protocol_ids(villager_type_id, profession_id, level_id)
                    .unwrap_or_default()
            }
            _ => VillagerData::default(),
        }
    }

    pub fn set_villager_data(&mut self, villager_data: VillagerData) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::villager::data(),
            MetadataValue::VillagerData(
                villager_data.get_villager_type().protocol_id(),
                villager_data.get_profession().protocol_id(),
                villager_data.get_level().get_protocol_id(),
            ),
        );
    }
}

impl<'entity> Deref for VillagerMeta<'entity> {
    type Target = AbstractVillagerMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_villager_meta
    }
}

impl<'entity> DerefMut for VillagerMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_villager_meta
    }
}
