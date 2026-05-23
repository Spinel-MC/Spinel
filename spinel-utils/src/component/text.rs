use crate::component::builder::ComponentBuilder;
use crate::component::color::TextColor;
use crate::component::events::{ClickEvent, HoverEvent};
use crate::component::style::Style;
use crate::component::variant::{ComponentType, NbtSource};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextComponent {
    pub content: ComponentType,
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

    pub fn literal(text: impl Into<String>) -> Self {
        ComponentBuilder::new(text).build()
    }

    pub fn literal_with_color(text: impl Into<String>, color: TextColor) -> Self {
        ComponentBuilder::new(text).color(color).build()
    }

    pub fn text(content: impl Into<String>) -> ComponentBuilder {
        ComponentBuilder::new(content)
    }

    pub fn translatable(key: impl Into<String>) -> ComponentBuilder {
        ComponentBuilder::from_content(ComponentType::Translatable {
            key: key.into(),
            fallback: None,
            args: Vec::new(),
        })
    }

    pub fn keybind(key: impl Into<String>) -> ComponentBuilder {
        ComponentBuilder::from_content(ComponentType::Keybind(key.into()))
    }

    pub fn selector(selector: impl Into<String>) -> ComponentBuilder {
        ComponentBuilder::from_content(ComponentType::Selector {
            selector: selector.into(),
            separator: None,
        })
    }

    pub fn score(name: impl Into<String>, objective: impl Into<String>) -> ComponentBuilder {
        ComponentBuilder::from_content(ComponentType::Score {
            name: name.into(),
            objective: objective.into(),
            value: None,
        })
    }

    pub fn nbt(nbt_path: impl Into<String>, source: NbtSource) -> ComponentBuilder {
        ComponentBuilder::from_content(ComponentType::Nbt {
            nbt_path: nbt_path.into(),
            source,
            interpret: false,
            separator: None,
        })
    }

    pub fn click_event(&self) -> Option<&ClickEvent> {
        self.style.click_event.as_ref()
    }

    pub fn hover_event(&self) -> Option<&HoverEvent> {
        self.style.hover_event.as_ref()
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{\"text\":\"\"}".to_owned())
    }

    pub fn to_plain_string(&self) -> String {
        let mut builder = String::new();
        self.write_plain_string(&mut builder);
        builder
    }

    pub fn to_legacy_string(&self) -> String {
        let mut builder = String::new();
        self.write_legacy_string(&mut builder, &Style::empty());
        builder
    }

    pub fn to_ansi_string(&self) -> String {
        self.to_plain_string()
    }

    pub fn to_nbt_compound(&self) -> spinel_nbt::NbtCompound {
        let value = serde_json::to_value(self).unwrap_or_default();
        let object = value.as_object().cloned().unwrap_or_default();
        spinel_nbt::json_to_nbt_compound(object)
    }

    fn write_plain_string(&self, builder: &mut String) {
        if let ComponentType::Text(text) = &self.content {
            builder.push_str(text);
        }
        for child in &self.extra {
            child.write_plain_string(builder);
        }
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
}

impl From<&str> for TextComponent {
    fn from(value: &str) -> Self {
        Self::literal(value)
    }
}

impl From<String> for TextComponent {
    fn from(value: String) -> Self {
        Self::literal(value)
    }
}
