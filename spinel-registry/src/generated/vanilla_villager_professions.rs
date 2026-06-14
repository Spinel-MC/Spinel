use crate::{Identifier, VillagerProfession};
impl VillagerProfession {
    pub const NONE: Self = Self::new(0, Identifier::vanilla_static("none"));
    pub const ARMORER: Self = Self::new(1, Identifier::vanilla_static("armorer"));
    pub const BUTCHER: Self = Self::new(2, Identifier::vanilla_static("butcher"));
    pub const CARTOGRAPHER: Self = Self::new(3, Identifier::vanilla_static("cartographer"));
    pub const CLERIC: Self = Self::new(4, Identifier::vanilla_static("cleric"));
    pub const FARMER: Self = Self::new(5, Identifier::vanilla_static("farmer"));
    pub const FISHERMAN: Self = Self::new(6, Identifier::vanilla_static("fisherman"));
    pub const FLETCHER: Self = Self::new(7, Identifier::vanilla_static("fletcher"));
    pub const LEATHERWORKER: Self = Self::new(8, Identifier::vanilla_static("leatherworker"));
    pub const LIBRARIAN: Self = Self::new(9, Identifier::vanilla_static("librarian"));
    pub const MASON: Self = Self::new(10, Identifier::vanilla_static("mason"));
    pub const NITWIT: Self = Self::new(11, Identifier::vanilla_static("nitwit"));
    pub const SHEPHERD: Self = Self::new(12, Identifier::vanilla_static("shepherd"));
    pub const TOOLSMITH: Self = Self::new(13, Identifier::vanilla_static("toolsmith"));
    pub const WEAPONSMITH: Self = Self::new(14, Identifier::vanilla_static("weaponsmith"));
    pub fn from_protocol_id(protocol_id: i32) -> Option<Self> {
        match protocol_id {
            0 => Some(Self::NONE),
            1 => Some(Self::ARMORER),
            2 => Some(Self::BUTCHER),
            3 => Some(Self::CARTOGRAPHER),
            4 => Some(Self::CLERIC),
            5 => Some(Self::FARMER),
            6 => Some(Self::FISHERMAN),
            7 => Some(Self::FLETCHER),
            8 => Some(Self::LEATHERWORKER),
            9 => Some(Self::LIBRARIAN),
            10 => Some(Self::MASON),
            11 => Some(Self::NITWIT),
            12 => Some(Self::SHEPHERD),
            13 => Some(Self::TOOLSMITH),
            14 => Some(Self::WEAPONSMITH),
            _ => None,
        }
    }
    pub fn from_key(key: &Identifier) -> Option<Self> {
        match key.path.as_ref() {
            "none" => Some(Self::NONE),
            "armorer" => Some(Self::ARMORER),
            "butcher" => Some(Self::BUTCHER),
            "cartographer" => Some(Self::CARTOGRAPHER),
            "cleric" => Some(Self::CLERIC),
            "farmer" => Some(Self::FARMER),
            "fisherman" => Some(Self::FISHERMAN),
            "fletcher" => Some(Self::FLETCHER),
            "leatherworker" => Some(Self::LEATHERWORKER),
            "librarian" => Some(Self::LIBRARIAN),
            "mason" => Some(Self::MASON),
            "nitwit" => Some(Self::NITWIT),
            "shepherd" => Some(Self::SHEPHERD),
            "toolsmith" => Some(Self::TOOLSMITH),
            "weaponsmith" => Some(Self::WEAPONSMITH),
            _ => None,
        }
    }
}
