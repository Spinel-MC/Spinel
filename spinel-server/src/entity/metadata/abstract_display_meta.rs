use crate::entity::metadata::{EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::{Quaternionf, Vector3f};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BillboardConstraints {
    #[default]
    Fixed,
    Vertical,
    Horizontal,
    Center,
}

impl BillboardConstraints {
    const fn protocol_id(self) -> i8 {
        self as i8
    }

    const fn from_protocol_id(protocol_id: i8) -> Self {
        match protocol_id {
            1 => Self::Vertical,
            2 => Self::Horizontal,
            3 => Self::Center,
            _ => Self::Fixed,
        }
    }
}

pub struct AbstractDisplayMeta<'entity> {
    entity_meta: EntityMeta<'entity>,
}

impl<'entity> AbstractDisplayMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Self {
        Self { entity_meta }
    }

    pub fn get_transformation_interpolation_start_delta(&self) -> i32 {
        self.var_int(&definitions::display::interpolation_delay(), 0)
    }

    pub fn set_transformation_interpolation_start_delta(&mut self, value: i32) {
        self.set_var_int(&definitions::display::interpolation_delay(), value);
    }

    pub fn get_transformation_interpolation_duration(&self) -> i32 {
        self.var_int(
            &definitions::display::get_transformation_interpolation_duration(),
            0,
        )
    }

    pub fn set_transformation_interpolation_duration(&mut self, value: i32) {
        self.set_var_int(
            &definitions::display::get_transformation_interpolation_duration(),
            value,
        );
    }

    pub fn get_position_rotation_interpolation_duration(&self) -> i32 {
        self.var_int(
            &definitions::display::position_rotation_interpolation_duration(),
            0,
        )
    }

    pub fn set_position_rotation_interpolation_duration(&mut self, value: i32) {
        self.set_var_int(
            &definitions::display::position_rotation_interpolation_duration(),
            value,
        );
    }

    pub fn get_translation(&self) -> Vector3f {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::display::get_translation())
        {
            MetadataValue::Vector3f(translation) => translation,
            _ => Vector3f {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }

    pub fn set_translation(&mut self, translation: Vector3f) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::display::get_translation(),
            MetadataValue::Vector3f(translation),
        );
    }

    pub fn get_scale(&self) -> Vector3f {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::display::get_scale())
        {
            MetadataValue::Vector3f(scale) => scale,
            _ => Vector3f {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        }
    }

    pub fn set_scale(&mut self, scale: Vector3f) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::display::get_scale(),
            MetadataValue::Vector3f(scale),
        );
    }

    pub fn get_left_rotation(&self) -> Quaternionf {
        self.get_rotation(&definitions::display::rotation_left())
    }

    pub fn set_left_rotation(&mut self, rotation: Quaternionf) {
        self.set_rotation(&definitions::display::rotation_left(), rotation);
    }

    pub fn get_right_rotation(&self) -> Quaternionf {
        self.get_rotation(&definitions::display::rotation_right())
    }

    pub fn set_right_rotation(&mut self, rotation: Quaternionf) {
        self.set_rotation(&definitions::display::rotation_right(), rotation);
    }

    pub fn get_billboard_render_constraints(&self) -> BillboardConstraints {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::display::billboard_constraints())
        {
            MetadataValue::Byte(protocol_id) => BillboardConstraints::from_protocol_id(protocol_id),
            _ => BillboardConstraints::Fixed,
        }
    }

    pub fn set_billboard_render_constraints(&mut self, constraints: BillboardConstraints) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::display::billboard_constraints(),
            MetadataValue::Byte(constraints.protocol_id()),
        );
    }

    pub fn get_brightness_override(&self) -> i32 {
        self.var_int(&definitions::display::get_brightness_override(), -1)
    }

    pub fn set_brightness_override(&mut self, brightness_override: i32) {
        self.set_var_int(
            &definitions::display::get_brightness_override(),
            brightness_override,
        );
    }

    pub fn set_brightness(&mut self, block_light: i32, sky_light: i32) {
        self.set_brightness_override((block_light & 0xF) << 4 | (sky_light & 0xF) << 20);
    }

    pub fn get_block_light(&self) -> i32 {
        self.light_at_shift(4)
    }

    pub fn get_sky_light(&self) -> i32 {
        self.light_at_shift(20)
    }

    pub fn get_view_range(&self) -> f32 {
        self.float(&definitions::display::get_view_range(), 1.0)
    }

    pub fn set_view_range(&mut self, view_range: f32) {
        self.set_float(&definitions::display::get_view_range(), view_range);
    }

    pub fn get_shadow_radius(&self) -> f32 {
        self.float(&definitions::display::get_shadow_radius(), 0.0)
    }

    pub fn set_shadow_radius(&mut self, shadow_radius: f32) {
        self.set_float(&definitions::display::get_shadow_radius(), shadow_radius);
    }

    pub fn get_shadow_strength(&self) -> f32 {
        self.float(&definitions::display::get_shadow_strength(), 1.0)
    }

    pub fn set_shadow_strength(&mut self, shadow_strength: f32) {
        self.set_float(&definitions::display::get_shadow_strength(), shadow_strength);
    }

    pub fn get_width(&self) -> f32 {
        self.float(&definitions::display::get_width(), 0.0)
    }

    pub fn set_width(&mut self, width: f32) {
        self.set_float(&definitions::display::get_width(), width);
    }

    pub fn get_height(&self) -> f32 {
        self.float(&definitions::display::get_height(), 0.0)
    }

    pub fn set_height(&mut self, height: f32) {
        self.set_float(&definitions::display::get_height(), height);
    }

    pub fn get_glow_color_override(&self) -> i32 {
        self.var_int(&definitions::display::get_glow_color_override(), -1)
    }

    pub fn set_glow_color_override(&mut self, glow_color_override: i32) {
        self.set_var_int(
            &definitions::display::get_glow_color_override(),
            glow_color_override,
        );
    }

    fn var_int(
        &self,
        definition: &crate::entity::metadata::MetadataDefinition,
        default_value: i32,
    ) -> i32 {
        match self.get_entity().get_metadata().get_value(definition) {
            MetadataValue::VarInt(value) => value,
            _ => default_value,
        }
    }

    fn set_var_int(
        &mut self,
        definition: &crate::entity::metadata::MetadataDefinition,
        value: i32,
    ) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set(definition, MetadataValue::VarInt(value));
    }

    fn float(
        &self,
        definition: &crate::entity::metadata::MetadataDefinition,
        default_value: f32,
    ) -> f32 {
        match self.get_entity().get_metadata().get_value(definition) {
            MetadataValue::Float(value) => value,
            _ => default_value,
        }
    }

    fn set_float(&mut self, definition: &crate::entity::metadata::MetadataDefinition, value: f32) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set(definition, MetadataValue::Float(value));
    }

    fn get_rotation(&self, definition: &crate::entity::metadata::MetadataDefinition) -> Quaternionf {
        match self.get_entity().get_metadata().get_value(definition) {
            MetadataValue::Quaternionf(rotation) => rotation,
            _ => Quaternionf {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
        }
    }

    fn set_rotation(
        &mut self,
        definition: &crate::entity::metadata::MetadataDefinition,
        rotation: Quaternionf,
    ) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set(definition, MetadataValue::Quaternionf(rotation));
    }

    fn light_at_shift(&self, shift: i32) -> i32 {
        let brightness_override = self.get_brightness_override();
        if brightness_override <= 0 {
            return 0;
        }
        (brightness_override >> shift) & 0xF
    }
}

impl<'entity> Deref for AbstractDisplayMeta<'entity> {
    type Target = EntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.entity_meta
    }
}

impl<'entity> DerefMut for AbstractDisplayMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_meta
    }
}
