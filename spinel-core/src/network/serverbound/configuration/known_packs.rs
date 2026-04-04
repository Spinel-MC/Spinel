use spinel_macros::packet;

#[packet(
    id: "select_known_packs",
    state: ConnectionState::Configuration,
    recipient: Recipient::Server
)]
pub struct KnownPacksPacket {
    pub known_packs: Vec<(String, String, String)>,
}

impl KnownPacksPacket {
    pub fn new(known_packs: Vec<(String, String, String)>) -> Self {
        Self { known_packs }
    }
}
