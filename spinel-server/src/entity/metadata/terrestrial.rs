#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ArmadilloState {
    #[default]
    Idle,
    Rolling,
    Scared,
    Unrolling,
}

impl ArmadilloState {
    pub const fn protocol_id(self) -> i32 {
        match self {
            Self::Idle => 0,
            Self::Rolling => 1,
            Self::Scared => 2,
            Self::Unrolling => 3,
        }
    }

    pub fn from_protocol_id(protocol_id: i32) -> Option<Self> {
        match protocol_id {
            0 => Some(Self::Idle),
            1 => Some(Self::Rolling),
            2 => Some(Self::Scared),
            3 => Some(Self::Unrolling),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SnifferState {
    #[default]
    Idling,
    FeelingHappy,
    Scenting,
    Sniffing,
    Searching,
    Digging,
    Rising,
}

impl SnifferState {
    pub const fn protocol_id(self) -> i32 {
        match self {
            Self::Idling => 0,
            Self::FeelingHappy => 1,
            Self::Scenting => 2,
            Self::Sniffing => 3,
            Self::Searching => 4,
            Self::Digging => 5,
            Self::Rising => 6,
        }
    }

    pub const fn from_protocol_id(protocol_id: i32) -> Option<Self> {
        match protocol_id {
            0 => Some(Self::Idling),
            1 => Some(Self::FeelingHappy),
            2 => Some(Self::Scenting),
            3 => Some(Self::Sniffing),
            4 => Some(Self::Searching),
            5 => Some(Self::Digging),
            6 => Some(Self::Rising),
            _ => None,
        }
    }
}

macro_rules! ordinal_enum {
    ($name:ident, $default:ident, $($variant:ident),+ $(,)?) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub enum $name {
            #[default]
            $default,
            $($variant),+
        }

        impl $name {
            pub const fn protocol_id(self) -> i32 {
                self as i32
            }

            pub const fn from_protocol_id(protocol_id: i32) -> Option<Self> {
                match protocol_id {
                    protocol_id if protocol_id == Self::$default as i32 => Some(Self::$default),
                    $(protocol_id if protocol_id == Self::$variant as i32 => Some(Self::$variant),)+
                    _ => None,
                }
            }
        }
    };
}

ordinal_enum!(HorseMarking, None, White, WhiteField, WhiteDots, BlackDots);
ordinal_enum!(
    PandaGene, Normal, Aggressive, Lazy, Worried, Playful, Weak, Brown
);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HorseVariant {
    marking: HorseMarking,
    color: HorseColor,
}

impl HorseVariant {
    pub const fn new(marking: HorseMarking, color: HorseColor) -> Self {
        Self { marking, color }
    }

    pub fn from_protocol_id(protocol_id: i32) -> Option<Self> {
        Some(Self::new(
            HorseMarking::from_protocol_id(protocol_id >> 8)?,
            HorseColor::from_protocol_id(protocol_id & 0xff)?,
        ))
    }

    pub const fn protocol_id(self) -> i32 {
        (self.marking.protocol_id() << 8) + self.color.protocol_id()
    }

    pub const fn marking(self) -> HorseMarking {
        self.marking
    }

    pub const fn color(self) -> HorseColor {
        self.color
    }

    pub const fn with_marking(self, marking: HorseMarking) -> Self {
        Self { marking, ..self }
    }

    pub const fn with_color(self, color: HorseColor) -> Self {
        Self { color, ..self }
    }
}
use spinel_registry::HorseColor;
