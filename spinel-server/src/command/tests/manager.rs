use super::super::manager::CommandManager;
use crate::command::{
    Command, CommandArgument, CommandArgumentValue, CommandConditionContext,
    CommandExecutionResult, CommandExecutor, CommandSender, RelativeVec3,
};
use crate::network::ConnectionState;
use crate::server::MinecraftServer;
use spinel_core::network::clientbound::play::commands::{
    ArgumentParserType, COMMAND_NODE_IS_EXECUTABLE, CommandNode, CommandsPacket,
};
use spinel_network::{DataType, VarIntWrapper};
use std::io::{Cursor, Read};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::time::Duration;
use uuid::Uuid;

#[test]
fn command_manager_rejects_duplicate_roots_and_aliases() {
    let mut command_manager = CommandManager::new();

    command_manager.register(Command::new("spawn").with_alias("summon"));

    let duplicate_registration = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        command_manager.register(Command::new("summon"));
    }));

    assert!(duplicate_registration.is_err());
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
    command_manager.register(
        Command::new("stop").with_default_executor(CommandExecutor::from_function(unused_executor)),
    );

    let commands_packet = command_manager.declare_commands_packet();
    let command_node = root_command_node(&commands_packet, "stop");

    assert_eq!(command_node.children, Vec::<i32>::new());
    assert_ne!(command_node.flags & COMMAND_NODE_IS_EXECUTABLE, 0);
}

