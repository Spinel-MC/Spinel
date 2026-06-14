use crate::world::{
    Biome, Block, BlockLookupCondition, BlockPosition, Chunk, ChunkPosition, ChunkSection,
};
use spinel_registry::Registries;

#[test]
fn chunk_section_public_palette_mutation_keeps_network_block_count_coherent() {
    let mut section = ChunkSection::new(0);
    assert!(
        section
            .block_palette_mut()
            .set(0, Block::STONE.default_state())
    );

    let network_section = section
        .to_network_section(&Registries::new_vanilla())
        .unwrap();

    assert_eq!(network_section.block_count, 1);
    assert_eq!(
        section.block_palette().get(0),
        Some(Block::STONE.default_state())
    );
    assert_eq!(section.biome_palette().get(0), Some(Biome::PLAINS));
}

#[test]
fn chunk_section_clear_clone_and_light_arrays_match_minestom_section_surface() {
    let mut section = ChunkSection::new(0);
    section
        .block_palette_mut()
        .fill(Block::STONE.default_state());
    let sky_light = vec![0x11; 2048];
    let block_light = vec![0x22; 2048];
    section.set_sky_light(&sky_light).unwrap();
    section.set_block_light(&block_light).unwrap();
    let cloned_section = section.clone();

    section.clear();

    assert_eq!(
        section.block_palette().get(0),
        Some(Block::AIR.default_state())
    );
    assert_eq!(
        cloned_section.block_palette().get(0),
        Some(Block::STONE.default_state())
    );
    assert_eq!(cloned_section.sky_light(), sky_light);
    assert_eq!(cloned_section.block_light(), block_light);
    assert!(section.set_sky_light(&[0; 1]).is_err());
    assert!(section.set_block_light(&[0; 2047]).is_err());
}

#[test]
fn empty_section_light_storage_is_lazy_like_minestom() {
    let mut section = ChunkSection::new(0);

    assert!(!section.has_allocated_sky_light());
    assert!(!section.has_allocated_block_light());

    section.set_sky_light(&[0; 2048]).unwrap();
    section.set_block_light(&[0; 2048]).unwrap();

    assert!(!section.has_allocated_sky_light());
    assert!(!section.has_allocated_block_light());

    section.set_sky_light_level(0, 0, 0, 15);
    section.set_block_light_level(0, 0, 0, 7);

    assert!(section.has_allocated_sky_light());
    assert!(section.has_allocated_block_light());

    section.clear_light();

    assert!(!section.has_allocated_sky_light());
    assert!(!section.has_allocated_block_light());
}

#[test]
fn fully_lit_section_uses_minestom_canonical_storage() {
    let mut section = ChunkSection::new(0);

    section.set_sky_light(&[255; 2048]).unwrap();

    assert_eq!(section.sky_light(), [255; 2048]);
    assert_eq!(section.allocated_sky_light_bytes(), 0);
}

#[test]
fn chunk_conditional_block_lookup_distinguishes_cached_entries_and_palette_types() {
    let mut chunk = Chunk::new(ChunkPosition::new(0, 0));
    let stone_position = BlockPosition::new(1, 1, 1);
    let chest_position = BlockPosition::new(2, 1, 2);
    assert!(chunk.set_block(stone_position, Block::STONE));
    assert!(chunk.set_block(chest_position, Block::CHEST));

    assert_eq!(
        chunk.block_with_condition(stone_position, BlockLookupCondition::None),
        Some(Block::STONE)
    );
    assert_eq!(
        chunk.block_with_condition(stone_position, BlockLookupCondition::Cached),
        None
    );
    assert_eq!(
        chunk.block_with_condition(stone_position, BlockLookupCondition::Type),
        Some(Block::STONE)
    );
    assert_eq!(
        chunk.block_with_condition(chest_position, BlockLookupCondition::Cached),
        Some(Block::CHEST)
    );
}
