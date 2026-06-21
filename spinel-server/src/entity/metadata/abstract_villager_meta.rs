use crate::entity::metadata::{AgeableMobMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use std::ops::{Deref, DerefMut};

pub struct AbstractVillagerMeta<'entity> {
    ageable_mob_meta: AgeableMobMeta<'entity>,
}

impl<'entity> AbstractVillagerMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Self {
        Self {
            ageable_mob_meta: AgeableMobMeta::new(entity_meta),
        }
    }

    pub fn get_head_shake_timer(&self) -> i32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::abstract_villager::get_head_shake_timer())
        {
            MetadataValue::VarInt(head_shake_timer) => head_shake_timer,
            _ => 0,
        }
    }

    pub fn set_head_shake_timer(&mut self, head_shake_timer: i32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::abstract_villager::get_head_shake_timer(),
            MetadataValue::VarInt(head_shake_timer),
        );
    }
}

impl<'entity> Deref for AbstractVillagerMeta<'entity> {
    type Target = AgeableMobMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.ageable_mob_meta
    }
}

impl<'entity> DerefMut for AbstractVillagerMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ageable_mob_meta
    }
}
