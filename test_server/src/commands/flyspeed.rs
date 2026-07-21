use spinel::server::MinecraftServer;
use spinel::server::command::{
    Command, CommandArgument, CommandConditionContext, CommandContext, CommandExecutionResult,
    CommandExecutor, CommandSender,
};
use spinel::utils::component::Component;

pub struct FlyspeedCommand;

impl FlyspeedCommand {
    pub fn new() -> Command {
        let mut command = Command::new("flyspeed");
        command.set_condition(Some(Self::requires_gamemaster));
        command.add_syntax(
            CommandExecutor::from_function(Self::execute_flyspeed),
            [CommandArgument::float("speed")],
        );
        command
    }

    fn requires_gamemaster(
        condition_context: CommandConditionContext,
        _input: Option<&str>,
    ) -> bool {
        condition_context.permission_level() >= 2
    }

    fn execute_flyspeed(
        server: &mut MinecraftServer,
        mut sender: CommandSender<'_>,
        context: &mut CommandContext,
    ) -> CommandExecutionResult {
        let Some(flying_speed) = context.raw("speed").and_then(Self::parse_flying_speed) else {
            return CommandExecutionResult::invalid_syntax();
        };
        let Some(player) = sender.player(server) else {
            return CommandExecutionResult::precondition_failed();
        };

        if player.set_flying_speed(flying_speed).is_err() {
            return CommandExecutionResult::precondition_failed();
        }

        player.send_system_message(Component::text(format!(
            "Flying speed set to {flying_speed}."
        )));
        CommandExecutionResult::success()
    }

    fn parse_flying_speed(raw_flying_speed: &str) -> Option<f32> {
        let flying_speed = raw_flying_speed.parse::<f32>().ok()?;
        flying_speed.is_finite().then_some(flying_speed)
    }
}
