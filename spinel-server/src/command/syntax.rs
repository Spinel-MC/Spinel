use crate::command::{CommandArgument, CommandCondition, CommandExecutor};

pub struct CommandSyntax {
    executor: CommandExecutor,
    arguments: Vec<CommandArgument>,
    condition: Option<CommandCondition>,
}

impl CommandSyntax {
    pub fn new(executor: CommandExecutor, arguments: Vec<CommandArgument>) -> Self {
        Self {
            executor,
            arguments,
            condition: None,
        }
    }

    pub fn with_condition(mut self, condition: CommandCondition) -> Self {
        self.condition = Some(condition);
        self
    }

    pub fn with_condition_option(mut self, condition: Option<CommandCondition>) -> Self {
        self.condition = condition;
        self
    }

    pub fn set_condition(&mut self, condition: Option<CommandCondition>) {
        self.condition = condition;
    }

    pub const fn executor(&self) -> CommandExecutor {
        self.executor
    }

    pub fn arguments(&self) -> &[CommandArgument] {
        &self.arguments
    }

    pub const fn condition(&self) -> Option<CommandCondition> {
        self.condition
    }

    pub fn syntax_string(&self) -> String {
        self.arguments
            .iter()
            .map(CommandArgument::syntax_part)
            .collect::<Vec<_>>()
            .join(" ")
    }
}
