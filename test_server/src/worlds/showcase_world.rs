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
            generate_showcase_blocks(unit);
        });

        ShowcaseSigns::install(world)?;

        Ok(())
    }
}

fn generate_showcase_blocks(unit: &mut spinel::server::world::GenerationUnit) {
    let start = unit.absolute_start();
    let end = unit.absolute_end();
    let mut modifier = unit.modifier();
    modifier.fill_height(start.y, 0, Block::STONE);
    modifier.fill_height(0, 1, Block::BEDROCK);
    modifier.fill_height(1, 3, Block::STONE);
    modifier.fill_height(3, 4, Block::GRASS_BLOCK);
    fill_terrain_surface(&mut modifier, start, end);

    for x in start.x.max(-8)..end.x.min(9) {
        for z in start.z.max(-8)..end.z.min(9) {
            modifier.set_block(BlockPosition::new(x, 3, z), spawn_platform_block(x, z));
        }
    }
    for x in start.x..end.x {
        for z in start.z..end.z {
            if marker_position(x, z) {
                modifier.set_block(BlockPosition::new(x, 4, z), Block::SEA_LANTERN);
            }
            if !pillar_position(x, z) {
                continue;
            }
            for y in 4..=8 {
                modifier.set_block(BlockPosition::new(x, y, z), pillar_block(x, z));
            }
        }
    }
}

fn fill_terrain_surface(
    modifier: &mut spinel::server::world::UnitModifier<'_>,
    start: BlockPosition,
    end: BlockPosition,
) {
    for x in (start.x..end.x).step_by(8) {
        for z in (start.z..end.z).step_by(8) {
            if (x.div_euclid(8) - z.div_euclid(8)).rem_euclid(7) == 0 {
                modifier.fill_area(
                    BlockPosition::new(x, 3, z),
                    BlockPosition::new(x + 8, 4, z + 8),
                    Block::DARK_PRISMARINE,
                );
            }
        }
    }
    for x in (start.x..end.x).step_by(4) {
        for z in (start.z..end.z).step_by(4) {
            if (x.div_euclid(4) + z.div_euclid(4)).rem_euclid(5) == 0 {
                modifier.fill_area(
                    BlockPosition::new(x, 3, z),
                    BlockPosition::new(x + 4, 4, z + 4),
                    Block::MOSS_BLOCK,
                );
            }
        }
    }
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
