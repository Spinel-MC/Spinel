use super::super::set_equipment::{EntityEquipmentEntry, EntityEquipmentSlot, SetEquipmentPacket};
use spinel_network::DataType;
use spinel_network::types::Slot;

#[test]
fn set_equipment_packet_uses_minestom_legacy_slot_continuation_bit() {
    let packet = SetEquipmentPacket::new(
        7,
        vec![
            EntityEquipmentEntry {
                slot: EntityEquipmentSlot::MainHand,
                item: Slot::default(),
            },
            EntityEquipmentEntry {
                slot: EntityEquipmentSlot::OffHand,
                item: Slot::default(),
            },
        ],
    );
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();

    assert_eq!(SetEquipmentPacket::get_id(), 0x64);
    assert_eq!(payload[1] as i8, -128);
    assert_eq!(payload[3], 1);
}
