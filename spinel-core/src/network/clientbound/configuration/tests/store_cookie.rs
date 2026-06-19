use super::super::store_cookie::StoreCookiePacket;
use spinel_network::DataType;
use spinel_network::types::Identifier;

#[test]
fn store_cookie_packet_roundtrips_bounded_payload_bytes() {
    let packet = StoreCookiePacket {
        key: Identifier::minecraft("server_links"),
        payload: vec![1, 2, 3],
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = StoreCookiePacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet.key, Identifier::minecraft("server_links"));
    assert_eq!(decoded_packet.payload, [1, 2, 3]);
}
