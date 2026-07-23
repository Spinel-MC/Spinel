use crate::entity::metadata::{
    MetadataBitMaskDefinition, MetadataByteMaskDefinition, MetadataDefinition,
};
use crate::entity::{EntityPose, EntityState};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_utils::component::text::TextComponent;

pub struct EntityMeta<'entity> {
    state: &'entity mut EntityState,
}

impl<'entity> EntityMeta<'entity> {
    pub(crate) fn new(state: &'entity mut EntityState) -> Self {
        Self { state }
    }

    pub fn set_notify_about_changes(&mut self, should_notify_about_changes: bool) {
        self.state
            .get_metadata_mut()
            .set_change_notifications_enabled(should_notify_about_changes);
    }

    pub fn get_metadata_value(&self, definition: &MetadataDefinition) -> MetadataValue {
        self.state.get_metadata().get_value(definition)
    }

    pub fn set_metadata_value(&mut self, definition: &MetadataDefinition, value: MetadataValue) {
        self.state.get_metadata_mut().set(definition, value);
    }

    pub fn get_metadata_flag(&self, definition: &MetadataBitMaskDefinition) -> bool {
        self.state.get_metadata().get_flag(definition)
    }

    pub fn set_metadata_flag(
        &mut self,
        definition: &MetadataBitMaskDefinition,
        flag_is_enabled: bool,
    ) {
        self.state
            .get_metadata_mut()
            .set_flag(definition, flag_is_enabled);
    }

    pub fn get_metadata_byte(&self, definition: &MetadataByteMaskDefinition) -> i8 {
        self.state.get_metadata().get_byte(definition)
    }

    pub fn set_metadata_byte(&mut self, definition: &MetadataByteMaskDefinition, byte_value: i8) {
        self.state
            .get_metadata_mut()
            .set_byte(definition, byte_value);
    }

    pub fn is_on_fire(&self) -> bool {
        self.get_metadata_flag(&crate::entity::metadata::definitions::is_on_fire())
    }

    pub fn set_on_fire(&mut self, is_on_fire: bool) {
        self.set_metadata_flag(
            &crate::entity::metadata::definitions::is_on_fire(),
            is_on_fire,
        );
    }

    pub fn is_sneaking(&self) -> bool {
        self.get_metadata_flag(&crate::entity::metadata::definitions::is_crouching())
    }

    pub fn set_sneaking(&mut self, is_sneaking: bool) {
        self.set_metadata_flag(
            &crate::entity::metadata::definitions::is_crouching(),
            is_sneaking,
        );
    }

    pub fn is_sprinting(&self) -> bool {
        self.get_metadata_flag(&crate::entity::metadata::definitions::is_sprinting())
    }

    pub fn set_sprinting(&mut self, is_sprinting: bool) {
        self.set_metadata_flag(
            &crate::entity::metadata::definitions::is_sprinting(),
            is_sprinting,
        );
    }

    pub fn is_swimming(&self) -> bool {
        self.get_metadata_flag(&crate::entity::metadata::definitions::is_swimming())
    }

    pub fn set_swimming(&mut self, is_swimming: bool) {
        self.set_metadata_flag(
            &crate::entity::metadata::definitions::is_swimming(),
            is_swimming,
        );
    }

    pub fn is_invisible(&self) -> bool {
        self.get_metadata_flag(&crate::entity::metadata::definitions::is_invisible())
    }

    pub fn set_invisible(&mut self, is_invisible: bool) {
        self.set_metadata_flag(
            &crate::entity::metadata::definitions::is_invisible(),
            is_invisible,
        );
    }

    pub fn has_glowing_effect(&self) -> bool {
        self.get_metadata_flag(&crate::entity::metadata::definitions::has_glowing_effect())
    }

    pub fn set_has_glowing_effect(&mut self, has_glowing_effect: bool) {
        self.set_metadata_flag(
            &crate::entity::metadata::definitions::has_glowing_effect(),
            has_glowing_effect,
        );
    }

