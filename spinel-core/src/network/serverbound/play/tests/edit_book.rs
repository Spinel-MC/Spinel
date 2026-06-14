use super::super::edit_book::EditBookPacket;
use spinel_network::DataType;

#[test]
fn edit_book_packet_matches_minestom_slot_pages_and_optional_title_shape() {
    let packet = EditBookPacket {
        slot: 2,
        pages: vec!["one".to_string(), "two".to_string()],
        title: Some("title".to_string()),
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = EditBookPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet.slot, 2);
    assert_eq!(decoded_packet.pages, ["one", "two"]);
    assert_eq!(decoded_packet.title.as_deref(), Some("title"));
}
