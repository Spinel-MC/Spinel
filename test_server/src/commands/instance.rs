use super::gamemode_command::GamemodeCommand;
use super::showcase::ShowcaseCommand;
use super::spawn_command::SpawnCommand;
use spinel::server::MinecraftServer;

pub struct TestServerCommands;

impl TestServerCommands {
    pub fn register(server: &mut MinecraftServer) {
        server.command_manager.register(GamemodeCommand::new());
        server.command_manager.register(SpawnCommand::command());
        server.command_manager.register(ShowcaseCommand::command());
    }
}
