use crate::entity::metadata::LivingEntityMetaRef;
use crate::entity::{EntityPose, GenericEntity};
use spinel_utils::component::text::TextComponent;

#[derive(Clone, Copy)]
pub struct EntityMetaRef<'entity> {
    entity: &'entity GenericEntity,
}

impl<'entity> EntityMetaRef<'entity> {
    pub(crate) const fn new(entity: &'entity GenericEntity) -> Self {
        Self { entity }
    }

    pub fn as_living(self) -> Option<LivingEntityMetaRef<'entity>> {
        self.entity
            .get_entity_type()
            .is_living()
            .then(|| LivingEntityMetaRef::from_entity_meta(self))
    }

    pub fn is_on_fire(&self) -> bool {
        self.entity.is_on_fire()
    }

    pub fn is_invisible(&self) -> bool {
        self.entity.is_invisible()
    }

    pub fn get_custom_name(&self) -> Option<TextComponent> {
        self.entity.get_custom_name()
    }

    pub fn is_custom_name_visible(&self) -> bool {
        self.entity.is_custom_name_visible()
    }

    pub fn is_silent(&self) -> bool {
        self.entity.is_silent()
    }

    pub fn has_no_gravity(&self) -> bool {
        self.entity.has_no_gravity()
    }

    pub fn get_pose(&self) -> EntityPose {
        self.entity.get_pose()
    }

    pub fn get_ticks_frozen(&self) -> i32 {
        self.entity.get_ticks_frozen()
    }

    pub(crate) const fn get_entity(&self) -> &GenericEntity {
        self.entity
    }
}
