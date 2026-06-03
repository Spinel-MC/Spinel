use crate::app::TestServer;
use spinel::macros::import_module;

pub mod app;
pub mod biomes;
pub mod commands;
pub mod events;
pub mod showcase;
pub mod worlds;

import_module!("minecraft:server_list_ping");
import_module!("minecraft:login");

#[tokio::main]
async fn main() {
    TestServer::new().run().await;
}
