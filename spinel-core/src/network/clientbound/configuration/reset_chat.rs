use spinel_macros::packet;

#[packet(
    id: "reset_chat",
    state: ConnectionState::Configuration,
    recipient: Recipient::Client
)]
pub struct ResetChatPacket;
