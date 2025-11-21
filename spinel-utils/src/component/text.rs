use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::json;

use crate::component::{builder::ComponentBuilder, style::Style, variant::ComponentType};

#[derive(Clone, Debug, Default)]
pub struct TextComponent {
    pub content: ComponentType<String>,
    pub style: Style,
    pub extra: Vec<TextComponent>,
}

impl TextComponent {
    pub fn empty() -> TextComponent {
        Self {
            content: ComponentType::Empty,
            style: Style::empty(),
            extra: Vec::new(),
        }
    }

    /// Create a translatable text component
    pub const fn translatable(key: &'static str) -> Self {
        TextComponent {
            content: ComponentType::StaticTranslatable(key),
            style: Style::empty(),
            extra: Vec::new(),
        }
    }

    /// Create a literal text component
    pub const fn literal(text: &'static str) -> Self {
        TextComponent {
            content: ComponentType::StaticText(text),
            style: Style::empty(),
            extra: Vec::new(),
        }
    }

    /// Create a literal text component with color
    pub const fn literal_with_color(
        text: &'static str,
        color: crate::component::color::TextColor,
    ) -> Self {
        TextComponent {
            content: ComponentType::StaticText(text),
            style: Style::const_with_color(color),
            extra: Vec::new(),
        }
    }

    /// Create a translatable component with color
    pub const fn translatable_with_color(
        key: &'static str,
        color: crate::component::color::TextColor,
    ) -> Self {
        TextComponent {
            content: ComponentType::StaticTranslatable(key),
            style: Style::const_with_color(color),
            extra: Vec::new(),
        }
    }

    pub fn text(content: String) -> ComponentBuilder {
        ComponentBuilder {
            content: ComponentType::Text(content),
            style: Style::empty(),
            extra: Vec::new(),
        }
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{\"text\":\"\"}".to_owned())
    }

    pub fn to_legacy_string(&self) -> String {
        let mut builder = String::new();
        self.write_legacy_string(&mut builder, &Style::empty());
        builder
    }

    pub fn to_plain_string(&self) -> String {
        let mut builder = String::new();
        self.write_legacy_string(&mut builder, &Style::empty());
        builder
    }

    fn write_legacy_string(&self, builder: &mut String, parent_style: &Style) {
        let current_style = parent_style.merge_with_parent(self);
        builder.push_str(&current_style.to_legacy_codes());
        if let ComponentType::Text(text) = &self.content {
            builder.push_str(text);
        }
        for child in &self.extra {
            child.write_legacy_string(builder, &current_style);
        }
    }

    pub fn to_ansi_string(&self) -> String {
        let mut ansi = String::new();

        if let Some(color) = &self.style.color {
            let ansi_code = match color {
                crate::component::color::TextColor::Named(named) => match named {
                    crate::component::color::NamedTextColor::Black => "\\x1b[30m",
                    crate::component::color::NamedTextColor::DarkBlue => "\\x1b[34m",
                    crate::component::color::NamedTextColor::DarkGreen => "\\x1b[32m",
                    crate::component::color::NamedTextColor::DarkAqua => "\\x1b[36m",
                    crate::component::color::NamedTextColor::DarkRed => "\\x1b[31m",
                    crate::component::color::NamedTextColor::DarkPurple => "\\x1b[35m",
                    crate::component::color::NamedTextColor::Gold => "\\x1b[33m",
                    crate::component::color::NamedTextColor::Gray => "\\x1b[37m",
                    crate::component::color::NamedTextColor::DarkGray => "\\x1b[90m",
                    crate::component::color::NamedTextColor::Blue => "\\x1b[94m",
                    crate::component::color::NamedTextColor::Green => "\\x1b[92m",
                    crate::component::color::NamedTextColor::Aqua => "\\x1b[96m",
                    crate::component::color::NamedTextColor::Red => "\\x1b[91m",
                    crate::component::color::NamedTextColor::LightPurple => "\\x1b[95m",
                    crate::component::color::NamedTextColor::Yellow => "\\x1b[93m",
                    crate::component::color::NamedTextColor::White => "\\x1b[97m",
                },
                crate::component::color::TextColor::Hex(_) => "",
            };
            ansi.push_str(ansi_code);
        }

        if self.style.bold.unwrap_or(false) {
            ansi.push_str("\\x1b[1m");
        }
        if self.style.italic.unwrap_or(false) {
            ansi.push_str("\\x1b[3m");
        }
        if self.style.underlined.unwrap_or(false) {
            ansi.push_str("\\x1b[4m");
        }
        if self.style.strikethrough.unwrap_or(false) {
            ansi.push_str("\\x1b[9m");
        }

        let text_content = match &self.content {
            ComponentType::Text(text) => text.as_str(),
            ComponentType::StaticText(text) => text,
            _ => "unimplemented component type",
        };
        ansi.push_str(text_content);

        ansi.push_str("\\x1b[0m");

        ansi
    }
}

