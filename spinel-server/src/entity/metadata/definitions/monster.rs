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
    ($name:ident, $index:expr, $mask:expr) => {
        pub fn $name() -> MetadataBitMaskDefinition {
            MetadataBitMaskDefinition::new($index, $mask, false)
        }
    };
}

pub mod base_piglin {
    use super::*;
    value_definition!(
        is_immune_to_zombification,
        16,
        MetadataValue::Boolean(false)
    );
}

pub mod piglin {
    use super::*;
    value_definition!(is_baby, 17, MetadataValue::Boolean(false));
    value_definition!(is_charging_crossbow, 18, MetadataValue::Boolean(false));
    value_definition!(is_dancing, 19, MetadataValue::Boolean(false));
}

pub mod blaze {
    use super::*;
    value_definition!(flags, 16, MetadataValue::Byte(0));
    flag_definition!(is_on_fire, 16, 0x01);
}

pub mod bogged {
    use super::*;
    value_definition!(is_sheared, 16, MetadataValue::Boolean(false));
}

pub mod creaking {
    use super::*;
    value_definition!(can_move, 16, MetadataValue::Boolean(true));
    value_definition!(is_active, 17, MetadataValue::Boolean(false));
    value_definition!(is_tearing_down, 18, MetadataValue::Boolean(false));
    value_definition!(home_position, 19, MetadataValue::OptionalPosition(None));
}

pub mod creeper {
    use super::*;
    value_definition!(state, 16, MetadataValue::VarInt(-1));
    value_definition!(is_charged, 17, MetadataValue::Boolean(false));
    value_definition!(is_ignited, 18, MetadataValue::Boolean(false));
}

pub mod guardian {
    use super::*;
    value_definition!(is_retracting_spikes, 16, MetadataValue::Boolean(false));
    value_definition!(target_entity_id, 17, MetadataValue::VarInt(0));
}

pub mod raider {
    use super::*;
    value_definition!(is_celebrating, 16, MetadataValue::Boolean(false));
}

pub mod pillager {
    use super::*;
    value_definition!(is_charging, 17, MetadataValue::Boolean(false));
}

pub mod spellcaster_illager {
    use super::*;
    value_definition!(spell, 17, MetadataValue::Byte(0));
}

pub mod witch {
    use super::*;
    value_definition!(is_drinking_potion, 17, MetadataValue::Boolean(false));
}

pub mod spider {
    use super::*;
    value_definition!(flags, 16, MetadataValue::Byte(0));
    flag_definition!(is_climbing, 16, 0x01);
}

pub mod vex {
    use super::*;
    value_definition!(flags, 16, MetadataValue::Byte(0));
    flag_definition!(is_attacking, 16, 0x01);
}

pub mod warden {
    use super::*;
    value_definition!(anger_level, 16, MetadataValue::VarInt(0));
}

pub mod wither {
    use super::*;
    value_definition!(center_head_target, 16, MetadataValue::VarInt(0));
    value_definition!(left_head_target, 17, MetadataValue::VarInt(0));
    value_definition!(right_head_target, 18, MetadataValue::VarInt(0));
    value_definition!(invulnerable_time, 19, MetadataValue::VarInt(0));
}

pub mod zoglin {
    use super::*;
    value_definition!(is_baby, 16, MetadataValue::Boolean(false));
}

pub mod zombie {
    use super::*;
    value_definition!(is_baby, 16, MetadataValue::Boolean(false));
    value_definition!(is_becoming_drowned, 18, MetadataValue::Boolean(false));
}

pub mod zombie_villager {
    use super::*;
    value_definition!(is_converting, 16, MetadataValue::Boolean(false));
    value_definition!(villager_data, 17, MetadataValue::VillagerData(0, 0, 1));
}

pub mod enderman {
    use super::*;
    value_definition!(carried_block, 16, MetadataValue::OptionalBlockState(0));
    value_definition!(is_screaming, 17, MetadataValue::Boolean(false));
    value_definition!(is_staring, 18, MetadataValue::Boolean(false));
}

pub mod ender_dragon {
    use super::*;
    value_definition!(phase, 16, MetadataValue::VarInt(10));
}

pub mod ghast {
    use super::*;
    value_definition!(is_attacking, 16, MetadataValue::Boolean(false));
}

pub mod phantom {
    use super::*;
    value_definition!(size, 16, MetadataValue::VarInt(0));
}

pub mod slime {
    use super::*;
    value_definition!(size, 16, MetadataValue::VarInt(1));
}
