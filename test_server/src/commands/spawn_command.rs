use spinel::nbt::NbtCompound;
use spinel::registry::EntityType;
use spinel::server::MinecraftServer;
use spinel::server::command::{
    Command, CommandArgument, CommandArgumentValue, CommandContext, CommandExecutionResult,
    CommandSender, RelativeVec3,
};
use spinel::server::entity::EntityPosition;

pub struct SpawnCommand;

impl SpawnCommand {
    pub fn command() -> Command {
        Command::new("spawn").with_syntax(
            Self::spawn_entity,
            vec![
                CommandArgument::entity_type("entity"),
                CommandArgument::relative_vec3("position").with_default_value(
                    CommandArgumentValue::RelativeVec3(RelativeVec3::relative_origin()),
                ),
                CommandArgument::nbt_compound("nbt")
                    .with_default_value(CommandArgumentValue::NbtCompound(NbtCompound::new())),
            ],
        )
    }

    fn spawn_entity(
        server: &mut MinecraftServer,
        mut sender: CommandSender<'_>,
        context: &mut CommandContext,
    ) -> CommandExecutionResult {
        let Some(entity_type) = context.entity_type("entity") else {
            return CommandExecutionResult::invalid_syntax();
        };
        let Some(position) = context.relative_vec3("position") else {
            return CommandExecutionResult::invalid_syntax();
        };
        let Some(player) = sender.player(server) else {
            return CommandExecutionResult::precondition_failed();
        };
        let player_position = player.get_position();
        let Some(world) = player.get_world() else {
            return CommandExecutionResult::precondition_failed();
        };
        let (x, y, z) = position.resolve(
            player_position.get_x(),
            player_position.get_y(),
            player_position.get_z(),
        );
        let entity_position = EntityPosition::new(
            x,
            y,
            z,
            player_position.get_yaw(),
            player_position.get_pitch(),
        );
        match world.spawn_entity(entity_type, entity_position, context.nbt_compound("nbt")) {
            Ok(_) => CommandExecutionResult::success(),
            Err(_) => CommandExecutionResult::precondition_failed(),
        }
    }

    pub fn command_autocompletes_every_vanilla_entity_type(command: &Command) -> bool {
        command
            .syntaxes()
            .iter()
            .flat_map(|syntax| syntax.arguments())
            .any(|argument| {
                argument.id() == "entity"
                    && EntityType::ALL
                        .iter()
                        .all(|entity_type| EntityType::from_key(entity_type.path()).is_some())
            })
    }
}
