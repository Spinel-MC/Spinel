use crate::Identifier;
use crate::data_components::nbt_reader::{compound_from_nbt, f32_field_or, string_field};
use crate::data_components::{DataComponentValue, RegistryTagReference};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Debug, PartialEq)]
pub struct BlocksAttacks {
    block_delay_seconds: f32,
    disable_cooldown_scale: f32,
    damage_reductions: Vec<DamageReduction>,
    item_damage: ItemDamageFunction,
    bypassed_by: Option<Identifier>,
    block_sound: Option<Identifier>,
    disable_sound: Option<Identifier>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ItemDamageFunction {
    threshold: f32,
    base: f32,
    factor: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DamageReduction {
    horizontal_blocking_angle: f32,
    damage_type: Option<RegistryTagReference>,
    base: f32,
    factor: f32,
}

impl BlocksAttacks {
    #[must_use]
    pub fn new(
        block_delay_seconds: f32,
        disable_cooldown_scale: f32,
        damage_reductions: Vec<DamageReduction>,
        item_damage: ItemDamageFunction,
        bypassed_by: Option<Identifier>,
        block_sound: Option<Identifier>,
        disable_sound: Option<Identifier>,
    ) -> Self {
        Self {
            block_delay_seconds,
            disable_cooldown_scale,
            damage_reductions,
            item_damage,
            bypassed_by,
            block_sound,
            disable_sound,
        }
    }

    #[must_use]
    pub const fn block_delay_seconds(&self) -> f32 {
        self.block_delay_seconds
    }

    #[must_use]
    pub const fn disable_cooldown_scale(&self) -> f32 {
        self.disable_cooldown_scale
    }

    #[must_use]
    pub fn damage_reductions(&self) -> &[DamageReduction] {
        &self.damage_reductions
    }

    #[must_use]
    pub const fn item_damage(&self) -> ItemDamageFunction {
        self.item_damage
    }

    #[must_use]
    pub const fn bypassed_by(&self) -> Option<&Identifier> {
        self.bypassed_by.as_ref()
    }

    #[must_use]
    pub const fn block_sound(&self) -> Option<&Identifier> {
        self.block_sound.as_ref()
    }

    #[must_use]
    pub const fn disable_sound(&self) -> Option<&Identifier> {
        self.disable_sound.as_ref()
    }
}

impl ItemDamageFunction {
    pub const DEFAULT: Self = Self::new(1.0, 0.0, 1.0);

    #[must_use]
    pub const fn new(threshold: f32, base: f32, factor: f32) -> Self {
        Self {
            threshold,
            base,
            factor,
        }
    }

    #[must_use]
    pub const fn threshold(self) -> f32 {
        self.threshold
    }

    #[must_use]
    pub const fn base(self) -> f32 {
        self.base
    }

    #[must_use]
    pub const fn factor(self) -> f32 {
        self.factor
    }
}

impl DamageReduction {
    pub const DEFAULT: Self = Self {
        horizontal_blocking_angle: 90.0,
        damage_type: None,
        base: 0.0,
        factor: 1.0,
    };

    #[must_use]
    pub const fn new(
        horizontal_blocking_angle: f32,
        damage_type: Option<RegistryTagReference>,
        base: f32,
        factor: f32,
    ) -> Self {
        Self {
            horizontal_blocking_angle,
            damage_type,
            base,
            factor,
        }
    }

    #[must_use]
    pub const fn horizontal_blocking_angle(&self) -> f32 {
        self.horizontal_blocking_angle
    }

    #[must_use]
    pub const fn damage_type(&self) -> Option<&RegistryTagReference> {
        self.damage_type.as_ref()
    }

    #[must_use]
    pub const fn base(&self) -> f32 {
        self.base
    }

    #[must_use]
    pub const fn factor(&self) -> f32 {
        self.factor
    }
}

impl DataComponentValue for BlocksAttacks {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        if self.block_delay_seconds != 0.0 {
            compound.insert(
                "block_delay_seconds".to_string(),
                Nbt::Float(self.block_delay_seconds),
            );
        }
        if self.disable_cooldown_scale != 1.0 {
            compound.insert(
                "disable_cooldown_scale".to_string(),
                Nbt::Float(self.disable_cooldown_scale),
            );
        }
        compound.insert(
            "damage_reductions".to_string(),
            Nbt::List(
                self.damage_reductions
                    .iter()
                    .map(DamageReduction::to_nbt)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
        );
        compound.insert("item_damage".to_string(), self.item_damage.to_nbt());
        if let Some(bypassed_by) = &self.bypassed_by {
            compound.insert(
                "bypassed_by".to_string(),
                Nbt::String(format!("#{bypassed_by}")),
            );
        }
        if let Some(block_sound) = &self.block_sound {
            compound.insert(
                "block_sound".to_string(),
                Nbt::String(block_sound.to_string()),
            );
        }
        if let Some(disable_sound) = &self.disable_sound {
            compound.insert(
                "disabled_sound".to_string(),
                Nbt::String(disable_sound.to_string()),
            );
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let damage_reductions = match compound.get("damage_reductions") {
            Some(Nbt::List(reductions)) => reductions
                .iter()
                .map(DamageReduction::from_nbt)
                .collect::<Option<Vec<_>>>()?,
            None => vec![DamageReduction::DEFAULT],
            _ => return None,
        };
        Some(Self {
            block_delay_seconds: f32_field_or(compound, "block_delay_seconds", 0.0)?,
            disable_cooldown_scale: f32_field_or(compound, "disable_cooldown_scale", 1.0)?,
            damage_reductions,
            item_damage: match compound.get("item_damage") {
                Some(item_damage) => ItemDamageFunction::from_nbt(item_damage)?,
                None => ItemDamageFunction::DEFAULT,
            },
            bypassed_by: optional_tag_key(compound, "bypassed_by")?,
            block_sound: optional_identifier(compound, "block_sound")?,
            disable_sound: optional_identifier(compound, "disabled_sound")?,
        })
    }
}

impl ItemDamageFunction {
    fn to_nbt(self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert("threshold".to_string(), Nbt::Float(self.threshold));
        compound.insert("base".to_string(), Nbt::Float(self.base));
        compound.insert("factor".to_string(), Nbt::Float(self.factor));
        Nbt::Compound(compound)
    }

    fn from_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        Some(Self {
            threshold: f32_field_or(compound, "threshold", 1.0)?,
            base: f32_field_or(compound, "base", 0.0)?,
            factor: f32_field_or(compound, "factor", 1.0)?,
        })
    }
}

impl DamageReduction {
    fn to_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        if self.horizontal_blocking_angle != 90.0 {
            compound.insert(
                "horizontal_blocking_angle".to_string(),
                Nbt::Float(self.horizontal_blocking_angle),
            );
        }
        if let Some(damage_type) = &self.damage_type {
            compound.insert("type".to_string(), damage_type.to_nbt());
        }
        compound.insert("base".to_string(), Nbt::Float(self.base));
        compound.insert("factor".to_string(), Nbt::Float(self.factor));
        Nbt::Compound(compound)
    }

    fn from_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let damage_type = match compound.get("type") {
            Some(damage_type) => Some(RegistryTagReference::from_nbt(damage_type)?),
            None => None,
        };
        Some(Self {
            horizontal_blocking_angle: f32_field_or(compound, "horizontal_blocking_angle", 90.0)?,
            damage_type,
            base: f32_field_or(compound, "base", 0.0)?,
            factor: f32_field_or(compound, "factor", 1.0)?,
        })
    }
}

fn optional_tag_key(compound: &NbtCompound, name: &str) -> Option<Option<Identifier>> {
    match compound.get(name) {
        Some(_) => {
            let tag_key = string_field(compound, name)?;
            Some(Some(
                tag_key.strip_prefix('#').unwrap_or(&tag_key).parse().ok()?,
            ))
        }
        None => Some(None),
    }
}

fn optional_identifier(compound: &NbtCompound, name: &str) -> Option<Option<Identifier>> {
    match compound.get(name) {
        Some(_) => Some(Some(string_field(compound, name)?.parse().ok()?)),
        None => Some(None),
    }
}
