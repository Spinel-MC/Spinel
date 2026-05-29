use crate::data_components::nbt_reader::{
    bool_field_or, compound_from_nbt, f32_field_or, i32_field_or,
};
use crate::data_components::{DataComponentValue, RegistryTagReference};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Debug, PartialEq)]
pub struct Tool {
    rules: Vec<ToolRule>,
    default_mining_speed: f32,
    damage_per_block: i32,
    can_destroy_blocks_in_creative: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ToolRule {
    blocks: RegistryTagReference,
    speed: Option<f32>,
    correct_for_drops: Option<bool>,
}

impl Tool {
    #[must_use]
    pub fn new(
        rules: Vec<ToolRule>,
        default_mining_speed: f32,
        damage_per_block: i32,
        can_destroy_blocks_in_creative: bool,
    ) -> Self {
        Self {
            rules,
            default_mining_speed,
            damage_per_block,
            can_destroy_blocks_in_creative,
        }
    }

    #[must_use]
    pub fn rules(&self) -> &[ToolRule] {
        &self.rules
    }

    #[must_use]
    pub const fn default_mining_speed(&self) -> f32 {
        self.default_mining_speed
    }

    #[must_use]
    pub const fn damage_per_block(&self) -> i32 {
        self.damage_per_block
    }

    #[must_use]
    pub const fn can_destroy_blocks_in_creative(&self) -> bool {
        self.can_destroy_blocks_in_creative
    }
}

impl ToolRule {
    #[must_use]
    pub const fn new(
        blocks: RegistryTagReference,
        speed: Option<f32>,
        correct_for_drops: Option<bool>,
    ) -> Self {
        Self {
            blocks,
            speed,
            correct_for_drops,
        }
    }

    #[must_use]
    pub const fn blocks(&self) -> &RegistryTagReference {
        &self.blocks
    }

    #[must_use]
    pub const fn speed(&self) -> Option<f32> {
        self.speed
    }

    #[must_use]
    pub const fn correct_for_drops(&self) -> Option<bool> {
        self.correct_for_drops
    }
}

impl DataComponentValue for Tool {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert(
            "rules".to_string(),
            Nbt::List(
                self.rules
                    .iter()
                    .map(ToolRule::to_nbt)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
        );
        if self.default_mining_speed != 1.0 {
            compound.insert(
                "default_mining_speed".to_string(),
                Nbt::Float(self.default_mining_speed),
            );
        }
        if self.damage_per_block != 1 {
            compound.insert(
                "damage_per_block".to_string(),
                Nbt::Int(self.damage_per_block),
            );
        }
        if !self.can_destroy_blocks_in_creative {
            compound.insert("can_destroy_blocks_in_creative".to_string(), Nbt::Byte(0));
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let rules = match compound.get("rules") {
            Some(Nbt::List(rules)) => rules
                .iter()
                .map(ToolRule::from_nbt)
                .collect::<Option<Vec<_>>>()?,
            None => Vec::new(),
            _ => return None,
        };
        Some(Self {
            rules,
            default_mining_speed: f32_field_or(compound, "default_mining_speed", 1.0)?,
            damage_per_block: i32_field_or(compound, "damage_per_block", 1)?,
            can_destroy_blocks_in_creative: bool_field_or(
                compound,
                "can_destroy_blocks_in_creative",
                true,
            )?,
        })
    }
}

impl ToolRule {
    fn to_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert("blocks".to_string(), self.blocks.to_nbt());
        if let Some(speed) = self.speed {
            compound.insert("speed".to_string(), Nbt::Float(speed));
        }
        if let Some(correct_for_drops) = self.correct_for_drops {
            compound.insert(
                "correct_for_drops".to_string(),
                Nbt::Byte(i8::from(correct_for_drops)),
            );
        }
        Nbt::Compound(compound)
    }

    fn from_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let speed = match compound.get("speed") {
            Some(_) => Some(f32_field_or(compound, "speed", 0.0)?),
            None => None,
        };
        let correct_for_drops = match compound.get("correct_for_drops") {
            Some(_) => Some(bool_field_or(compound, "correct_for_drops", false)?),
            None => None,
        };
        Some(Self {
            blocks: RegistryTagReference::from_nbt(compound.get("blocks")?)?,
            speed,
            correct_for_drops,
        })
    }
}
