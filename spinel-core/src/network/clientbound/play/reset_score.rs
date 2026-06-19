use spinel_macros::packet;

#[packet(id: "reset_score", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct ResetScorePacket {
    pub owner: String,
    pub objective_name: Option<String>,
}
