use crate::entity::metadata::{EntityMeta, VillagerData, ZombieMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct ZombieVillagerMeta<'entity> {
    zombie_meta: ZombieMeta<'entity>,
}

impl<'entity> ZombieVillagerMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::ZOMBIE_VILLAGER).then(|| Self {
            zombie_meta: ZombieMeta::from_subtype_entity_meta(entity_meta),
        })
    }

    pub fn is_converting(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::zombie_villager::is_converting())
        {
            MetadataValue::Boolean(is_converting) => is_converting,
            _ => false,
        }
    }

    pub fn set_converting(&mut self, is_converting: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::zombie_villager::is_converting(),
            MetadataValue::Boolean(is_converting),
        );
    }

    pub fn get_villager_data(&self) -> VillagerData {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::zombie_villager::get_villager_data())
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
            &definitions::zombie_villager::get_villager_data(),
            MetadataValue::VillagerData(
                villager_data.get_villager_type().protocol_id(),
                villager_data.get_profession().protocol_id(),
                villager_data.get_level().get_protocol_id(),
            ),
        );
    }
}

impl<'entity> Deref for ZombieVillagerMeta<'entity> {
    type Target = ZombieMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.zombie_meta
    }
}

impl<'entity> DerefMut for ZombieVillagerMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.zombie_meta
    }
}
