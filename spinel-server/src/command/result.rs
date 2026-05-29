use crate::command::{CommandData, ParsedCommand};

#[derive(Clone, Debug, PartialEq)]
pub struct CommandResult {
    result_type: CommandResultType,
    input: String,
    command_data: Option<CommandData>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandResultType {
    Success,
    InvalidSyntax,
    Cancelled,
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommandExecutionResult {
    result_type: CommandExecutionResultType,
    command_data: Option<CommandData>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandExecutionResultType {
    Success,
    Cancelled,
    PreconditionFailed,
    InvalidSyntax,
    ExecutorException,
    Unknown,
}

impl CommandResult {
    pub fn new(
        result_type: CommandResultType,
        input: impl Into<String>,
        command_data: Option<CommandData>,
    ) -> Self {
        Self {
            result_type,
            input: input.into(),
            command_data,
        }
    }

    pub fn from_execution_result(
        parsed_command: &ParsedCommand<'_>,
        execution_result: CommandExecutionResult,
    ) -> Self {
        let result_type = match execution_result.result_type() {
            CommandExecutionResultType::Success => CommandResultType::Success,
            CommandExecutionResultType::InvalidSyntax => CommandResultType::InvalidSyntax,
            CommandExecutionResultType::Unknown => CommandResultType::Unknown,
            CommandExecutionResultType::Cancelled
            | CommandExecutionResultType::PreconditionFailed
            | CommandExecutionResultType::ExecutorException => CommandResultType::Cancelled,
        };
        Self::new(
            result_type,
            parsed_command.context().input().to_string(),
            execution_result.command_data().cloned(),
        )
    }

    pub const fn result_type(&self) -> CommandResultType {
        self.result_type
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub const fn command_data(&self) -> Option<&CommandData> {
        self.command_data.as_ref()
    }

    pub const fn packet_listener_result(&self) -> bool {
        matches!(self.result_type, CommandResultType::Success)
    }
}

impl CommandExecutionResult {
    pub const fn new(
        result_type: CommandExecutionResultType,
        command_data: Option<CommandData>,
    ) -> Self {
        Self {
            result_type,
            command_data,
        }
    }

    pub const fn success() -> Self {
        Self::new(CommandExecutionResultType::Success, None)
    }

    pub const fn invalid_syntax() -> Self {
        Self::new(CommandExecutionResultType::InvalidSyntax, None)
    }

    pub const fn precondition_failed() -> Self {
        Self::new(CommandExecutionResultType::PreconditionFailed, None)
    }

    pub const fn unknown() -> Self {
        Self::new(CommandExecutionResultType::Unknown, None)
    }

    pub const fn result_type(&self) -> CommandExecutionResultType {
        self.result_type
    }

    pub const fn command_data(&self) -> Option<&CommandData> {
        self.command_data.as_ref()
    }
}
