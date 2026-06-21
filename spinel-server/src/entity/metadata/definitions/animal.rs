use crate::entity::metadata::{
    MetadataBitMaskDefinition, MetadataByteMaskDefinition, MetadataDefinition,
};
use spinel_network::types::Position;
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

pub mod allay {
    use super::*;
    value_definition!(is_dancing, 16, MetadataValue::Boolean(false));
    value_definition!(can_duplicate, 17, MetadataValue::Boolean(true));
}

pub mod armadillo {
    use super::*;
    value_definition!(get_state, 17, MetadataValue::ArmadilloState(0));
}

pub mod bat {
    use super::*;
    value_definition!(flags, 16, MetadataValue::Byte(0));
    flag_definition!(is_hanging, 16, 0x01, false);
}

pub mod dolphin {
    use super::*;
    value_definition!(
        get_treasure_position,
        16,
        MetadataValue::Position(Position { x: 0, y: 0, z: 0 })
    );
    value_definition!(has_fish, 17, MetadataValue::Boolean(false));
    value_definition!(get_moisture_level, 18, MetadataValue::VarInt(2400));
}

pub mod abstract_fish {
    use super::*;
    value_definition!(from_bucket, 16, MetadataValue::Boolean(false));
}

pub mod puffer_fish {
    use super::*;
    value_definition!(puff_state, 17, MetadataValue::VarInt(0));
}

pub mod salmon {
    use super::*;
    value_definition!(get_size, 17, MetadataValue::VarInt(0));
}

pub mod tropical_fish {
    use super::*;
    value_definition!(get_variant, 17, MetadataValue::VarInt(0));
}

pub mod ageable_mob {
    use super::*;
    value_definition!(is_baby, 16, MetadataValue::Boolean(false));
}

pub mod sniffer {
    use super::*;
    value_definition!(get_state, 17, MetadataValue::SnifferState(0));
    value_definition!(get_drop_seed_at_tick, 18, MetadataValue::VarInt(0));
}

pub mod abstract_horse {
    use super::*;
    value_definition!(flags, 17, MetadataValue::Byte(0));
    flag_definition!(is_tame, 17, 0x02, false);
    flag_definition!(has_bred, 17, 0x08, false);
    flag_definition!(is_eating, 17, 0x10, false);
    flag_definition!(is_rearing, 17, 0x20, false);
    flag_definition!(is_mouth_open, 17, 0x40, false);
}

pub mod horse {
    use super::*;
    value_definition!(get_variant, 18, MetadataValue::VarInt(0));
}

pub mod camel {
    use super::*;
    value_definition!(is_dashing, 18, MetadataValue::Boolean(false));
    value_definition!(get_last_pose_change_tick, 19, MetadataValue::Long(0));
}

pub mod chested_horse {
    use super::*;
    value_definition!(has_chest, 18, MetadataValue::Boolean(false));
}

pub mod llama {
    use super::*;
    value_definition!(get_strength, 19, MetadataValue::VarInt(0));
    value_definition!(get_carpet_color, 20, MetadataValue::VarInt(-1));
    value_definition!(get_variant, 21, MetadataValue::VarInt(0));
}

pub mod axolotl {
    use super::*;
    value_definition!(get_variant, 17, MetadataValue::VarInt(0));
    value_definition!(is_playing_dead, 18, MetadataValue::Boolean(false));
    value_definition!(is_from_bucket, 19, MetadataValue::Boolean(false));
}

pub mod bee {
    use super::*;
    value_definition!(flags, 17, MetadataValue::Byte(0));
    flag_definition!(is_angry, 17, 0x02, false);
    flag_definition!(has_stung, 17, 0x04, false);
    flag_definition!(has_nectar, 17, 0x08, false);
    value_definition!(anger_time_ticks, 18, MetadataValue::Long(-1));
}

pub mod glow_squid {
    use super::*;
    value_definition!(dark_ticks_remaining, 17, MetadataValue::VarInt(0));
}

pub mod fox {
    use super::*;
    value_definition!(get_variant, 17, MetadataValue::VarInt(0));
    value_definition!(flags, 18, MetadataValue::Byte(0));
    flag_definition!(is_sitting, 18, 0x01, false);
    flag_definition!(is_crouching, 18, 0x04, false);
    flag_definition!(is_interested, 18, 0x08, false);
    flag_definition!(is_pouncing, 18, 0x10, false);
    flag_definition!(is_sleeping, 18, 0x20, false);
    flag_definition!(is_faceplanted, 18, 0x40, false);
    flag_definition!(is_defending, 18, -0x80i8, false);
    value_definition!(
        get_first_uuid,
        19,
        MetadataValue::OptionalLivingEntityReference(None)
    );
    value_definition!(
        get_second_uuid,
        20,
        MetadataValue::OptionalLivingEntityReference(None)
    );
}

pub mod frog {
    use super::*;
    value_definition!(get_variant, 17, MetadataValue::FrogVariant(1));
    value_definition!(get_tongue_target, 18, MetadataValue::OptionalVarInt(Some(0)));
}

