use spinel::{
    macros::import_module,
    network::types::Identifier,
    server::{MinecraftServer, world::Block},
};
pub mod events;

import_module!("minecraft:server_list_ping");
import_module!("minecraft:login");

#[tokio::main]
async fn main() {
    let mut server = MinecraftServer::new();
    let world_id = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    if let Some(world) = server.world_manager.world_mut(world_id) {
        world.set_generator(|unit| {
            unit.modifier().fill_height(0, 3, Block::grass_block());
        });
    }
    println!("Starting Spinel Server on 127.0.0.1:25565");
    server.start("127.0.0.1", 25565).await;
}
