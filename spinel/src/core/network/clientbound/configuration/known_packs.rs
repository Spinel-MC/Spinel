use crate::{self as spinel, core::network::clientbound::configuration::known_packs};
use spinel_macros::packet_dispatcher;

#[packet_dispatcher(id: 0x0E)]
pub struct KnownPacksPacket {
    pub known_packs: Vec<(String, String, String)>
}

impl KnownPacksPacket {
    pub fn new(known_packs: Vec<(String, String, String)>) -> Self {
        Self {
            known_packs,
        }
    }
}