use crate::entity::{EntityPosition, GenericEntity};
use spinel_nbt::{Nbt, NbtCompound};
use spinel_registry::EntityType;
use spinel_utils::component::text::TextComponent;

impl GenericEntity {
    pub(crate) fn apply_summon_nbt(&mut self, nbt: &NbtCompound) {
        self.apply_summon_position(nbt);
        self.apply_summon_rotation(nbt);
        self.apply_summon_base_state(nbt);
        self.apply_summon_living_state(nbt);
        self.apply_summon_type_state(nbt);
    }

    fn apply_summon_position(&mut self, nbt: &NbtCompound) {
        let Some([x, y, z]) = nbt_vector(nbt.get("Pos")) else {
            return;
        };
        self.set_position(EntityPosition::new(
            x,
            y,
            z,
            self.position().yaw(),
            self.position().pitch(),
        ));
    }

    fn apply_summon_rotation(&mut self, nbt: &NbtCompound) {
        let Some([yaw, pitch]) = nbt_rotation(nbt.get("Rotation")) else {
            return;
        };
        self.set_position(self.position().with_view(yaw, pitch));
    }

    fn apply_summon_base_state(&mut self, nbt: &NbtCompound) {
        apply_boolean(nbt, "OnGround", |value| self.set_on_ground(value));
        apply_boolean(nbt, "HasVisualFire", |value| self.set_on_fire(value));
        apply_boolean(nbt, "Invisible", |value| self.set_invisible(value));
        apply_boolean(nbt, "Glowing", |value| self.set_glowing(value));
        apply_boolean(nbt, "Silent", |value| self.set_silent(value));
        apply_boolean(nbt, "NoGravity", |value| self.set_no_gravity(value));
        apply_boolean(nbt, "CustomNameVisible", |value| {
            self.set_custom_name_visible(value);
        });
        let custom_name = nbt
            .get("CustomName")
            .and_then(nbt_string)
            .and_then(|custom_name| serde_json::from_str::<TextComponent>(custom_name).ok());
        if custom_name.is_some() {
            self.set_custom_name(custom_name);
        }
    }

    fn apply_summon_living_state(&mut self, nbt: &NbtCompound) {
        if !self.entity_type().is_living() {
            return;
        }
        apply_boolean(nbt, "Invulnerable", |value| {
            self.set_invulnerable(value);
        });
        if let Some(health) = nbt.get("Health").and_then(nbt_number) {
            self.set_health(health as f32);
        }
    }

    fn apply_summon_type_state(&mut self, nbt: &NbtCompound) {
        match self.entity_type() {
            EntityType::ARMOR_STAND => self.apply_armor_stand_summon_state(nbt),
            EntityType::SLIME | EntityType::MAGMA_CUBE => {
                if let Some(size) = nbt.get("Size").and_then(nbt_i32) {
                    self.set_slime_size(size);
                }
            }
            _ => {}
        }
    }

    fn apply_armor_stand_summon_state(&mut self, nbt: &NbtCompound) {
        apply_boolean(nbt, "Small", |value| {
            self.set_armor_stand_small(value);
        });
        apply_boolean(nbt, "ShowArms", |value| {
            self.set_armor_stand_arms(value);
        });
        apply_boolean(nbt, "NoBasePlate", |value| {
            self.set_armor_stand_no_base_plate(value);
        });
        apply_boolean(nbt, "Marker", |value| {
            self.set_armor_stand_marker(value);
        });
    }
}

fn apply_boolean(nbt: &NbtCompound, key: &str, mut apply: impl FnMut(bool)) {
    if let Some(value) = nbt.get(key).and_then(nbt_boolean) {
        apply(value);
    }
}

fn nbt_boolean(nbt: &Nbt) -> Option<bool> {
    match nbt {
        Nbt::Byte(value) => Some(*value != 0),
        _ => None,
    }
}

fn nbt_string(nbt: &Nbt) -> Option<&str> {
    match nbt {
        Nbt::String(value) => Some(value),
        _ => None,
    }
}

fn nbt_number(nbt: &Nbt) -> Option<f64> {
    match nbt {
        Nbt::Byte(value) => Some(f64::from(*value)),
        Nbt::Short(value) => Some(f64::from(*value)),
        Nbt::Int(value) => Some(f64::from(*value)),
        Nbt::Long(value) => Some(*value as f64),
        Nbt::Float(value) => Some(f64::from(*value)),
        Nbt::Double(value) => Some(*value),
        _ => None,
    }
}

fn nbt_i32(nbt: &Nbt) -> Option<i32> {
    match nbt {
        Nbt::Byte(value) => Some(i32::from(*value)),
        Nbt::Short(value) => Some(i32::from(*value)),
        Nbt::Int(value) => Some(*value),
        Nbt::Long(value) => i32::try_from(*value).ok(),
        _ => None,
    }
}

fn nbt_vector(nbt: Option<&Nbt>) -> Option<[f64; 3]> {
    let Nbt::List(values) = nbt? else {
        return None;
    };
    if values.len() != 3 {
        return None;
    }
    Some([
        nbt_number(&values[0])?,
        nbt_number(&values[1])?,
        nbt_number(&values[2])?,
    ])
}

fn nbt_rotation(nbt: Option<&Nbt>) -> Option<[f32; 2]> {
    let Nbt::List(values) = nbt? else {
        return None;
    };
    if values.len() != 2 {
        return None;
    }
    Some([
        nbt_number(&values[0])? as f32,
        nbt_number(&values[1])? as f32,
    ])
}
