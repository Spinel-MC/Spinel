use crate::world::{
    AnvilChunkLoader, Block, BlockPosition, Chunk, ChunkLoader, ChunkLoaderOperation, ChunkPosition,
};
use spinel_registry::biome::Biome;
use std::fs;
use std::io;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

#[test]
fn anvil_chunk_loader_supports_parallel_io() -> io::Result<()> {
    let loader = AnvilChunkLoader::new(unique_test_world_directory("parallel"))?;

    assert!(loader.supports_parallel_loading());
    assert!(loader.supports_parallel_saving());

    Ok(())
}

#[test]
fn anvil_chunk_loader_round_trips_chunk_data() -> io::Result<()> {
    let world_directory = unique_test_world_directory("round_trip");
    let loader = AnvilChunkLoader::new(world_directory.clone())?;
    let position = ChunkPosition::new(-2, 3);
    let mut chunk = Chunk::new_with_generation(position, false);
    let stone_position = BlockPosition::new(1, 4, 2);
    let biome_position = BlockPosition::new(4, 8, 4);

    chunk.set_block(stone_position, Block::STONE);
    chunk.set_biome(biome_position, Biome::DESERT);
    let section = chunk.section_mut(0).unwrap();
    section.set_sky_light(&vec![255; 2048]).unwrap();
    section.set_block_light(&vec![17; 2048]).unwrap();
    loader.save_chunk(&chunk)?;

    let loaded_chunk = loader
        .load_chunk(position)?
        .expect("saved chunk should load");

    assert_eq!(loaded_chunk.block(stone_position), Block::STONE);
    assert_eq!(loaded_chunk.biome(biome_position), Biome::DESERT);
    assert_eq!(loaded_chunk.sky_light(BlockPosition::new(0, 0, 0)), 15);
    assert_eq!(loaded_chunk.block_light(BlockPosition::new(0, 0, 0)), 1);

    fs::remove_dir_all(world_directory)?;
    Ok(())
}

#[test]
#[ignore]
fn measure_anvil_chunk_loader_disk_operations() -> io::Result<()> {
    let sample_count = 256usize;
    let world_directory = unique_test_world_directory("measure_disk_operations");
    let save_loader = AnvilChunkLoader::new(world_directory.clone())?;
    let positions = (0..sample_count)
        .map(|chunk_index| ChunkPosition::new(chunk_index as i32, 0))
        .collect::<Vec<_>>();
    let chunks = positions
        .iter()
        .copied()
        .map(measured_chunk)
        .collect::<Vec<_>>();

    let save_started_at = Instant::now();
    for chunk in &chunks {
        save_loader.save_chunk(chunk)?;
    }
    let save_elapsed = save_started_at.elapsed();

    let load_loader = AnvilChunkLoader::new(world_directory.clone())?;
    let load_started_at = Instant::now();
    for position in positions.iter().copied() {
        let loaded_chunk = load_loader.load_chunk(position)?;
        assert!(loaded_chunk.is_some());
    }
    let load_elapsed = load_started_at.elapsed();

    let existing_region_check_loader = AnvilChunkLoader::new(world_directory.clone())?;
    let existing_region_check_started_at = Instant::now();
    for position in positions
        .iter()
        .map(|position| ChunkPosition::new(position.x, 1))
    {
        let loaded_chunk = existing_region_check_loader.load_chunk(position)?;
        assert!(loaded_chunk.is_none());
    }
    let existing_region_check_elapsed = existing_region_check_started_at.elapsed();

    let missing_region_directory = unique_test_world_directory("measure_missing_region_check");
    let missing_region_check_loader = AnvilChunkLoader::new(missing_region_directory.clone())?;
    let missing_region_check_started_at = Instant::now();
    for chunk_index in 0..sample_count {
        let position = ChunkPosition::new(10_000 + chunk_index as i32, 10_000);
        let loaded_chunk = missing_region_check_loader.load_chunk(position)?;
        assert!(loaded_chunk.is_none());
    }
    let missing_region_check_elapsed = missing_region_check_started_at.elapsed();

    eprintln!(
        "anvil save chunks: count={} elapsed={:?} ns_per_op={:.2}",
        sample_count,
        save_elapsed,
        nanoseconds_per_operation(save_elapsed, sample_count)
    );
    eprintln!(
        "anvil load chunks from disk: count={} elapsed={:?} ns_per_op={:.2}",
        sample_count,
        load_elapsed,
        nanoseconds_per_operation(load_elapsed, sample_count)
    );
    eprintln!(
        "anvil check missing chunk in existing region: count={} elapsed={:?} ns_per_op={:.2}",
        sample_count,
        existing_region_check_elapsed,
        nanoseconds_per_operation(existing_region_check_elapsed, sample_count)
    );
    eprintln!(
        "anvil check missing region: count={} elapsed={:?} ns_per_op={:.2}",
        sample_count,
        missing_region_check_elapsed,
        nanoseconds_per_operation(missing_region_check_elapsed, sample_count)
    );

    fs::remove_dir_all(world_directory)?;
    fs::remove_dir_all(missing_region_directory)?;
    Ok(())
}

#[test]
fn anvil_chunk_loader_reports_corrupt_chunk_and_recovers_as_loader_miss() -> io::Result<()> {
    let world_directory = unique_test_world_directory("corrupt_chunk");
    let loader = AnvilChunkLoader::new(world_directory.clone())?;
    let position = ChunkPosition::new(0, 0);
    let mut region_header = vec![0u8; 8192];
    region_header[0..4].copy_from_slice(&[0, 0, 2, 1]);
    fs::write(
        world_directory.join("region").join("r.0.0.mca"),
        region_header,
    )?;

    let loaded_chunk = loader.load_chunk(position)?;
    let failures = loader.drain_failures();

    assert!(loaded_chunk.is_none());
    assert_eq!(failures.len(), 1);
    assert_eq!(failures[0].operation, ChunkLoaderOperation::LoadChunk);
    assert_eq!(failures[0].chunk_position, Some(position));
    assert!(!failures[0].message.is_empty());
    assert!(loader.drain_failures().is_empty());

    fs::remove_dir_all(world_directory)?;
    Ok(())
}

fn measured_chunk(position: ChunkPosition) -> Chunk {
    let mut chunk = Chunk::new_with_generation(position, false);
    let local_x = position.x.rem_euclid(16);
    chunk.set_block(BlockPosition::new(local_x, 4, 0), Block::STONE);
    chunk
}

fn nanoseconds_per_operation(elapsed: std::time::Duration, operation_count: usize) -> f64 {
    elapsed.as_nanos() as f64 / operation_count as f64
}

fn unique_test_world_directory(test_name: &str) -> std::path::PathBuf {
    let unique_suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!("spinel_anvil_{test_name}_{unique_suffix}"))
}
