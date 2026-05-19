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
