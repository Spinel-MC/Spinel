use super::super::tag_query::TagQueryPacket;
use spinel_network::DataType;

#[test]
fn tag_query_packet_encodes_missing_data_as_tag_end() {
    let packet = TagQueryPacket::new(7, None);
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = TagQueryPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(TagQueryPacket::get_id(), 0x79);
    assert_eq!(payload, vec![0x07, 0x00]);
    assert_eq!(decoded_packet.transaction_id, 7);
    assert_eq!(decoded_packet.data.0, None);
}
