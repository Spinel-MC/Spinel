use crate::entity::metadata::{MetadataBitMaskDefinition, MetadataDefinition};
use spinel_network::types::Slot;
use spinel_network::types::entity_metadata::MetadataValue;

fn empty_item(index: u8) -> MetadataDefinition {
    MetadataDefinition::new(
        index,
        MetadataValue::Slot(Slot::from_item_stack(&spinel_registry::ItemStack::air())),
    )
}

pub fn item_stack() -> MetadataDefinition {
    empty_item(8)
}

pub fn projectile_item() -> MetadataDefinition {
    empty_item(8)
}

pub fn abstract_arrow_flags() -> MetadataDefinition {
    MetadataDefinition::new(8, MetadataValue::Byte(0))
}

pub fn is_critical_arrow() -> MetadataBitMaskDefinition {
    MetadataBitMaskDefinition::new(8, 0x01, false)
}

pub fn has_no_clip_arrow() -> MetadataBitMaskDefinition {
    MetadataBitMaskDefinition::new(8, 0x02, false)
}

pub fn piercing_level() -> MetadataDefinition {
    MetadataDefinition::new(9, MetadataValue::Byte(0))
}

pub fn is_arrow_in_ground() -> MetadataDefinition {
    MetadataDefinition::new(10, MetadataValue::Boolean(false))
}

pub fn arrow_color() -> MetadataDefinition {
    MetadataDefinition::new(11, MetadataValue::VarInt(-1))
}

pub fn trident_loyalty_level() -> MetadataDefinition {
    MetadataDefinition::new(11, MetadataValue::Byte(0))
}

pub fn trident_has_enchantment_glint() -> MetadataDefinition {
    MetadataDefinition::new(12, MetadataValue::Boolean(false))
}

pub fn wither_skull_is_invulnerable() -> MetadataDefinition {
    MetadataDefinition::new(8, MetadataValue::Boolean(false))
}

pub fn firework_info() -> MetadataDefinition {
    empty_item(8)
}

pub fn firework_shooter_entity_id() -> MetadataDefinition {
    MetadataDefinition::new(9, MetadataValue::OptionalVarInt(None))
}

pub fn firework_is_shot_at_angle() -> MetadataDefinition {
    MetadataDefinition::new(10, MetadataValue::Boolean(false))
}

pub mod thrown_item {
    use super::*;

    pub fn item() -> MetadataDefinition {
        empty_item(8)
    }
}

pub mod eye_of_ender {
    use super::*;

    pub fn item() -> MetadataDefinition {
        empty_item(8)
    }
}

pub mod abstract_arrow {
    use super::*;

    pub fn arrow_flags() -> MetadataDefinition {
        abstract_arrow_flags()
    }

    pub fn is_critical() -> MetadataBitMaskDefinition {
        is_critical_arrow()
    }

    pub fn is_no_clip() -> MetadataBitMaskDefinition {
        has_no_clip_arrow()
    }

    pub fn piercing_level() -> MetadataDefinition {
        super::piercing_level()
    }

    pub fn in_ground() -> MetadataDefinition {
        is_arrow_in_ground()
    }
}

pub mod arrow {
    use super::*;

    pub fn color() -> MetadataDefinition {
        arrow_color()
    }
}

pub mod thrown_trident {
    use super::*;

    pub fn loyalty_level() -> MetadataDefinition {
        trident_loyalty_level()
    }

    pub fn has_enchantment_glint() -> MetadataDefinition {
        trident_has_enchantment_glint()
    }
}

pub mod wither_skull {
    use super::*;

    pub fn is_invulnerable() -> MetadataDefinition {
        wither_skull_is_invulnerable()
    }
}

pub mod firework_rocket {
    use super::*;

    pub fn item() -> MetadataDefinition {
        firework_info()
    }

    pub fn shooter_entity_id() -> MetadataDefinition {
        firework_shooter_entity_id()
    }

    pub fn is_shot_at_angle() -> MetadataDefinition {
        firework_is_shot_at_angle()
    }
}
