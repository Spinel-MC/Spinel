use crate::component::text::TextComponent;
use crate::component::variant::{ComponentType, NbtSource};
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use serde_json::{Value, json};

impl Serialize for TextComponent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut fields = component_fields(self);
        merge_style(&mut fields, &self.style);
        if !self.extra.is_empty() {
            fields.insert("extra".to_owned(), json!(self.extra));
        }
        let mut map = serializer.serialize_map(Some(fields.len()))?;
        for (key, value) in fields {
            map.serialize_entry(&key, &value)?;
        }
        map.end()
    }
}

fn component_fields(component: &TextComponent) -> serde_json::Map<String, Value> {
    let mut fields = serde_json::Map::new();
    match &component.content {
        ComponentType::Empty => insert_text(&mut fields, ""),
        ComponentType::Text(text) => insert_text(&mut fields, text),
        ComponentType::Translatable {
            key,
            fallback,
            args,
        } => {
            fields.insert("translate".to_owned(), json!(key));
            insert_optional(&mut fields, "fallback", fallback);
            if !args.is_empty() {
                fields.insert("with".to_owned(), json!(args));
            }
        }
        ComponentType::Score {
            name,
            objective,
            value,
        } => {
            fields.insert(
                "score".to_owned(),
                json!({ "name": name, "objective": objective }),
            );
            insert_optional(&mut fields, "value", value);
        }
        ComponentType::Selector {
            selector,
            separator,
        } => {
            fields.insert("selector".to_owned(), json!(selector));
            insert_optional_component(&mut fields, "separator", separator.as_deref());
        }
        ComponentType::Keybind(key) => {
            fields.insert("keybind".to_owned(), json!(key));
        }
        ComponentType::Nbt {
            nbt_path,
            source,
            interpret,
            separator,
        } => {
            fields.insert("nbt".to_owned(), json!(nbt_path));
            insert_nbt_source(&mut fields, source);
            if *interpret {
                fields.insert("interpret".to_owned(), json!(true));
            }
            insert_optional_component(&mut fields, "separator", separator.as_deref());
        }
    }
    fields
}

fn merge_style(
    fields: &mut serde_json::Map<String, Value>,
    style: &crate::component::style::Style,
) {
    let Ok(Value::Object(style_fields)) = serde_json::to_value(style) else {
        return;
    };
    for (key, value) in style_fields {
        fields.insert(key, value);
    }
}

fn insert_text(fields: &mut serde_json::Map<String, Value>, text: &str) {
    fields.insert("text".to_owned(), json!(text));
}

fn insert_optional(
    fields: &mut serde_json::Map<String, Value>,
    name: &str,
    value: &Option<String>,
) {
    if let Some(value) = value {
        fields.insert(name.to_owned(), json!(value));
    }
}

fn insert_optional_component(
    fields: &mut serde_json::Map<String, Value>,
    name: &str,
    component: Option<&TextComponent>,
) {
    if let Some(component) = component {
        fields.insert(name.to_owned(), json!(component));
    }
}

fn insert_nbt_source(fields: &mut serde_json::Map<String, Value>, source: &NbtSource) {
    match source {
        NbtSource::Block(block) => fields.insert("block".to_owned(), json!(block)),
        NbtSource::Entity(entity) => fields.insert("entity".to_owned(), json!(entity)),
        NbtSource::Storage(storage) => fields.insert("storage".to_owned(), json!(storage)),
    };
}
