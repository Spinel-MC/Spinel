use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextColor {
    Named(NamedTextColor),
    Hex(String),
}

impl TextColor {
    pub fn from_named(color: NamedTextColor) -> Self {
        TextColor::Named(color)
    }

    pub fn from_hex(hex: impl Into<String>) -> Self {
        TextColor::Hex(hex.into())
    }

    pub fn to_legacy_code(&self) -> String {
        match self {
            TextColor::Named(named) => named.to_legacy_code(),
            TextColor::Hex(_) => String::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NamedTextColor {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
}

impl NamedTextColor {
    pub fn as_str(self) -> &'static str {
        match self {
            NamedTextColor::Black => "black",
            NamedTextColor::DarkBlue => "dark_blue",
            NamedTextColor::DarkGreen => "dark_green",
            NamedTextColor::DarkAqua => "dark_aqua",
            NamedTextColor::DarkRed => "dark_red",
            NamedTextColor::DarkPurple => "dark_purple",
            NamedTextColor::Gold => "gold",
            NamedTextColor::Gray => "gray",
            NamedTextColor::DarkGray => "dark_gray",
            NamedTextColor::Blue => "blue",
            NamedTextColor::Green => "green",
            NamedTextColor::Aqua => "aqua",
            NamedTextColor::Red => "red",
            NamedTextColor::LightPurple => "light_purple",
            NamedTextColor::Yellow => "yellow",
            NamedTextColor::White => "white",
        }
    }

    pub fn to_legacy_code(self) -> String {
        let code = match self {
            NamedTextColor::Black => '0',
            NamedTextColor::DarkBlue => '1',
            NamedTextColor::DarkGreen => '2',
            NamedTextColor::DarkAqua => '3',
            NamedTextColor::DarkRed => '4',
            NamedTextColor::DarkPurple => '5',
            NamedTextColor::Gold => '6',
            NamedTextColor::Gray => '7',
            NamedTextColor::DarkGray => '8',
            NamedTextColor::Blue => '9',
            NamedTextColor::Green => 'a',
            NamedTextColor::Aqua => 'b',
            NamedTextColor::Red => 'c',
            NamedTextColor::LightPurple => 'd',
            NamedTextColor::Yellow => 'e',
            NamedTextColor::White => 'f',
        };
        format!("\u{00a7}{code}")
    }
}
