use super::super::set_held_slot::SetHeldSlotPacket;
use spinel_network::DataType;
use std::io::{Cursor, Read};

#[test]
fn set_held_slot_packet_matches_minestom_held_item_change_shape() {
    let mut payload = Vec::new();
    SetHeldSlotPacket { slot: 3 }.encode(&mut payload).unwrap();

    let mut reader = Cursor::new(payload);
    let decoded_packet = SetHeldSlotPacket::decode(&mut reader).unwrap();
    let mut remaining = Vec::new();
    reader.read_to_end(&mut remaining).unwrap();

    assert_eq!(SetHeldSlotPacket::get_id(), 0x67);
    assert_eq!(decoded_packet.slot, 3);
    assert!(remaining.is_empty());
}