    pub fn get_air_ticks(&self) -> i32 {
        match self.get_metadata_value(&crate::entity::metadata::definitions::get_air_ticks()) {
            MetadataValue::VarInt(air_ticks) => air_ticks,
            _ => 300,
        }
    }

    pub fn set_air_ticks(&mut self, air_ticks: i32) {
        self.set_metadata_value(
            &crate::entity::metadata::definitions::get_air_ticks(),
            MetadataValue::VarInt(air_ticks),
        );
    }

    pub fn get_custom_name(&self) -> Option<TextComponent> {
        match self.get_metadata_value(&crate::entity::metadata::definitions::get_custom_name()) {
            MetadataValue::OptionalText(custom_name) => custom_name,
            _ => None,
        }
    }

    pub fn set_custom_name(&mut self, custom_name: Option<TextComponent>) {
        self.set_metadata_value(
            &crate::entity::metadata::definitions::get_custom_name(),
            MetadataValue::OptionalText(custom_name),
        );
    }

    pub fn is_custom_name_visible(&self) -> bool {
        match self.get_metadata_value(&crate::entity::metadata::definitions::custom_name_visible())
        {
            MetadataValue::Boolean(is_custom_name_visible) => is_custom_name_visible,
            _ => false,
        }
    }

    pub fn set_custom_name_visible(&mut self, is_custom_name_visible: bool) {
        self.set_metadata_value(
            &crate::entity::metadata::definitions::custom_name_visible(),
            MetadataValue::Boolean(is_custom_name_visible),
        );
    }

    pub fn is_silent(&self) -> bool {
        match self.get_metadata_value(&crate::entity::metadata::definitions::is_silent()) {
            MetadataValue::Boolean(is_silent) => is_silent,
            _ => false,
        }
    }

    pub fn set_silent(&mut self, is_silent: bool) {
        self.set_metadata_value(
            &crate::entity::metadata::definitions::is_silent(),
            MetadataValue::Boolean(is_silent),
        );
    }

    pub fn has_no_gravity(&self) -> bool {
        match self.get_metadata_value(&crate::entity::metadata::definitions::has_no_gravity()) {
            MetadataValue::Boolean(has_no_gravity) => has_no_gravity,
            _ => false,
        }
    }

    pub fn set_has_no_gravity(&mut self, has_no_gravity: bool) {
        self.set_metadata_value(
            &crate::entity::metadata::definitions::has_no_gravity(),
            MetadataValue::Boolean(has_no_gravity),
        );
    }

    pub fn get_pose(&self) -> EntityPose {
        match self.get_metadata_value(&crate::entity::metadata::definitions::get_pose()) {
            MetadataValue::Pose(pose) => {
                EntityPose::from_protocol_id(pose).unwrap_or(EntityPose::Standing)
            }
            _ => EntityPose::Standing,
        }
    }

    pub fn set_pose(&mut self, pose: EntityPose) {
        self.set_metadata_value(
            &crate::entity::metadata::definitions::get_pose(),
            MetadataValue::Pose(pose.get_protocol_id()),
        );
    }

    pub fn get_ticks_frozen(&self) -> i32 {
        match self.get_metadata_value(&crate::entity::metadata::definitions::ticks_frozen()) {
            MetadataValue::VarInt(ticks_frozen) => ticks_frozen,
            _ => 0,
        }
    }

    pub fn set_ticks_frozen(&mut self, ticks_frozen: i32) {
        self.set_metadata_value(
            &crate::entity::metadata::definitions::ticks_frozen(),
            MetadataValue::VarInt(ticks_frozen),
        );
    }

    pub(crate) const fn get_entity_type(&self) -> spinel_registry::EntityType {
        self.state.get_entity_type()
    }

    pub(crate) const fn get_entity_state(&self) -> &EntityState {
        self.state
    }

    pub(crate) fn get_entity_state_mut(&mut self) -> &mut EntityState {
        self.state
    }

    pub(crate) const fn get_state(&self) -> &EntityState {
        self.state
    }

    pub(crate) fn get_state_mut(&mut self) -> &mut EntityState {
        self.state
    }
}
