use super::super::map_item_data::MapItemDataPacket;
use spinel_network::data_type::DataType;
use spinel_network::types::{MapColorPatch, MapDecoration, MapId};
use spinel_utils::component::text::TextComponent;

#[test]
fn map_item_data_packet_roundtrips() {
    let packet = MapItemDataPacket {
        map_id: MapId { id: 12 },
        scale: 3,
        locked: true,
        decorations: Some(vec![MapDecoration {
            map_decoration_type_id: 2,
            x: 7,
            y: -3,
            rotation: 11,
            name: Some(TextComponent::literal("Marker")),
        }]),
        color_patch: Some(MapColorPatch {
            start_x: 1,
            start_y: 2,
            width: 2,
            height: 2,
            map_colors: vec![1, 2, 3, 4],
        }),
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = MapItemDataPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet, packet);
}
