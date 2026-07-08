use super::super::instance::World;
use crate::world::{AnvilChunkLoader, Block, BlockPosition, Chunk, ChunkPosition};
use spinel_network::types::Identifier;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn anvil_loader_miss_uses_world_chunk_supplier_like_minestom() -> io::Result<()> {
    let world_directory = unique_test_world_directory("loader_miss_supplier");
    let _ = fs::remove_dir_all(&world_directory);
    let mut world = test_world();
    world.set_chunk_loader(AnvilChunkLoader::new(world_directory.clone())?);
    world.set_chunk_supplier(|position| {
        let mut chunk = Chunk::new_lighting(position);
        chunk.set_block(BlockPosition::new(0, 0, 0), Block::GOLD_BLOCK);
        chunk
    });

    let chunk = world.load_chunk(ChunkPosition::new(33, 0))?;

    assert!(chunk.is_lighting_chunk());
    assert_eq!(chunk.block(BlockPosition::new(0, 0, 0)), Block::GOLD_BLOCK);
    fs::remove_dir_all(world_directory)?;
    Ok(())
}

#[test]
fn anvil_stored_chunk_skips_generation_and_remains_lighting_capable() -> io::Result<()> {
    let world_directory = unique_test_world_directory("stored_chunk_lighting");
    let _ = fs::remove_dir_all(&world_directory);
    let position = ChunkPosition::new(33, -33);
    let mut saved_chunk = Chunk::new_lighting_with_generation(position, false);
    saved_chunk.set_block(BlockPosition::new(0, 0, 0), Block::BEDROCK);
    let save_loader = AnvilChunkLoader::new(world_directory.clone())?;
    crate::world::ChunkLoader::save_chunk(&save_loader, &saved_chunk)?;
    let generation_count = Arc::new(AtomicUsize::new(0));
    let mut world = test_world();
    world.set_chunk_loader(AnvilChunkLoader::new(world_directory.clone())?);
    world.set_chunk_supplier(Chunk::new_lighting);
    {
        let generation_count = Arc::clone(&generation_count);
        world.set_generator(move |unit| {
            generation_count.fetch_add(1, Ordering::SeqCst);
            unit.modifier()
                .set_block(BlockPosition::new(0, 0, 0), Block::STONE);
        });
    }

    let loaded_chunk = world.load_chunk(position)?;

    assert!(loaded_chunk.is_lighting_chunk());
    assert_eq!(
        loaded_chunk.block(BlockPosition::new(0, 0, 0)),
        Block::BEDROCK
    );
    assert_eq!(generation_count.load(Ordering::SeqCst), 0);
    fs::remove_dir_all(world_directory)?;
    Ok(())
}

fn test_world() -> World {
    World::new_with_dimension_name(
        uuid::Uuid::new_v4(),
        spinel_registry::dimension_type::DimensionType::OVERWORLD,
        Identifier::minecraft("overworld"),
    )
}

fn unique_test_world_directory(test_name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "spinel-anvil-chunk-loading-{test_name}-{}",
        std::process::id()
    ))
}
