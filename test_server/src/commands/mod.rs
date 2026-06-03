pub mod kill;
pub mod showcase;
pub mod spawn_command;

use self::showcase::ShowcaseCommand;
use self::spawn_command::SpawnCommand;
use spinel::server::MinecraftServer;

pub struct TestServerCommands;

impl TestServerCommands {
    pub fn register(server: &mut MinecraftServer) {
        server.command_manager.register(SpawnCommand::command());
        server.command_manager.register(ShowcaseCommand::command());
    }
}
