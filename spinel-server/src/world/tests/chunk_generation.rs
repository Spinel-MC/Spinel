use crate::world::{
    Biome, Block, BlockInstance, BlockPosition, BlockSize, Chunk, ChunkPosition, ChunkSection,
    GenerationUnit, Generator, World,
};
use spinel_nbt::{Nbt, NbtCompound};
use spinel_network::encoder::PacketEncoder;
use spinel_network::types::Identifier;
use spinel_registry::block_entity_type::BlockEntityType;
use spinel_registry::{Registries, RegistryKey};
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[test]
fn generator_generate_all_visits_units_in_input_order() {
    let generated_starts = Arc::new(Mutex::new(Vec::new()));
    let observed_starts = Arc::clone(&generated_starts);
    let generator = move |unit: &mut GenerationUnit| {
        observed_starts.lock().unwrap().push(unit.absolute_start());
    };
    let mut units = vec![
        GenerationUnit::new(
            BlockSize::new(16, 16, 16),
            BlockPosition::new(0, 0, 0),
            vec![ChunkSection::new(0)],
        ),
        GenerationUnit::new(
            BlockSize::new(16, 16, 16),
            BlockPosition::new(16, 0, 0),
            vec![ChunkSection::new(0)],
        ),
    ];

    generator.generate_all(&mut units).unwrap();

    assert_eq!(
        *generated_starts.lock().unwrap(),
        vec![BlockPosition::new(0, 0, 0), BlockPosition::new(16, 0, 0)]
    );
}

