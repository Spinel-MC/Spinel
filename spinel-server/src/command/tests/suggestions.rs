use crate::command::{
    Command, CommandArgument, CommandContext, CommandExecutionResult, CommandManager,
    CommandSender, CommandSenderKind, Suggestion, SuggestionEntry,
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

    assert_eq!(root_suggestions.start(), 0);
    assert_eq!(root_suggestions.length(), 2);
    assert_eq!(root_suggestions.entries()[0].entry(), "spawn");
    assert_eq!(argument_suggestions.start(), "teleport ".len());
    assert_eq!(argument_suggestions.length(), 1);
    assert_eq!(argument_suggestions.entries()[0].entry(), "Alex");
}

fn custom_argument_with_suggestions(id: &str) -> CommandArgument {
    let mut argument = CommandArgument::custom_parser(id, ArgumentParserType::String, "String");
    argument.set_suggestion_callback(suggest_players);
    argument
}

fn suggest_players(
    _sender_kind: CommandSenderKind,
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
