use super::super::manager::CommandManager;
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
