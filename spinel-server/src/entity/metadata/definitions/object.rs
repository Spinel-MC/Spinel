use crate::entity::metadata::MetadataDefinition;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::{Particle, Position, Slot};
use spinel_utils::component::text::TextComponent;

macro_rules! value_definition {
    ($name:ident, $index:expr, $value:expr) => {
        pub fn $name() -> MetadataDefinition {
            MetadataDefinition::new($index, $value)
        }
    };
}

fn empty_item(index: u8) -> MetadataDefinition {
    MetadataDefinition::new(
        index,
        MetadataValue::Slot(Slot::from_item_stack(&spinel_registry::ItemStack::air())),
    )
}

pub mod experience_orb {
    use super::*;
    value_definition!(value, 8, MetadataValue::VarInt(0));
}

pub mod falling_block {
    use super::*;
    value_definition!(
        spawn_position,
        8,
        MetadataValue::Position(Position { x: 0, y: 0, z: 0 })
    );
}

pub mod area_effect_cloud {
    use super::*;
    value_definition!(radius, 8, MetadataValue::Float(0.5));
    value_definition!(waiting, 9, MetadataValue::Boolean(false));
    value_definition!(particle, 10, MetadataValue::Particle(Particle::effect()));
}

pub mod fishing_hook {
    use super::*;
    value_definition!(hooked_entity_id, 8, MetadataValue::VarInt(0));
    value_definition!(is_catchable, 9, MetadataValue::Boolean(false));
}

pub mod abstract_vehicle {
    use super::*;
    value_definition!(shaking_power, 8, MetadataValue::VarInt(0));
    value_definition!(shaking_direction, 9, MetadataValue::VarInt(1));
    value_definition!(shaking_multiplier, 10, MetadataValue::Float(0.0));
}

pub mod boat {
    use super::*;
    value_definition!(is_left_paddle_turning, 11, MetadataValue::Boolean(false));
    value_definition!(is_right_paddle_turning, 12, MetadataValue::Boolean(false));
    value_definition!(splash_timer, 13, MetadataValue::VarInt(0));
}

pub mod abstract_minecart {
    use super::*;
    value_definition!(custom_block_state, 11, MetadataValue::OptionalBlockState(0));
    value_definition!(custom_block_y_position, 12, MetadataValue::VarInt(6));
}

pub mod furnace_minecart {
    use super::*;
    value_definition!(has_fuel, 13, MetadataValue::Boolean(false));
}

pub mod command_block_minecart {
    use super::*;
    value_definition!(command, 13, MetadataValue::String(String::new()));
    value_definition!(last_output, 14, MetadataValue::Text(TextComponent::empty()));
}

pub mod end_crystal {
    use super::*;
    value_definition!(beam_target, 8, MetadataValue::OptionalPosition(None));
    value_definition!(show_bottom, 9, MetadataValue::Boolean(true));
}

pub mod smart_fireball {
    use super::*;
    pub fn item() -> MetadataDefinition {
        empty_item(8)
    }
}

pub mod fireball {
    use super::*;
    pub fn item() -> MetadataDefinition {
        empty_item(8)
    }
}

pub mod hanging {
    use super::*;
    value_definition!(direction, 8, MetadataValue::Direction(3));
}

pub mod item_frame {
    use super::*;
    pub fn item() -> MetadataDefinition {
        empty_item(9)
    }
    value_definition!(rotation, 10, MetadataValue::VarInt(0));
}

pub mod painting {
    use super::*;
    value_definition!(variant, 9, MetadataValue::PaintingVariant(24));
}

pub mod item_entity {
    use super::*;
    pub fn item() -> MetadataDefinition {
        empty_item(8)
    }
}

pub mod primed_tnt {
    use super::*;
    value_definition!(fuse_time, 8, MetadataValue::VarInt(80));
    value_definition!(
        block_state,
        9,
        MetadataValue::BlockState(spinel_registry::vanilla_world_blocks::Block::TNT.state_id())
    );
}

pub mod ominous_item_spawner {
    use super::*;
    pub fn item() -> MetadataDefinition {
        empty_item(8)
    }
}
