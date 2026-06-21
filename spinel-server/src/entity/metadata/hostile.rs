#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CreeperState {
    #[default]
    Idle,
    Fuse,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpellcasterIllagerSpell {
    #[default]
    None,
    SummonVex,
    Attack,
    Wololo,
    Disappear,
    Blindness,
}

impl SpellcasterIllagerSpell {
    pub const fn get_protocol_id(self) -> i8 {
        self as i8
    }

    pub const fn from_protocol_id(protocol_id: i8) -> Option<Self> {
        match protocol_id {
            0 => Some(Self::None),
            1 => Some(Self::SummonVex),
            2 => Some(Self::Attack),
            3 => Some(Self::Wololo),
            4 => Some(Self::Disappear),
            5 => Some(Self::Blindness),
            _ => None,
        }
    }
}

impl CreeperState {
    pub const fn get_protocol_id(self) -> i32 {
        match self {
            Self::Idle => -1,
            Self::Fuse => 1,
        }
    }

    pub const fn from_protocol_id(protocol_id: i32) -> Self {
        match protocol_id {
            -1 => Self::Idle,
            _ => Self::Fuse,
        }
    }
}
