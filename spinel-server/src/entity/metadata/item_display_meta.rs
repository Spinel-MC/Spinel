use crate::entity::metadata::{AbstractDisplayMeta, EntityMeta, definitions};
use spinel_network::types::Slot;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::{EntityType, ItemStack};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DisplayContext {
    #[default]
    None,
    ThirdPersonLeftHand,
    ThirdPersonRightHand,
    FirstPersonLeftHand,
    FirstPersonRightHand,
    Head,
    Gui,
    Ground,
    Fixed,
}

impl DisplayContext {
    const fn protocol_id(self) -> i8 {
        self as i8
    }

    const fn from_protocol_id(protocol_id: i8) -> Self {
        match protocol_id {
            1 => Self::ThirdPersonLeftHand,
            2 => Self::ThirdPersonRightHand,
            3 => Self::FirstPersonLeftHand,
            4 => Self::FirstPersonRightHand,
            5 => Self::Head,
            6 => Self::Gui,
            7 => Self::Ground,
            8 => Self::Fixed,
            _ => Self::None,
        }
    }
}

pub struct ItemDisplayMeta<'entity> {
    abstract_display_meta: AbstractDisplayMeta<'entity>,
}

impl<'entity> ItemDisplayMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::ITEM_DISPLAY).then(|| Self {
            abstract_display_meta: AbstractDisplayMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_item_stack(&self) -> ItemStack {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::item_display::displayed_item())
        {
            MetadataValue::Slot(slot) => slot.to_item_stack(),
            _ => ItemStack::air(),
        }
    }

    pub fn set_item_stack(&mut self, item_stack: ItemStack) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::item_display::displayed_item(),
            MetadataValue::Slot(Slot::from_item_stack(&item_stack)),
        );
    }

    pub fn get_display_context(&self) -> DisplayContext {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::item_display::display_type())
        {
            MetadataValue::Byte(protocol_id) => DisplayContext::from_protocol_id(protocol_id),
            _ => DisplayContext::None,
        }
    }

    pub fn set_display_context(&mut self, display_context: DisplayContext) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::item_display::display_type(),
            MetadataValue::Byte(display_context.protocol_id()),
        );
    }
}

impl<'entity> Deref for ItemDisplayMeta<'entity> {
    type Target = AbstractDisplayMeta<'entity>;
    fn deref(&self) -> &Self::Target {
        &self.abstract_display_meta
    }
}

impl<'entity> DerefMut for ItemDisplayMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_display_meta
    }
}
