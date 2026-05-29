use crate::command::{
    Command, CommandExecutionResult, CommandParseResult, CommandParser, CommandResult,
    CommandResultType, CommandSender,
};
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

    pub fn execute(
        &self,
        server: &mut MinecraftServer,
        client: &mut Client,
        command_line: &str,
    ) -> CommandResult {
        match CommandParser::parse(&self.commands, command_line) {
            CommandParseResult::Valid(mut parsed_command) => {
                self.execute_parsed_command(server, client, &mut parsed_command)
            }
            CommandParseResult::Invalid(parsed_command) => CommandResult::new(
                CommandResultType::InvalidSyntax,
                parsed_command.context().input(),
                None,
            ),
            CommandParseResult::Unknown => {
                CommandResult::new(CommandResultType::Unknown, command_line.trim(), None)
            }
        }
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

#[cfg(test)]
mod tests {
    use super::CommandManager;
    use crate::command::{
        Command, CommandArgument, CommandArgumentValue, CommandExecutionResult, CommandSender,
        RelativeVec3,
    };
    use crate::server::MinecraftServer;
    use spinel_core::network::clientbound::play::commands::ArgumentParserType;

    #[test]
    fn command_manager_rejects_duplicate_roots_and_aliases() {
        let mut command_manager = CommandManager::new();

        assert!(command_manager.register(Command::new("spawn").with_alias("summon")));
        assert!(!command_manager.register(Command::new("summon")));
    }

    #[test]
    fn command_manager_declares_minestom_argument_nodes_for_spawn_syntax() {
        let mut command_manager = CommandManager::new();
        command_manager.register(spawn_command());

        let commands_packet = command_manager.declare_commands_packet();

        assert_eq!(commands_packet.root_index, 0);
        assert_eq!(commands_packet.nodes[0].children, vec![1]);
        assert_eq!(commands_packet.nodes[1].name.as_deref(), Some("spawn"));
        assert_eq!(commands_packet.nodes[2].name.as_deref(), Some("entity"));
        assert_eq!(
            commands_packet.nodes[2].parser,
            Some(ArgumentParserType::ResourceLocation)
        );
        assert_eq!(
            commands_packet.nodes[2].suggestions_type.as_deref(),
            Some("minecraft:summonable_entities")
        );
        assert_eq!(commands_packet.nodes[3].name.as_deref(), Some("position"));
        assert_eq!(
            commands_packet.nodes[3].parser,
            Some(ArgumentParserType::Vec3)
        );
    }

    fn spawn_command() -> Command {
        Command::new("spawn").with_syntax(
            unused_executor,
            vec![
                CommandArgument::entity_type("entity"),
                CommandArgument::relative_vec3("position").with_default_value(
                    CommandArgumentValue::RelativeVec3(RelativeVec3::relative_origin()),
                ),
            ],
        )
    }

    fn unused_executor(
        _server: &mut MinecraftServer,
        _sender: CommandSender<'_>,
        _context: &mut crate::command::CommandContext,
    ) -> CommandExecutionResult {
        CommandExecutionResult::success()
    }
}
