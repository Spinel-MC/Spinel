use spinel::server::MinecraftServer;
use spinel::server::command::{
    ArgumentType, Command, CommandConditionContext, CommandContext, CommandExecutionResult,
    CommandExecutor, CommandSender,
};

pub struct GamemodeCommand;

impl GamemodeCommand {
    pub fn new() -> Command {
        let gamemode = ArgumentType::GameMode("gamemode");
        let target = ArgumentType::Entity("target").only_players(true);

        let mut command = Command::new("gamemode");

        command.set_condition(Some(Self::requires_gamemaster));
        command.add_syntax(
            CommandExecutor::from_function(Self::execute_gamemode),
            [gamemode],
        );
        command.add_syntax(
            CommandExecutor::from_function(Self::execute_gamemode),
            [gamemode, target],
        );

        command
    }

    fn requires_gamemaster(
        condition_context: CommandConditionContext,
        _input: Option<&str>,
    ) -> bool {
        condition_context.permission_level() >= 2
    }

    fn execute_gamemode(
        _server: &mut MinecraftServer,
        _sender: CommandSender<'_>,
        context: &mut CommandContext,
    ) -> CommandExecutionResult {
        let Some(_gamemode) = context.get("gamemode") else {
            return CommandExecutionResult::invalid_syntax();
        };
        let _target = context.get("target");

        CommandExecutionResult::success()
    }
}
