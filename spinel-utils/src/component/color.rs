use serde::{Deserialize, Deserializer};

#[derive(Clone, Debug)]
pub struct TextColor {
    pub value: String,
}

impl serde::Serialize for TextColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.value)
    }
}

impl<'de> Deserialize<'de> for TextColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(TextColor { value: s })
    }
}

impl TextColor {
    pub fn from_named(value: NamedTextColor) -> Self {
        Self {
            value: String::from(value.as_str()),
        }
    }

    pub fn from_int(value: u32) -> Self {
        Self {
            value: format!("#{:06X}", value),
        }
    }

    pub fn from_hex(value: String) -> Self {
        if u32::from_str_radix(&value[1..], 16).is_ok() == true {
            return Self { value: value };
        }

        Self {
            value: String::from("#000000"),
        }
    }

    pub fn to_string(self) -> String {
        self.value
    }

    pub fn to_legacy_code(&self) -> String {
        match self.value.as_str() {
            "black" => "§0".to_string(),
            "dark_blue" => "§1".to_string(),
            "dark_green" => "§2".to_string(),
            "dark_aqua" => "§3".to_string(),
            "dark_red" => "§4".to_string(),
            "dark_purple" => "§5".to_string(),
            "gold" => "§6".to_string(),
            "gray" => "§7".to_string(),
            "dark_gray" => "§8".to_string(),
            "blue" => "§9".to_string(),
            "green" => "§a".to_string(),
            "aqua" => "§b".to_string(),
            "red" => "§c".to_string(),
            "light_purple" => "§d".to_string(),
            "yellow" => "§e".to_string(),
            "white" => "§f".to_string(),
            hex if hex.starts_with('#') => {
                format!(
                    "§x{}",
                    hex[1..]
                        .chars()
                        .map(|c| format!("§{}", c))
                        .collect::<String>()
                )
            }
            _ => "".to_string(),
        }
    }
}

pub enum NamedTextColor {
    Aqua,
    Black,
    Blue,
    DarkAqua,
    DarkBlue,
    DarkGray,
    DarkGreen,
    DarkPurple,
    DarkRed,
    Gold,
    Gray,
    Green,
    LightPurple,
    Red,
    White,
    Yellow,
}

impl NamedTextColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            NamedTextColor::Aqua => "aqua",
            NamedTextColor::Black => "black",
            NamedTextColor::Blue => "blue",
            NamedTextColor::DarkAqua => "dark_aqua",
            NamedTextColor::DarkBlue => "dark_blue",
            NamedTextColor::DarkGray => "dark_gray",
            NamedTextColor::DarkGreen => "dark_green",
            NamedTextColor::DarkPurple => "dark_purple",
            NamedTextColor::DarkRed => "dark_red",
            NamedTextColor::Gold => "gold",
            NamedTextColor::Gray => "gray",
            NamedTextColor::Green => "green",
            NamedTextColor::LightPurple => "light_purple",
            NamedTextColor::Red => "red",
            NamedTextColor::White => "white",
            NamedTextColor::Yellow => "yellow",
        }
    }
}
