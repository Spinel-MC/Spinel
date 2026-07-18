use crate::component::text::TextComponent;
use serde::de::Error;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use spinel_nbt::{Nbt, NbtCompound, json_to_nbt, nbt_to_json};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub enum ClickEvent {
    OpenUrl(String),
    OpenFile(String),
    RunCommand(String),
    SuggestCommand(String),
    ChangePage(i32),
    CopyToClipboard(String),
    ShowDialog(Nbt),
    Custom { id: String, payload: Nbt },
}

#[derive(Clone, Debug, PartialEq)]
pub enum HoverEvent {
    ShowText(Box<TextComponent>),
    ShowItem(HoverItem),
    ShowEntity(HoverEntity),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HoverItem {
    pub id: String,
    pub count: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HoverEntity {
    pub id: String,
    pub uuid: Uuid,
    pub name: Option<Box<TextComponent>>,
}

#[derive(Deserialize)]
struct RawClickEvent {
    action: String,
    #[serde(flatten)]
    payload: BTreeMap<String, Value>,
}

#[derive(Deserialize)]
struct RawHoverEvent {
    action: String,
    #[serde(flatten)]
    payload: BTreeMap<String, Value>,
}

impl Serialize for ClickEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut click_event = serializer.serialize_map(Some(3))?;
        match self {
            Self::OpenUrl(url) => {
                click_event.serialize_entry("action", "open_url")?;
                click_event.serialize_entry("url", url)?;
            }
            Self::OpenFile(path) => {
                click_event.serialize_entry("action", "open_file")?;
                click_event.serialize_entry("path", path)?;
            }
            Self::RunCommand(command) => {
                click_event.serialize_entry("action", "run_command")?;
                click_event.serialize_entry("command", command)?;
            }
            Self::SuggestCommand(command) => {
                click_event.serialize_entry("action", "suggest_command")?;
                click_event.serialize_entry("command", command)?;
            }
            Self::ChangePage(page) => {
                click_event.serialize_entry("action", "change_page")?;
                click_event.serialize_entry("page", page)?;
            }
            Self::CopyToClipboard(value) => {
                click_event.serialize_entry("action", "copy_to_clipboard")?;
                click_event.serialize_entry("value", value)?;
            }
            Self::ShowDialog(dialog) => {
                click_event.serialize_entry("action", "show_dialog")?;
                click_event.serialize_entry("dialog", &nbt_to_json(dialog.clone()))?;
            }
            Self::Custom { id, payload } => {
                click_event.serialize_entry("action", "custom")?;
                click_event.serialize_entry("id", id)?;
                click_event.serialize_entry("payload", &nbt_to_json(payload.clone()))?;
            }
        }
        click_event.end()
    }
}

impl<'de> Deserialize<'de> for ClickEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut raw_click_event = RawClickEvent::deserialize(deserializer)?;
        match raw_click_event.action.as_str() {
            "open_url" => {
                decode_string_payload(&mut raw_click_event.payload, "url").map(ClickEvent::OpenUrl)
            }
            "open_file" => decode_string_payload(&mut raw_click_event.payload, "path")
                .map(ClickEvent::OpenFile),
            "run_command" => decode_string_payload(&mut raw_click_event.payload, "command")
                .map(ClickEvent::RunCommand),
            "suggest_command" => decode_string_payload(&mut raw_click_event.payload, "command")
                .map(ClickEvent::SuggestCommand),
            "change_page" => {
                decode_page_payload(&mut raw_click_event.payload).map(ClickEvent::ChangePage)
            }
            "copy_to_clipboard" => decode_string_payload(&mut raw_click_event.payload, "value")
                .map(ClickEvent::CopyToClipboard),
            "show_dialog" => decode_nbt_payload(&mut raw_click_event.payload, "dialog")
                .map(ClickEvent::ShowDialog),
            "custom" => decode_custom_click_payload(&mut raw_click_event.payload),
            action => Err(D::Error::custom(format!(
                "unknown click event action: {action}"
            ))),
        }
    }
}

impl Serialize for HoverEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut hover_event = serializer.serialize_map(Some(4))?;
        match self {
            Self::ShowText(component) => {
                hover_event.serialize_entry("action", "show_text")?;
                hover_event.serialize_entry("value", component)?;
            }
            Self::ShowItem(item) => {
                hover_event.serialize_entry("action", "show_item")?;
                hover_event.serialize_entry("id", &item.id)?;
                if item.count != 1 {
                    hover_event.serialize_entry("count", &item.count)?;
                }
            }
            Self::ShowEntity(entity) => {
                hover_event.serialize_entry("action", "show_entity")?;
                hover_event.serialize_entry("id", &entity.id)?;
                hover_event.serialize_entry("uuid", &entity.uuid)?;
                if let Some(name) = &entity.name {
                    hover_event.serialize_entry("name", name)?;
                }
            }
        }
        hover_event.end()
    }
}

