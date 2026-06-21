use crate::entity::metadata::{
    MetadataBitMaskDefinition, MetadataByteMaskDefinition, MetadataDefinition,
};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::{Quaternionf, Slot, Vector3f};
use spinel_utils::component::text::TextComponent;

macro_rules! value_definition {
    ($name:ident, $index:expr, $value:expr) => {
        pub fn $name() -> MetadataDefinition {
            MetadataDefinition::new($index, $value)
        }
    };
}

macro_rules! flag_definition {
    ($name:ident, $index:expr, $mask:expr) => {
        pub fn $name() -> MetadataBitMaskDefinition {
            MetadataBitMaskDefinition::new($index, $mask, false)
        }
    };
}

pub mod interaction {
    use super::*;
    value_definition!(get_width, 8, MetadataValue::Float(1.0));
    value_definition!(get_height, 9, MetadataValue::Float(1.0));
    value_definition!(responsive, 10, MetadataValue::Boolean(false));
}

pub mod display {
    use super::*;
    value_definition!(interpolation_delay, 8, MetadataValue::VarInt(0));
    value_definition!(
        get_transformation_interpolation_duration,
        9,
        MetadataValue::VarInt(0)
    );
    value_definition!(
        position_rotation_interpolation_duration,
        10,
        MetadataValue::VarInt(0)
    );
    value_definition!(
        get_translation,
        11,
        MetadataValue::Vector3f(Vector3f {
            x: 0.0,
            y: 0.0,
            z: 0.0
        })
    );
    value_definition!(
        get_scale,
        12,
        MetadataValue::Vector3f(Vector3f {
            x: 1.0,
            y: 1.0,
            z: 1.0
        })
    );
    value_definition!(
        rotation_left,
        13,
        MetadataValue::Quaternionf(Quaternionf {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0
        })
    );
    value_definition!(
        rotation_right,
        14,
        MetadataValue::Quaternionf(Quaternionf {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0
        })
    );
    value_definition!(billboard_constraints, 15, MetadataValue::Byte(0));
    value_definition!(get_brightness_override, 16, MetadataValue::VarInt(-1));
    value_definition!(get_view_range, 17, MetadataValue::Float(1.0));
    value_definition!(get_shadow_radius, 18, MetadataValue::Float(0.0));
    value_definition!(get_shadow_strength, 19, MetadataValue::Float(1.0));
    value_definition!(get_width, 20, MetadataValue::Float(0.0));
    value_definition!(get_height, 21, MetadataValue::Float(0.0));
    value_definition!(get_glow_color_override, 22, MetadataValue::VarInt(-1));
}

pub mod block_display {
    use super::*;
    value_definition!(displayed_block_state, 23, MetadataValue::BlockState(0));
}

pub mod item_display {
    use super::*;
    value_definition!(
        displayed_item,
        23,
        MetadataValue::Slot(Slot::from_item_stack(&spinel_registry::ItemStack::air()))
    );
    value_definition!(display_type, 24, MetadataValue::Byte(0));
}

pub mod text_display {
    use super::*;
    value_definition!(get_text, 23, MetadataValue::Text(TextComponent::empty()));
    value_definition!(get_line_width, 24, MetadataValue::VarInt(200));
    value_definition!(get_background_color, 25, MetadataValue::VarInt(0x40000000));
    value_definition!(get_text_opacity, 26, MetadataValue::Byte(-1));
    value_definition!(flags, 27, MetadataValue::Byte(0));
    flag_definition!(has_shadow, 27, 0x01);
    flag_definition!(is_see_through, 27, 0x02);
    flag_definition!(uses_default_background_color, 27, 0x04);
    flag_definition!(is_left_aligned, 27, 0x08);
    flag_definition!(is_right_aligned, 27, 0x10);

    pub fn get_alignment() -> MetadataByteMaskDefinition {
        MetadataByteMaskDefinition::new(27, 0x18, 3, 0)
    }
}
