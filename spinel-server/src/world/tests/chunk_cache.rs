use crate::entity::EntityPosition;
use crate::world::{Block, BlockPosition, ChunkCache, ChunkPosition, World};
use spinel_network::types::Identifier;

#[test]
fn point_position_chunk_access_matches_minestom_overloads() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let block_position = BlockPosition::new(-1, 64, 16);
    let entity_position = EntityPosition::new(-0.1, 64.0, 16.9, 0.0, 0.0);
    let optional_position = EntityPosition::new(32.0, 64.0, 32.0, 0.0, 0.0);

    world.load_chunk_at(block_position).unwrap();

    assert!(world.chunk_at_position(block_position).is_some());
    assert!(world.chunk_at_position(entity_position).is_some());
    assert!(world.is_chunk_loaded_at(block_position));
    assert!(world.is_chunk_loaded_at(entity_position));

    world.enable_auto_chunk_load(false);
    assert!(world.load_optional_chunk_at(optional_position).is_none());

    world.enable_auto_chunk_load(true);
    assert!(world.load_optional_chunk_at(optional_position).is_some());
    assert!(world.is_chunk_loaded_at(optional_position));
}

#[test]
fn batch_optional_load_reports_loaded_positions_and_propagates_policy() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let positions = [ChunkPosition::new(0, 0), ChunkPosition::new(1, 0)];

    world.enable_auto_chunk_load(false);
    assert!(world.load_optional_chunks(&positions).unwrap().is_empty());

    world.enable_auto_chunk_load(true);
    assert_eq!(world.load_optional_chunks(&positions).unwrap(), positions);
}

#[test]
fn retrieve_chunk_reuses_current_chunk_and_resolves_neighbors() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let current_position = ChunkPosition::new(0, 0);
    let neighbor_position = ChunkPosition::new(1, 0);
    world.load_chunk(current_position).unwrap();
    world.load_chunk(neighbor_position).unwrap();

    let current_chunk = world.chunk(current_position).unwrap();

    assert_eq!(
        world
            .retrieve_chunk(Some(current_chunk), BlockPosition::new(1, 64, 1))
            .unwrap()
            .identifier(),
        current_chunk.identifier()
    );
    let neighbor_chunk = world
        .retrieve_chunk(Some(current_chunk), BlockPosition::new(16, 64, 1))
        .unwrap();

    assert_eq!(
        ChunkPosition::new(neighbor_chunk.x(), neighbor_chunk.z()),
        neighbor_position
    );
}

#[test]
fn chunk_cache_switches_chunks_and_uses_configured_default_for_missing_neighbors() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let current_position = ChunkPosition::new(0, 0);
    let neighbor_position = ChunkPosition::new(1, 0);
    world.load_chunk(current_position).unwrap();
    world.load_chunk(neighbor_position).unwrap();
    world
        .set_block(BlockPosition::new(15, 64, 0), Block::STONE)
        .unwrap();
    world
        .set_block(BlockPosition::new(16, 64, 0), Block::DIRT)
        .unwrap();

    let current_chunk = world.chunk(current_position);
    let mut cache = ChunkCache::with_default_block(&world, current_chunk, Block::BARRIER);

    assert_eq!(cache.block(BlockPosition::new(15, 64, 0)), Block::STONE);
    assert_eq!(cache.block(BlockPosition::new(16, 64, 0)), Block::DIRT);
    let cached_neighbor = cache.chunk().unwrap();
    assert_eq!(
        ChunkPosition::new(cached_neighbor.x(), cached_neighbor.z()),
        neighbor_position
    );
    assert_eq!(cache.block(BlockPosition::new(32, 64, 0)), Block::BARRIER);
    assert!(cache.chunk().is_none());
}

#[test]
fn unload_chunk_accepts_chunk_identity_without_moving_cache_ownership() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let position = ChunkPosition::new(2, -3);
    world.load_chunk(position).unwrap();
    let detached_chunk = world.chunk(position).unwrap().copy_for_position(position);

    assert!(world.unload_chunk(&detached_chunk).unwrap());
    assert!(world.chunk(position).is_none());
}
