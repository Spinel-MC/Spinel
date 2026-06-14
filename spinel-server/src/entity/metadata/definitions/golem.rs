use crate::entity::metadata::{MetadataBitMaskDefinition, MetadataDefinition};
use spinel_network::types::entity_metadata::MetadataValue;

macro_rules! value_definition {
    ($name:ident, $index:expr, $value:expr) => {
        pub fn $name() -> MetadataDefinition {
            MetadataDefinition::new($index, $value)
        }
    };
}

macro_rules! flag_definition {
    ($name:ident, $index:expr, $mask:expr, $default:expr) => {
        pub fn $name() -> MetadataBitMaskDefinition {
            MetadataBitMaskDefinition::new($index, $mask, $default)
        }
    };
}

pub mod iron_golem {
    use super::*;
    value_definition!(flags, 16, MetadataValue::Byte(0));
    flag_definition!(is_player_created, 16, 0x01, false);
}

pub mod snow_golem {
    use super::*;
    value_definition!(flags, 16, MetadataValue::Byte(0));
    flag_definition!(has_pumpkin_hat, 16, 0x10, true);
}

pub mod shulker {
    use super::*;
    value_definition!(attach_face, 16, MetadataValue::Direction(0));
    value_definition!(shield_height, 17, MetadataValue::Byte(0));
    value_definition!(color, 18, MetadataValue::Byte(16));
}

pub mod copper_golem {
    use super::*;
    value_definition!(weather_state, 16, MetadataValue::WeatherState(0));
    value_definition!(state, 17, MetadataValue::CopperGolemState(0));
}
