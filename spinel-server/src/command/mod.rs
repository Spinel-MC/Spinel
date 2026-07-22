mod argument;
mod argument_facade;
mod argument_type;
mod builder;
mod callback;
mod command;
mod condition;
mod context;
mod data;
mod exception;
mod executor;
mod manager;
mod parser;
mod result;
mod sender;
mod suggestion;
mod syntax;

#[cfg(test)]
mod tests;

pub use argument::{
    CommandArgument, CommandArgumentKind, CommandArgumentValue, CoordinateType, RelativeCoordinate,
    RelativeVec3,
};
pub use argument_facade::ArgumentType;
pub use builder::arguments::{
    AnyArgument, AnyArgumentValue, Argument, ArgumentBehavior, ArgumentClass, ArgumentInteger,
    ArgumentNumber,
};
pub use callback::ArgumentCallback;
pub use command::{Command, GlobalCommandListener};
pub use condition::{CommandCondition, CommandConditionContext};
pub use context::CommandContext;
pub use data::CommandData;
pub use exception::ArgumentError;
pub use executor::{CommandExecutor, CommandExecutorFunction, CommandExecutorMethod};
pub use manager::CommandManager;
pub use parser::{CommandParseResult, CommandParser, ParsedCommand};
pub use result::{
    CommandExecutionResult, CommandExecutionResultType, CommandResult, CommandResultType,
};
pub use sender::{CommandSender, CommandSenderKind};
pub use suggestion::{
    Suggestion, SuggestionCallback, SuggestionCallbackFunction, SuggestionCallbackMethod,
    SuggestionEntry, SuggestionType,
};
pub use syntax::CommandSyntax;
