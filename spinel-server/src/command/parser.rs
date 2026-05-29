use crate::command::{
    Command, CommandArgument, CommandArgumentKind, CommandArgumentValue, CommandContext,
    CommandSyntax, CoordinateType, RelativeCoordinate, RelativeVec3,
};
use spinel_nbt::{Nbt, NbtCompound};
use spinel_registry::EntityType;
use std::collections::HashMap;

pub struct CommandParser;

pub struct ParsedCommand<'a> {
    command: &'a Command,
    syntax: Option<&'a CommandSyntax>,
    context: CommandContext,
}

pub enum CommandParseResult<'a> {
    Valid(ParsedCommand<'a>),
    Invalid(ParsedCommand<'a>),
    Unknown,
}

impl CommandParser {
    pub fn parse<'a>(commands: &'a [Command], command_line: &str) -> CommandParseResult<'a> {
        let trimmed_command_line = command_line.trim().trim_start_matches('/');
        let (command_name, command_arguments) = trimmed_command_line
            .split_once(char::is_whitespace)
            .unwrap_or((trimmed_command_line, ""));
        let Some(command) = commands
            .iter()
            .find(|command| command.name_matches(command_name))
        else {
            return CommandParseResult::Unknown;
        };
        let Some((context, syntax)) =
            Self::parse_context(command, trimmed_command_line, command_arguments)
        else {
            return CommandParseResult::Invalid(ParsedCommand {
                command,
                syntax: None,
                context: CommandContext::empty(trimmed_command_line),
            });
        };
        CommandParseResult::Valid(ParsedCommand {
            command,
            syntax,
            context,
        })
    }

    fn parse_context<'a>(
        command: &'a Command,
        command_input: &str,
        command_arguments: &str,
    ) -> Option<(CommandContext, Option<&'a CommandSyntax>)> {
        command
            .syntaxes()
            .iter()
            .find_map(|syntax| {
                let mut remaining_input = command_arguments.trim();
                let mut parsed_arguments = HashMap::new();
                let mut raw_arguments = HashMap::new();
                for argument in syntax.arguments() {
                    let parsed_argument = Self::parse_argument(argument, remaining_input)?;
                    remaining_input = parsed_argument.remaining_input.trim_start();
                    raw_arguments.insert(argument.id().to_string(), parsed_argument.raw_input);
                    parsed_arguments.insert(argument.id().to_string(), parsed_argument.value);
                }
                if remaining_input.is_empty() {
                    return Some((
                        CommandContext::new(command_input, parsed_arguments, raw_arguments),
                        Some(syntax),
                    ));
                }
                None
            })
            .or_else(|| {
                command
                    .default_executor()
                    .map(|_| (CommandContext::empty(command_input), None))
            })
    }

    fn parse_argument<'a>(
        argument: &CommandArgument,
        input: &'a str,
    ) -> Option<ParsedArgument<'a>> {
        if input.is_empty() {
            return argument.default_value().map(|value| ParsedArgument {
                raw_input: String::new(),
                value,
                remaining_input: input,
            });
        }
        match argument.kind() {
            CommandArgumentKind::EntityType => Self::parse_entity_type(input),
            CommandArgumentKind::RelativeVec3 => Self::parse_relative_vec3(input),
            CommandArgumentKind::NbtCompound => Self::parse_nbt_compound(input),
            CommandArgumentKind::Parser { .. } => None,
        }
    }

    fn parse_entity_type(input: &str) -> Option<ParsedArgument<'_>> {
        let (entity_type_key, remaining_input) = next_word(input);
        let entity_type = EntityType::from_key(entity_type_key)?;
        Some(ParsedArgument {
            raw_input: entity_type_key.to_string(),
            value: CommandArgumentValue::EntityType(entity_type),
            remaining_input,
        })
    }

    fn parse_relative_vec3(input: &str) -> Option<ParsedArgument<'_>> {
        let trimmed_input = input.trim_start();
        let (x, x_remaining_input) = next_word(trimmed_input);
        let (y, y_remaining_input) = next_word(x_remaining_input.trim_start());
        let (z, remaining_input) = next_word(y_remaining_input.trim_start());
        let coordinates = [
            parse_coordinate(x)?,
            parse_coordinate(y)?,
            parse_coordinate(z)?,
        ];
        let has_local_coordinate = coordinates
            .iter()
            .any(|coordinate| coordinate.coordinate_type() == CoordinateType::Local);
        let has_world_coordinate = coordinates
            .iter()
            .any(|coordinate| coordinate.coordinate_type() != CoordinateType::Local);
        if has_local_coordinate && has_world_coordinate {
            return None;
        }
        Some(ParsedArgument {
            raw_input: [x, y, z].join(" "),
            value: CommandArgumentValue::RelativeVec3(RelativeVec3::new(
                coordinates[0],
                coordinates[1],
                coordinates[2],
            )),
            remaining_input,
        })
    }

    fn parse_nbt_compound(input: &str) -> Option<ParsedArgument<'_>> {
        let nbt_input = input.trim();
        if !nbt_input.starts_with('{') || !nbt_input.ends_with('}') {
            return None;
        }
        let compound = parse_flat_nbt_compound(nbt_input)?;
        Some(ParsedArgument {
            raw_input: nbt_input.to_string(),
            value: CommandArgumentValue::NbtCompound(compound),
            remaining_input: "",
        })
    }
}

