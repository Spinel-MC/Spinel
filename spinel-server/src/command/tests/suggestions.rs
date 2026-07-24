use crate::command::{
    ArgumentType, Command, CommandArgument, CommandConditionContext, CommandContext,
    CommandExecutionResult, CommandExecutor, CommandManager, CommandSender, CommandSenderKind,
    Suggestion, SuggestionCallback, SuggestionEntry,
};
use crate::server::MinecraftServer;
use spinel_core::network::clientbound::play::commands::ArgumentParserType;

#[test]
fn command_manager_suggests_roots_and_argument_callbacks_like_reference_tab_complete() {
    let mut command_manager = CommandManager::new();
    command_manager.register(Command::new("spawn").with_alias("summon"));
    command_manager.register(Command::new("teleport").with_syntax(
        CommandExecutor::from_function(unused_executor),
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
fn command_manager_deduplicates_matching_argument_suggestions_across_syntax_branches() {
    let mut command_manager = CommandManager::new();
    command_manager.register(
        Command::new("enchant")
            .with_syntax(
                CommandExecutor::from_function(unused_executor),
                vec![
                    custom_argument_with_suggestions("targets"),
                    CommandArgument::integer("level"),
                ],
            )
            .with_syntax(
                CommandExecutor::from_function(unused_executor),
                vec![custom_argument_with_suggestions("targets")],
            ),
    );

    let suggestions = command_manager.suggest(CommandSenderKind::Player, "/enchant A");
    let alex_suggestions = suggestions
        .entries()
        .iter()
        .filter(|entry| entry.entry() == "Alex")
        .count();

    assert_eq!(alex_suggestions, 1);
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

#[test]
fn entity_argument_types_suggest_vanilla_selector_roots() {
    let mut command_manager = CommandManager::new();
    command_manager.register(Command::new("flyspeed").with_syntax(
        CommandExecutor::from_function(unused_executor),
        vec![ArgumentType::Players("targets")],
    ));
    command_manager.register(Command::new("kill").with_syntax(
        CommandExecutor::from_function(unused_executor),
        vec![ArgumentType::Entities("targets")],
    ));

    let player_target_suggestions =
        command_manager.suggest_for_source(CommandConditionContext::player(2), "/flyspeed @");
    let entity_target_suggestions =
        command_manager.suggest_for_source(CommandConditionContext::player(2), "/kill @");

    assert!(suggestion_has_entry(&player_target_suggestions, "@s"));
    assert!(suggestion_has_entry(&player_target_suggestions, "@p"));
    assert!(suggestion_has_entry(&player_target_suggestions, "@a"));
    assert!(suggestion_has_entry(&player_target_suggestions, "@r"));
    assert!(!suggestion_has_entry(&player_target_suggestions, "@e"));
    assert!(suggestion_has_entry(&entity_target_suggestions, "@e"));
}
#[test]
fn command_manager_suggests_subcommand_arguments_like_minestom_tab_complete() {
    let mut command_manager = CommandManager::new();
    let subcommand = Command::new("bar").with_syntax(
        CommandExecutor::from_function(unused_executor),
        vec![
            custom_argument_with_suggestions("wordArg1"),
            custom_argument_with_suggestions("wordArg2"),
        ],
    );
    command_manager.register(Command::new("foo").with_subcommand(subcommand));

    let suggestions = command_manager.suggest(CommandSenderKind::Player, "/foo bar ");

    assert_eq!(suggestions.start(), "/foo bar ".len());
    assert_eq!(suggestions.length(), 0);
    assert_eq!(suggestions.entries()[0].entry(), "Alex");
}

#[test]
fn command_manager_uses_suggestion_callback_after_matching_subcommand_literal_branch() {
    let mut command_manager = CommandManager::new();
    let first_subcommand = Command::new("literal1").with_syntax(
        CommandExecutor::from_function(unused_executor),
        vec![custom_argument_with_suggestions("wordArg1")],
    );
    let second_subcommand = Command::new("literal2").with_syntax(
        CommandExecutor::from_function(unused_executor),
        vec![custom_argument_with_suggestions("wordArg2")],
    );
    command_manager.register(
        Command::new("foo")
            .with_subcommand(first_subcommand)
            .with_subcommand(second_subcommand),
    );

    let suggestions = command_manager.suggest(CommandSenderKind::Player, "/foo literal2 ");

    assert_eq!(suggestions.start(), "/foo literal2 ".len());
    assert_eq!(suggestions.length(), 0);
    assert_eq!(suggestions.entries()[0].entry(), "Alex");
}
#[test]
fn command_manager_does_not_treat_current_literal_token_as_argument_cursor() {
    let mut command_manager = CommandManager::new();
    let subcommand = Command::new("bar").with_syntax(
        CommandExecutor::from_function(unused_executor),
        vec![custom_argument_with_suggestions("wordArg1")],
    );
    command_manager.register(Command::new("foo").with_subcommand(subcommand));

    let suggestions = command_manager.suggest(CommandSenderKind::Player, "/foo bar");

    assert!(suggestions.entries().is_empty());
}
fn custom_argument_with_suggestions(id: &str) -> CommandArgument {
    let mut argument = CommandArgument::custom_parser(id, ArgumentParserType::String, "String");
    argument.set_suggestion_callback(SuggestionCallback::from_function(suggest_players));
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
