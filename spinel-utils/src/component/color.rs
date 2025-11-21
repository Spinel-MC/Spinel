use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextColor {
    Named(NamedTextColor),
    Hex(String),
}

impl TextColor {
    pub fn from_named(color: NamedTextColor) -> Self {
        TextColor::Named(color)
    }

    pub fn from_hex(hex: String) -> Self {
        TextColor::Hex(hex)
    }

    pub fn to_legacy_code(&self) -> String {
        match self {
            TextColor::Named(named) => named.to_legacy_code(),
            TextColor::Hex(_) => String::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub fn as_str(&self) -> &str {
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

    pub fn to_legacy_code(&self) -> String {
        match self {
            NamedTextColor::Black => "§0".to_string(),
            NamedTextColor::DarkBlue => "§1".to_string(),
            NamedTextColor::DarkGreen => "§2".to_string(),
            NamedTextColor::DarkAqua => "§3".to_string(),
            NamedTextColor::DarkRed => "§4".to_string(),
            NamedTextColor::DarkPurple => "§5".to_string(),
            NamedTextColor::Gold => "§6".to_string(),
            NamedTextColor::Gray => "§7".to_string(),
            NamedTextColor::DarkGray => "§8".to_string(),
            NamedTextColor::Blue => "§9".to_string(),
            NamedTextColor::Green => "§a".to_string(),
            NamedTextColor::Aqua => "§b".to_string(),
            NamedTextColor::Red => "§c".to_string(),
            NamedTextColor::LightPurple => "§d".to_string(),
            NamedTextColor::Yellow => "§e".to_string(),
            NamedTextColor::White => "§f".to_string(),
        }
    }
}
