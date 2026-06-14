use spinel_macros::packet;

#[packet(
    id: "clear_dialog",
    state: spinel_network::ConnectionState::Play,
    recipient: spinel_network::Recipient::Client
)]
pub struct ClearDialogPacket;
