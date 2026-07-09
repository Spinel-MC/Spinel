use super::super::manager::CommandManager;
use crate::command::{
    Command, CommandArgument, CommandArgumentValue, CommandConditionContext,
    CommandExecutionResult, CommandSender, RelativeVec3,
};
use crate::server::MinecraftServer;
use spinel_core::network::clientbound::play::commands::{
    ArgumentParserType, COMMAND_NODE_IS_EXECUTABLE, CommandNode, CommandsPacket,
};
use spinel_network::DataType;

#[test]
fn command_manager_rejects_duplicate_roots_and_aliases() {
    let mut command_manager = CommandManager::new();

    assert!(command_manager.register(Command::new("spawn").with_alias("summon")));
    assert!(!command_manager.register(Command::new("summon")));
}

#[test]
fn command_manager_declares_reference_argument_nodes_for_spawn_syntax() {
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

#[test]
fn command_manager_declares_default_executor_literal_as_executable() {
    let mut command_manager = CommandManager::new();
    command_manager.register(Command::new("stop").with_default_executor(unused_executor));

    let commands_packet = command_manager.declare_commands_packet();
    let command_node = root_command_node(&commands_packet, "stop");

    assert_eq!(command_node.children, Vec::<i32>::new());
    assert_ne!(command_node.flags & COMMAND_NODE_IS_EXECUTABLE, 0);
}

#[test]
fn command_manager_declares_empty_syntax_literal_as_executable() {
    let mut command_manager = CommandManager::new();
    command_manager.register(
        Command::new("restart").with_syntax(unused_executor, Vec::<CommandArgument>::new()),
    );

    let commands_packet = command_manager.declare_commands_packet();
    let command_node = root_command_node(&commands_packet, "restart");

    assert_eq!(command_node.children, Vec::<i32>::new());
    assert_ne!(command_node.flags & COMMAND_NODE_IS_EXECUTABLE, 0);
}

#[test]
fn command_manager_declares_reference_argument_node_properties() {
    let mut command_manager = CommandManager::new();
    command_manager.register(command_with_single_argument(
        "word",
        CommandArgument::word("target"),
    ));
    command_manager.register(command_with_single_argument(
        "string",
        CommandArgument::string("target"),
    ));
    command_manager.register(command_with_single_argument(
        "greedy",
        CommandArgument::string_array("target"),
    ));
    command_manager.register(command_with_single_argument(
        "float",
        CommandArgument::float("target"),
    ));
    command_manager.register(command_with_single_argument(
        "double",
        CommandArgument::double("target"),
    ));
    command_manager.register(command_with_single_argument(
        "integer",
        CommandArgument::integer("target"),
    ));
    command_manager.register(command_with_single_argument(
        "long",
        CommandArgument::long("target"),
    ));
    command_manager.register(command_with_single_argument(
        "entity",
        CommandArgument::entity("target"),
    ));
    command_manager.register(command_with_single_argument(
        "time",
        CommandArgument::time("target"),
    ));
    command_manager.register(command_with_single_argument(
        "resource",
        CommandArgument::resource("target", "minecraft:block"),
    ));
    command_manager.register(command_with_single_argument(
        "resource_or_tag",
        CommandArgument::resource_or_tag("target", "minecraft:item"),
    ));

    let commands_packet = command_manager.declare_commands_packet();

    assert_eq!(
        first_argument_node(&commands_packet, "word").properties,
        vec![0]
    );
    assert_eq!(
        first_argument_node(&commands_packet, "string").properties,
        vec![1]
    );
    assert_eq!(
        first_argument_node(&commands_packet, "greedy").properties,
        vec![2]
    );
    assert_eq!(
        first_argument_node(&commands_packet, "float").properties,
        vec![0]
    );
    assert_eq!(
        first_argument_node(&commands_packet, "double").properties,
        vec![0]
    );
    assert_eq!(
        first_argument_node(&commands_packet, "integer").properties,
        vec![0]
    );
    assert_eq!(
        first_argument_node(&commands_packet, "long").properties,
        vec![0]
    );
    assert_eq!(
        first_argument_node(&commands_packet, "entity").properties,
        vec![0]
    );
    assert_eq!(
        first_argument_node(&commands_packet, "time").properties,
        vec![0, 0, 0, 0]
    );
    assert_eq!(
        first_argument_node(&commands_packet, "resource").properties,
        encoded_string("minecraft:block")
    );
    assert_eq!(
        first_argument_node(&commands_packet, "resource_or_tag").properties,
        encoded_string("minecraft:item")
    );
}

#[test]
fn command_manager_filters_declared_commands_by_source_condition() {
    let mut command_manager = CommandManager::new();
    command_manager.register(Command::new("help"));
    command_manager.register(Command::new("op").with_condition(requires_admin));

    let ordinary_packet =
        command_manager.declare_commands_packet_for_source(CommandConditionContext::player(0));
    let admin_packet =
        command_manager.declare_commands_packet_for_source(CommandConditionContext::player(3));

    assert!(root_command_exists(&ordinary_packet, "help"));
    assert!(!root_command_exists(&ordinary_packet, "op"));
    assert!(root_command_exists(&admin_packet, "op"));
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

fn command_with_single_argument(command_name: &str, argument: CommandArgument) -> Command {
    Command::new(command_name).with_syntax(unused_executor, vec![argument])
}

fn first_argument_node<'a>(
    commands_packet: &'a CommandsPacket,
    command_name: &str,
) -> &'a CommandNode {
    let command_node = root_command_node(commands_packet, command_name);
    let argument_node_index = command_node.children[0];
    &commands_packet.nodes[argument_node_index as usize]
}

fn root_command_node<'a>(
    commands_packet: &'a CommandsPacket,
    command_name: &str,
) -> &'a CommandNode {
    let command_node_index = commands_packet.nodes[commands_packet.root_index as usize]
        .children
        .iter()
        .copied()
        .find(|node_index| {
            commands_packet.nodes[*node_index as usize].name.as_deref() == Some(command_name)
        })
        .unwrap();
    &commands_packet.nodes[command_node_index as usize]
}

fn encoded_string(value: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    value.to_string().encode(&mut bytes).unwrap();
    bytes
}

fn unused_executor(
    _server: &mut MinecraftServer,
    _sender: CommandSender<'_>,
    _context: &mut crate::command::CommandContext,
) -> CommandExecutionResult {
    CommandExecutionResult::success()
}

fn root_command_exists(commands_packet: &CommandsPacket, command_name: &str) -> bool {
    commands_packet.nodes[commands_packet.root_index as usize]
        .children
        .iter()
        .copied()
        .any(|node_index| {
            commands_packet.nodes[node_index as usize].name.as_deref() == Some(command_name)
        })
}

fn requires_admin(condition_context: CommandConditionContext, _input: Option<&str>) -> bool {
    condition_context.permission_level() >= 3
}