impl<'de> Deserialize<'de> for HoverEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut raw_hover_event = RawHoverEvent::deserialize(deserializer)?;
        match raw_hover_event.action.as_str() {
            "show_text" => decode_component_payload(&mut raw_hover_event.payload, "value")
                .map(|component| HoverEvent::ShowText(Box::new(component))),
            "show_item" => {
                decode_hover_item_payload(&mut raw_hover_event.payload).map(HoverEvent::ShowItem)
            }
            "show_entity" => decode_hover_entity_payload(&mut raw_hover_event.payload)
                .map(HoverEvent::ShowEntity),
            action => Err(D::Error::custom(format!(
                "unknown hover event action: {action}"
            ))),
        }
    }
}

fn decode_string_payload<E>(payload: &mut BTreeMap<String, Value>, field: &str) -> Result<String, E>
where
    E: Error,
{
    let Some(payload_value) = payload.remove(field).or_else(|| payload.remove("value")) else {
        return Err(E::custom(format!("missing event payload: {field}")));
    };
    match payload_value {
        Value::String(text) => Ok(text),
        other => Err(E::custom(format!(
            "event payload {field} must be a string, got {other}"
        ))),
    }
}

fn decode_page_payload<E>(payload: &mut BTreeMap<String, Value>) -> Result<i32, E>
where
    E: Error,
{
    let Some(payload_value) = payload.remove("page").or_else(|| payload.remove("value")) else {
        return Err(E::custom("missing click event payload: page"));
    };
    match payload_value {
        Value::Number(page) => page
            .as_i64()
            .and_then(|page| i32::try_from(page).ok())
            .ok_or_else(|| E::custom("click event page must be a valid int")),
        Value::String(page) => page
            .parse::<i32>()
            .map_err(|_| E::custom("click event page must be a valid int")),
        other => Err(E::custom(format!(
            "click event payload page must be an int, got {other}"
        ))),
    }
}

fn decode_nbt_payload<E>(payload: &mut BTreeMap<String, Value>, field: &str) -> Result<Nbt, E>
where
    E: Error,
{
    let Some(payload_value) = payload.remove(field) else {
        return Err(E::custom(format!("missing click event payload: {field}")));
    };
    json_to_nbt(payload_value).ok_or_else(|| E::custom(format!("invalid nbt payload: {field}")))
}

fn decode_custom_click_payload<E>(payload: &mut BTreeMap<String, Value>) -> Result<ClickEvent, E>
where
    E: Error,
{
    let id = decode_string_payload(payload, "id")?;
    let payload = match payload.remove("payload") {
        Some(payload_value) => json_to_nbt(payload_value)
            .ok_or_else(|| E::custom("invalid custom click event payload"))?,
        None => Nbt::Compound(NbtCompound::new()),
    };
    Ok(ClickEvent::Custom { id, payload })
}

fn decode_component_payload<E>(
    payload: &mut BTreeMap<String, Value>,
    field: &str,
) -> Result<TextComponent, E>
where
    E: Error,
{
    let Some(payload_value) = payload.remove(field).or_else(|| payload.remove("contents")) else {
        return Err(E::custom(format!("missing hover event payload: {field}")));
    };
    serde_json::from_value(payload_value).map_err(E::custom)
}

fn decode_hover_item_payload<E>(payload: &mut BTreeMap<String, Value>) -> Result<HoverItem, E>
where
    E: Error,
{
    let mut payload = take_nested_contents(payload).unwrap_or_else(|| payload.clone());
    let id = decode_string_payload(&mut payload, "id")?;
    let count = match payload.remove("count") {
        Some(Value::Number(count)) => count
            .as_i64()
            .and_then(|count| i32::try_from(count).ok())
            .ok_or_else(|| E::custom("hover item count must be a valid int"))?,
        Some(other) => {
            return Err(E::custom(format!(
                "hover item count must be an int, got {other}"
            )));
        }
        None => 1,
    };
    Ok(HoverItem { id, count })
}

fn decode_hover_entity_payload<E>(payload: &mut BTreeMap<String, Value>) -> Result<HoverEntity, E>
where
    E: Error,
{
    let mut payload = take_nested_contents(payload).unwrap_or_else(|| payload.clone());
    let id = if payload.contains_key("uuid") {
        decode_string_payload(&mut payload, "id")?
    } else {
        decode_string_payload(&mut payload, "type")?
    };
    let uuid = if payload.contains_key("uuid") {
        decode_string_payload(&mut payload, "uuid")?
    } else {
        decode_string_payload(&mut payload, "id")?
    };
    let uuid = Uuid::parse_str(&uuid).map_err(E::custom)?;
    let name = payload
        .remove("name")
        .map(serde_json::from_value)
        .transpose()
        .map_err(E::custom)?
        .map(Box::new);
    Ok(HoverEntity { id, uuid, name })
}

fn take_nested_contents(payload: &mut BTreeMap<String, Value>) -> Option<BTreeMap<String, Value>> {
    match payload.remove("contents") {
        Some(Value::Object(contents)) => Some(contents.into_iter().collect()),
        _ => None,
    }
}
