use super::super::show_dialog::ShowDialogPacket;
use spinel_network::data_type::DataType;

#[test]
fn show_dialog_packet_roundtrips() {
    let packet = ShowDialogPacket {
        encoded_dialog_payload: vec![1, 2, 3, 4, 5],
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = ShowDialogPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet, packet);
}
