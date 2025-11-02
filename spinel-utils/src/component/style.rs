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

        pub fn empty() -> Style {
                Style { color: None, bold: None, italic: None, underlined: None, strikethrough: None, obfuscated: None }        
        }

        pub fn to_legacy_codes(&self) -> String {
                let mut s = String::new();
                if let Some(color) = &self.color {
                s.push_str(&color.to_legacy_code());
                }
                if self.bold.unwrap_or(false) { s.push_str("§l"); }
                if self.italic.unwrap_or(false) { s.push_str("§o"); }
                if self.underlined.unwrap_or(false) { s.push_str("§n"); }
                if self.strikethrough.unwrap_or(false) { s.push_str("§m"); }
                if self.obfuscated.unwrap_or(false) { s.push_str("§k"); }
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
}