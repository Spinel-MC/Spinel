use spinel_macros::packet;

#[packet(
    id: "code_of_conduct",
    state: ConnectionState::Configuration,
    recipient: Recipient::Client
)]
pub struct CodeOfConductPacket {
    pub code_of_conduct: String,
}
