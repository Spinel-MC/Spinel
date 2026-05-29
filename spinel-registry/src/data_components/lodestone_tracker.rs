use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::{
    bool_field_or, compound_from_nbt, i32_field_or, string_field,
};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LodestoneTracker {
    target: Option<WorldPosition>,
    tracked: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldPosition {
    dimension: String,
    x: i32,
    y: i32,
    z: i32,
}

impl LodestoneTracker {
    #[must_use]
    pub const fn new(target: Option<WorldPosition>, tracked: bool) -> Self {
        Self { target, tracked }
    }

    #[must_use]
    pub const fn target(&self) -> Option<&WorldPosition> {
        self.target.as_ref()
    }

    #[must_use]
    pub const fn tracked(&self) -> bool {
        self.tracked
    }

    #[must_use]
    pub fn with_target(&self, target: Option<WorldPosition>) -> Self {
        Self {
            target,
            tracked: self.tracked,
        }
    }

    #[must_use]
    pub fn with_tracked(&self, tracked: bool) -> Self {
        Self {
            target: self.target.clone(),
            tracked,
        }
    }
}

impl WorldPosition {
    #[must_use]
    pub fn new(dimension: String, x: i32, y: i32, z: i32) -> Self {
        Self { dimension, x, y, z }
    }

    #[must_use]
    pub fn dimension(&self) -> &str {
        &self.dimension
    }

    #[must_use]
    pub const fn x(&self) -> i32 {
        self.x
    }

    #[must_use]
    pub const fn y(&self) -> i32 {
        self.y
    }

    #[must_use]
    pub const fn z(&self) -> i32 {
        self.z
    }
}

impl DataComponentValue for LodestoneTracker {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        if let Some(target) = &self.target {
            compound.insert("target".to_string(), target.to_nbt());
        }
        if !self.tracked {
            compound.insert("tracked".to_string(), Nbt::Byte(0));
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let target = match compound.get("target") {
            Some(target) => Some(WorldPosition::from_nbt(target)?),
            None => None,
        };
        Some(Self {
            target,
            tracked: bool_field_or(compound, "tracked", true)?,
        })
    }
}

impl WorldPosition {
    fn to_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert("dimension".to_string(), Nbt::String(self.dimension.clone()));
        let mut position = NbtCompound::new();
        position.insert("x".to_string(), Nbt::Int(self.x));
        position.insert("y".to_string(), Nbt::Int(self.y));
        position.insert("z".to_string(), Nbt::Int(self.z));
        compound.insert("pos".to_string(), Nbt::Compound(position));
        Nbt::Compound(compound)
    }

    fn from_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let position = compound_from_nbt(compound.get("pos")?)?;
        Some(Self {
            dimension: string_field(compound, "dimension")?,
            x: i32_field_or(position, "x", 0)?,
            y: i32_field_or(position, "y", 0)?,
            z: i32_field_or(position, "z", 0)?,
        })
    }
}
