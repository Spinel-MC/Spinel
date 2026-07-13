#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerHand {
    Main,
    Off,
}

impl PlayerHand {
    pub const fn get_protocol_id(self) -> i32 {
        match self {
            Self::Main => 0,
            Self::Off => 1,
        }
    }

    pub fn from_protocol_id(protocol_id: i32) -> Option<Self> {
        match protocol_id {
            0 => Some(Self::Main),
            1 => Some(Self::Off),
            _ => None,
        }
    }
}
