use crate::entity::metadata::{EntityMeta, LivingEntityMeta, definitions};
use spinel_network::types::Vector3f;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct ArmorStandMeta<'entity> {
    living_entity_meta: LivingEntityMeta<'entity>,
}

impl<'entity> ArmorStandMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::ARMOR_STAND).then(|| Self {
            living_entity_meta: LivingEntityMeta::new(entity_meta),
        })
    }

    pub fn is_small(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .flag(&definitions::armor_stand::is_small())
    }

    pub fn set_small(&mut self, is_small: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::armor_stand::is_small(), is_small);
    }

    pub fn has_arms(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .flag(&definitions::armor_stand::has_arms())
    }

    pub fn set_has_arms(&mut self, has_arms: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::armor_stand::has_arms(), has_arms);
    }

    pub fn has_no_base_plate(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .flag(&definitions::armor_stand::has_no_base_plate())
    }

    pub fn set_has_no_base_plate(&mut self, has_no_base_plate: bool) {
        self.get_entity_mut().get_metadata_mut().set_flag(
            &definitions::armor_stand::has_no_base_plate(),
            has_no_base_plate,
        );
    }

    pub fn is_marker(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .flag(&definitions::armor_stand::is_marker())
    }

    pub fn set_marker(&mut self, is_marker: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::armor_stand::is_marker(), is_marker);
    }

    pub fn get_head_rotation(&self) -> Vector3f {
        self.rotation_vector(&definitions::armor_stand::get_head_rotation())
    }

    pub fn set_head_rotation(&mut self, rotation: Vector3f) {
        self.set_rotation_vector(&definitions::armor_stand::get_head_rotation(), rotation);
    }

    pub fn get_body_rotation(&self) -> Vector3f {
        self.rotation_vector(&definitions::armor_stand::get_body_rotation())
    }

    pub fn set_body_rotation(&mut self, rotation: Vector3f) {
        self.set_rotation_vector(&definitions::armor_stand::get_body_rotation(), rotation);
    }

    pub fn get_left_arm_rotation(&self) -> Vector3f {
        self.rotation_vector(&definitions::armor_stand::get_left_arm_rotation())
    }

    pub fn set_left_arm_rotation(&mut self, rotation: Vector3f) {
        self.set_rotation_vector(&definitions::armor_stand::get_left_arm_rotation(), rotation);
    }

    pub fn get_right_arm_rotation(&self) -> Vector3f {
        self.rotation_vector(&definitions::armor_stand::get_right_arm_rotation())
    }

    pub fn set_right_arm_rotation(&mut self, rotation: Vector3f) {
        self.set_rotation_vector(&definitions::armor_stand::get_right_arm_rotation(), rotation);
    }

    pub fn get_left_leg_rotation(&self) -> Vector3f {
        self.rotation_vector(&definitions::armor_stand::get_left_leg_rotation())
    }

    pub fn set_left_leg_rotation(&mut self, rotation: Vector3f) {
        self.set_rotation_vector(&definitions::armor_stand::get_left_leg_rotation(), rotation);
    }

    pub fn get_right_leg_rotation(&self) -> Vector3f {
        self.rotation_vector(&definitions::armor_stand::get_right_leg_rotation())
    }

    pub fn set_right_leg_rotation(&mut self, rotation: Vector3f) {
        self.set_rotation_vector(&definitions::armor_stand::get_right_leg_rotation(), rotation);
    }

    fn rotation_vector(
        &self,
        definition: &crate::entity::metadata::MetadataDefinition,
    ) -> Vector3f {
        match self.get_entity().get_metadata().get_value(definition) {
            MetadataValue::Rotation(x, y, z) => Vector3f { x, y, z },
            _ => Vector3f {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }

    fn set_rotation_vector(
        &mut self,
        definition: &crate::entity::metadata::MetadataDefinition,
        rotation: Vector3f,
    ) {
        self.get_entity_mut().get_metadata_mut().set(
            definition,
            MetadataValue::Rotation(rotation.x, rotation.y, rotation.z),
        );
    }
}

impl<'entity> Deref for ArmorStandMeta<'entity> {
    type Target = LivingEntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.living_entity_meta
    }
}

impl<'entity> DerefMut for ArmorStandMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.living_entity_meta
    }
}
