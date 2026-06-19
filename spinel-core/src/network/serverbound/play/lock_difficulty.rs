use spinel_macros::packet;

#[packet(id: "lock_difficulty", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct LockDifficultyPacket {
    pub is_locked: bool,
}
