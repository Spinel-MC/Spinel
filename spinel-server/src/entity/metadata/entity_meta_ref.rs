use crate::entity::{EntityPose, EntityState};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_utils::component::text::TextComponent;

#[derive(Clone, Copy)]
pub struct EntityMetaRef<'entity> {
    state: &'entity EntityState,
}

impl<'entity> EntityMetaRef<'entity> {
    pub(crate) const fn new(state: &'entity EntityState) -> Self {
        Self { state }
    }

    pub fn is_on_fire(&self) -> bool {
        self.state
            .get_metadata()
            .get_flag(&crate::entity::metadata::definitions::is_on_fire())
    }

    pub fn is_invisible(&self) -> bool {
        self.state
            .get_metadata()
            .get_flag(&crate::entity::metadata::definitions::is_invisible())
    }

    pub fn get_custom_name(&self) -> Option<TextComponent> {
        match self
            .state
            .get_metadata()
            .get_value(&crate::entity::metadata::definitions::get_custom_name())
        {
            MetadataValue::OptionalText(custom_name) => custom_name,
            _ => None,
        }
    }

    pub fn is_custom_name_visible(&self) -> bool {
        match self
            .state
            .get_metadata()
            .get_value(&crate::entity::metadata::definitions::custom_name_visible())
        {
            MetadataValue::Boolean(is_custom_name_visible) => is_custom_name_visible,
            _ => false,
        }
    }

    pub fn is_silent(&self) -> bool {
        match self
            .state
            .get_metadata()
            .get_value(&crate::entity::metadata::definitions::is_silent())
        {
            MetadataValue::Boolean(is_silent) => is_silent,
            _ => false,
        }
    }

    pub fn has_no_gravity(&self) -> bool {
        match self
            .state
            .get_metadata()
            .get_value(&crate::entity::metadata::definitions::has_no_gravity())
        {
            MetadataValue::Boolean(has_no_gravity) => has_no_gravity,
            _ => false,
        }
    }

    pub fn get_pose(&self) -> EntityPose {
        match self
            .state
            .get_metadata()
            .get_value(&crate::entity::metadata::definitions::get_pose())
        {
            MetadataValue::Pose(pose) => {
                EntityPose::from_protocol_id(pose).unwrap_or(EntityPose::Standing)
            }
            _ => EntityPose::Standing,
        }
    }

    pub fn get_ticks_frozen(&self) -> i32 {
        match self
            .state
            .get_metadata()
            .get_value(&crate::entity::metadata::definitions::ticks_frozen())
        {
            MetadataValue::VarInt(ticks_frozen) => ticks_frozen,
            _ => 0,
        }
    }

    pub(crate) const fn get_state(&self) -> &EntityState {
        self.state
    }
}
