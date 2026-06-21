use crate::entity::metadata::{EntityMeta, HangingMeta, definitions};
use spinel_network::types::Slot;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::{EntityType, ItemStack};
use std::ops::{Deref, DerefMut};

pub struct ItemFrameMeta<'entity> {
    hanging_meta: HangingMeta<'entity>,
}

impl<'entity> ItemFrameMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        is_item_frame_type(entity_meta.get_entity().get_entity_type()).then(|| Self {
            hanging_meta: HangingMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_item(&self) -> ItemStack {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::item_frame::get_item())
        {
            MetadataValue::Slot(item) => item.to_item_stack(),
            _ => ItemStack::air(),
        }
    }

    pub fn set_item(&mut self, item: ItemStack) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::item_frame::get_item(),
            MetadataValue::Slot(Slot::from_item_stack(&item)),
        );
    }

    pub fn get_rotation(&self) -> i32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::item_frame::get_rotation())
        {
            MetadataValue::VarInt(rotation) => rotation,
            _ => 0,
        }
    }

    pub fn set_rotation(&mut self, rotation: i32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::item_frame::get_rotation(),
            MetadataValue::VarInt(rotation),
        );
    }
}

impl<'entity> Deref for ItemFrameMeta<'entity> {
    type Target = HangingMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.hanging_meta
    }
}

impl<'entity> DerefMut for ItemFrameMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.hanging_meta
    }
}

fn is_item_frame_type(entity_type: EntityType) -> bool {
    matches!(
        entity_type,
        EntityType::ITEM_FRAME | EntityType::GLOW_ITEM_FRAME
    )
}
