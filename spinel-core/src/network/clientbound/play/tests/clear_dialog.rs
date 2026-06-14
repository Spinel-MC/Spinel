use super::super::clear_dialog::ClearDialogPacket;

#[test]
fn clear_dialog_uses_play_packet_id() {
    assert_eq!(ClearDialogPacket::get_id(), 0x89);
}