#[test]
fn chunk_load_callback_observes_cached_world_membership_and_completed_generation() {
    let observed_lifecycle = Arc::new(Mutex::new(Vec::new()));
    let supplier_lifecycle = Arc::clone(&observed_lifecycle);
    let mut world = World::new(Identifier::minecraft("test"));
    world.set_chunk_supplier(move |position| {
        let generation_lifecycle = Arc::clone(&supplier_lifecycle);
        let load_lifecycle = Arc::clone(&supplier_lifecycle);
        let mut chunk = Chunk::new(position);
        chunk.set_generation_callback(move |generated_chunk| {
            generation_lifecycle
                .lock()
                .unwrap()
                .push(("generate", generated_chunk.world()));
        });
        chunk.set_load_callback(move |loaded_chunk| {
            load_lifecycle
                .lock()
                .unwrap()
                .push(("load", loaded_chunk.world()));
        });
        chunk
    });
    let world_uuid = world.uuid;

    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();

    assert_eq!(
        *observed_lifecycle.lock().unwrap(),
        vec![("generate", Some(world_uuid)), ("load", Some(world_uuid))]
    );
}

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
fn generation_callback_observes_generator_and_pending_fork_output() {
    let observed_blocks = Arc::new(Mutex::new(Vec::new()));
    let callback_observed_blocks = Arc::clone(&observed_blocks);
    let mut world = World::new(Identifier::minecraft("test"));
    world.set_chunk_supplier(move |position| {
        let callback_observed_blocks = Arc::clone(&callback_observed_blocks);
        let mut chunk = Chunk::new(position);
        chunk.set_generation_callback(move |generated_chunk| {
            callback_observed_blocks.lock().unwrap().push((
                ChunkPosition::new(generated_chunk.x(), generated_chunk.z()),
                generated_chunk.block(BlockPosition::new(16, 0, 0)),
                generated_chunk.block(BlockPosition::new(17, 0, 0)),
                generated_chunk.should_generate(),
            ));
        });
        chunk
    });
    world.set_generator(|unit| {
        if unit.absolute_start().x == 0 {
            unit.fork_blocks(|fork| {
                fork.set_block(BlockPosition::new(16, 0, 0), Block::BEDROCK);
            });
            return;
        }
        unit.modifier()
            .set_block(BlockPosition::new(17, 0, 0), Block::STONE);
    });

    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    world.load_chunk(ChunkPosition::new(1, 0)).unwrap();

    let observed_blocks = observed_blocks.lock().unwrap();
    let neighbor_observation = observed_blocks
        .iter()
        .find(|(position, _, _, _)| *position == ChunkPosition::new(1, 0))
        .unwrap();

    assert_eq!(neighbor_observation.1, Block::BEDROCK);
    assert_eq!(neighbor_observation.2, Block::STONE);
    assert!(neighbor_observation.3);
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
fn generated_heightmap_matches_minestom_top_block_edge() {
    let mut world = World::new(Identifier::minecraft("test"));
    world.set_generator(|unit| {
        unit.modifier().fill_height(0, 40, Block::STONE);
    });

    let chunk = world.load_chunk(ChunkPosition::new(0, 0)).unwrap();

    assert_eq!(
        first_heightmap_height(chunk.motion_blocking_heightmap()),
        39
    );
}

#[test]
fn generated_heightmap_refreshes_after_top_block_place_and_remove() {
    let mut world = World::new(Identifier::minecraft("test"));
    world.set_generator(|unit| {
        unit.modifier().fill_height(0, 40, Block::STONE);
    });
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();

    world
        .set_block(BlockPosition::new(0, 45, 0), Block::STONE)
        .unwrap();
    assert_eq!(
        first_heightmap_height(
            world
                .chunk(ChunkPosition::new(0, 0))
                .unwrap()
                .motion_blocking_heightmap()
        ),
        45
    );

    world
        .set_block(BlockPosition::new(0, 45, 0), Block::AIR)
        .unwrap();
    assert_eq!(
        first_heightmap_height(
            world
                .chunk(ChunkPosition::new(0, 0))
                .unwrap()
                .motion_blocking_heightmap()
        ),
        39
    );
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
    chunk.set_block_instance(
        BlockPosition::new(1, 64, 2),
        BlockInstance::from(Block::CHEST).with_nbt(Some(nbt)),
    );

    let chunk_data = chunk.data(&registries).unwrap();

    assert_eq!(chunk_data.block_entities.len(), 1);
    assert_eq!(
        chunk_data.block_entities[0].block_entity_type,
        BlockEntityType::Chest
    );
}

#[test]
fn chunk_data_cache_reuses_and_rebuilds_after_every_packet_affecting_mutation() {
    let registries = Registries::new_vanilla();
    let block_position = BlockPosition::new(1, 64, 2);
    let mut chunk = Chunk::new(ChunkPosition::new(0, 0));

    let first_chunk_data = chunk.data(&registries).unwrap();
    let second_chunk_data = chunk.data(&registries).unwrap();
    assert!(Arc::ptr_eq(&first_chunk_data, &second_chunk_data));
    assert_eq!(chunk.chunk_data_build_count(), 1);
    drop(first_chunk_data);
    drop(second_chunk_data);

    chunk.set_block(block_position, Block::STONE);
    chunk.data(&registries).unwrap();
    assert_eq!(chunk.chunk_data_build_count(), 2);

    chunk.set_biome(block_position, Biome::BADLANDS);
    chunk.data(&registries).unwrap();
    assert_eq!(chunk.chunk_data_build_count(), 3);

    chunk.set_block_instance(block_position, Block::CHEST.into());
    chunk.data(&registries).unwrap();
    assert_eq!(chunk.chunk_data_build_count(), 4);

    assert!(chunk.relight_block_light_at(block_position.y));
    chunk.data(&registries).unwrap();
    assert_eq!(chunk.chunk_data_build_count(), 5);
}

#[test]
fn chunk_data_cache_releases_data_when_callers_release_it() {
    let registries = Registries::new_vanilla();
    let chunk = Chunk::new(ChunkPosition::new(0, 0));

    let chunk_data = chunk.data(&registries).unwrap();
    assert_eq!(chunk.chunk_data_build_count(), 1);

    drop(chunk_data);
    chunk.data(&registries).unwrap();

    assert_eq!(chunk.chunk_data_build_count(), 2);
}

#[test]
fn block_change_to_non_block_entity_block_removes_stored_block_entity() {
    let mut chunk = crate::world::Chunk::new(ChunkPosition::new(0, 0));
    let block_entity_position = BlockPosition::new(1, 64, 2);
    chunk.set_block_instance(block_entity_position, Block::CHEST.into());

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
    let generated_chunks_per_second = chunk_positions.len() as f64 / load_elapsed.as_secs_f64();

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
        "chunks={} load={:?} generated_chunks_per_second={:.2} data={:?} light={:?} cached_data={:?} full_packet={:?}",
        chunk_positions.len(),
        load_elapsed,
        generated_chunks_per_second,
        data_elapsed,
        light_elapsed,
        cached_data_elapsed,
        packet_elapsed
    );

    assert!(
        generated_chunks_per_second >= 5_000.0,
        "debug chunk generation throughput was {generated_chunks_per_second:.2} chunks per second"
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

#[test]
fn showcase_bulk_generation_matches_per_coordinate_surface_behavior() {
    let mut world = World::new(Identifier::minecraft("showcase_parity"));
    world.set_generator(generate_showcase_benchmark_blocks);

    for chunk_x in -1..=1 {
        for chunk_z in -1..=1 {
            let chunk_position = ChunkPosition::new(chunk_x, chunk_z);
            world.load_chunk(chunk_position).unwrap();
            let chunk = world.chunk(chunk_position).unwrap();
            for local_x in 0..16 {
                for local_z in 0..16 {
                    let x = chunk_x * 16 + local_x;
                    let z = chunk_z * 16 + local_z;
                    let expected_surface = if (-8..=8).contains(&x) && (-8..=8).contains(&z) {
                        showcase_benchmark_spawn_platform_block(x, z)
                    } else {
                        showcase_benchmark_terrain_surface_block(x, z)
                    };
                    assert_eq!(
                        chunk.block(BlockPosition::new(x, 3, z)),
                        expected_surface,
                        "surface mismatch at {x}, {z}"
                    );
                }
            }
        }
    }
}

#[test]
#[ignore]
fn showcase_chunk_packet_construction_stays_within_tick_budget() {
    let mut world = World::new(Identifier::minecraft("showcase_packet_bench"));
    world.set_generator(generate_showcase_benchmark_blocks);
    let registries = Registries::new_vanilla();
    let chunk_positions = (-4..=4)
        .flat_map(|chunk_x| (-4..=4).map(move |chunk_z| ChunkPosition::new(chunk_x, chunk_z)))
        .collect::<Vec<_>>();
    chunk_positions.iter().for_each(|position| {
        world.load_chunk(*position).unwrap();
    });

    let data_start = Instant::now();
    chunk_positions.iter().for_each(|position| {
        let chunk = world.chunk(*position).unwrap();
        let _ = chunk.data(&registries).unwrap();
    });
    let data_elapsed = data_start.elapsed();
    let light_start = Instant::now();
    chunk_positions.iter().for_each(|position| {
        let chunk = world.chunk(*position).unwrap();
        let _ = chunk.light_data();
    });
    let light_elapsed = light_start.elapsed();
    let packet_elapsed = data_elapsed + light_elapsed;
    let packets_per_second = chunk_positions.len() as f64 / packet_elapsed.as_secs_f64();

    println!(
        "showcase_chunk_packets_per_second={packets_per_second:.2} data={data_elapsed:?} light={light_elapsed:?}"
    );
    assert!(
        packets_per_second >= 100.0,
        "showcase chunk packet construction produced {packets_per_second:.2} packets per second"
    );
}

#[test]
#[ignore]
fn showcase_generator_exceeds_live_debug_throughput_floor() {
    let mut world = World::new(Identifier::minecraft("showcase_bench"));
    world.set_generator(|unit| {
        generate_showcase_benchmark_blocks(unit);
    });
    let chunk_positions = (-8..=8)
        .flat_map(|chunk_x| (-8..=8).map(move |chunk_z| ChunkPosition::new(chunk_x, chunk_z)))
        .collect::<Vec<_>>();

    let generation_start = Instant::now();
    chunk_positions.iter().for_each(|position| {
        world.load_chunk(*position).unwrap();
    });
    let generation_elapsed = generation_start.elapsed();
    let generated_chunks_per_second =
        chunk_positions.len() as f64 / generation_elapsed.as_secs_f64();

    println!("showcase_generator_chunks_per_second={generated_chunks_per_second:.2}");

    assert!(
        generated_chunks_per_second >= 5_000.0,
        "showcase generator produced {generated_chunks_per_second:.2} chunks per second"
    );
}

#[test]
#[ignore]
fn measure_showcase_chunk_pipeline_at_flight_scale() {
    const CHUNK_COUNT: usize = 6_000;
    const WINDOW_SIZE: usize = 500;

    let mut world = World::new(Identifier::minecraft("showcase_flight_scale"));
    world.set_generator(generate_showcase_benchmark_blocks);
    let registries = Registries::new_vanilla();
    let chunk_positions = (0..CHUNK_COUNT)
        .map(|chunk_x| ChunkPosition::new(chunk_x as i32, 0))
        .collect::<Vec<_>>();

    let generation_start = Instant::now();
    let mut generation_window_start = generation_start;
    chunk_positions
        .iter()
        .enumerate()
        .for_each(|(chunk_index, position)| {
            world.load_chunk(*position).unwrap();
            if (chunk_index + 1).is_multiple_of(WINDOW_SIZE) {
                let window_elapsed = generation_window_start.elapsed();
                println!(
                    "generation_window={} chunks_per_second={:.2}",
                    (chunk_index + 1) / WINDOW_SIZE,
                    WINDOW_SIZE as f64 / window_elapsed.as_secs_f64()
                );
                generation_window_start = Instant::now();
            }
        });
    let generation_elapsed = generation_start.elapsed();

    let packet_start = Instant::now();
    let mut packet_window_start = packet_start;
    let mut encoder = PacketEncoder::new();
    encoder.set_compression(256);
    chunk_positions
        .iter()
        .enumerate()
        .for_each(|(chunk_index, position)| {
            let packet = world
                .chunk(*position)
                .unwrap()
                .full_data_packet(&registries)
                .unwrap();
            let payload = packet.encode_to_buffer().unwrap().into_buffer();
            encoder
                .encode_frame(
                    spinel_core::network::clientbound::play::chunk_data::ChunkDataAndUpdateLightPacket::get_id(),
                    &payload,
                )
                .unwrap();
            if (chunk_index + 1).is_multiple_of(WINDOW_SIZE) {
                let window_elapsed = packet_window_start.elapsed();
                println!(
                    "packet_window={} chunks_per_second={:.2}",
                    (chunk_index + 1) / WINDOW_SIZE,
                    WINDOW_SIZE as f64 / window_elapsed.as_secs_f64()
                );
                packet_window_start = Instant::now();
            }
        });
    let packet_elapsed = packet_start.elapsed();

    println!(
        "flight_scale_chunks={CHUNK_COUNT} generation_chunks_per_second={:.2} packet_chunks_per_second={:.2}",
        CHUNK_COUNT as f64 / generation_elapsed.as_secs_f64(),
        CHUNK_COUNT as f64 / packet_elapsed.as_secs_f64()
    );
}

fn first_heightmap_height(packed_heights: Vec<i64>) -> i32 {
    (packed_heights[0] as u64 & 0x1ff) as i32 - 65
}

fn generate_showcase_benchmark_blocks(unit: &mut GenerationUnit) {
    let start = unit.absolute_start();
    let end = unit.absolute_end();
    let mut modifier = unit.modifier();
    modifier.fill_height(start.y, 0, Block::STONE);
    modifier.fill_height(0, 1, Block::BEDROCK);
    modifier.fill_height(1, 3, Block::STONE);
    modifier.fill_height(3, 4, Block::GRASS_BLOCK);
    fill_showcase_benchmark_terrain_surface(&mut modifier, start, end);

    for x in start.x.max(-8)..end.x.min(9) {
        for z in start.z.max(-8)..end.z.min(9) {
            modifier.set_block(
                BlockPosition::new(x, 3, z),
                showcase_benchmark_spawn_platform_block(x, z),
            );
        }
    }
    for x in start.x..end.x {
        for z in start.z..end.z {
            if [(-8, -8), (-8, 8), (8, -8), (8, 8)].contains(&(x, z)) {
                modifier.set_block(BlockPosition::new(x, 4, z), Block::SEA_LANTERN);
            }
            if ![(12, 0), (-12, 0), (0, 12), (0, -12)].contains(&(x, z)) {
                continue;
            }
            for y in 4..=8 {
                modifier.set_block(BlockPosition::new(x, y, z), Block::COPPER_BLOCK);
            }
        }
    }
}

fn fill_showcase_benchmark_terrain_surface(
    modifier: &mut super::super::generator::UnitModifier<'_>,
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

fn showcase_benchmark_spawn_platform_block(x: i32, z: i32) -> Block {
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

fn showcase_benchmark_terrain_surface_block(x: i32, z: i32) -> Block {
    if (x.div_euclid(4) + z.div_euclid(4)).rem_euclid(5) == 0 {
        return Block::MOSS_BLOCK;
    }
    if (x.div_euclid(8) - z.div_euclid(8)).rem_euclid(7) == 0 {
        return Block::DARK_PRISMARINE;
    }
    Block::GRASS_BLOCK
}
