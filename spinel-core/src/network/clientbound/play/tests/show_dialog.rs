use super::super::show_dialog::ShowDialogPacket;
use spinel_network::DataType;
use spinel_registry::dialog::Dialog;

#[test]
fn show_dialog_packet_uses_minestom_holder_id_shape() {
    let packet = ShowDialogPacket::from_vanilla_dialog(&Dialog::QUICK_ACTIONS).unwrap();
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = ShowDialogPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(ShowDialogPacket::get_id(), 0x8a);
    assert_eq!(decoded_packet.dialog_id, 2);
}
