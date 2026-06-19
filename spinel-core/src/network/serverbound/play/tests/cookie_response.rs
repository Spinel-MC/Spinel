use super::super::cookie_response::CookieResponsePacket;
use spinel_network::data_type::DataType;
use spinel_network::types::Identifier;

#[test]
fn cookie_response_packet_roundtrips() {
    let packet = CookieResponsePacket {
        key: Identifier::minecraft("session"),
        payload: Some(vec![1, 2, 3]),
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = CookieResponsePacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet, packet);
    assert_eq!(CookieResponsePacket::get_id_const(), 0x14);
}
