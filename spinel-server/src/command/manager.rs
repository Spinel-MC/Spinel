use crate::command::{
    Command, CommandConditionContext, CommandExecutionResult, CommandParseResult, CommandParser,
    CommandResult, CommandResultType, CommandSender, CommandSenderKind, Suggestion,
    SuggestionEntry,
};
use crate::entity::Player;
use crate::events::player_command::PlayerCommandEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::clientbound::play::commands::{CommandNode, CommandsPacket};

pub struct CommandManager {
    commands: Vec<Command>,
}

impl CommandManager {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn register(&mut self, command: Command) {
        let duplicate_command_name = command
            .names()
            .into_iter()
            .find(|command_name| self.command_exists(command_name));
        assert!(
            duplicate_command_name.is_none(),
            "A command with the name {} is already registered!",
            duplicate_command_name.unwrap_or_default()
        );
        self.commands.push(command);
    }

    pub fn unregister(&mut self, command: &Command) {
        self.commands
            .retain(|registered_command| registered_command.name() != command.name());
    }

    pub fn get_command(&self, command_name: &str) -> Option<&Command> {
        self.commands
            .iter()
            .find(|command| command.name_matches(command_name))
    }

    pub fn get_commands(&self) -> &[Command] {
        &self.commands
    }

    pub fn command_exists(&self, command_name: &str) -> bool {
        self.get_command(command_name).is_some()
    }

    pub(crate) fn suggest(&self, sender_kind: CommandSenderKind, input: &str) -> Suggestion {
        self.suggest_for_source(CommandConditionContext::from(sender_kind), input)
    }

    pub(crate) fn suggest_for_source(
        &self,
        condition_context: CommandConditionContext,
        input: &str,
    ) -> Suggestion {
        self.suggest_for_source_with_server(None, condition_context, input)
    }

    pub(crate) fn suggest_for_source_with_server(
        &self,
        server: Option<&MinecraftServer>,
        condition_context: CommandConditionContext,
        input: &str,
    ) -> Suggestion {
        let command_input = Self::normalized_suggestion_input(input);
        let command_has_prefix = command_input.starts_with('/');
        let command_text = command_input.trim_start_matches('/');
        let command_ends_with_space = input.ends_with(char::is_whitespace);
        let command_parts = command_text.split_whitespace().collect::<Vec<_>>();
        let mut suggestion = if command_parts.len() <= 1 && !command_ends_with_space {
            self.suggest_root_commands(condition_context, command_text)
        } else {
            self.suggest_command_arguments(
                server,
                condition_context,
                command_text,
                command_ends_with_space,
                &command_parts,
            )
        };
        if command_has_prefix {
            suggestion.set_start(suggestion.start() + 1);
        }
        suggestion
    }

    pub fn create_declare_commands_packet(&self, player: &Player) -> CommandsPacket {
        self.declare_commands_packet_for_source(CommandConditionContext::player(
            player.get_permission_level(),
        ))
    }

    fn normalized_suggestion_input(input: &str) -> String {
        input.to_string()
    }

    fn suggest_root_commands(
        &self,
        condition_context: CommandConditionContext,
        typed_command_name: &str,
    ) -> Suggestion {
        let mut suggestion = Suggestion::new(typed_command_name, 0, typed_command_name.len());
        self.get_commands()
            .iter()
            .filter(|command| Self::command_condition_allows(command, condition_context, None))
            .flat_map(Command::names)
            .filter(|command_name| command_name.starts_with(typed_command_name))
            .map(SuggestionEntry::new)
            .for_each(|entry| suggestion.add_entry(entry));
        suggestion
    }

