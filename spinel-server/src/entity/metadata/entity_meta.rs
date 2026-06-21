use crate::entity::metadata::{
    MetadataBitMaskDefinition, MetadataByteMaskDefinition, MetadataDefinition,
};
use crate::entity::{EntityPose, GenericEntity};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_utils::component::text::TextComponent;

pub struct EntityMeta<'entity> {
    entity: &'entity mut GenericEntity,
}

impl<'entity> EntityMeta<'entity> {
    pub(crate) fn new(entity: &'entity mut GenericEntity) -> Self {
        Self { entity }
    }

    pub fn set_notify_about_changes(&mut self, should_notify_about_changes: bool) {
        self.entity
            .get_metadata_mut()
            .set_change_notifications_enabled(should_notify_about_changes);
    }

    pub fn get_metadata_value(&self, definition: &MetadataDefinition) -> MetadataValue {
        self.entity.get_metadata().get_value(definition)
    }

    pub fn set_metadata_value(&mut self, definition: &MetadataDefinition, value: MetadataValue) {
        self.entity.get_metadata_mut().set(definition, value);
    }

    pub fn get_metadata_flag(&self, definition: &MetadataBitMaskDefinition) -> bool {
        self.entity.get_metadata().flag(definition)
    }

    pub fn set_metadata_flag(
        &mut self,
        definition: &MetadataBitMaskDefinition,
        flag_is_enabled: bool,
    ) {
        self.entity
            .get_metadata_mut()
            .set_flag(definition, flag_is_enabled);
    }

    pub fn get_metadata_byte(&self, definition: &MetadataByteMaskDefinition) -> i8 {
        self.entity.get_metadata().byte(definition)
    }

    pub fn set_metadata_byte(&mut self, definition: &MetadataByteMaskDefinition, byte_value: i8) {
        self.entity.get_metadata_mut().set_byte(definition, byte_value);
    }
    pub fn is_on_fire(&self) -> bool {
        self.entity.is_on_fire()
    }

    pub fn set_on_fire(&mut self, is_on_fire: bool) {
        self.entity.set_on_fire(is_on_fire);
    }

    pub fn is_sneaking(&self) -> bool {
        self.entity.is_sneaking()
    }

    pub fn set_sneaking(&mut self, is_sneaking: bool) {
        self.entity.set_sneaking(is_sneaking);
    }

    pub fn is_sprinting(&self) -> bool {
        self.entity.is_sprinting()
    }

    pub fn set_sprinting(&mut self, is_sprinting: bool) {
        self.entity.set_sprinting(is_sprinting);
    }

    pub fn is_swimming(&self) -> bool {
        self.entity.is_swimming()
    }

    pub fn set_swimming(&mut self, is_swimming: bool) {
        self.entity.set_swimming(is_swimming);
    }

    pub fn is_invisible(&self) -> bool {
        self.entity.is_invisible()
    }

    pub fn set_invisible(&mut self, is_invisible: bool) {
        self.entity.set_invisible(is_invisible);
    }

    pub fn has_glowing_effect(&self) -> bool {
        self.entity.is_glowing()
    }

    pub fn set_has_glowing_effect(&mut self, has_glowing_effect: bool) {
        self.entity.set_glowing(has_glowing_effect);
    }

    pub fn get_air_ticks(&self) -> i32 {
        self.entity.get_air_ticks()
    }

    pub fn set_air_ticks(&mut self, air_ticks: i32) {
        self.entity.set_air_ticks(air_ticks);
    }

    pub fn get_custom_name(&self) -> Option<TextComponent> {
        self.entity.get_custom_name()
    }

    pub fn set_custom_name(&mut self, custom_name: Option<TextComponent>) {
        self.entity.set_custom_name(custom_name);
    }

    pub fn is_custom_name_visible(&self) -> bool {
        self.entity.is_custom_name_visible()
    }

    pub fn set_custom_name_visible(&mut self, is_custom_name_visible: bool) {
        self.entity.set_custom_name_visible(is_custom_name_visible);
    }

    pub fn is_silent(&self) -> bool {
        self.entity.is_silent()
    }

    pub fn set_silent(&mut self, is_silent: bool) {
        self.entity.set_silent(is_silent);
    }

    pub fn has_no_gravity(&self) -> bool {
        self.entity.has_no_gravity()
    }

    pub fn set_has_no_gravity(&mut self, has_no_gravity: bool) {
        self.entity.set_no_gravity(has_no_gravity);
    }

    pub fn get_pose(&self) -> EntityPose {
        self.entity.get_pose()
    }

    pub fn set_pose(&mut self, pose: EntityPose) {
        self.entity.set_pose(pose);
    }

    pub fn get_ticks_frozen(&self) -> i32 {
        self.entity.get_ticks_frozen()
    }

    pub fn set_ticks_frozen(&mut self, ticks_frozen: i32) {
        self.entity.set_ticks_frozen(ticks_frozen);
    }

    pub(crate) fn get_entity(&self) -> &GenericEntity {
        self.entity
    }

    pub(crate) fn get_entity_mut(&mut self) -> &mut GenericEntity {
        self.entity
    }
}
