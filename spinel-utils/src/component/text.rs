use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::json;

use crate::component::{builder::ComponentBuilder, style::Style, variant::ComponentType};

#[derive(Clone, Debug, Default)]
pub struct TextComponent {
    pub component_type: ComponentType,
    pub style: Style,
    pub children: Vec<TextComponent>,
}

impl TextComponent {
    pub fn empty() -> TextComponent {
        Self {
            component_type: ComponentType::Empty,
            style: Style::empty(),
            children: Vec::new(),
        }
    }

    pub fn text(content: String) -> ComponentBuilder {
        ComponentBuilder {
            component_type: ComponentType::Text(content),
            style: Style::empty(),
            children: Vec::new(),
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
        if let ComponentType::Text(text) = &self.component_type {
            builder.push_str(text);
        }
        for child in &self.children {
            child.write_legacy_string(builder, &current_style);
        }
    }

    pub fn to_ansi_string(&self) -> String {
        let mut ansi = String::new();

        if let Some(color) = &self.style.color {
            let value = color.value.as_str();
            let ansi_code = match value {
                "black" => "\x1b[30m".to_string(),
                "dark_blue" => "\x1b[34m".to_string(),
                "dark_green" => "\x1b[32m".to_string(),
                "dark_aqua" => "\x1b[36m".to_string(),
                "dark_red" => "\x1b[31m".to_string(),
                "dark_purple" => "\x1b[35m".to_string(),
                "gold" => "\x1b[33m".to_string(),
                "gray" => "\x1b[37m".to_string(),
                "dark_gray" => "\x1b[90m".to_string(),
                "blue" => "\x1b[94m".to_string(),
                "green" => "\x1b[92m".to_string(),
                "aqua" => "\x1b[96m".to_string(),
                "red" => "\x1b[91m".to_string(),
                "light_purple" => "\x1b[95m".to_string(),
                "yellow" => "\x1b[93m".to_string(),
                "white" => "\x1b[97m".to_string(),
                _ => String::new(),
            };
            ansi.push_str(&ansi_code);
        }

        if self.style.bold.unwrap_or(false) {
            ansi.push_str("\x1b[1m");
        }
        if self.style.italic.unwrap_or(false) {
            ansi.push_str("\x1b[3m");
        }
        if self.style.underlined.unwrap_or(false) {
            ansi.push_str("\x1b[4m");
        }
        if self.style.strikethrough.unwrap_or(false) {
            ansi.push_str("\x1b[9m");
        }

        let text_content = if let ComponentType::Text(text) = &self.component_type {
            text
        } else {
            "unimplemented component type"
        };
        ansi.push_str(text_content);

        ansi.push_str("\x1b[0m");

        ansi
    }
}

impl Serialize for TextComponent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = BTreeMap::new();

        match &self.component_type {
            ComponentType::Empty => (),
            ComponentType::Text(text) => {
                map.insert("text".to_string(), json!(text));
            }
            ComponentType::Translatable { key, args } => {
                map.insert("translate".to_string(), json!(key));
                if !args.is_empty() {
                    map.insert("with".to_string(), json!(args));
                }
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
            ComponentType::Keybind(key) => {
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

        if !self.children.is_empty() {
            map.insert("extra".to_string(), json!(self.children));
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

        let component_type = if let Some(text) = intermediate.text {
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
            component_type,
            style: intermediate.style,
            children: intermediate.extra.unwrap_or_default(),
        })
    }
}
