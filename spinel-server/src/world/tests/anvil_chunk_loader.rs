use crate::world::{AnvilChunkLoader, Block, BlockPosition, Chunk, ChunkLoader, ChunkPosition};
use spinel_registry::biome::Biome;
use std::fs;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

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

fn unique_test_world_directory(test_name: &str) -> std::path::PathBuf {
    let unique_suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!("spinel_anvil_{test_name}_{unique_suffix}"))
}
