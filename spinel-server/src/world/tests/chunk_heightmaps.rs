use crate::world::chunk_heightmaps::ChunkHeightmaps;

#[test]
fn chunk_heightmaps_use_minestom_short_storage() {
    assert_eq!(std::mem::size_of::<ChunkHeightmaps>(), 512);
}