    fn suggest_command_arguments(
        &self,
        server: Option<&MinecraftServer>,
        condition_context: CommandConditionContext,
        command_text: &str,
        command_ends_with_space: bool,
        command_parts: &[&str],
    ) -> Suggestion {
        let command_name = command_parts.first().copied().unwrap_or_default();
        let Some(command) = self.get_command(command_name) else {
            return Suggestion::new(command_text, command_text.len(), 0);
        };
        if !Self::command_condition_allows(command, condition_context, Some(command_text)) {
            return Suggestion::new(command_text, command_text.len(), 0);
        }
        let current_argument_index = if command_ends_with_space {
            command_parts.len().saturating_sub(1)
        } else {
            command_parts.len().saturating_sub(2)
        };
        let current_argument_text = if command_ends_with_space {
            ""
        } else {
            command_parts.last().copied().unwrap_or_default()
        };
        let current_argument_start = command_text
            .len()
            .saturating_sub(current_argument_text.len());
        let mut suggestion = Suggestion::new(
            command_text,
            current_argument_start,
            current_argument_text.len(),
        );
        command
            .syntaxes()
            .iter()
            .filter(|syntax| {
                Self::syntax_condition_allows(syntax, condition_context, Some(command_text))
            })
            .filter_map(|syntax| syntax.arguments().get(current_argument_index))
            .filter_map(crate::command::CommandArgument::suggestion_callback)
            .for_each(|callback| {
                let context = crate::command::CommandContext::empty(command_text);
                callback.suggest(server, condition_context, &context, &mut suggestion);
            });
        suggestion
    }

    pub(crate) fn execute(
        &self,
        server: &mut MinecraftServer,
        client: &mut Client,
        command_line: &str,
    ) -> CommandResult {
        let command_line = command_line.trim();
        let command_line = match self.dispatch_player_command_event(server, client, command_line) {
            Some(command_line) => command_line,
            None => {
                return CommandResult::new(CommandResultType::Cancelled, command_line, None);
            }
        };

        match CommandParser::parse(self.get_commands(), &command_line) {
            CommandParseResult::Valid(mut parsed_command) => {
                self.execute_parsed_command(server, client, &mut parsed_command)
            }
            CommandParseResult::Invalid(parsed_command) => CommandResult::new(
                CommandResultType::InvalidSyntax,
                parsed_command.context().input(),
                None,
            ),
            CommandParseResult::Incomplete(parsed_command) => CommandResult::new(
                CommandResultType::Unknown,
                parsed_command.context().input(),
                None,
            ),
            CommandParseResult::Unknown => {
                CommandResult::new(CommandResultType::Unknown, command_line, None)
            }
        }
    }

    fn dispatch_player_command_event(
        &self,
        server: &mut MinecraftServer,
        client: &mut Client,
        command_line: &str,
    ) -> Option<String> {
        let Some(player) = server.world_manager.player_pointer_for_client(client) else {
            return Some(command_line.to_string());
        };
        let mut event = PlayerCommandEvent::new(player, command_line);
        event.dispatch(server, client);
        (!event.is_cancelled()).then(|| event.into_command())
    }

    fn execute_parsed_command(
        &self,
        server: &mut MinecraftServer,
        client: &mut Client,
        parsed_command: &mut crate::command::ParsedCommand<'_>,
    ) -> CommandResult {
        let condition_context = Self::condition_context_for_client(server, client);
        if let Some(global_listener) = parsed_command.command().global_listener() {
            global_listener(
                server,
                CommandSender::Player(client),
                parsed_command.context_mut(),
            );
        }
        if let Some(command_condition) = parsed_command.command().condition() {
            if !command_condition(condition_context, Some(parsed_command.context().input())) {
                return CommandResult::from_execution_result(
                    parsed_command,
                    CommandExecutionResult::precondition_failed(),
                );
            }
        }
        if let Some(syntax) = parsed_command.syntax() {
            if let Some(syntax_condition) = syntax.condition() {
                if !syntax_condition(condition_context, Some(parsed_command.context().input())) {
                    return CommandResult::from_execution_result(
                        parsed_command,
                        CommandExecutionResult::precondition_failed(),
                    );
                }
            }
        }
        let execution_result = match parsed_command.syntax() {
            Some(syntax) => syntax.executor().execute(
                server,
                CommandSender::Player(client),
                parsed_command.context_mut(),
            ),
            None => match parsed_command.command().default_executor() {
                Some(default_executor) => default_executor.execute(
                    server,
                    CommandSender::Player(client),
                    parsed_command.context_mut(),
                ),
                None => CommandExecutionResult::invalid_syntax(),
            },
        };
        CommandResult::from_execution_result(parsed_command, execution_result)
    }

