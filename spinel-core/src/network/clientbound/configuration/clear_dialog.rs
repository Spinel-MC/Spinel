use spinel_macros::packet;

#[packet(
    id: "clear_dialog",
    state: ConnectionState::Configuration,
    recipient: Recipient::Client
)]
pub struct ClearDialogPacket;
