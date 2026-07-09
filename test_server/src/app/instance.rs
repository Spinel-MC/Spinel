use crate::commands::TestServerCommands;
use crate::events::TestServerEventHandlers;
use crate::worlds::ShowcaseWorld;
use spinel::server::MinecraftServer;

pub struct TestServer {
    bind_address: &'static str,
    port: u16,
}

impl TestServer {
    pub const fn new(port: u16) -> Self {
        Self {
            bind_address: "127.0.0.1",
            port,
        }
    }

    pub async fn run(self) {
        let Some(server) = self.server() else {
            return;
        };

        server.start(self.bind_address, self.port).await;
    }

    fn server(&self) -> Option<MinecraftServer> {
        let mut server = MinecraftServer::new();
        TestServerEventHandlers::register(&mut server);
        TestServerCommands::register(&mut server);
        ShowcaseWorld::install(&mut server).ok()?;
        Some(server)
    }
}
