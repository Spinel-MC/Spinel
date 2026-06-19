use spinel_macros::packet;

#[packet(
    id: "accept_code_of_conduct",
    state: ConnectionState::Configuration,
    recipient: Recipient::Server
)]
pub struct AcceptCodeOfConductPacket;
