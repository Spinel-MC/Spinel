use spinel_macros::packet;

#[packet(id: "set_display_objective", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct DisplayScoreboardPacket {
    pub position: i8,
    pub score_name: String,
}

impl DisplayScoreboardPacket {
    pub fn below_name(score_name: impl Into<String>) -> Self {
        Self {
            position: 2,
            score_name: score_name.into(),
        }
    }
}
