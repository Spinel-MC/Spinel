use crate::command::{
    Command, CommandArgument, CommandArgumentKind, CommandArgumentValue, CommandConditionContext,
    CommandContext, CommandSyntax, CoordinateType, RelativeCoordinate, RelativeVec3,
};
use spinel_nbt::parse_snbt_compound;
use spinel_registry::{BlockState, EntityType, vanilla_world_blocks::Block};
use std::collections::HashMap;

pub struct CommandParser;

pub struct ParsedCommand<'a> {
    command: &'a Command,
    syntax: Option<&'a CommandSyntax>,
    context: CommandContext,
    error_cursor: Option<usize>,
}

pub enum CommandParseResult<'a> {
    Valid(ParsedCommand<'a>),
    Incomplete(ParsedCommand<'a>),
    Invalid(ParsedCommand<'a>),
    Unknown,
}

impl CommandParser {
    pub fn parse<'a>(
        commands: &'a [Command],
        condition_context: CommandConditionContext,
        command_line: &str,
    ) -> CommandParseResult<'a> {
        let trimmed_command_line = command_line.trim().trim_start_matches('/');
        let (command_name, command_arguments) = trimmed_command_line
            .split_once(char::is_whitespace)
            .unwrap_or((trimmed_command_line, ""));
        let Some(root_command) = commands.iter().find(|command| {
            command.name_matches(command_name)
                && command_is_visible_to_source(command, condition_context, trimmed_command_line)
        }) else {
            return CommandParseResult::Unknown;
        };
        let (command, command_arguments, command_cursor) = Self::parse_subcommand(
            root_command,
            condition_context,
            trimmed_command_line,
            command_arguments.trim(),
        );
        let parse_context = Self::parse_context(command, trimmed_command_line, command_arguments);
        let Some((context, syntax)) = parse_context else {
            let parsed_command = ParsedCommand {
                command,
                syntax: None,
                context: CommandContext::empty(trimmed_command_line),
                error_cursor: Some(command_cursor),
            };
            if command_arguments.is_empty() {
                return CommandParseResult::Incomplete(parsed_command);
            }
            return CommandParseResult::Invalid(parsed_command);
        };
        CommandParseResult::Valid(ParsedCommand {
            command,
            syntax,
            context,
            error_cursor: None,
        })
    }

    fn parse_subcommand<'a, 'b>(
        command: &'a Command,
        condition_context: CommandConditionContext,
        command_input: &str,
        command_arguments: &'b str,
    ) -> (&'a Command, &'b str, usize) {
        let command_cursor = command.name().len() + usize::from(!command_arguments.is_empty());
        parse_subcommand_from_cursor(
            command,
            condition_context,
            command_input,
            command_arguments,
            command_cursor,
        )
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
                    .filter(|_| command_arguments.trim().is_empty())
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
        if argument.typed_properties().is_some() {
            return Self::parse_typed_argument(argument, input);
        }
        match argument.kind() {
            CommandArgumentKind::Literal => Self::parse_literal(argument, input),
            CommandArgumentKind::EntityType => Self::parse_entity_type(input),
            CommandArgumentKind::RelativeVec3 => Self::parse_relative_vec3(input),
            CommandArgumentKind::RelativeBlockPosition => {
                Self::parse_relative_block_position(input)
            }
            CommandArgumentKind::BlockState => Self::parse_block_state(input),
            CommandArgumentKind::NbtCompound => Self::parse_nbt_compound(input),
            CommandArgumentKind::Parser {
                allows_space,
                uses_remaining_input,
                ..
            } => Self::parse_parser_argument(input, allows_space, uses_remaining_input),
        }
    }

    fn parse_typed_argument<'a>(
        argument: &CommandArgument,
        input: &'a str,
    ) -> Option<ParsedArgument<'a>> {
        let trimmed_input = input.trim_start();
        let (raw_input, remaining_input) = next_word(trimmed_input);
        argument.parse_typed(raw_input).map(|value| ParsedArgument {
            raw_input: raw_input.to_string(),
            value,
            remaining_input,
        })
    }
    fn parse_literal<'a>(argument: &CommandArgument, input: &'a str) -> Option<ParsedArgument<'a>> {
        let trimmed_input = input.trim_start();
        let (literal_input, remaining_input) = next_word(trimmed_input);
        (literal_input == argument.id()).then(|| ParsedArgument {
            raw_input: literal_input.to_string(),
            value: CommandArgumentValue::String(literal_input.to_string()),
            remaining_input,
        })
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

    fn parse_relative_block_position(input: &str) -> Option<ParsedArgument<'_>> {
        let trimmed_input = input.trim_start();
        let (x, x_remaining_input) = next_word(trimmed_input);
        let (y, y_remaining_input) = next_word(x_remaining_input.trim_start());
        let (z, remaining_input) = next_word(y_remaining_input.trim_start());
        let coordinates = [
            parse_block_coordinate(x)?,
            parse_block_coordinate(y)?,
            parse_block_coordinate(z)?,
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

    fn parse_block_state(input: &str) -> Option<ParsedArgument<'_>> {
        let trimmed_input = input.trim_start();
        let (raw_block_state, remaining_input) = next_word(trimmed_input);
        let block_state = parse_block_state_input(raw_block_state)?;
        Some(ParsedArgument {
            raw_input: raw_block_state.to_string(),
            value: CommandArgumentValue::BlockState(block_state),
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

    fn parse_parser_argument(
        input: &str,
        allows_space: bool,
        uses_remaining_input: bool,
    ) -> Option<ParsedArgument<'_>> {
        let trimmed_input = input.trim_start();
        if trimmed_input.is_empty() {
            return None;
        }
        let (raw_input, remaining_input) = if uses_remaining_input {
            (trimmed_input, "")
        } else if allows_space {
            next_quoted_or_word(trimmed_input)
        } else {
            next_word(trimmed_input)
        };
        if raw_input.is_empty() {
            return None;
        }
        Some(ParsedArgument {
            raw_input: raw_input.to_string(),
            value: CommandArgumentValue::String(raw_input.to_string()),
            remaining_input,
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

    pub const fn error_cursor(&self) -> Option<usize> {
        self.error_cursor
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

fn command_is_visible_to_source(
    command: &Command,
    condition_context: CommandConditionContext,
    command_input: &str,
) -> bool {
    command
        .condition()
        .map(|condition| condition(condition_context, Some(command_input)))
        .unwrap_or(true)
}

fn parse_subcommand_from_cursor<'a, 'b>(
    command: &'a Command,
    condition_context: CommandConditionContext,
    command_input: &str,
    command_arguments: &'b str,
    command_cursor: usize,
) -> (&'a Command, &'b str, usize) {
    let (next_word, remaining_input) = next_word(command_arguments);
    if next_word.is_empty() {
        return (command, command_arguments, command_cursor);
    }
    let Some(subcommand) = command.subcommands().iter().find(|subcommand| {
        subcommand.name_matches(next_word)
            && command_is_visible_to_source(subcommand, condition_context, command_input)
    }) else {
        return (command, command_arguments, command_cursor);
    };
    let remaining_command_arguments = remaining_input.trim_start();
    let consumed_separator_length = usize::from(!remaining_command_arguments.is_empty());
    parse_subcommand_from_cursor(
        subcommand,
        condition_context,
        command_input,
        remaining_command_arguments,
        command_cursor + next_word.len() + consumed_separator_length,
    )
}

fn next_quoted_or_word(input: &str) -> (&str, &str) {
    let Some(quoted_input) = input.strip_prefix('"') else {
        return next_word(input);
    };
    let Some(closing_quote_index) = quoted_input.find('"') else {
        return (input, "");
    };
    let raw_input = &quoted_input[..closing_quote_index];
    let remaining_input = &quoted_input[closing_quote_index + 1..];
    (raw_input, remaining_input)
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

fn parse_block_coordinate(input: &str) -> Option<RelativeCoordinate> {
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
    input
        .parse::<i32>()
        .ok()
        .map(|value| RelativeCoordinate::absolute(f64::from(value)))
}

fn parse_block_state_input(input: &str) -> Option<BlockState> {
    let Some(property_start) = input.find('[') else {
        return Block::from_key(input).map(BlockState::from);
    };
    if property_start == 0 || !input.ends_with(']') {
        return None;
    }
    let block_key = &input[..property_start];
    let properties_input = &input[property_start + 1..input.len() - 1];
    let block_state = Block::from_key(block_key).map(BlockState::from)?;
    properties_input
        .split(',')
        .filter(|property_input| !property_input.is_empty())
        .try_fold(block_state, apply_block_state_property)
}

fn apply_block_state_property(block_state: BlockState, property_input: &str) -> Option<BlockState> {
    let (property_name, property_value) = property_input.split_once('=')?;
    block_state.with_property(property_name, property_value)
}
