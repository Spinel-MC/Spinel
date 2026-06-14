use crate::command::{
    Command, CommandExecutionResult, CommandParseResult, CommandParser, CommandResult,
    CommandResultType, CommandSender, CommandSenderKind, Suggestion, SuggestionEntry,
};
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

    pub fn register(&mut self, command: Command) -> bool {
        if command.names().iter().any(|name| self.command_exists(name)) {
            return false;
        }
        self.commands.push(command);
        true
    }

    pub fn unregister(&mut self, command_name: &str) -> bool {
        let original_command_count = self.commands.len();
        self.commands
            .retain(|command| command.name() != command_name);
        self.commands.len() != original_command_count
    }

    pub fn command(&self, command_name: &str) -> Option<&Command> {
        self.commands
            .iter()
            .find(|command| command.name_matches(command_name))
    }

    pub fn commands(&self) -> &[Command] {
        &self.commands
    }

    pub fn command_exists(&self, command_name: &str) -> bool {
        self.command(command_name).is_some()
    }

    pub fn suggest(&self, sender_kind: CommandSenderKind, input: &str) -> Suggestion {
        let command_input = Self::normalized_suggestion_input(input);
        let command_text = command_input.trim_start_matches('/');
        let command_ends_with_space = input.ends_with(char::is_whitespace);
        let command_parts = command_text.split_whitespace().collect::<Vec<_>>();
        if command_parts.len() <= 1 && !command_ends_with_space {
            return self.suggest_root_commands(command_text);
        }
        self.suggest_command_arguments(
            sender_kind,
            command_text,
            command_ends_with_space,
            &command_parts,
        )
    }

    fn normalized_suggestion_input(input: &str) -> String {
        input.to_string()
    }

    fn suggest_root_commands(&self, typed_command_name: &str) -> Suggestion {
        let typed_command_start = 0;
        let typed_command_length = typed_command_name.len();
        let mut suggestion = Suggestion::new(
            typed_command_name,
            typed_command_start,
            typed_command_length,
        );
        self.commands
            .iter()
            .flat_map(Command::names)
            .filter(|command_name| command_name.starts_with(typed_command_name))
            .map(SuggestionEntry::new)
            .for_each(|entry| suggestion.add_entry(entry));
        suggestion
    }

    fn suggest_command_arguments(
        &self,
        sender_kind: CommandSenderKind,
        command_text: &str,
        command_ends_with_space: bool,
        command_parts: &[&str],
    ) -> Suggestion {
        let command_name = command_parts.first().copied().unwrap_or_default();
        let Some(command) = self.command(command_name) else {
            return Suggestion::new(command_text, command_text.len(), 0);
        };
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
            .filter_map(|syntax| syntax.arguments().get(current_argument_index))
            .filter_map(crate::command::CommandArgument::suggestion_callback)
            .for_each(|callback| {
                let context = crate::command::CommandContext::empty(command_text);
                callback(sender_kind, &context, &mut suggestion);
            });
        suggestion
    }

    pub fn execute(
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

        match CommandParser::parse(&self.commands, &command_line) {
            CommandParseResult::Valid(mut parsed_command) => {
                self.execute_parsed_command(server, client, &mut parsed_command)
            }
            CommandParseResult::Invalid(parsed_command) => CommandResult::new(
                CommandResultType::InvalidSyntax,
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
        let sender_kind = CommandSender::Player(client).kind();
        if let Some(global_listener) = parsed_command.command().global_listener() {
            global_listener(
                server,
                CommandSender::Player(client),
                parsed_command.context_mut(),
            );
        }
        if let Some(command_condition) = parsed_command.command().condition() {
            if !command_condition(sender_kind, Some(parsed_command.context().input())) {
                return CommandResult::from_execution_result(
                    parsed_command,
                    CommandExecutionResult::precondition_failed(),
                );
            }
        }
        if let Some(syntax) = parsed_command.syntax() {
            if let Some(syntax_condition) = syntax.condition() {
                if !syntax_condition(sender_kind, Some(parsed_command.context().input())) {
                    return CommandResult::from_execution_result(
                        parsed_command,
                        CommandExecutionResult::precondition_failed(),
                    );
                }
            }
        }
        let execution_result = match parsed_command.syntax() {
            Some(syntax) => (syntax.executor())(
                server,
                CommandSender::Player(client),
                parsed_command.context_mut(),
            ),
            None => match parsed_command.command().default_executor() {
                Some(default_executor) => default_executor(
                    server,
                    CommandSender::Player(client),
                    parsed_command.context_mut(),
                ),
                None => CommandExecutionResult::invalid_syntax(),
            },
        };
        CommandResult::from_execution_result(parsed_command, execution_result)
    }

    pub fn declare_commands_packet(&self) -> CommandsPacket {
        let mut nodes = vec![CommandNode::root(Vec::new())];
        let root_children = self
            .commands
            .iter()
            .map(|command| self.append_command_node(command, &mut nodes))
            .collect::<Vec<_>>();
        nodes[0].children = root_children;
        CommandsPacket {
            nodes,
            root_index: 0,
        }
    }

    fn append_command_node(&self, command: &Command, nodes: &mut Vec<CommandNode>) -> i32 {
        let command_node_index = nodes.len() as i32;
        nodes.push(CommandNode::literal(command.name(), Vec::new(), false));
        let syntax_children = command
            .syntaxes()
            .iter()
            .map(|syntax| self.append_argument_chain(syntax.arguments(), 0, nodes))
            .collect::<Vec<_>>();
        let subcommand_children = command
            .subcommands()
            .iter()
            .map(|subcommand| self.append_command_node(subcommand, nodes))
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
        nodes.push(CommandNode::argument(
            argument.id(),
            argument.parser(),
            Vec::new(),
            node_is_executable,
            argument.suggestions_type(),
        ));
        if has_next_argument {
            let child_index = self.append_argument_chain(arguments, argument_index + 1, nodes);
            nodes[node_index as usize].children = vec![child_index];
        }
        node_index
    }
}

impl Default for CommandManager {
    fn default() -> Self {
        Self::new()
    }
}
