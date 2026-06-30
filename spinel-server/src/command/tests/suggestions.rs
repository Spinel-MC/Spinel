use crate::command::{
    Command, CommandArgument, CommandConditionContext, CommandContext, CommandExecutionResult,
    CommandManager, CommandSender, CommandSenderKind, Suggestion, SuggestionEntry,
};
use crate::server::MinecraftServer;
use spinel_core::network::clientbound::play::commands::ArgumentParserType;

#[test]
fn command_manager_suggests_roots_and_argument_callbacks_like_minestom_tab_complete() {
    let mut command_manager = CommandManager::new();
    command_manager.register(Command::new("spawn").with_alias("summon"));
    command_manager.register(Command::new("teleport").with_syntax(
        unused_executor,
        vec![custom_argument_with_suggestions("target")],
    ));

    let root_suggestions = command_manager.suggest(CommandSenderKind::Player, "/sp");
    let argument_suggestions = command_manager.suggest(CommandSenderKind::Player, "/teleport A");
    let trailing_space_suggestions =
        command_manager.suggest(CommandSenderKind::Player, "/teleport ");

    assert_eq!(root_suggestions.start(), 1);
    assert_eq!(root_suggestions.length(), 2);
    assert_eq!(root_suggestions.entries()[0].entry(), "spawn");
    assert_eq!(argument_suggestions.start(), "/teleport ".len());
    assert_eq!(argument_suggestions.length(), 1);
    assert_eq!(argument_suggestions.entries()[0].entry(), "Alex");
    assert_eq!(trailing_space_suggestions.start(), "/teleport ".len());
    assert_eq!(trailing_space_suggestions.length(), 0);
    assert_eq!(trailing_space_suggestions.entries()[0].entry(), "Alex");
}

#[test]
fn command_manager_filters_root_suggestions_by_source_condition() {
    let mut command_manager = CommandManager::new();
    command_manager.register(Command::new("op").with_condition(requires_admin));
    command_manager.register(Command::new("tell"));

    let ordinary_suggestions =
        command_manager.suggest_for_source(CommandConditionContext::player(0), "/");
    let admin_suggestions =
        command_manager.suggest_for_source(CommandConditionContext::player(3), "/");

    assert!(!suggestion_has_entry(&ordinary_suggestions, "op"));
    assert!(suggestion_has_entry(&ordinary_suggestions, "tell"));
    assert!(suggestion_has_entry(&admin_suggestions, "op"));
}
fn custom_argument_with_suggestions(id: &str) -> CommandArgument {
    let mut argument = CommandArgument::custom_parser(id, ArgumentParserType::String, "String");
    argument.set_suggestion_callback(suggest_players);
    argument
}

fn suggest_players(
    _server: Option<&MinecraftServer>,
    _condition_context: CommandConditionContext,
    _context: &CommandContext,
    suggestion: &mut Suggestion,
) {
    suggestion.add_entry(SuggestionEntry::new("Alex"));
}

fn unused_executor(
    _server: &mut MinecraftServer,
    _sender: CommandSender<'_>,
    _context: &mut CommandContext,
) -> CommandExecutionResult {
    CommandExecutionResult::success()
}

fn suggestion_has_entry(suggestion: &Suggestion, entry: &str) -> bool {
    suggestion
        .entries()
        .iter()
        .any(|suggestion_entry| suggestion_entry.entry() == entry)
}

fn requires_admin(condition_context: CommandConditionContext, _input: Option<&str>) -> bool {
    condition_context.permission_level() >= 3
}
