use crate::network::serverbound::login::login_plugin_response::LoginPluginResponsePacket;
use spinel_network::DataType;
use std::io::Cursor;

#[test]
fn login_plugin_response_uses_optional_remaining_bytes_shape() {
    let response = LoginPluginResponsePacket {
        message_id: 300,
        data: Some(vec![1, 2, 3]),
    };
    let mut encoded_response = Vec::new();
    response.encode(&mut encoded_response).unwrap();

    assert_eq!(
        LoginPluginResponsePacket::decode(&mut Cursor::new(encoded_response)).unwrap(),
        response
    );

    let empty_response = LoginPluginResponsePacket {
        message_id: 1,
        data: None,
    };
    let mut encoded_empty_response = Vec::new();
    empty_response.encode(&mut encoded_empty_response).unwrap();

    assert_eq!(
        LoginPluginResponsePacket::decode(&mut Cursor::new(encoded_empty_response)).unwrap(),
        empty_response
    );
}
