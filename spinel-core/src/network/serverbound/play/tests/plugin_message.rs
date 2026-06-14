use super::super::plugin_message::ServerboundPlayCustomPayloadPacket;
use spinel_network::DataType;
use std::io::{Cursor, Read};

#[test]
fn serverbound_play_plugin_message_decodes_raw_remaining_payload() {
    let mut payload = Vec::new();
    "minecraft:brand".to_string().encode(&mut payload).unwrap();
    payload.extend([1, 2, 3, 4]);

    let mut reader = Cursor::new(payload);
    let packet = ServerboundPlayCustomPayloadPacket::decode(&mut reader).unwrap();
    let mut remaining = Vec::new();
    reader.read_to_end(&mut remaining).unwrap();

    assert_eq!(packet.channel, "minecraft:brand");
    assert_eq!(packet.data, vec![1, 2, 3, 4]);
    assert!(remaining.is_empty());
    assert_eq!(ServerboundPlayCustomPayloadPacket::get_id(), 0x15);
}
