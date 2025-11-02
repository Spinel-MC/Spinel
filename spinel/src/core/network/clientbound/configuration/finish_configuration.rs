use crate as spinel;
use spinel_macros::packet_dispatcher;

#[packet_dispatcher(id: 0x03, state: ConnectionState::Configuration)]
pub struct FinishConfigurationPacket;

impl FinishConfigurationPacket {
    pub fn new() -> Self {
            Self
    }
}

impl Default for FinishConfigurationPacket {
    fn default() -> Self {
        Self::new()
    }
}