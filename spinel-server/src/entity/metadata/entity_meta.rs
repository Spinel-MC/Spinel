use crate::entity::GenericEntity;
use crate::entity::metadata::{ArmadilloMeta, BeeMeta, GhastMeta, GuardianMeta, VexMeta};
use spinel_utils::component::text::TextComponent;

pub struct EntityMeta<'entity> {
    entity: &'entity mut GenericEntity,
}

impl<'entity> EntityMeta<'entity> {
    pub(crate) fn new(entity: &'entity mut GenericEntity) -> Self {
        Self { entity }
    }

    pub fn bee(self) -> Option<BeeMeta<'entity>> {
        BeeMeta::from_entity_meta(self)
    }

    pub fn armadillo(self) -> Option<ArmadilloMeta<'entity>> {
        ArmadilloMeta::from_entity_meta(self)
    }

    pub fn ghast(self) -> Option<GhastMeta<'entity>> {
        GhastMeta::from_entity_meta(self)
    }

    pub fn guardian(self) -> Option<GuardianMeta<'entity>> {
        GuardianMeta::from_entity_meta(self)
    }

    pub fn vex(self) -> Option<VexMeta<'entity>> {
        VexMeta::from_entity_meta(self)
    }

    pub fn set_notify_about_changes(&mut self, should_notify_about_changes: bool) {
        self.entity
            .metadata_mut()
            .set_change_notifications_enabled(should_notify_about_changes);
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

    pub fn air_ticks(&self) -> i32 {
        self.entity.air_ticks()
    }

    pub fn set_air_ticks(&mut self, air_ticks: i32) {
        self.entity.set_air_ticks(air_ticks);
    }

    pub fn custom_name(&self) -> Option<TextComponent> {
        self.entity.custom_name()
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

    pub fn pose(&self) -> i32 {
        self.entity.pose()
    }

    pub fn set_pose(&mut self, pose: i32) {
        self.entity.set_pose(pose);
    }

    pub fn ticks_frozen(&self) -> i32 {
        self.entity.ticks_frozen()
    }

    pub fn set_ticks_frozen(&mut self, ticks_frozen: i32) {
        self.entity.set_ticks_frozen(ticks_frozen);
    }

    pub(crate) fn entity(&self) -> &GenericEntity {
        self.entity
    }

    pub(crate) fn entity_mut(&mut self) -> &mut GenericEntity {
        self.entity
    }
}
