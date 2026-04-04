use spinel::{macros::import_module, server::MinecraftServer};
pub mod events;

import_module!("minecraft:server_list_ping");
import_module!("minecraft:login");

#[tokio::main]
async fn main() {
    let server = MinecraftServer::new();
    println!("Starting Spinel Server on 127.0.0.1:25565");
    server.start("127.0.0.1", 25565).await;
}
