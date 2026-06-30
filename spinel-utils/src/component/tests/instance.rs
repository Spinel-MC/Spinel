use crate::component::color::{NamedTextColor, TextColor};
use crate::component::events::{ClickEvent, HoverEvent};
use crate::component::text::TextComponent;
use serde_json::json;
use spinel_nbt::Nbt;

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
    assert_eq!(value["clickEvent"]["action"], json!("run_command"));
    assert_eq!(value["hoverEvent"]["action"], json!("show_text"));
    assert_eq!(value["insertion"], json!("inserted"));
    assert_eq!(value["extra"][0]["keybind"], json!("key.jump"));
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
