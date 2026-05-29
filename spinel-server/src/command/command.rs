use crate::command::{
    CommandArgument, CommandCondition, CommandContext, CommandExecutionResult, CommandSender,
    CommandSyntax,
};
use crate::server::MinecraftServer;
use serde_json::{Value, json};
use std::collections::BTreeSet;

pub type CommandExecutor = for<'a> fn(
    &mut MinecraftServer,
    CommandSender<'a>,
    &mut CommandContext,
) -> CommandExecutionResult;
pub type GlobalCommandListener =
    for<'a> fn(&mut MinecraftServer, CommandSender<'a>, &mut CommandContext);

pub struct Command {
    name: String,
    aliases: Vec<String>,
    subcommands: Vec<Command>,
    syntaxes: Vec<CommandSyntax>,
    default_executor: Option<CommandExecutor>,
    condition: Option<CommandCondition>,
    global_listener: Option<GlobalCommandListener>,
}

impl Command {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            aliases: Vec::new(),
            subcommands: Vec::new(),
            syntaxes: Vec::new(),
            default_executor: None,
            condition: None,
            global_listener: None,
        }
    }

    pub fn with_alias(mut self, alias: impl Into<String>) -> Self {
        self.aliases.push(alias.into());
        self
    }

    pub fn with_default_executor(mut self, executor: CommandExecutor) -> Self {
        self.default_executor = Some(executor);
        self
    }

    pub fn set_default_executor(&mut self, executor: Option<CommandExecutor>) {
        self.default_executor = executor;
    }

    pub fn with_condition(mut self, condition: CommandCondition) -> Self {
        self.condition = Some(condition);
        self
    }

    pub fn set_condition(&mut self, condition: Option<CommandCondition>) {
        self.condition = condition;
    }

    pub fn with_global_listener(mut self, listener: GlobalCommandListener) -> Self {
        self.global_listener = Some(listener);
        self
    }

    pub fn set_global_listener(&mut self, listener: Option<GlobalCommandListener>) {
        self.global_listener = listener;
    }

    pub fn with_subcommand(mut self, command: Command) -> Self {
        self.add_subcommand(command);
        self
    }

    pub fn add_subcommand(&mut self, command: Command) {
        self.subcommands.push(command);
    }

    pub fn add_subcommands(&mut self, commands: Vec<Command>) {
        commands
            .into_iter()
            .for_each(|command| self.add_subcommand(command));
    }

    pub fn with_syntax(
        mut self,
        executor: CommandExecutor,
        arguments: Vec<CommandArgument>,
    ) -> Self {
        self.add_syntax(executor, arguments);
        self
    }

    pub fn add_syntax(&mut self, executor: CommandExecutor, arguments: Vec<CommandArgument>) {
        self.syntaxes.push(CommandSyntax::new(executor, arguments));
    }

    pub fn add_conditional_syntax(
        &mut self,
        condition: Option<CommandCondition>,
        executor: CommandExecutor,
        arguments: Vec<CommandArgument>,
    ) {
        self.syntaxes
            .push(CommandSyntax::new(executor, arguments).with_condition_option(condition));
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn aliases(&self) -> &[String] {
        &self.aliases
    }

    pub fn names(&self) -> Vec<&str> {
        self.aliases
            .iter()
            .map(String::as_str)
            .chain(std::iter::once(self.name.as_str()))
            .collect()
    }

    pub fn subcommands(&self) -> &[Command] {
        &self.subcommands
    }

    pub fn syntaxes(&self) -> &[CommandSyntax] {
        &self.syntaxes
    }

    pub const fn default_executor(&self) -> Option<CommandExecutor> {
        self.default_executor
    }

    pub const fn condition(&self) -> Option<CommandCondition> {
        self.condition
    }

    pub const fn global_listener(&self) -> Option<GlobalCommandListener> {
        self.global_listener
    }

    pub fn name_matches(&self, name: &str) -> bool {
        self.name == name || self.aliases.iter().any(|alias| alias == name)
    }

    pub fn syntaxes_strings(&self) -> BTreeSet<String> {
        self.names()
            .into_iter()
            .flat_map(|name| {
                let syntax_strings = self.syntaxes.iter().map(move |syntax| {
                    let syntax_string = syntax.syntax_string();
                    if syntax_string.is_empty() {
                        name.to_string()
                    } else {
                        format!("{name} {syntax_string}")
                    }
                });
                let subcommand_strings = self.subcommands.iter().flat_map(move |subcommand| {
                    subcommand
                        .syntaxes_strings()
                        .into_iter()
                        .map(move |syntax_string| format!("{name} {syntax_string}"))
                });
                syntax_strings.chain(subcommand_strings)
            })
            .collect()
    }

    pub fn syntaxes_tree(&self) -> String {
        self.syntaxes_tree_value().to_string()
    }

    fn syntaxes_tree_value(&self) -> Value {
        json!({
            "names": self.names(),
            "nodes": self.subcommands.iter().map(Command::syntaxes_tree_value).collect::<Vec<_>>(),
            "arguments": self.syntaxes.iter().map(CommandSyntax::syntax_string).collect::<Vec<_>>()
        })
    }
}
