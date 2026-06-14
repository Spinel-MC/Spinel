use std::io::Cursor;

use crate::data_type::DataType;
use crate::types::chunk::{ChunkSection, PalettedContainer};

#[test]
fn chunk_section_decodes_biome_palette_with_sixty_four_entries() {
    let section = ChunkSection {
        block_count: 0,
        block_states: PalettedContainer {
            bits_per_entry: 0,
            palette: Some(vec![0]),
            data: Vec::new(),
        },
        biomes: PalettedContainer {
            bits_per_entry: 1,
            palette: Some(vec![0, 1]),
            data: vec![0],
        },
    };
    let mut encoded = Vec::new();
    section.encode(&mut encoded).unwrap();
    let mut cursor = Cursor::new(&encoded);

    let decoded = ChunkSection::decode(&mut cursor).unwrap();

    assert_eq!(decoded.biomes.data, vec![0]);
    assert_eq!(cursor.position(), encoded.len() as u64);
}

#[test]
fn paletted_container_rejects_unsupported_width() {
    let error = PalettedContainer::decode(&mut Cursor::new(vec![65])).unwrap_err();

    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
}