pub mod ocelot {
    use super::*;
    value_definition!(is_trusting, 17, MetadataValue::Boolean(false));
}

pub mod panda {
    use super::*;
    value_definition!(get_breed_timer, 17, MetadataValue::VarInt(0));
    value_definition!(get_sneeze_timer, 18, MetadataValue::VarInt(0));
    value_definition!(get_eat_timer, 19, MetadataValue::VarInt(0));
    value_definition!(get_main_gene, 20, MetadataValue::Byte(0));
    value_definition!(get_hidden_gene, 21, MetadataValue::Byte(0));
    value_definition!(flags, 22, MetadataValue::Byte(0));
    flag_definition!(is_sneezing, 22, 0x02, false);
    flag_definition!(is_rolling, 22, 0x04, false);
    flag_definition!(is_sitting, 22, 0x08, false);
    flag_definition!(is_on_back, 22, 0x10, false);
}

pub mod chicken {
    use super::*;
    value_definition!(get_variant, 17, MetadataValue::ChickenVariant(1));
}

pub mod cow {
    use super::*;
    value_definition!(get_variant, 17, MetadataValue::CowVariant(1));
}

pub mod pig {
    use super::*;
    value_definition!(boost_time, 17, MetadataValue::VarInt(0));
    value_definition!(get_variant, 18, MetadataValue::PigVariant(1));
}

pub mod rabbit {
    use super::*;
    value_definition!(kind, 17, MetadataValue::VarInt(0));
}

pub mod turtle {
    use super::*;
    value_definition!(has_egg, 17, MetadataValue::Boolean(false));
    value_definition!(is_laying_egg, 18, MetadataValue::Boolean(false));
}

pub mod polar_bear {
    use super::*;
    value_definition!(is_standing_up, 17, MetadataValue::Boolean(false));
}

pub mod mooshroom {
    use super::*;
    value_definition!(get_variant, 17, MetadataValue::VarInt(0));
}

pub mod hoglin {
    use super::*;
    value_definition!(
        is_immune_to_zombification,
        17,
        MetadataValue::Boolean(false)
    );
}

pub mod sheep {
    use super::*;

    value_definition!(flags, 17, MetadataValue::Byte(0));

    pub fn color_id() -> MetadataByteMaskDefinition {
        MetadataByteMaskDefinition::new(17, 0x0f, 0, 0)
    }

    flag_definition!(is_sheared, 17, 0x10, false);
}

pub mod strider {
    use super::*;
    value_definition!(fungus_boost, 17, MetadataValue::VarInt(0));
    value_definition!(is_shaking, 18, MetadataValue::Boolean(false));
}

pub mod goat {
    use super::*;
    value_definition!(is_screaming, 17, MetadataValue::Boolean(false));
    value_definition!(has_left_horn, 18, MetadataValue::Boolean(true));
    value_definition!(has_right_horn, 19, MetadataValue::Boolean(true));
}

pub mod tameable_animal {
    use super::*;
    value_definition!(flags, 17, MetadataValue::Byte(0));
    flag_definition!(is_sitting, 17, 0x01, false);
    flag_definition!(is_tamed, 17, 0x04, false);
    value_definition!(
        get_owner,
        18,
        MetadataValue::OptionalLivingEntityReference(None)
    );
}

pub mod cat {
    use super::*;
    value_definition!(get_variant, 19, MetadataValue::CatVariant(1));
    value_definition!(is_lying, 20, MetadataValue::Boolean(false));
    value_definition!(is_relaxed, 21, MetadataValue::Boolean(false));
    value_definition!(get_collar_color, 22, MetadataValue::VarInt(14));
}

pub mod wolf {
    use super::*;
    value_definition!(is_begging, 19, MetadataValue::Boolean(false));
    value_definition!(get_collar_color, 20, MetadataValue::VarInt(14));
    value_definition!(get_anger_time, 21, MetadataValue::Long(-1));
    value_definition!(get_variant, 22, MetadataValue::WolfVariant(3));
    value_definition!(get_sound_variant, 23, MetadataValue::WolfSoundVariant(2));
}

pub mod parrot {
    use super::*;
    value_definition!(get_variant, 19, MetadataValue::VarInt(0));
}

pub mod abstract_nautilus {
    use super::*;
    value_definition!(is_dashing, 19, MetadataValue::Boolean(false));
}

pub mod zombie_nautilus {
    use super::*;
    value_definition!(get_variant, 20, MetadataValue::ZombieNautilusVariant(0));
}

pub mod abstract_villager {
    use super::*;
    value_definition!(get_head_shake_timer, 17, MetadataValue::VarInt(0));
}

pub mod villager {
    use super::*;
    value_definition!(data, 18, MetadataValue::VillagerData(0, 0, 1));
}

pub mod happy_ghast {
    use super::*;
    value_definition!(is_leash_holder, 17, MetadataValue::Boolean(false));
    value_definition!(stays_still, 18, MetadataValue::Boolean(false));
}
