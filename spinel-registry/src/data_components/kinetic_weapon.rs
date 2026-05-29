use crate::Identifier;
use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::{
    compound_from_nbt, f32_field_or, i32_field_or, string_field,
};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Debug, PartialEq)]
pub struct KineticWeapon {
    contact_cooldown_ticks: i32,
    delay_ticks: i32,
    dismount_conditions: Option<KineticWeaponCondition>,
    knockback_conditions: Option<KineticWeaponCondition>,
    damage_conditions: Option<KineticWeaponCondition>,
    forward_movement: f32,
    damage_multiplier: f32,
    sound: Option<Identifier>,
    hit_sound: Option<Identifier>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KineticWeaponCondition {
    max_duration_ticks: i32,
    min_speed: f32,
    min_relative_speed: f32,
}

impl KineticWeapon {
    #[must_use]
    pub fn new(
        contact_cooldown_ticks: i32,
        delay_ticks: i32,
        dismount_conditions: Option<KineticWeaponCondition>,
        knockback_conditions: Option<KineticWeaponCondition>,
        damage_conditions: Option<KineticWeaponCondition>,
        forward_movement: f32,
        damage_multiplier: f32,
        sound: Option<Identifier>,
        hit_sound: Option<Identifier>,
    ) -> Self {
        Self {
            contact_cooldown_ticks,
            delay_ticks,
            dismount_conditions,
            knockback_conditions,
            damage_conditions,
            forward_movement,
            damage_multiplier,
            sound,
            hit_sound,
        }
    }

    #[must_use]
    pub const fn contact_cooldown_ticks(&self) -> i32 {
        self.contact_cooldown_ticks
    }

    #[must_use]
    pub const fn delay_ticks(&self) -> i32 {
        self.delay_ticks
    }

    #[must_use]
    pub const fn dismount_conditions(&self) -> Option<KineticWeaponCondition> {
        self.dismount_conditions
    }

    #[must_use]
    pub const fn knockback_conditions(&self) -> Option<KineticWeaponCondition> {
        self.knockback_conditions
    }

    #[must_use]
    pub const fn damage_conditions(&self) -> Option<KineticWeaponCondition> {
        self.damage_conditions
    }

    #[must_use]
    pub const fn forward_movement(&self) -> f32 {
        self.forward_movement
    }

    #[must_use]
    pub const fn damage_multiplier(&self) -> f32 {
        self.damage_multiplier
    }

    #[must_use]
    pub const fn sound(&self) -> Option<&Identifier> {
        self.sound.as_ref()
    }

    #[must_use]
    pub const fn hit_sound(&self) -> Option<&Identifier> {
        self.hit_sound.as_ref()
    }
}

impl KineticWeaponCondition {
    #[must_use]
    pub const fn new(max_duration_ticks: i32, min_speed: f32, min_relative_speed: f32) -> Self {
        Self {
            max_duration_ticks,
            min_speed,
            min_relative_speed,
        }
    }

    #[must_use]
    pub const fn max_duration_ticks(&self) -> i32 {
        self.max_duration_ticks
    }

    #[must_use]
    pub const fn min_speed(&self) -> f32 {
        self.min_speed
    }

    #[must_use]
    pub const fn min_relative_speed(&self) -> f32 {
        self.min_relative_speed
    }
}

impl DataComponentValue for KineticWeapon {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        if self.contact_cooldown_ticks != 10 {
            compound.insert(
                "contact_cooldown_ticks".to_string(),
                Nbt::Int(self.contact_cooldown_ticks),
            );
        }
        if self.delay_ticks != 0 {
            compound.insert("delay_ticks".to_string(), Nbt::Int(self.delay_ticks));
        }
        insert_condition(
            &mut compound,
            "dismount_conditions",
            self.dismount_conditions,
        );
        insert_condition(
            &mut compound,
            "knockback_conditions",
            self.knockback_conditions,
        );
        insert_condition(&mut compound, "damage_conditions", self.damage_conditions);
        if self.forward_movement != 0.0 {
            compound.insert(
                "forward_movement".to_string(),
                Nbt::Float(self.forward_movement),
            );
        }
        if self.damage_multiplier != 1.0 {
            compound.insert(
                "damage_multiplier".to_string(),
                Nbt::Float(self.damage_multiplier),
            );
        }
        if let Some(sound) = &self.sound {
            compound.insert("sound".to_string(), Nbt::String(sound.to_string()));
        }
        if let Some(hit_sound) = &self.hit_sound {
            compound.insert("hit_sound".to_string(), Nbt::String(hit_sound.to_string()));
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        Some(Self {
            contact_cooldown_ticks: i32_field_or(compound, "contact_cooldown_ticks", 10)?,
            delay_ticks: i32_field_or(compound, "delay_ticks", 0)?,
            dismount_conditions: optional_condition(compound, "dismount_conditions")?,
            knockback_conditions: optional_condition(compound, "knockback_conditions")?,
            damage_conditions: optional_condition(compound, "damage_conditions")?,
            forward_movement: f32_field_or(compound, "forward_movement", 0.0)?,
            damage_multiplier: f32_field_or(compound, "damage_multiplier", 1.0)?,
            sound: optional_identifier(compound, "sound")?,
            hit_sound: optional_identifier(compound, "hit_sound")?,
        })
    }
}

fn insert_condition(
    compound: &mut NbtCompound,
    name: &str,
    condition: Option<KineticWeaponCondition>,
) {
    if let Some(condition) = condition {
        compound.insert(name.to_string(), condition.to_nbt());
    }
}

fn optional_condition(
    compound: &NbtCompound,
    name: &str,
) -> Option<Option<KineticWeaponCondition>> {
    match compound.get(name) {
        Some(value) => Some(Some(KineticWeaponCondition::from_nbt(value)?)),
        None => Some(None),
    }
}

fn optional_identifier(compound: &NbtCompound, name: &str) -> Option<Option<Identifier>> {
    match compound.get(name) {
        Some(_) => Some(Some(string_field(compound, name)?.parse().ok()?)),
        None => Some(None),
    }
}

impl KineticWeaponCondition {
    fn to_nbt(self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert(
            "max_duration_ticks".to_string(),
            Nbt::Int(self.max_duration_ticks),
        );
        if self.min_speed != 0.0 {
            compound.insert("min_speed".to_string(), Nbt::Float(self.min_speed));
        }
        if self.min_relative_speed != 0.0 {
            compound.insert(
                "min_relative_speed".to_string(),
                Nbt::Float(self.min_relative_speed),
            );
        }
        Nbt::Compound(compound)
    }

    fn from_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        Some(Self {
            max_duration_ticks: i32_field_or(compound, "max_duration_ticks", 0)?,
            min_speed: f32_field_or(compound, "min_speed", 0.0)?,
            min_relative_speed: f32_field_or(compound, "min_relative_speed", 0.0)?,
        })
    }
}
