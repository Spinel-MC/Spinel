use crate::world::{Biome, Block, BlockEntity, BlockPosition, ChunkPosition, World};
use spinel_nbt::{Nbt, NbtCompound};
use spinel_network::types::Identifier;
use spinel_registry::block_entity_type::BlockEntityType;
use spinel_registry::{Registries, RegistryKey};
use std::time::Instant;

#[test]
fn biome_write_changes_one_quart_cell() {
    let registries = Registries::new_vanilla();
    let mut world = World::new(Identifier::minecraft("test"));
    world.set_generator(|unit| {
        unit.modifier()
            .set_biome(BlockPosition::new(0, 0, 0), Biome::BADLANDS);
    });

    let chunk = world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    let chunk_data = chunk.data(&registries).unwrap();
    let biome_palette = &chunk_data.sections[4].biomes;

    assert_eq!(biome_palette.bits_per_entry, 1);
    assert_eq!(biome_palette.palette.as_ref().map(Vec::len), Some(2));
    assert!(biome_palette.palette.as_ref().is_some_and(|palette| {
        palette.contains(&registries.biome().get_id(&Biome::BADLANDS).unwrap())
    }));
    assert!(!biome_palette.data.is_empty());
}

#[test]
fn unregistered_biome_key_fails_chunk_serialization() {
    let registries = Registries::new_vanilla();
    let mut world = World::new(Identifier::minecraft("test"));
    world.set_generator(|unit| {
        unit.modifier().set_biome(
            BlockPosition::new(0, 0, 0),
            RegistryKey::new(Identifier::minecraft("missing_biome")),
        );
    });

    let chunk = world.load_chunk(ChunkPosition::new(0, 0)).unwrap();

    assert!(chunk.data(&registries).is_err());
}

#[test]
fn fork_into_unloaded_neighbor_applies_when_neighbor_loads() {
    let registries = Registries::new_vanilla();
    let mut world = World::new(Identifier::minecraft("test"));
    world.set_generator(|unit| {
        if unit.absolute_start().x != 0 {
            return;
        }
        unit.fork(
            BlockPosition::new(16, 0, 0),
            BlockPosition::new(17, 1, 1),
            |fork| {
                fork.modifier()
                    .set_block(BlockPosition::new(16, 0, 0), Block::BEDROCK);
            },
        );
    });

    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    let neighbor = world.load_chunk(ChunkPosition::new(1, 0)).unwrap();
    let neighbor_data = neighbor.data(&registries).unwrap();

    assert_eq!(neighbor_data.sections[4].block_count, 1);
}

#[test]
fn generated_blocks_prime_chunk_heightmaps() {
    let registries = Registries::new_vanilla();
    let mut world = World::new(Identifier::minecraft("test"));
    world.set_generator(|unit| {
        unit.modifier().fill_height(-64, 0, Block::BEDROCK);
    });

    let chunk = world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    let chunk_data = chunk.data(&registries).unwrap();
    let heightmap_has_generated_heights = chunk_data.heightmaps.iter().any(|heightmap| {
        heightmap
            .data
            .iter()
            .any(|packed_height_entry| *packed_height_entry != 0)
    });

    assert!(heightmap_has_generated_heights);
}

#[test]
fn dynamic_fork_uses_written_block_bounds() {
    let registries = Registries::new_vanilla();
    let mut world = World::new(Identifier::minecraft("test"));
    world.set_generator(|unit| {
        if unit.absolute_start().x != 0 {
            return;
        }
        unit.fork_blocks(|fork| {
            fork.set_block(BlockPosition::new(16, 0, 0), Block::BEDROCK);
        });
    });

    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    let neighbor = world.load_chunk(ChunkPosition::new(1, 0)).unwrap();
    let neighbor_data = neighbor.data(&registries).unwrap();

    assert_eq!(neighbor_data.sections[4].block_count, 1);
}

