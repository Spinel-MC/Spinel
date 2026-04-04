use spinel_macros::packet;

#[packet(
    id: "status_request",
    state: ConnectionState::Status,
    recipient: Recipient::Server
)]
pub struct StatusRequestPacket;
