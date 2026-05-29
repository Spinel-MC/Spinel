use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::string_from_nbt;
use spinel_nbt::Nbt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MapPostProcessing {
    Lock,
    Scale,
}

impl MapPostProcessing {
    #[must_use]
    pub const fn nbt_name(self) -> &'static str {
        match self {
            Self::Lock => "lock",
            Self::Scale => "scale",
        }
    }

    #[must_use]
    pub const fn protocol_id(self) -> i32 {
        match self {
            Self::Lock => 0,
            Self::Scale => 1,
        }
    }

    #[must_use]
    pub fn from_nbt_name(name: &str) -> Option<Self> {
        match name {
            "lock" => Some(Self::Lock),
            "scale" => Some(Self::Scale),
            _ => None,
        }
    }
}

impl DataComponentValue for MapPostProcessing {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::String(self.nbt_name().to_string())
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        Self::from_nbt_name(&string_from_nbt(component_nbt)?)
    }
}
