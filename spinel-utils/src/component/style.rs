use serde::{Deserialize, Serialize};

use crate::component::{color::TextColor, text::TextComponent};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Style {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<TextColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlined: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscated: Option<bool>,
    //TODO: click events, and hover events.
}

impl Style {
    pub const fn empty() -> Self {
        Style {
            color: None,
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
        }
    }

    pub const fn const_with_color(color: TextColor) -> Self {
        Style {
            color: Some(color),
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
        }
    }

    pub fn to_legacy_codes(&self) -> String {
        let mut s = String::new();
        if let Some(color) = &self.color {
            s.push_str(&color.to_legacy_code());
        }
        if self.bold.unwrap_or(false) {
            s.push_str("§l");
        }
        if self.italic.unwrap_or(false) {
            s.push_str("§o");
        }
        if self.underlined.unwrap_or(false) {
            s.push_str("§n");
        }
        if self.strikethrough.unwrap_or(false) {
            s.push_str("§m");
        }
        if self.obfuscated.unwrap_or(false) {
            s.push_str("§k");
        }
        s
    }

    pub fn merge_with_parent(&self, child: &TextComponent) -> Style {
        Style {
            color: child.style.color.clone().or_else(|| self.color.clone()),
            bold: child.style.bold.or(self.bold),
            italic: child.style.italic.or(self.italic),
            underlined: child.style.underlined.or(self.underlined),
            strikethrough: child.style.strikethrough.or(self.strikethrough),
            obfuscated: child.style.obfuscated.or(self.obfuscated),
        }
    }

    pub fn to_nbt(&self) -> spinel_nbt::NbtCompound {
        let mut compound = spinel_nbt::NbtCompound::new();
        if let Some(color) = &self.color {
            match color {
                TextColor::Named(named) => compound.insert(
                    "color".to_string(),
                    spinel_nbt::Nbt::String(named.as_str().to_string()),
                ),
                TextColor::Hex(hex) => compound.insert(
                    "color".to_string(),
                    spinel_nbt::Nbt::String(hex.to_string()),
                ),
            }
        }
        if let Some(bold) = self.bold {
            compound.insert(
                "bold".to_string(),
                spinel_nbt::Nbt::Byte(if bold { 1 } else { 0 }),
            );
        }
        if let Some(italic) = self.italic {
            compound.insert(
                "italic".to_string(),
                spinel_nbt::Nbt::Byte(if italic { 1 } else { 0 }),
            );
        }
        if let Some(underlined) = self.underlined {
            compound.insert(
                "underlined".to_string(),
                spinel_nbt::Nbt::Byte(if underlined { 1 } else { 0 }),
            );
        }
        if let Some(strikethrough) = self.strikethrough {
            compound.insert(
                "strikethrough".to_string(),
                spinel_nbt::Nbt::Byte(if strikethrough { 1 } else { 0 }),
            );
        }
        if let Some(obfuscated) = self.obfuscated {
            compound.insert(
                "obfuscated".to_string(),
                spinel_nbt::Nbt::Byte(if obfuscated { 1 } else { 0 }),
            );
        }
        compound
    }
}
