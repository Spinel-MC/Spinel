use crate::component::color::{NamedTextColor, TextColor};
use crate::component::events::{ClickEvent, HoverEntity, HoverEvent, HoverItem};
use crate::component::text::TextComponent;
use serde_json::json;
use spinel_nbt::{Nbt, NbtCompound};
use uuid::Uuid;

#[test]
fn text_component_accepts_string_and_str() {
    let borrowed = TextComponent::text("borrowed").build();
    let owned = TextComponent::text("owned".to_owned()).build();

    assert_eq!(borrowed.to_plain_string(), "borrowed");
    assert_eq!(owned.to_plain_string(), "owned");
}

#[test]
fn component_serializes_rich_json() {
    let component = TextComponent::text("click")
        .color(TextColor::from_named(NamedTextColor::Aqua))
        .click_event(ClickEvent::RunCommand("/help".to_owned()))
        .hover_event(HoverEvent::ShowText(Box::new(TextComponent::from("hover"))))
        .insertion("inserted")
        .append(TextComponent::keybind("key.jump").build())
        .build();
    let value = serde_json::to_value(component).unwrap();

    assert_eq!(value["text"], json!("click"));
    assert_eq!(value["color"], json!("aqua"));
    assert_eq!(value["click_event"]["action"], json!("run_command"));
    assert_eq!(value["click_event"]["command"], json!("/help"));
    assert_eq!(value["hover_event"]["action"], json!("show_text"));
    assert_eq!(value["hover_event"]["value"]["text"], json!("hover"));
    assert_eq!(value["insertion"], json!("inserted"));
    assert_eq!(value["extra"][0]["keybind"], json!("key.jump"));
}

#[test]
fn component_serializes_minestom_click_event_payload_fields() {
    let mut custom_payload = NbtCompound::new();
    custom_payload.insert("confirmed".to_owned(), Nbt::Byte(1));
    let click_events = [
        (
            ClickEvent::OpenUrl("https://example.com".to_owned()),
            json!({"action": "open_url", "url": "https://example.com"}),
        ),
        (
            ClickEvent::OpenFile("C:/tmp/file.txt".to_owned()),
            json!({"action": "open_file", "path": "C:/tmp/file.txt"}),
        ),
        (
            ClickEvent::RunCommand("/help".to_owned()),
            json!({"action": "run_command", "command": "/help"}),
        ),
        (
            ClickEvent::SuggestCommand("/op".to_owned()),
            json!({"action": "suggest_command", "command": "/op"}),
        ),
        (
            ClickEvent::ChangePage(4),
            json!({"action": "change_page", "page": 4}),
        ),
        (
            ClickEvent::CopyToClipboard("copy".to_owned()),
            json!({"action": "copy_to_clipboard", "value": "copy"}),
        ),
        (
            ClickEvent::ShowDialog(Nbt::String("minecraft:quick_actions".to_owned())),
            json!({"action": "show_dialog", "dialog": "minecraft:quick_actions"}),
        ),
        (
            ClickEvent::Custom {
                id: "minecraft:test".to_owned(),
                payload: Nbt::Compound(custom_payload),
            },
            json!({"action": "custom", "id": "minecraft:test", "payload": {"confirmed": true}}),
        ),
    ];

    for (click_event, expected) in click_events {
        let component = TextComponent::text("click")
            .click_event(click_event)
            .build();
        let value = serde_json::to_value(component).unwrap();

        assert_eq!(value["click_event"], expected);
        assert!(value.get("clickEvent").is_none());
        assert!(value["click_event"].get("contents").is_none());
    }
}

#[test]
fn component_deserializes_modern_and_legacy_click_event_payloads() {
    let modern_component: TextComponent = serde_json::from_value(json!({
        "text": "click",
        "click_event": {
            "action": "suggest_command",
            "command": "/op"
        }
    }))
    .unwrap();
    let legacy_component: TextComponent = serde_json::from_value(json!({
        "text": "click",
        "clickEvent": {
            "action": "suggest_command",
            "value": "/op"
        }
    }))
    .unwrap();

    assert_eq!(
        modern_component.click_event(),
        Some(&ClickEvent::SuggestCommand("/op".to_owned()))
    );
    assert_eq!(
        legacy_component.click_event(),
        Some(&ClickEvent::SuggestCommand("/op".to_owned()))
    );
}

