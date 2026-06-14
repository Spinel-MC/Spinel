use crate::world::Chunk;

#[test]
fn dynamic_chunks_do_not_reserve_inline_lighting_occlusion_storage() {
    assert!(std::mem::size_of::<Chunk>() <= 1536);
}
