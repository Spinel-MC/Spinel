use spinel::{
    macros::import_module,
    registry::{Identifier, biome::Biome},
    server::{
        MinecraftServer,
        world::{Block, BlockPosition},
    },
};

use crate::biomes::example_biome::ExampleBiome;

pub mod biomes;
pub mod events;

import_module!("minecraft:server_list_ping");
import_module!("minecraft:login");

#[tokio::main]
async fn main() {
    let mut server = MinecraftServer::new();

    let custom_biome_key = match server.register_biome(
        Identifier::minecraft("custom_biome"),
        ExampleBiome {}.into(),
    ) {
        Ok(custom_biome) => custom_biome,
        Err(error) => {
            eprintln!("Could not register custom biome: {error:?}");
            return;
        }
    };

    let world_id = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    if let Some(world) = server.world_manager.world_mut(world_id) {
        world.set_generator(move |unit| {
            unit.modifier().fill_biome(Biome::PLAINS);
            unit.modifier().fill_height(1, 4, Block::GRASS_BLOCK);
            unit.modifier().fill_biome(custom_biome_key.clone());

            unit.modifier()
                .set_biome(BlockPosition::new(10, 10, 10), custom_biome_key.clone());
            unit.modifier().fill_height(1, 4, Block::DARK_PRISMARINE);
            unit.modifier().fill_height(0, 1, Block::BEDROCK);
        });
    }
    println!("Starting Spinel Server on 127.0.0.1:25565");
    server.start("127.0.0.1", 25565).await;
}
