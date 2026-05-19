use crate::world::{Biome, Block, BlockEntity, BlockPosition, ChunkPosition, World};
use spinel_nbt::{Nbt, NbtCompound};
use spinel_network::types::Identifier;
use spinel_registry::block_entity_type::BlockEntityType;
use spinel_registry::{Registries, RegistryKey};

#[test]
fn biome_write_changes_one_quart_cell() {
    let registries = Registries::new_vanilla();
    let mut world = World::new(Identifier::minecraft("test"));
    world.set_generator(|unit| {
        unit.modifier()
            .set_biome(BlockPosition::new(0, 0, 0), Biome::BADLANDS);
    });

    let chunk = world.load_chunk(ChunkPosition::new(0, 0));
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

    let chunk = world.load_chunk(ChunkPosition::new(0, 0));

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

    world.load_chunk(ChunkPosition::new(0, 0));
    let neighbor = world.load_chunk(ChunkPosition::new(1, 0));
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

    let chunk = world.load_chunk(ChunkPosition::new(0, 0));
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

    world.load_chunk(ChunkPosition::new(0, 0));
    let neighbor = world.load_chunk(ChunkPosition::new(1, 0));
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

    world.load_chunk(ChunkPosition::new(0, 0));
    let neighbor = world.load_chunk(ChunkPosition::new(1, 0));
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

    let chunk = world.load_chunk(ChunkPosition::new(0, 0));
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

    let chunk = world.load_chunk(ChunkPosition::new(0, 0));
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

    world.load_chunk(ChunkPosition::new(0, 0));
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
