use crate::component::text::TextComponent;
use crate::component::variant::{ComponentType, NbtSource};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

impl<'de> Deserialize<'de> for TextComponent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        component_from_value(value).map_err(serde::de::Error::custom)
    }
}

fn component_from_value(value: Value) -> Result<TextComponent, String> {
    let Value::Object(mut object) = value else {
        return Ok(TextComponent::literal(value_to_text(value)));
    };
    let content = parse_content(&mut object)?;
    let extra = take_components(&mut object, "extra")?.unwrap_or_default();
    let style = serde_json::from_value(Value::Object(object)).map_err(|error| error.to_string())?;
    Ok(TextComponent {
        content,
        style,
        extra,
    })
}

fn parse_content(object: &mut serde_json::Map<String, Value>) -> Result<ComponentType, String> {
    if let Some(text) = object.remove("text") {
        return Ok(ComponentType::Text(value_to_text(text)));
    }
    if let Some(key) = object.remove("translate").and_then(value_to_string) {
        let args = take_components(object, "with")?.unwrap_or_default();
        return Ok(ComponentType::Translatable {
            key,
            fallback: object.remove("fallback").and_then(value_to_string),
            args,
        });
    }
    parse_non_text_content(object)
}

fn parse_non_text_content(
    object: &mut serde_json::Map<String, Value>,
) -> Result<ComponentType, String> {
    if let Some(score) = object.remove("score") {
        return parse_score(score);
    }
    if let Some(selector) = object.remove("selector").and_then(value_to_string) {
        return Ok(ComponentType::Selector {
            selector,
            separator: take_component(object, "separator")?,
        });
    }
    if let Some(key) = object.remove("keybind").and_then(value_to_string) {
        return Ok(ComponentType::Keybind(key));
    }
    if let Some(nbt_path) = object.remove("nbt").and_then(value_to_string) {
        return parse_nbt(object, nbt_path);
    }
    Ok(ComponentType::Empty)
}

fn parse_score(value: Value) -> Result<ComponentType, String> {
    let Value::Object(mut score) = value else {
        return Err("score component must be an object".to_owned());
    };
    let name = score
        .remove("name")
        .and_then(value_to_string)
        .unwrap_or_default();
    let objective = score
        .remove("objective")
        .and_then(value_to_string)
        .unwrap_or_default();
    let value = score.remove("value").and_then(value_to_string);
    Ok(ComponentType::Score {
        name,
        objective,
        value,
    })
}

fn parse_nbt(
    object: &mut serde_json::Map<String, Value>,
    nbt_path: String,
) -> Result<ComponentType, String> {
    let source = if let Some(block) = object.remove("block").and_then(value_to_string) {
        NbtSource::Block(block)
    } else if let Some(entity) = object.remove("entity").and_then(value_to_string) {
        NbtSource::Entity(entity)
    } else if let Some(storage) = object.remove("storage").and_then(value_to_string) {
        NbtSource::Storage(storage)
    } else {
        return Err("nbt component is missing source".to_owned());
    };
    let interpret = object
        .remove("interpret")
        .and_then(|value| value.as_bool())
        .unwrap_or(false);
    Ok(ComponentType::Nbt {
        nbt_path,
        source,
        interpret,
        separator: take_component(object, "separator")?,
    })
}

fn take_components(
    object: &mut serde_json::Map<String, Value>,
    name: &str,
) -> Result<Option<Vec<TextComponent>>, String> {
    let Some(value) = object.remove(name) else {
        return Ok(None);
    };
    let Value::Array(values) = value else {
        return Err(format!("{name} must be an array"));
    };
    values
        .into_iter()
        .map(component_from_value)
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}

fn take_component(
    object: &mut serde_json::Map<String, Value>,
    name: &str,
) -> Result<Option<Box<TextComponent>>, String> {
    object
        .remove(name)
        .map(component_from_value)
        .transpose()
        .map(|component| component.map(Box::new))
}

fn value_to_string(value: Value) -> Option<String> {
    match value {
        Value::String(value) => Some(value),
        _ => None,
    }
}

fn value_to_text(value: Value) -> String {
    match value {
        Value::String(value) => value,
        Value::Number(value) => value.to_string(),
        Value::Bool(value) => value.to_string(),
        Value::Null => String::new(),
        value => serde_json::to_string(&value).unwrap_or_default(),
    }
}
