use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::{compound_from_nbt, i32_field_or, string_field};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwingAnimation {
    animation_type: SwingAnimationType,
    duration: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwingAnimationType {
    None,
    Whack,
    Stab,
}

impl SwingAnimation {
    pub const DEFAULT: Self = Self::new(SwingAnimationType::Whack, 6);

    #[must_use]
    pub const fn new(animation_type: SwingAnimationType, duration: i32) -> Self {
        Self {
            animation_type,
            duration,
        }
    }

    #[must_use]
    pub const fn animation_type(&self) -> SwingAnimationType {
        self.animation_type
    }

    #[must_use]
    pub const fn duration(&self) -> i32 {
        self.duration
    }
}

impl SwingAnimationType {
    #[must_use]
    pub const fn nbt_name(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Whack => "whack",
            Self::Stab => "stab",
        }
    }

    #[must_use]
    pub const fn protocol_id(self) -> i32 {
        match self {
            Self::None => 0,
            Self::Whack => 1,
            Self::Stab => 2,
        }
    }

    #[must_use]
    pub fn from_nbt_name(name: &str) -> Option<Self> {
        match name {
            "none" => Some(Self::None),
            "whack" => Some(Self::Whack),
            "stab" => Some(Self::Stab),
            _ => None,
        }
    }
}

impl DataComponentValue for SwingAnimation {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        if self.animation_type != SwingAnimationType::Whack {
            compound.insert(
                "type".to_string(),
                Nbt::String(self.animation_type.nbt_name().to_string()),
            );
        }
        if self.duration != 6 {
            compound.insert("duration".to_string(), Nbt::Int(self.duration));
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let animation_type = match compound.get("type") {
            Some(_) => SwingAnimationType::from_nbt_name(&string_field(compound, "type")?)?,
            None => SwingAnimationType::Whack,
        };
        Some(Self {
            animation_type,
            duration: i32_field_or(compound, "duration", 6)?,
        })
    }
}
