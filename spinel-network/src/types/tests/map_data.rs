use crate::data_type::DataType;
use crate::types::{MapColorPatch, MapDecoration, MapId};
use spinel_utils::component::text::TextComponent;

#[test]
fn map_id_roundtrips() {
    let map_id = MapId { id: 37 };
    let mut payload = Vec::new();

    map_id.encode(&mut payload).unwrap();
    let decoded_map_id = MapId::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_map_id, map_id);
}

#[test]
fn map_decoration_roundtrips() {
    let decoration = MapDecoration {
        map_decoration_type_id: 4,
        x: 5,
        y: -6,
        rotation: 13,
        name: Some(TextComponent::literal("Home")),
    };
    let mut payload = Vec::new();

    decoration.encode(&mut payload).unwrap();
    let decoded_decoration = MapDecoration::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_decoration, decoration);
}

#[test]
fn optional_map_color_patch_roundtrips() {
    let patch = Some(MapColorPatch {
        start_x: 1,
        start_y: 2,
        width: 3,
        height: 4,
        map_colors: vec![9, 8, 7, 6],
    });
    let mut payload = Vec::new();

    MapColorPatch::encode_optional(&patch, &mut payload).unwrap();
    let decoded_patch = MapColorPatch::decode_optional(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_patch, patch);
}
