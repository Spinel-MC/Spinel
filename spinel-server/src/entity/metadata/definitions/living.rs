use crate::entity::metadata::{MetadataBitMaskDefinition, MetadataDefinition};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::{MainHand, ResolvableProfile};
use spinel_utils::component::text::TextComponent;

pub fn living_entity_flags() -> MetadataDefinition {
    MetadataDefinition::new(8, MetadataValue::Byte(0))
}

pub fn is_hand_active() -> MetadataBitMaskDefinition {
    MetadataBitMaskDefinition::new(8, 0x01, false)
}

pub fn active_hand() -> MetadataBitMaskDefinition {
    MetadataBitMaskDefinition::new(8, 0x02, false)
}

pub fn is_riptide_spin_attack() -> MetadataBitMaskDefinition {
    MetadataBitMaskDefinition::new(8, 0x04, false)
}

pub fn additional_hearts() -> MetadataDefinition {
    player::additional_hearts()
}

pub mod living_entity {
    use super::*;

    pub fn flags() -> MetadataDefinition {
        living_entity_flags()
    }

    pub fn is_hand_active() -> MetadataBitMaskDefinition {
        super::is_hand_active()
    }

    pub fn active_hand() -> MetadataBitMaskDefinition {
        super::active_hand()
    }

    pub fn is_riptide_spin_attack() -> MetadataBitMaskDefinition {
        super::is_riptide_spin_attack()
    }

    pub fn health() -> MetadataDefinition {
        MetadataDefinition::new(9, MetadataValue::Float(1.0))
    }

    pub fn potion_effect_particles() -> MetadataDefinition {
        MetadataDefinition::new(10, MetadataValue::ParticleList(Vec::new()))
    }

    pub fn is_potion_effect_ambient() -> MetadataDefinition {
        MetadataDefinition::new(11, MetadataValue::Boolean(false))
    }

    pub fn number_of_arrows() -> MetadataDefinition {
        MetadataDefinition::new(12, MetadataValue::VarInt(0))
    }

    pub fn number_of_bee_stingers() -> MetadataDefinition {
        MetadataDefinition::new(13, MetadataValue::VarInt(0))
    }

    pub fn location_of_bed() -> MetadataDefinition {
        MetadataDefinition::new(14, MetadataValue::OptionalPosition(None))
    }
}

pub mod avatar {
    use super::*;

    pub fn main_hand() -> MetadataDefinition {
        MetadataDefinition::new(15, MetadataValue::MainHand(MainHand::Right))
    }

    pub fn displayed_model_parts_flags() -> MetadataDefinition {
        MetadataDefinition::new(16, MetadataValue::Byte(0))
    }

    pub fn is_cape_enabled() -> MetadataBitMaskDefinition {
        MetadataBitMaskDefinition::new(16, 0x01, false)
    }

    pub fn is_jacket_enabled() -> MetadataBitMaskDefinition {
        MetadataBitMaskDefinition::new(16, 0x02, false)
    }

    pub fn is_left_sleeve_enabled() -> MetadataBitMaskDefinition {
        MetadataBitMaskDefinition::new(16, 0x04, false)
    }

    pub fn is_right_sleeve_enabled() -> MetadataBitMaskDefinition {
        MetadataBitMaskDefinition::new(16, 0x08, false)
    }

    pub fn is_left_pants_leg_enabled() -> MetadataBitMaskDefinition {
        MetadataBitMaskDefinition::new(16, 0x10, false)
    }

    pub fn is_right_pants_leg_enabled() -> MetadataBitMaskDefinition {
        MetadataBitMaskDefinition::new(16, 0x20, false)
    }

    pub fn is_hat_enabled() -> MetadataBitMaskDefinition {
        MetadataBitMaskDefinition::new(16, 0x40, false)
    }
}

pub mod player {
    use super::*;

    pub fn additional_hearts() -> MetadataDefinition {
        MetadataDefinition::new(17, MetadataValue::Float(0.0))
    }

    pub fn score() -> MetadataDefinition {
        MetadataDefinition::new(18, MetadataValue::VarInt(0))
    }

    pub fn left_shoulder_entity_data() -> MetadataDefinition {
        MetadataDefinition::new(19, MetadataValue::OptionalVarInt(None))
    }

    pub fn right_shoulder_entity_data() -> MetadataDefinition {
        MetadataDefinition::new(20, MetadataValue::OptionalVarInt(None))
    }
}

pub mod mannequin {
    use super::*;

    pub fn profile() -> MetadataDefinition {
        MetadataDefinition::new(
            17,
            MetadataValue::ResolvableProfile(ResolvableProfile::default()),
        )
    }

    pub fn immovable() -> MetadataDefinition {
        MetadataDefinition::new(18, MetadataValue::Boolean(false))
    }

    pub fn description() -> MetadataDefinition {
        MetadataDefinition::new(
            19,
            MetadataValue::OptionalText(Some(
                TextComponent::translatable("entity.minecraft.mannequin.label").build(),
            )),
        )
    }
}

pub mod armor_stand {
    use super::*;

    pub fn flags() -> MetadataDefinition {
        MetadataDefinition::new(15, MetadataValue::Byte(0))
    }

    pub fn is_small() -> MetadataBitMaskDefinition {
        MetadataBitMaskDefinition::new(15, 0x01, false)
    }

    pub fn has_arms() -> MetadataBitMaskDefinition {
        MetadataBitMaskDefinition::new(15, 0x04, false)
    }

    pub fn has_no_base_plate() -> MetadataBitMaskDefinition {
        MetadataBitMaskDefinition::new(15, 0x08, false)
    }

    pub fn is_marker() -> MetadataBitMaskDefinition {
        MetadataBitMaskDefinition::new(15, 0x10, false)
    }

    pub fn head_rotation() -> MetadataDefinition {
        rotation(16, 0.0, 0.0, 0.0)
    }

    pub fn body_rotation() -> MetadataDefinition {
        rotation(17, 0.0, 0.0, 0.0)
    }

    pub fn left_arm_rotation() -> MetadataDefinition {
        rotation(18, -10.0, 0.0, -10.0)
    }

    pub fn right_arm_rotation() -> MetadataDefinition {
        rotation(19, -15.0, 0.0, 10.0)
    }

    pub fn left_leg_rotation() -> MetadataDefinition {
        rotation(20, -1.0, 0.0, -1.0)
    }

    pub fn right_leg_rotation() -> MetadataDefinition {
        rotation(21, 1.0, 0.0, 1.0)
    }

    fn rotation(index: u8, x: f32, y: f32, z: f32) -> MetadataDefinition {
        MetadataDefinition::new(index, MetadataValue::Rotation(x, y, z))
    }
}

pub mod mob {
    use super::*;

    pub fn flags() -> MetadataDefinition {
        MetadataDefinition::new(15, MetadataValue::Byte(0))
    }

    pub fn no_ai() -> MetadataBitMaskDefinition {
        MetadataBitMaskDefinition::new(15, 0x01, false)
    }

    pub fn is_left_handed() -> MetadataBitMaskDefinition {
        MetadataBitMaskDefinition::new(15, 0x02, false)
    }

    pub fn is_aggressive() -> MetadataBitMaskDefinition {
        MetadataBitMaskDefinition::new(15, 0x04, false)
    }
}
