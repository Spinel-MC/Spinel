use crate::showcase::{EntityShowcase, InventoryShowcase, PlayerShowcase, WorldShowcase};
use spinel::server::{
    MinecraftServer,
    command::{Command, CommandContext, CommandExecutionResult, CommandSender},
    entity::PlayerHand,
};

pub struct ShowcaseCommand;

impl ShowcaseCommand {
    pub fn command() -> Command {
        Command::new("showcase")
            .with_subcommand(Command::new("player").with_default_executor(Self::player))
            .with_subcommand(Command::new("entity").with_default_executor(Self::entity))
            .with_subcommand(Command::new("inventory").with_default_executor(Self::inventory))
            .with_subcommand(Command::new("world").with_default_executor(Self::world))
    }

    fn player(
        server: &mut MinecraftServer,
        mut sender: CommandSender<'_>,
        _context: &mut CommandContext,
    ) -> CommandExecutionResult {
        let Some(player) = sender.player(server) else {
            return CommandExecutionResult::precondition_failed();
        };
        match PlayerShowcase::apply(player) {
            Ok(_) => CommandExecutionResult::success(),
            Err(_) => CommandExecutionResult::precondition_failed(),
        }
    }

    fn entity(
        server: &mut MinecraftServer,
        mut sender: CommandSender<'_>,
        _context: &mut CommandContext,
    ) -> CommandExecutionResult {
        let Some((world_id, position, _)) = player_world_and_position(server, &mut sender) else {
            return CommandExecutionResult::precondition_failed();
        };
        match EntityShowcase::spawn(server, world_id, position) {
            Ok(pathfinding_stick) => {
                let Some(player) = sender.player(server) else {
                    return CommandExecutionResult::precondition_failed();
                };
                player.set_item_in_hand(PlayerHand::Main, pathfinding_stick);
                CommandExecutionResult::success()
            }
            Err(_) => CommandExecutionResult::precondition_failed(),
        }
    }

    fn inventory(
        server: &mut MinecraftServer,
        mut sender: CommandSender<'_>,
        _context: &mut CommandContext,
    ) -> CommandExecutionResult {
        let Some(player) = sender.player(server) else {
            return CommandExecutionResult::precondition_failed();
        };
        match InventoryShowcase::apply(player) {
            Ok(_) => CommandExecutionResult::success(),
            Err(_) => CommandExecutionResult::precondition_failed(),
        }
    }

    fn world(
        server: &mut MinecraftServer,
        mut sender: CommandSender<'_>,
        _context: &mut CommandContext,
    ) -> CommandExecutionResult {
        let Some((world_id, position, _)) = player_world_and_position(server, &mut sender) else {
            return CommandExecutionResult::precondition_failed();
        };
        match WorldShowcase::apply(server, world_id, position) {
            Ok(_) => CommandExecutionResult::success(),
            Err(_) => CommandExecutionResult::precondition_failed(),
        }
    }
}

fn player_world_and_position(
    server: &mut MinecraftServer,
    sender: &mut CommandSender<'_>,
) -> Option<(
    spinel::uuid::Uuid,
    spinel::server::entity::EntityPosition,
    spinel::server::entity::EntityId,
)> {
    let player = sender.player(server)?;
    Some((
        player.world()?.uuid(),
        player.position(),
        player.entity_id(),
    ))
}
