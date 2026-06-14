use crate::{Identifier, VillagerType};
impl VillagerType {
    pub const DESERT: Self = Self::new(0, Identifier::vanilla_static("desert"));
    pub const JUNGLE: Self = Self::new(1, Identifier::vanilla_static("jungle"));
    pub const PLAINS: Self = Self::new(2, Identifier::vanilla_static("plains"));
    pub const SAVANNA: Self = Self::new(3, Identifier::vanilla_static("savanna"));
    pub const SNOW: Self = Self::new(4, Identifier::vanilla_static("snow"));
    pub const SWAMP: Self = Self::new(5, Identifier::vanilla_static("swamp"));
    pub const TAIGA: Self = Self::new(6, Identifier::vanilla_static("taiga"));
    pub fn from_protocol_id(protocol_id: i32) -> Option<Self> {
        match protocol_id {
            0 => Some(Self::DESERT),
            1 => Some(Self::JUNGLE),
            2 => Some(Self::PLAINS),
            3 => Some(Self::SAVANNA),
            4 => Some(Self::SNOW),
            5 => Some(Self::SWAMP),
            6 => Some(Self::TAIGA),
            _ => None,
        }
    }
    pub fn from_key(key: &Identifier) -> Option<Self> {
        match key.path.as_ref() {
            "desert" => Some(Self::DESERT),
            "jungle" => Some(Self::JUNGLE),
            "plains" => Some(Self::PLAINS),
            "savanna" => Some(Self::SAVANNA),
            "snow" => Some(Self::SNOW),
            "swamp" => Some(Self::SWAMP),
            "taiga" => Some(Self::TAIGA),
            _ => None,
        }
    }
}