#[test]
fn command_manager_declares_empty_syntax_literal_as_executable() {
    let mut command_manager = CommandManager::new();
    command_manager.register(Command::new("restart").with_syntax(
        CommandExecutor::from_function(unused_executor),
        Vec::<CommandArgument>::new(),
    ));

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

#[test]
fn chat_command_execution_keeps_registered_commands_available_during_refresh() {
    let (mut client, mut peer_stream) = test_client_pair();
    let mut server = MinecraftServer::new();
    let world_uuid = server
        .world_manager
        .create_world(spinel_registry::dimension_type::DimensionType::OVERWORLD);
    server.command_manager.register(
        Command::new("kill")
            .with_condition(requires_admin)
            .with_default_executor(CommandExecutor::from_function(unused_executor)),
    );
    server.command_manager.register(
        Command::new("refresh")
            .with_default_executor(CommandExecutor::from_function(refresh_sender_commands)),
    );
    client.state = ConnectionState::Play;
    client.server_ptr = Some(&mut server as *mut MinecraftServer as usize);
    let mut player = crate::entity::Player::new(Uuid::nil(), "Player".to_owned(), 0, client.addr);
    player.set_client(&mut client);
    player.mark_entered_world();
    server
        .world_manager
        .world_mut(world_uuid)
        .unwrap()
        .add_entity(crate::entity::Entity::Player(player));

    assert!(crate::network::play::chat_command::execute_chat_command(
        &mut client,
        "refresh",
        &mut server,
    ));

    let command_packets = read_available_packet_frames(&mut peer_stream)
        .into_iter()
        .filter(|(packet_id, _)| *packet_id == CommandsPacket::get_id())
        .map(|(_, payload)| CommandsPacket::decode(&mut payload.as_slice()).unwrap())
        .collect::<Vec<_>>();

    assert_eq!(command_packets.len(), 1);
    assert!(root_command_exists(&command_packets[0], "kill"));
}

#[test]
fn command_manager_unregisters_registered_command_names() {
    let mut command_manager = CommandManager::new();
    command_manager.register(Command::new("spawn").with_alias("summon"));
    command_manager.unregister(&Command::new("spawn").with_alias("summon"));

    assert!(!command_manager.command_exists("spawn"));
    assert!(!command_manager.command_exists("summon"));
}

fn spawn_command() -> Command {
    Command::new("spawn").with_syntax(
        CommandExecutor::from_function(unused_executor),
        vec![
            CommandArgument::entity_type("entity"),
            CommandArgument::relative_vec3("position").with_default_value(
                CommandArgumentValue::RelativeVec3(RelativeVec3::relative_origin()),
            ),
        ],
    )
}

fn command_with_single_argument(command_name: &str, argument: CommandArgument) -> Command {
    Command::new(command_name).with_syntax(
        CommandExecutor::from_function(unused_executor),
        vec![argument],
    )
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

fn refresh_sender_commands(
    server: &mut MinecraftServer,
    mut sender: CommandSender<'_>,
    _context: &mut crate::command::CommandContext,
) -> CommandExecutionResult {
    let Some(player) = sender.player(server) else {
        return CommandExecutionResult::precondition_failed();
    };
    player.set_permission_level(4).unwrap();
    player.refresh_commands().unwrap();
    CommandExecutionResult::success()
}

#[test]
fn command_manager_merges_typed_argument_prefix_before_literal_children() {
    let mut command_manager = CommandManager::new();
    command_manager.register(
        Command::new("setblock")
            .with_syntax(
                CommandExecutor::from_function(unused_executor),
                vec![
                    CommandArgument::relative_block_position("pos"),
                    CommandArgument::block_state("block"),
                ],
            )
            .with_syntax(
                CommandExecutor::from_function(unused_executor),
                vec![
                    CommandArgument::relative_block_position("pos"),
                    CommandArgument::block_state("block"),
                    CommandArgument::literal("destroy"),
                ],
            )
            .with_syntax(
                CommandExecutor::from_function(unused_executor),
                vec![
                    CommandArgument::relative_block_position("pos"),
                    CommandArgument::block_state("block"),
                    CommandArgument::literal("keep"),
                ],
            ),
    );

    let commands_packet = command_manager.declare_commands_packet();
    let setblock_node = root_command_node(&commands_packet, "setblock");
    let position_node = &commands_packet.nodes[setblock_node.children[0] as usize];
    let block_node = &commands_packet.nodes[position_node.children[0] as usize];
    let literal_names = block_node
        .children
        .iter()
        .map(|node_index| {
            commands_packet.nodes[*node_index as usize]
                .name
                .as_deref()
                .unwrap()
        })
        .collect::<Vec<_>>();

    assert_eq!(position_node.parser, Some(ArgumentParserType::BlockPos));
    assert_eq!(block_node.parser, Some(ArgumentParserType::BlockState));
    assert_ne!(block_node.flags & COMMAND_NODE_IS_EXECUTABLE, 0);
    assert_eq!(literal_names, vec!["destroy", "keep"]);
}

#[test]
fn command_manager_declares_bounded_integer_properties() {
    let mut level = crate::command::ArgumentType::integer("level");
    level.min(0);
    let mut command_manager = CommandManager::new();
    command_manager.register(command_with_single_argument("level", level.into()));
    assert_eq!(
        first_argument_node(&command_manager.declare_commands_packet(), "level").properties,
        vec![1, 0, 0, 0, 0]
    );
}

fn test_client_pair() -> (crate::network::client::instance::Client, TcpStream) {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let client_stream = TcpStream::connect(addr).unwrap();
    let (peer_stream, _) = listener.accept().unwrap();
    (
        crate::network::client::instance::Client::new(client_stream, addr),
        peer_stream,
    )
}

fn read_available_packet_frames(peer_stream: &mut TcpStream) -> Vec<(i32, Vec<u8>)> {
    peer_stream
        .set_read_timeout(Some(Duration::from_millis(25)))
        .unwrap();
    let mut packet_frames = Vec::new();
    loop {
        let frame_length = match VarIntWrapper::decode(peer_stream) {
            Ok(frame_length) => frame_length.0 as usize,
            Err(error)
                if error.kind() == std::io::ErrorKind::WouldBlock
                    || error.kind() == std::io::ErrorKind::TimedOut =>
            {
                break;
            }
            Err(error) => panic!("packet frame length decode failed: {error}"),
        };
        let mut frame = vec![0; frame_length];
        peer_stream.read_exact(&mut frame).unwrap();
        let mut frame_cursor = Cursor::new(frame);
        let packet_id = VarIntWrapper::decode(&mut frame_cursor).unwrap().0;
        let payload_start = frame_cursor.position() as usize;
        let payload = frame_cursor.into_inner()[payload_start..].to_vec();
        packet_frames.push((packet_id, payload));
    }
    packet_frames
}
