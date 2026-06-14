#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CopperGolemWeatherState {
    #[default]
    Unaffected,
    Exposed,
    Weathered,
    Oxidized,
}

impl CopperGolemWeatherState {
    pub const fn protocol_id(self) -> i32 {
        self as i32
    }

    pub const fn from_protocol_id(protocol_id: i32) -> Option<Self> {
        match protocol_id {
            0 => Some(Self::Unaffected),
            1 => Some(Self::Exposed),
            2 => Some(Self::Weathered),
            3 => Some(Self::Oxidized),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CopperGolemState {
    #[default]
    Idle,
    GettingItem,
    GettingNoItem,
    DroppingItem,
    DroppingNoItem,
}

impl CopperGolemState {
    pub const fn protocol_id(self) -> i32 {
        self as i32
    }

    pub const fn from_protocol_id(protocol_id: i32) -> Option<Self> {
        match protocol_id {
            0 => Some(Self::Idle),
            1 => Some(Self::GettingItem),
            2 => Some(Self::GettingNoItem),
            3 => Some(Self::DroppingItem),
            4 => Some(Self::DroppingNoItem),
            _ => None,
        }
    }
}
