use crate::command::{
    Command, CommandArgument, CommandArgumentKind, CommandArgumentValue, CommandContext,
    CommandSyntax, CoordinateType, RelativeCoordinate, RelativeVec3,
};
use spinel_nbt::parse_snbt_compound;
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
        let Some(root_command) = commands
            .iter()
            .find(|command| command.name_matches(command_name))
        else {
            return CommandParseResult::Unknown;
        };
        let (command, command_arguments) =
            Self::parse_subcommand(root_command, command_arguments.trim());
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

    fn parse_subcommand<'a, 'b>(
        command: &'a Command,
        command_arguments: &'b str,
    ) -> (&'a Command, &'b str) {
        let (next_word, remaining_input) = next_word(command_arguments);
        if next_word.is_empty() {
            return (command, command_arguments);
        }
        let Some(subcommand) = command
            .subcommands()
            .iter()
            .find(|subcommand| subcommand.name_matches(next_word))
        else {
            return (command, command_arguments);
        };
        Self::parse_subcommand(subcommand, remaining_input.trim_start())
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
        let compound = parse_snbt_compound(nbt_input).ok()?;
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
