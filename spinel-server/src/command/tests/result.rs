use crate::command::{CommandResult, CommandResultType};
use spinel_utils::component::color::{NamedTextColor, TextColor};
use spinel_utils::component::variant::ComponentType;

#[test]
fn command_result_builds_vanilla_unknown_command_feedback() {
    let components =
        CommandResult::new(CommandResultType::Unknown, "what", None).feedback_components();

    assert_eq!(components.len(), 2);
    assert!(matches!(
        &components[0].content,
        ComponentType::Translatable { key, .. } if key == "command.unknown.command"
    ));
    assert_eq!(
        components[0].style.color,
        Some(TextColor::from_named(NamedTextColor::Red))
    );
    assert!(matches!(
        &components[1].extra[1].content,
        ComponentType::Translatable { key, .. } if key == "command.context.here"
    ));
}

#[test]
fn command_result_builds_vanilla_invalid_syntax_feedback() {
    let components = CommandResult::new(CommandResultType::InvalidSyntax, "gamemode", None)
        .feedback_components();

    assert_eq!(components.len(), 2);
    assert!(matches!(
        &components[0].content,
        ComponentType::Translatable { key, .. } if key == "command.unknown.argument"
    ));
    assert_eq!(
        components[0].style.color,
        Some(TextColor::from_named(NamedTextColor::Red))
    );
    assert!(matches!(
        &components[1].extra[0].content,
        ComponentType::Text(text) if text == "gamemode"
    ));
}
