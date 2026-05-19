use crate::component::color::TextColor;
use crate::component::events::{ClickEvent, HoverEvent};
use crate::component::nbt::json_to_nbt_compound;
use crate::component::text::TextComponent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
    #[serde(rename = "clickEvent", skip_serializing_if = "Option::is_none")]
    pub click_event: Option<ClickEvent>,
    #[serde(rename = "hoverEvent", skip_serializing_if = "Option::is_none")]
    pub hover_event: Option<HoverEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
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
            click_event: None,
            hover_event: None,
            insertion: None,
            font: None,
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
            click_event: None,
            hover_event: None,
            insertion: None,
            font: None,
        }
    }

    pub fn to_legacy_codes(&self) -> String {
        let mut codes = String::new();
        if let Some(color) = &self.color {
            codes.push_str(&color.to_legacy_code());
        }
        add_code(&mut codes, self.bold, 'l');
        add_code(&mut codes, self.italic, 'o');
        add_code(&mut codes, self.underlined, 'n');
        add_code(&mut codes, self.strikethrough, 'm');
        add_code(&mut codes, self.obfuscated, 'k');
        codes
    }

    pub fn merge_with_parent(&self, child: &TextComponent) -> Style {
        Style {
            color: child.style.color.clone().or_else(|| self.color.clone()),
            bold: child.style.bold.or(self.bold),
            italic: child.style.italic.or(self.italic),
            underlined: child.style.underlined.or(self.underlined),
            strikethrough: child.style.strikethrough.or(self.strikethrough),
            obfuscated: child.style.obfuscated.or(self.obfuscated),
            click_event: child
                .click_event()
                .cloned()
                .or_else(|| self.click_event.clone()),
            hover_event: child
                .hover_event()
                .cloned()
                .or_else(|| self.hover_event.clone()),
            insertion: child
                .style
                .insertion
                .clone()
                .or_else(|| self.insertion.clone()),
            font: child.style.font.clone().or_else(|| self.font.clone()),
        }
    }

    pub fn to_nbt(&self) -> spinel_nbt::NbtCompound {
        let value = serde_json::to_value(self).unwrap_or_default();
        json_to_nbt_compound(value.as_object().cloned().unwrap_or_default())
    }
}

fn add_code(codes: &mut String, enabled: Option<bool>, code: char) {
    if enabled.unwrap_or(false) {
        codes.push('\u{00a7}');
        codes.push(code);
    }
}