#[test]
fn component_serializes_minestom_hover_event_payload_fields() {
    let entity_uuid = Uuid::parse_str("5ca89bac-bca8-413b-a136-fd8fa88a6ab0").unwrap();
    let hover_events = [
        (
            HoverEvent::ShowText(Box::new(TextComponent::literal("hover"))),
            json!({"action": "show_text", "value": {"text": "hover"}}),
        ),
        (
            HoverEvent::ShowItem(HoverItem {
                id: "minecraft:stone".to_owned(),
                count: 3,
            }),
            json!({"action": "show_item", "id": "minecraft:stone", "count": 3}),
        ),
        (
            HoverEvent::ShowEntity(HoverEntity {
                id: "minecraft:zombie".to_owned(),
                uuid: entity_uuid,
                name: Some(Box::new(TextComponent::literal("Zombie"))),
            }),
            json!({
                "action": "show_entity",
                "id": "minecraft:zombie",
                "uuid": "5ca89bac-bca8-413b-a136-fd8fa88a6ab0",
                "name": {"text": "Zombie"}
            }),
        ),
    ];

    for (hover_event, expected) in hover_events {
        let component = TextComponent::text("hover")
            .hover_event(hover_event)
            .build();
        let value = serde_json::to_value(component).unwrap();

        assert_eq!(value["hover_event"], expected);
        assert!(value.get("hoverEvent").is_none());
        assert!(value["hover_event"].get("contents").is_none());
    }
}

#[test]
fn component_serializes_click_event_to_modern_nbt_shape() {
    let component = TextComponent::text("click")
        .click_event(ClickEvent::SuggestCommand("/op".to_owned()))
        .build();
    let nbt = component.to_nbt_compound();
    let value = spinel_nbt::nbt_to_json(Nbt::Compound(nbt));

    assert_eq!(value["click_event"]["action"], json!("suggest_command"));
    assert_eq!(value["click_event"]["command"], json!("/op"));
    assert!(value.get("clickEvent").is_none());
    assert!(value["click_event"].get("value").is_none());
}

#[test]
fn component_serializes_to_nbt_without_runtime_todos() {
    let component = TextComponent::translatable("chat.type.text")
        .append("fallback")
        .build();
    let nbt = component.to_nbt_compound();

    assert_eq!(
        nbt.get("translate"),
        Some(&Nbt::String("chat.type.text".to_owned()))
    );
    assert!(matches!(nbt.get("extra"), Some(Nbt::List(_))));
}

#[test]
fn component_writes_named_color_to_ansi() {
    let component =
        TextComponent::literal_with_color("joined", TextColor::from_named(NamedTextColor::Yellow));

    assert_eq!(component.to_ansi(), "\x1b[38;2;255;255;85mjoined\x1b[0m");
}

#[test]
fn component_writes_hex_color_to_ansi() {
    let component = TextComponent::literal_with_color("custom", TextColor::from_hex("#12abEF"));

    assert_eq!(component.to_ansi(), "\x1b[38;2;18;171;239mcustom\x1b[0m");
}

#[test]
fn component_writes_plain_text_to_ansi_without_reset() {
    let component = TextComponent::literal("plain");

    assert_eq!(component.to_ansi(), "plain");
}

#[test]
fn component_resets_ansi_before_unstyled_sibling() {
    let component = TextComponent::text("")
        .append(TextComponent::literal_with_color(
            "styled",
            TextColor::from_named(NamedTextColor::Red),
        ))
        .append("plain")
        .build();

    assert_eq!(
        component.to_ansi(),
        "\x1b[38;2;255;85;85mstyled\x1b[0mplain"
    );
}
