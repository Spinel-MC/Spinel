use spinel_macros::packet;

#[packet(
    id: "clear_dialog",
    state: spinel_network::ConnectionState::Play,
    recipient: spinel_network::Recipient::Client
)]
pub struct ClearDialogPacket;

#[cfg(test)]
mod tests {
    use super::ClearDialogPacket;

    #[test]
    fn clear_dialog_uses_play_packet_id() {
        assert_eq!(ClearDialogPacket::get_id(), 0x89);
    }
}