#[test]
fn dynamic_fork_resizes_by_section_and_merges_sparse_writes() {
    let registries = Registries::new_vanilla();
    let mut world = World::new(Identifier::minecraft("test"));
    world.set_generator(|unit| {
        if unit.absolute_start().x != 0 {
            return;
        }
        unit.fork_blocks(|fork| {
            fork.set_block(BlockPosition::new(16, 0, 0), Block::BEDROCK);
            fork.set_block(BlockPosition::new(31, 31, 0), Block::STONE);
        });
    });

    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    let neighbor = world.load_chunk(ChunkPosition::new(1, 0)).unwrap();
    let neighbor_data = neighbor.data(&registries).unwrap();

    assert_eq!(neighbor_data.sections[4].block_count, 1);
    assert_eq!(neighbor_data.sections[5].block_count, 1);
}

#[test]
fn section_block_count_tracks_air_transitions() {
    let registries = Registries::new_vanilla();
    let mut world = World::new(Identifier::minecraft("test"));
    world.set_generator(|unit| {
        unit.modifier()
            .set_block(BlockPosition::new(0, 0, 0), Block::BEDROCK);
        unit.modifier()
            .set_block(BlockPosition::new(0, 0, 0), Block::STONE);
        unit.modifier()
            .set_block(BlockPosition::new(1, 0, 0), Block::BEDROCK);
        unit.modifier()
            .set_block(BlockPosition::new(0, 0, 0), Block::AIR);
    });

    let chunk = world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    let chunk_data = chunk.data(&registries).unwrap();

    assert_eq!(chunk_data.sections[4].block_count, 1);
}

#[test]
fn set_all_and_set_all_relative_write_generated_blocks() {
    let registries = Registries::new_vanilla();
    let mut world = World::new(Identifier::minecraft("test"));
    world.set_generator(|unit| {
        unit.modifier().set_all_relative(|x, y, z| {
            if x == y && y == z {
                Block::BEDROCK
            } else {
                Block::AIR
            }
        });
    });

    let chunk = world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    let chunk_data = chunk.data(&registries).unwrap();

    assert!(
        chunk_data
            .sections
            .iter()
            .any(|section| section.block_count > 0)
    );
}

#[test]
fn biome_writes_in_forks_fail_clearly() {
    let mut world = World::new(Identifier::minecraft("test"));
    world.set_generator(|unit| {
        unit.fork(
            BlockPosition::new(16, 0, 0),
            BlockPosition::new(17, 1, 1),
            |fork| {
                assert!(fork.modifier().try_fill_biome(Biome::BADLANDS).is_err());
            },
        );
    });

    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
}

#[test]
fn generation_biome_area_section_and_relative_writes_match_minestom_shape() {
    let mut world = World::new(Identifier::minecraft("test"));
    world.set_generator(|unit| {
        let mut modifier = unit.modifier();
        modifier.fill_biome_area(
            BlockPosition::new(0, 0, 0),
            BlockPosition::new(4, 4, 4),
            Biome::BADLANDS,
        );
        modifier.set_relative_biome(
            crate::world::RelativeBlockPosition::new(4, 4, 4),
            Biome::BAMBOO_JUNGLE,
        );
        modifier.fill_section_biome(1, Biome::CHERRY_GROVE);
    });

    let chunk = world.load_chunk(ChunkPosition::new(0, 0)).unwrap();

    assert_eq!(chunk.biome(BlockPosition::new(0, 0, 0)), Biome::BADLANDS);
    assert_eq!(
        chunk.biome(BlockPosition::new(4, -60, 4)),
        Biome::BAMBOO_JUNGLE
    );
    assert_eq!(
        chunk.biome(BlockPosition::new(0, 16, 0)),
        Biome::CHERRY_GROVE
    );
}

#[test]
fn generator_special_blocks_are_cached_for_current_chunk_and_forks() {
    let mut world = World::new(Identifier::minecraft("test"));
    world.set_generator(|unit| {
        if unit.absolute_start().x != 0 {
            return;
        }
        unit.modifier()
            .set_block(BlockPosition::new(0, 0, 0), Block::CHEST);
        unit.fork_blocks(|fork| {
            fork.set_block(BlockPosition::new(16, 0, 0), Block::CHEST);
        });
    });

    let current_chunk = world.load_chunk(ChunkPosition::new(0, 0)).unwrap();

    assert_eq!(
        current_chunk
            .section_at_block_y(0)
            .map(|section| section.special_blocks().len()),
        Some(1)
    );

    let neighbor_chunk = world.load_chunk(ChunkPosition::new(1, 0)).unwrap();

    assert_eq!(
        neighbor_chunk
            .section_at_block_y(0)
            .map(|section| section.special_blocks().len()),
        Some(1)
    );
}

