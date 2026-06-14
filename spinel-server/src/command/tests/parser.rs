use super::super::parser::{CommandParseResult, CommandParser};
use crate::command::{
    Command, CommandArgument, CommandArgumentValue, CommandExecutionResult, CommandSender,
    ParsedCommand, RelativeVec3,
};
use crate::server::MinecraftServer;
use spinel_registry::EntityType;

#[test]
fn parser_uses_default_relative_vec3_for_optional_position() {
    let command = spawn_command();
    let commands = [command];

    let parsed_command = valid_command(CommandParser::parse(&commands, "spawn zombie"));

    assert_eq!(
        parsed_command.context().entity_type("entity"),
        Some(EntityType::ZOMBIE)
    );
    assert_eq!(
        parsed_command.context().relative_vec3("position"),
        Some(RelativeVec3::relative_origin())
    );
}

#[test]
fn parser_reads_minestom_relative_vec3_shape() {
    let command = spawn_command();
    let commands = [command];

    let parsed_command = valid_command(CommandParser::parse(&commands, "spawn zombie ~ 2 ~-3"));

    assert_eq!(
        parsed_command
            .context()
            .relative_vec3("position")
            .unwrap()
            .resolve(10.0, 20.0, 30.0),
        (10.0, 2.0, 27.0)
    );
}

#[test]
fn parser_rejects_mixed_local_and_world_relative_vec3_shape() {
    let command = spawn_command();
    let commands = [command];

    match CommandParser::parse(&commands, "spawn zombie ^ ~ ^") {
        CommandParseResult::Invalid(_) => {}
        CommandParseResult::Valid(_) | CommandParseResult::Unknown => {
            panic!("expected invalid command")
        }
    }
}

#[test]
fn parser_reads_nested_nbt_compound_argument_shape() {
    let command = Command::new("spawn").with_syntax(
        unused_executor,
        vec![
            CommandArgument::entity_type("entity"),
            CommandArgument::relative_vec3("position"),
            CommandArgument::nbt_compound("nbt"),
        ],
    );
    let commands = [command];

    let parsed_command = valid_command(CommandParser::parse(
        &commands,
        r#"spawn armor_stand ~ ~ ~ {HasVisualFire:1b,Rotation:[90.0f,15.0f],CustomName:'{"text":"Guide"}'}"#,
    ));

    let nbt = parsed_command.context().nbt_compound("nbt").unwrap();
    assert_eq!(nbt.get("HasVisualFire"), Some(&spinel_nbt::Nbt::Byte(1)));
    assert!(
        matches!(nbt.get("Rotation"), Some(spinel_nbt::Nbt::List(values)) if values.len() == 2)
    );
    assert_eq!(
        nbt.get("CustomName"),
        Some(&spinel_nbt::Nbt::String(r#"{"text":"Guide"}"#.to_owned()))
    );
}

#[test]
fn parser_executes_nested_subcommand_with_shared_root_prefix() {
    let command = Command::new("showcase")
        .with_subcommand(Command::new("player").with_default_executor(unused_executor));
    let commands = [command];

    let parsed_command = valid_command(CommandParser::parse(&commands, "showcase player"));

    assert_eq!(parsed_command.command().name(), "player");
    assert_eq!(parsed_command.context().input(), "showcase player");
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

fn valid_command(command_parse_result: CommandParseResult<'_>) -> ParsedCommand<'_> {
    match command_parse_result {
        CommandParseResult::Valid(parsed_command) => parsed_command,
        CommandParseResult::Invalid(_) | CommandParseResult::Unknown => {
            panic!("expected valid command")
        }
    }
}
