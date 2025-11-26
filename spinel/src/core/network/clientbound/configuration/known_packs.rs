use spinel_macros::packet_dispatcher;

use crate as spinel;

#[packet_dispatcher(id: "select_known_packs", state: ConnectionState::Configuration)]
pub struct KnownPacksPacket {
    pub known_packs: Vec<(String, String, String)>,
}

impl KnownPacksPacket {
    pub fn new(known_packs: Vec<(String, String, String)>) -> Self {
        Self { known_packs }
    }
}