    pub(crate) fn declare_commands_packet(&self) -> CommandsPacket {
        self.declare_commands_packet_for_source(CommandConditionContext::server())
    }

    pub(crate) fn declare_commands_packet_for_source(
        &self,
        condition_context: CommandConditionContext,
    ) -> CommandsPacket {
        let mut nodes = vec![CommandNode::root(Vec::new())];
        let root_children = self
            .get_commands()
            .iter()
            .filter(|command| Self::command_condition_allows(command, condition_context, None))
            .map(|command| self.append_command_node(command, condition_context, &mut nodes))
            .collect::<Vec<_>>();
        nodes[0].children = root_children;
        CommandsPacket {
            nodes,
            root_index: 0,
        }
    }

    fn append_command_node(
        &self,
        command: &Command,
        condition_context: CommandConditionContext,
        nodes: &mut Vec<CommandNode>,
    ) -> i32 {
        let command_node_index = nodes.len() as i32;
        let command_has_literal_executor = command.default_executor().is_some()
            || command.syntaxes().iter().any(|syntax| {
                syntax.arguments().is_empty()
                    && Self::syntax_condition_allows(syntax, condition_context, None)
            });
        nodes.push(CommandNode::literal(
            command.name(),
            Vec::new(),
            command_has_literal_executor,
        ));
        let syntax_children = command
            .syntaxes()
            .iter()
            .filter(|syntax| !syntax.arguments().is_empty())
            .filter(|syntax| Self::syntax_condition_allows(syntax, condition_context, None))
            .map(|syntax| self.append_argument_chain(syntax.arguments(), 0, nodes))
            .collect::<Vec<_>>();
        let subcommand_children = command
            .subcommands()
            .iter()
            .filter(|subcommand| {
                Self::command_condition_allows(subcommand, condition_context, None)
            })
            .map(|subcommand| self.append_command_node(subcommand, condition_context, nodes))
            .collect::<Vec<_>>();
        nodes[command_node_index as usize].children = syntax_children
            .into_iter()
            .chain(subcommand_children)
            .collect();
        command_node_index
    }

    fn append_argument_chain(
        &self,
        arguments: &[crate::command::CommandArgument],
        argument_index: usize,
        nodes: &mut Vec<CommandNode>,
    ) -> i32 {
        let argument = &arguments[argument_index];
        let node_index = nodes.len() as i32;
        let has_next_argument = argument_index + 1 < arguments.len();
        let node_is_executable = arguments[argument_index + 1..]
            .iter()
            .all(crate::command::CommandArgument::is_optional);
        let mut node = CommandNode::argument(
            argument.id(),
            argument.parser(),
            Vec::new(),
            node_is_executable,
            argument.suggestions_type(),
        );
        node.properties = argument.protocol_properties();
        nodes.push(node);
        if has_next_argument {
            let child_index = self.append_argument_chain(arguments, argument_index + 1, nodes);
            nodes[node_index as usize].children = vec![child_index];
        }
        node_index
    }

    fn condition_context_for_client(
        server: &mut MinecraftServer,
        client: &Client,
    ) -> CommandConditionContext {
        let Some(player) = server.world_manager.player_pointer_for_client(client) else {
            return CommandConditionContext::player(0);
        };
        let permission_level = unsafe { &*player }.get_permission_level();
        CommandConditionContext::player(permission_level)
    }

    fn command_condition_allows(
        command: &Command,
        condition_context: CommandConditionContext,
        input: Option<&str>,
    ) -> bool {
        command
            .condition()
            .map(|condition| condition(condition_context, input))
            .unwrap_or(true)
    }

    fn syntax_condition_allows(
        syntax: &crate::command::CommandSyntax,
        condition_context: CommandConditionContext,
        input: Option<&str>,
    ) -> bool {
        syntax
            .condition()
            .map(|condition| condition(condition_context, input))
            .unwrap_or(true)
    }
}

impl Default for CommandManager {
    fn default() -> Self {
        Self::new()
    }
}
