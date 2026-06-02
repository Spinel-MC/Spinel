use spinel_macros::packet;

#[packet(id: "clear_titles", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct ClearTitlesPacket {
    pub reset_times: bool,
}

impl ClearTitlesPacket {
    pub const fn clear() -> Self {
        Self { reset_times: false }
    }

    pub const fn reset() -> Self {
        Self { reset_times: true }
    }
}
