use crate::world::Block;
use crate::world::section_palette::SectionPalette;

#[test]
fn two_value_block_palette_uses_reference_packed_storage() {
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
#[test]
fn storage_palette_builds_packed_indices_without_incremental_repacking() {
    let entries = vec![Block::AIR.default_state(), Block::STONE.default_state()];
    let palette_indices = (0..4096)
        .map(|entry_index| entry_index % entries.len())
        .collect::<Vec<_>>();

    let palette = SectionPalette::<_, 4096, 4>::from_storage_entries(entries, &palette_indices)
        .expect("valid storage palette should build");

    assert_eq!(palette.get(0), Some(Block::AIR.default_state()));
    assert_eq!(palette.get(1), Some(Block::STONE.default_state()));
    assert_eq!(palette.get(4095), Some(Block::STONE.default_state()));
    assert_eq!(palette.allocated_index_bytes(), 2048);
}
