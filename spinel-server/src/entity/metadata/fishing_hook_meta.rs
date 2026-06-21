use crate::entity::EntityId;
use crate::entity::metadata::{EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct FishingHookMeta<'entity> {
    entity_meta: EntityMeta<'entity>,
}

impl<'entity> FishingHookMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::FISHING_BOBBER)
            .then_some(Self { entity_meta })
    }

    pub fn get_hooked_entity_id(&self) -> i32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::fishing_hook::get_hooked_entity_id())
        {
            MetadataValue::VarInt(hooked_entity_id) => hooked_entity_id,
            _ => 0,
        }
    }

    pub fn set_hooked_entity(&mut self, hooked_entity_id: Option<EntityId>) {
        let protocol_entity_id = hooked_entity_id.map_or(0, |entity_id| entity_id.get_value() + 1);
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::fishing_hook::get_hooked_entity_id(),
            MetadataValue::VarInt(protocol_entity_id),
        );
    }

    pub fn is_catchable(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::fishing_hook::is_catchable())
        {
            MetadataValue::Boolean(is_catchable) => is_catchable,
            _ => false,
        }
    }

    pub fn set_catchable(&mut self, is_catchable: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::fishing_hook::is_catchable(),
            MetadataValue::Boolean(is_catchable),
        );
    }

    pub fn get_owner_entity_id(&self) -> Option<EntityId> {
        self.get_entity().get_fishing_hook_owner_entity_id()
    }

    pub fn set_owner_entity(&mut self, owner_entity_id: Option<EntityId>) {
        self.get_entity_mut()
            .set_fishing_hook_owner_entity_id(owner_entity_id);
    }
}

impl<'entity> Deref for FishingHookMeta<'entity> {
    type Target = EntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.entity_meta
    }
}

impl<'entity> DerefMut for FishingHookMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_meta
    }
}
