use crate::DataComponentValue;
use spinel_nbt::Nbt;

macro_rules! named_ordinal_component {
    ($name:ident, $default:ident, $default_name:literal, $(($variant:ident, $component_name:literal)),+ $(,)?) => {
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

            pub const fn component_name(self) -> &'static str {
                match self {
                    Self::$default => $default_name,
                    $(Self::$variant => $component_name),+
                }
            }
        }

        impl DataComponentValue for $name {
            fn to_component_nbt(&self) -> Nbt {
                Nbt::String(self.component_name().to_owned())
            }

            fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
                let Nbt::String(component_name) = component_nbt else {
                    return None;
                };
                match component_name.strip_prefix("minecraft:").unwrap_or(component_name) {
                    $default_name => Some(Self::$default),
                    $($component_name => Some(Self::$variant),)+
                    _ => None,
                }
            }
        }
    };
}

named_ordinal_component!(
    AxolotlVariant,
    Lucy,
    "lucy",
    (Wild, "wild"),
    (Gold, "gold"),
    (Cyan, "cyan"),
    (Blue, "blue")
);
named_ordinal_component!(FoxVariant, Red, "red", (Snow, "snow"));
named_ordinal_component!(
    HorseColor,
    White,
    "white",
    (Creamy, "creamy"),
    (Chestnut, "chestnut"),
    (Brown, "brown"),
    (Black, "black"),
    (Gray, "gray"),
    (DarkBrown, "dark_brown")
);
named_ordinal_component!(
    LlamaVariant,
    Creamy,
    "creamy",
    (White, "white"),
    (Brown, "brown"),
    (Gray, "gray")
);
named_ordinal_component!(MooshroomVariant, Red, "red", (Brown, "brown"));
named_ordinal_component!(
    ParrotColor,
    RedBlue,
    "red_blue",
    (Blue, "blue"),
    (Green, "green"),
    (YellowBlue, "yellow_blue"),
    (Grey, "grey")
);
named_ordinal_component!(
    SalmonSize,
    Small,
    "small",
    (Medium, "medium"),
    (Large, "large")
);

impl SalmonSize {
    pub const fn id(self) -> &'static str {
        self.component_name()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RabbitVariant {
    #[default]
    Brown,
    White,
    Black,
    BlackAndWhite,
    Gold,
    SaltAndPepper,
    KillerBunny,
}

impl RabbitVariant {
    pub const fn protocol_id(self) -> i32 {
        match self {
            Self::Brown => 0,
            Self::White => 1,
            Self::Black => 2,
            Self::BlackAndWhite => 3,
            Self::Gold => 4,
            Self::SaltAndPepper => 5,
            Self::KillerBunny => 99,
        }
    }

    pub const fn from_protocol_id(protocol_id: i32) -> Option<Self> {
        match protocol_id {
            0 => Some(Self::Brown),
            1 => Some(Self::White),
            2 => Some(Self::Black),
            3 => Some(Self::BlackAndWhite),
            4 => Some(Self::Gold),
            5 => Some(Self::SaltAndPepper),
            99 => Some(Self::KillerBunny),
            _ => None,
        }
    }

    pub const fn component_name(self) -> &'static str {
        match self {
            Self::Brown => "brown",
            Self::White => "white",
            Self::Black => "black",
            Self::BlackAndWhite => "black_and_white",
            Self::Gold => "gold",
            Self::SaltAndPepper => "salt_and_pepper",
            Self::KillerBunny => "killer_bunny",
        }
    }
}

impl DataComponentValue for RabbitVariant {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::String(self.component_name().to_owned())
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let Nbt::String(component_name) = component_nbt else {
            return None;
        };
        match component_name
            .strip_prefix("minecraft:")
            .unwrap_or(component_name)
        {
            "brown" => Some(Self::Brown),
            "white" => Some(Self::White),
            "black" => Some(Self::Black),
            "black_and_white" => Some(Self::BlackAndWhite),
            "gold" => Some(Self::Gold),
            "salt_and_pepper" => Some(Self::SaltAndPepper),
            "killer_bunny" => Some(Self::KillerBunny),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TropicalFishPattern {
    #[default]
    Kob,
    Sunstreak,
    Snooper,
    Dasher,
    Brinely,
    Spotty,
    Flopper,
    Stripey,
    Glitter,
    Blockfish,
    Betty,
    Clayfish,
}

impl TropicalFishPattern {
    pub const fn id(self) -> i32 {
        match self {
            Self::Kob => 0,
            Self::Sunstreak => 1 << 8,
            Self::Snooper => 2 << 8,
            Self::Dasher => 3 << 8,
            Self::Brinely => 4 << 8,
            Self::Spotty => 5 << 8,
            Self::Flopper => 1,
            Self::Stripey => (1 << 8) | 1,
            Self::Glitter => (2 << 8) | 1,
            Self::Blockfish => (3 << 8) | 1,
            Self::Betty => (4 << 8) | 1,
            Self::Clayfish => (5 << 8) | 1,
        }
    }

    pub const fn from_id(id: i32) -> Option<Self> {
        match id {
            0 => Some(Self::Kob),
            256 => Some(Self::Sunstreak),
            512 => Some(Self::Snooper),
            768 => Some(Self::Dasher),
            1024 => Some(Self::Brinely),
            1280 => Some(Self::Spotty),
            1 => Some(Self::Flopper),
            257 => Some(Self::Stripey),
            513 => Some(Self::Glitter),
            769 => Some(Self::Blockfish),
            1025 => Some(Self::Betty),
            1281 => Some(Self::Clayfish),
            _ => None,
        }
    }

    pub const fn component_name(self) -> &'static str {
        match self {
            Self::Kob => "kob",
            Self::Sunstreak => "sunstreak",
            Self::Snooper => "snooper",
            Self::Dasher => "dasher",
            Self::Brinely => "brinely",
            Self::Spotty => "spotty",
            Self::Flopper => "flopper",
            Self::Stripey => "stripey",
            Self::Glitter => "glitter",
            Self::Blockfish => "blockfish",
            Self::Betty => "betty",
            Self::Clayfish => "clayfish",
        }
    }
}

impl DataComponentValue for TropicalFishPattern {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::String(self.component_name().to_owned())
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let Nbt::String(component_name) = component_nbt else {
            return None;
        };
        match component_name
            .strip_prefix("minecraft:")
            .unwrap_or(component_name)
        {
            "kob" => Some(Self::Kob),
            "sunstreak" => Some(Self::Sunstreak),
            "snooper" => Some(Self::Snooper),
            "dasher" => Some(Self::Dasher),
            "brinely" => Some(Self::Brinely),
            "spotty" => Some(Self::Spotty),
            "flopper" => Some(Self::Flopper),
            "stripey" => Some(Self::Stripey),
            "glitter" => Some(Self::Glitter),
            "blockfish" => Some(Self::Blockfish),
            "betty" => Some(Self::Betty),
            "clayfish" => Some(Self::Clayfish),
            _ => None,
        }
    }
}
