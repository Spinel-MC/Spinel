use spinel_registry::{VillagerProfession, VillagerType};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum VillagerLevel {
    #[default]
    Novice,
    Apprentice,
    Journeyman,
    Expert,
    Master,
}

impl VillagerLevel {
    pub const fn protocol_id(self) -> i32 {
        match self {
            Self::Novice => 1,
            Self::Apprentice => 2,
            Self::Journeyman => 3,
            Self::Expert => 4,
            Self::Master => 5,
        }
    }

    pub const fn from_protocol_id(protocol_id: i32) -> Option<Self> {
        match protocol_id {
            1 => Some(Self::Novice),
            2 => Some(Self::Apprentice),
            3 => Some(Self::Journeyman),
            4 => Some(Self::Expert),
            5 => Some(Self::Master),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VillagerData {
    villager_type: VillagerType,
    profession: VillagerProfession,
    level: VillagerLevel,
}

impl VillagerData {
    pub const DEFAULT: Self = Self::new(
        VillagerType::DESERT,
        VillagerProfession::NONE,
        VillagerLevel::Novice,
    );

    pub const fn new(
        villager_type: VillagerType,
        profession: VillagerProfession,
        level: VillagerLevel,
    ) -> Self {
        Self {
            villager_type,
            profession,
            level,
        }
    }

    pub fn from_protocol_ids(
        villager_type_id: i32,
        profession_id: i32,
        level_id: i32,
    ) -> Option<Self> {
        Some(Self::new(
            VillagerType::from_protocol_id(villager_type_id)?,
            VillagerProfession::from_protocol_id(profession_id)?,
            VillagerLevel::from_protocol_id(level_id)?,
        ))
    }

    pub const fn villager_type(&self) -> &VillagerType {
        &self.villager_type
    }

    pub const fn profession(&self) -> &VillagerProfession {
        &self.profession
    }

    pub const fn level(&self) -> VillagerLevel {
        self.level
    }

    pub fn with_type(&self, villager_type: VillagerType) -> Self {
        Self::new(villager_type, self.profession.clone(), self.level)
    }

    pub fn with_profession(&self, profession: VillagerProfession) -> Self {
        Self::new(self.villager_type.clone(), profession, self.level)
    }

    pub fn with_level(&self, level: VillagerLevel) -> Self {
        Self::new(self.villager_type.clone(), self.profession.clone(), level)
    }
}

impl Default for VillagerData {
    fn default() -> Self {
        Self::DEFAULT
    }
}
