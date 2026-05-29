use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::string_from_nbt;
use spinel_nbt::Nbt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}

impl ItemRarity {
    #[must_use]
    pub const fn protocol_id(self) -> i32 {
        match self {
            Self::Common => 0,
            Self::Uncommon => 1,
            Self::Rare => 2,
            Self::Epic => 3,
        }
    }

    #[must_use]
    pub const fn nbt_name(self) -> &'static str {
        match self {
            Self::Common => "common",
            Self::Uncommon => "uncommon",
            Self::Rare => "rare",
            Self::Epic => "epic",
        }
    }

    #[must_use]
    pub fn from_nbt_name(name: &str) -> Option<Self> {
        match name {
            "common" => Some(Self::Common),
            "uncommon" => Some(Self::Uncommon),
            "rare" => Some(Self::Rare),
            "epic" => Some(Self::Epic),
            _ => None,
        }
    }
}

impl DataComponentValue for ItemRarity {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::String(self.nbt_name().to_string())
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        Self::from_nbt_name(&string_from_nbt(component_nbt)?)
    }
}