#[test]
fn chunk_block_entities_are_serialized() {
    let registries = Registries::new_vanilla();
    let mut chunk = crate::world::Chunk::new(ChunkPosition::new(0, 0));
    let mut nbt = NbtCompound::new();
    nbt.insert("id".to_string(), Nbt::String("minecraft:chest".to_string()));
    chunk.set_block_entity(BlockEntity::new(
        BlockPosition::new(1, 64, 2),
        BlockEntityType::Chest,
        nbt,
    ));

    let chunk_data = chunk.data(&registries).unwrap();

    assert_eq!(chunk_data.block_entities.len(), 1);
    assert_eq!(
        chunk_data.block_entities[0].block_entity_type,
        BlockEntityType::Chest
    );
}

#[test]
fn block_change_to_non_block_entity_block_removes_stored_block_entity() {
    let mut chunk = crate::world::Chunk::new(ChunkPosition::new(0, 0));
    let block_entity_position = BlockPosition::new(1, 64, 2);
    chunk.set_block_entity(BlockEntity::new(
        block_entity_position,
        BlockEntityType::Chest,
        NbtCompound::new(),
    ));

    chunk.set_block(block_entity_position, Block::STONE);

    assert!(chunk.block_entities().is_empty());
}

#[test]
#[ignore]
fn measure_chunk_generation_packet_and_light_cost() {
    let registries = Registries::new_vanilla();
    let mut world = World::new(Identifier::minecraft("bench"));
    world.set_generator(|unit| {
        unit.modifier().fill_height(-64, 64, Block::STONE);
    });
    let chunk_positions = (-8..=8)
        .flat_map(|chunk_x| (-8..=8).map(move |chunk_z| ChunkPosition::new(chunk_x, chunk_z)))
        .collect::<Vec<_>>();

    let load_start = Instant::now();
    chunk_positions.iter().for_each(|position| {
        world.load_chunk(*position).unwrap();
    });
    let load_elapsed = load_start.elapsed();

    let data_start = Instant::now();
    chunk_positions.iter().for_each(|position| {
        world.chunk(*position).unwrap().data(&registries).unwrap();
    });
    let data_elapsed = data_start.elapsed();

    let light_start = Instant::now();
    chunk_positions.iter().for_each(|position| {
        world.chunk(*position).unwrap().light_data();
    });
    let light_elapsed = light_start.elapsed();

    let cached_data_start = Instant::now();
    chunk_positions.iter().for_each(|position| {
        world.chunk(*position).unwrap().data(&registries).unwrap();
    });
    let cached_data_elapsed = cached_data_start.elapsed();

    let packet_start = Instant::now();
    chunk_positions.iter().for_each(|position| {
        world
            .chunk(*position)
            .unwrap()
            .full_data_packet(&registries)
            .unwrap();
    });
    let packet_elapsed = packet_start.elapsed();

    println!(
        "chunks={} load={:?} data={:?} light={:?} cached_data={:?} full_packet={:?}",
        chunk_positions.len(),
        load_elapsed,
        data_elapsed,
        light_elapsed,
        cached_data_elapsed,
        packet_elapsed
    );
}

#[test]
#[ignore]
fn measure_set_all_relative_chunk_generation_cost() {
    let mut world = World::new(Identifier::minecraft("bench"));
    world.set_generator(|unit| {
        unit.modifier()
            .set_all_relative(|relative_x, relative_y, relative_z| {
                if relative_y < 64 && (relative_x + relative_z) % 3 == 0 {
                    Block::STONE
                } else {
                    Block::AIR
                }
            });
    });
    let chunk_positions = (-8..=8)
        .flat_map(|chunk_x| (-8..=8).map(move |chunk_z| ChunkPosition::new(chunk_x, chunk_z)))
        .collect::<Vec<_>>();

    let load_start = Instant::now();
    chunk_positions.iter().for_each(|position| {
        world.load_chunk(*position).unwrap();
    });
    let load_elapsed = load_start.elapsed();

    println!(
        "chunks={} set_all_relative_load={:?}",
        chunk_positions.len(),
        load_elapsed
    );
}
