use crate::entity::metadata::{MetadataBitMaskDefinition, MetadataDefinition};
use spinel_network::types::Slot;
use spinel_network::types::entity_metadata::MetadataValue;

pub fn entity_flags() -> MetadataDefinition {
    MetadataDefinition::new(0, MetadataValue::Byte(0))
}

pub fn is_on_fire() -> MetadataBitMaskDefinition {
    MetadataBitMaskDefinition::new(0, 0x01, false)
}

pub fn is_crouching() -> MetadataBitMaskDefinition {
    MetadataBitMaskDefinition::new(0, 0x02, false)
}

pub fn is_sprinting() -> MetadataBitMaskDefinition {
    MetadataBitMaskDefinition::new(0, 0x08, false)
}

pub fn is_swimming() -> MetadataBitMaskDefinition {
    MetadataBitMaskDefinition::new(0, 0x10, false)
}

pub fn is_invisible() -> MetadataBitMaskDefinition {
    MetadataBitMaskDefinition::new(0, 0x20, false)
}

pub fn has_glowing_effect() -> MetadataBitMaskDefinition {
    MetadataBitMaskDefinition::new(0, 0x40, false)
}

pub fn is_flying_with_elytra() -> MetadataBitMaskDefinition {
    MetadataBitMaskDefinition::new(0, -0x80i8, false)
}

pub fn air_ticks() -> MetadataDefinition {
    MetadataDefinition::new(1, MetadataValue::VarInt(300))
}

pub fn custom_name() -> MetadataDefinition {
    MetadataDefinition::new(2, MetadataValue::OptionalText(None))
}

pub fn custom_name_visible() -> MetadataDefinition {
    MetadataDefinition::new(3, MetadataValue::Boolean(false))
}

pub fn is_silent() -> MetadataDefinition {
    MetadataDefinition::new(4, MetadataValue::Boolean(false))
}

pub fn has_no_gravity() -> MetadataDefinition {
    MetadataDefinition::new(5, MetadataValue::Boolean(false))
}

pub fn pose() -> MetadataDefinition {
    MetadataDefinition::new(6, MetadataValue::Pose(0))
}

pub fn ticks_frozen() -> MetadataDefinition {
    MetadataDefinition::new(7, MetadataValue::VarInt(0))
}

pub fn item_stack() -> MetadataDefinition {
    MetadataDefinition::new(
        8,
        MetadataValue::Slot(Slot::from_item_stack(&spinel_registry::ItemStack::air())),
    )
}

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
    MetadataDefinition::new(15, MetadataValue::Float(0.0))
}
