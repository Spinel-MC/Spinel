use spinel_macros::packet;

#[packet(
    id: "change_difficulty",
    state: ConnectionState::Play,
    recipient: Recipient::Server
)]
pub struct ChangeDifficultyPacket {
    pub difficulty: u8,
}
