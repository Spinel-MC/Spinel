use crate::entity::metadata::{AbstractDisplayMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use spinel_utils::component::text::TextComponent;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextAlignment {
    #[default]
    Center,
    Left,
    Right,
}

impl TextAlignment {
    const fn protocol_id(self) -> i8 {
        self as i8
    }

    const fn from_protocol_id(protocol_id: i8) -> Self {
        match protocol_id {
            1 => Self::Left,
            2 => Self::Right,
            _ => Self::Center,
        }
    }
}

pub struct TextDisplayMeta<'entity> {
    abstract_display_meta: AbstractDisplayMeta<'entity>,
}

impl<'entity> TextDisplayMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::TEXT_DISPLAY).then(|| Self {
            abstract_display_meta: AbstractDisplayMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_text(&self) -> TextComponent {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::text_display::get_text())
        {
            MetadataValue::Text(text) => text,
            _ => TextComponent::empty(),
        }
    }

    pub fn set_text(&mut self, text: TextComponent) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::text_display::get_text(),
            MetadataValue::Text(text),
        );
    }

    pub fn get_line_width(&self) -> i32 {
        self.var_int(&definitions::text_display::get_line_width(), 200)
    }

    pub fn set_line_width(&mut self, line_width: i32) {
        self.set_var_int(&definitions::text_display::get_line_width(), line_width);
    }

    pub fn get_background_color(&self) -> i32 {
        self.var_int(&definitions::text_display::get_background_color(), 0x40000000)
    }

    pub fn set_background_color(&mut self, background_color: i32) {
        self.set_var_int(
            &definitions::text_display::get_background_color(),
            background_color,
        );
    }

    pub fn get_text_opacity(&self) -> i8 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::text_display::get_text_opacity())
        {
            MetadataValue::Byte(text_opacity) => text_opacity,
            _ => -1,
        }
    }

    pub fn set_text_opacity(&mut self, text_opacity: i8) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::text_display::get_text_opacity(),
            MetadataValue::Byte(text_opacity),
        );
    }

    pub fn is_shadow(&self) -> bool {
        self.flag(&definitions::text_display::has_shadow())
    }

    pub fn set_shadow(&mut self, is_shadow: bool) {
        self.set_flag(&definitions::text_display::has_shadow(), is_shadow);
    }

    pub fn is_see_through(&self) -> bool {
        self.flag(&definitions::text_display::is_see_through())
    }

    pub fn set_see_through(&mut self, is_see_through: bool) {
        self.set_flag(&definitions::text_display::is_see_through(), is_see_through);
    }

    pub fn has_default_background(&self) -> bool {
        self.flag(&definitions::text_display::uses_default_background_color())
    }

    pub fn set_uses_default_background(&mut self, uses_default_background: bool) {
        self.set_flag(
            &definitions::text_display::uses_default_background_color(),
            uses_default_background,
        );
    }

    pub fn is_align_left(&self) -> bool {
        self.flag(&definitions::text_display::is_left_aligned())
    }

    pub fn set_align_left(&mut self, is_align_left: bool) {
        self.set_flag(&definitions::text_display::is_left_aligned(), is_align_left);
    }

    pub fn is_align_right(&self) -> bool {
        self.flag(&definitions::text_display::is_right_aligned())
    }

    pub fn set_align_right(&mut self, is_align_right: bool) {
        self.set_flag(
            &definitions::text_display::is_right_aligned(),
            is_align_right,
        );
    }

    pub fn get_alignment(&self) -> TextAlignment {
        TextAlignment::from_protocol_id(
            self.get_entity()
                .get_metadata()
                .byte(&definitions::text_display::get_alignment()),
        )
    }

    pub fn set_alignment(&mut self, alignment: TextAlignment) {
        self.get_entity_mut().get_metadata_mut().set_byte(
            &definitions::text_display::get_alignment(),
            alignment.protocol_id(),
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

    fn flag(&self, definition: &crate::entity::metadata::MetadataBitMaskDefinition) -> bool {
        self.get_entity().get_metadata().flag(definition)
    }

    fn set_flag(
        &mut self,
        definition: &crate::entity::metadata::MetadataBitMaskDefinition,
        value: bool,
    ) {
        self.get_entity_mut().get_metadata_mut().set_flag(definition, value);
    }
}

impl<'entity> Deref for TextDisplayMeta<'entity> {
    type Target = AbstractDisplayMeta<'entity>;
    fn deref(&self) -> &Self::Target {
        &self.abstract_display_meta
    }
}

impl<'entity> DerefMut for TextDisplayMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_display_meta
    }
}
