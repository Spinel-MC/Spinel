use crate::biomes::example_biome::ExampleBiome;
use crate::showcase::ShowcaseSigns;
use spinel::{
    registry::{Identifier, biome::Biome},
    server::{
        MinecraftServer,
        world::{Block, BlockPosition},
    },
};
use std::io;

pub struct ShowcaseWorld;

impl ShowcaseWorld {
    pub fn install(server: &mut MinecraftServer) -> io::Result<()> {
        let custom_biome_key = server
            .register_biome(
                Identifier::minecraft("custom_biome"),
                ExampleBiome {}.into(),
            )
            .map_err(|error| {
                eprintln!("Could not register custom biome: {error:?}");
                io::Error::other(format!("{error:?}"))
            })?;

        let world_id = server
            .world_manager
            .create_world(Identifier::minecraft("overworld"));
        let Some(world) = server.world_manager.world_mut(world_id) else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Showcase world was not registered.",
            ));
        };

        world.set_generator(move |unit| {
            unit.modifier().fill_biome(Biome::PLAINS);
            unit.modifier().fill_biome(custom_biome_key.clone());
            unit.modifier()
                .set_biome(BlockPosition::new(10, 10, 10), custom_biome_key.clone());
            unit.modifier().set_all(showcase_block);
        });

        ShowcaseSigns::install(world)?;

        Ok(())
    }
}

fn showcase_block(x: i32, y: i32, z: i32) -> Block {
    if y == 0 {
        return Block::BEDROCK;
    }
    if y < 3 {
        return Block::STONE;
    }
    if y == 3 && (-8..=8).contains(&x) && (-8..=8).contains(&z) {
        return spawn_platform_block(x, z);
    }
    if y == 3 {
        return terrain_surface_block(x, z);
    }
    if y == 4 && marker_position(x, z) {
        return Block::SEA_LANTERN;
    }
    if (4..=8).contains(&y) && pillar_position(x, z) {
        return pillar_block(x, z);
    }
    Block::AIR
}

fn spawn_platform_block(x: i32, z: i32) -> Block {
    if x == 0 || z == 0 {
        return Block::GOLD_BLOCK;
    }
    if (x + z).rem_euclid(4) == 0 {
        return Block::DIAMOND_BLOCK;
    }
    if (x - z).rem_euclid(4) == 0 {
        return Block::EMERALD_BLOCK;
    }
    Block::QUARTZ_BLOCK
}

fn terrain_surface_block(x: i32, z: i32) -> Block {
    if (x.div_euclid(4) + z.div_euclid(4)).rem_euclid(5) == 0 {
        return Block::MOSS_BLOCK;
    }
    if (x.div_euclid(8) - z.div_euclid(8)).rem_euclid(7) == 0 {
        return Block::DARK_PRISMARINE;
    }
    Block::GRASS_BLOCK
}

fn marker_position(x: i32, z: i32) -> bool {
    [(-8, -8), (-8, 8), (8, -8), (8, 8)]
        .into_iter()
        .any(|(marker_x, marker_z)| x == marker_x && z == marker_z)
}

fn pillar_position(x: i32, z: i32) -> bool {
    [(12, 0), (-12, 0), (0, 12), (0, -12)]
        .into_iter()
        .any(|(pillar_x, pillar_z)| x == pillar_x && z == pillar_z)
}

fn pillar_block(x: i32, z: i32) -> Block {
    if x.abs() > z.abs() {
        return Block::COPPER_BLOCK;
    }
    Block::LAPIS_BLOCK
}