impl<'a> ParsedCommand<'a> {
    pub const fn command(&self) -> &'a Command {
        self.command
    }

    pub const fn syntax(&self) -> Option<&'a CommandSyntax> {
        self.syntax
    }

    pub const fn context(&self) -> &CommandContext {
        &self.context
    }

    pub const fn context_mut(&mut self) -> &mut CommandContext {
        &mut self.context
    }
}

struct ParsedArgument<'a> {
    raw_input: String,
    value: CommandArgumentValue,
    remaining_input: &'a str,
}

fn next_word(input: &str) -> (&str, &str) {
    input.split_once(char::is_whitespace).unwrap_or((input, ""))
}

fn parse_coordinate(input: &str) -> Option<RelativeCoordinate> {
    if input == "~" {
        return Some(RelativeCoordinate::relative(0.0));
    }
    if let Some(relative_value) = input.strip_prefix('~') {
        return relative_value
            .parse::<f64>()
            .ok()
            .map(RelativeCoordinate::relative);
    }
    if input == "^" {
        return Some(RelativeCoordinate::local(0.0));
    }
    if let Some(local_value) = input.strip_prefix('^') {
        return local_value
            .parse::<f64>()
            .ok()
            .map(RelativeCoordinate::local);
    }
    input.parse::<f64>().ok().map(RelativeCoordinate::absolute)
}

fn parse_flat_nbt_compound(input: &str) -> Option<NbtCompound> {
    let content = input.strip_prefix('{')?.strip_suffix('}')?.trim();
    if content.is_empty() {
        return Some(NbtCompound::new());
    }
    content
        .split(',')
        .try_fold(NbtCompound::new(), |mut compound, entry| {
            let (key, value) = entry.split_once(':')?;
            compound.insert(key.trim().to_string(), parse_flat_nbt_value(value.trim())?);
            Some(compound)
        })
}

fn parse_flat_nbt_value(input: &str) -> Option<Nbt> {
    if let Some(byte_value) = input.strip_suffix('b') {
        return byte_value.parse::<i8>().ok().map(Nbt::Byte);
    }
    if let Some(short_value) = input.strip_suffix('s') {
        return short_value.parse::<i16>().ok().map(Nbt::Short);
    }
    if let Some(long_value) = input.strip_suffix('l') {
        return long_value.parse::<i64>().ok().map(Nbt::Long);
    }
    if let Some(float_value) = input.strip_suffix('f') {
        return float_value.parse::<f32>().ok().map(Nbt::Float);
    }
    if let Some(double_value) = input.strip_suffix('d') {
        return double_value.parse::<f64>().ok().map(Nbt::Double);
    }
    input.parse::<i32>().ok().map(Nbt::Int)
}

#[cfg(test)]
mod tests {
    use super::{CommandParseResult, CommandParser};
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
    fn parser_reads_flat_nbt_compound_argument_shape() {
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
            "spawn armor_stand ~ ~ ~ {HasVisualFire:1b}",
        ));

        assert_eq!(
            parsed_command
                .context()
                .nbt_compound("nbt")
                .unwrap()
                .get("HasVisualFire"),
            Some(&spinel_nbt::Nbt::Byte(1))
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

    fn valid_command(command_parse_result: CommandParseResult<'_>) -> ParsedCommand<'_> {
        match command_parse_result {
            CommandParseResult::Valid(parsed_command) => parsed_command,
            CommandParseResult::Invalid(_) | CommandParseResult::Unknown => {
                panic!("expected valid command")
            }
        }
    }
}