impl Serialize for TextComponent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = BTreeMap::new();

        match &self.content {
            ComponentType::Empty => (),
            ComponentType::Text(text) => {
                map.insert("text".to_string(), json!(text));
            }
            ComponentType::StaticText(text) => {
                map.insert("text".to_string(), json!(text));
            }
            ComponentType::Translatable { key, args } => {
                map.insert("translate".to_string(), json!(key));
                if !args.is_empty() {
                    map.insert("with".to_string(), json!(args));
                }
            }
            ComponentType::StaticTranslatable(key) => {
                map.insert("translate".to_string(), json!(key));
            }
            ComponentType::Score { name, objective } => {
                map.insert(
                    "score".to_string(),
                    json!({ "name": name, "objective": objective }),
                );
            }
            ComponentType::Selector(selector) => {
                map.insert("selector".to_string(), json!(selector));
            }
            ComponentType::StaticSelector(selector) => {
                map.insert("selector".to_string(), json!(selector));
            }
            ComponentType::Keybind(key) => {
                map.insert("keybind".to_string(), json!(key));
            }
            ComponentType::StaticKeybind(key) => {
                map.insert("keybind".to_string(), json!(key));
            }
            ComponentType::Nbt { nbt_path, source } => {
                map.insert("nbt".to_string(), json!(nbt_path));
                map.insert("source".to_string(), json!(source));
            }
        }

        if let Ok(value) = serde_json::to_value(&self.style) {
            if let Some(style_map) = value.as_object() {
                for (k, v) in style_map {
                    map.insert(k.clone(), v.clone());
                }
            }
        }

        if !self.extra.is_empty() {
            map.insert("extra".to_string(), json!(self.extra));
        }

        if map.is_empty() {
            map.insert("text".to_string(), json!(""));
        }

        map.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TextComponent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Score {
            name: String,
            objective: String,
        }
        #[derive(Deserialize)]
        struct Intermediate {
            text: Option<String>,
            translate: Option<String>,
            with: Option<Vec<TextComponent>>,
            score: Option<Score>,
            selector: Option<String>,
            keybind: Option<String>,
            nbt: Option<String>,
            source: Option<String>,
            extra: Option<Vec<TextComponent>>,
            #[serde(flatten)]
            style: Style,
        }

        let intermediate = Intermediate::deserialize(deserializer)?;

        let content = if let Some(text) = intermediate.text {
            ComponentType::Text(text)
        } else if let Some(key) = intermediate.translate {
            ComponentType::Translatable {
                key,
                args: intermediate.with.unwrap_or_default(),
            }
        } else if let Some(score) = intermediate.score {
            ComponentType::Score {
                name: score.name,
                objective: score.objective,
            }
        } else if let Some(selector) = intermediate.selector {
            ComponentType::Selector(selector)
        } else if let Some(key) = intermediate.keybind {
            ComponentType::Keybind(key)
        } else if let (Some(nbt_path), Some(source)) = (intermediate.nbt, intermediate.source) {
            ComponentType::Nbt { nbt_path, source }
        } else {
            ComponentType::Empty
        };

        Ok(TextComponent {
            content,
            style: intermediate.style,
            extra: intermediate.extra.unwrap_or_default(),
        })
    }
}
