use crate::world::Block;
use crate::world::section_palette::SectionPalette;

#[test]
fn two_value_block_palette_uses_minestom_packed_storage() {
    let mut palette = SectionPalette::<_, 4096, 4>::new(Block::AIR.default_state());

    (0..4096).step_by(2).for_each(|entry_index| {
        assert!(palette.set(entry_index, Block::STONE.default_state()));
    });

    assert_eq!(palette.allocated_index_bytes(), 2048);
    assert_eq!(palette.get(0), Some(Block::STONE.default_state()));
    assert_eq!(palette.get(1), Some(Block::AIR.default_state()));
}

#[test]
fn block_palette_upsizes_without_changing_existing_entries() {
    let mut palette = SectionPalette::<_, 4096, 4>::new(0u16);

    (0u16..17).for_each(|entry| {
        assert!(palette.set(entry as usize, entry));
    });

    (0u16..17).for_each(|entry| {
        assert_eq!(palette.get(entry as usize), Some(entry));
    });
    assert_eq!(palette.get(17), Some(0));
    assert_eq!(palette.allocated_index_bytes(), 2736);
}
