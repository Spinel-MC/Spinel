use crate as spinel;
use spinel_macros::packet_dispatcher;
use spinel_network::types::alias::{Array, Identifier};

#[packet_dispatcher(id: 0x0C)]
pub struct FeatureFlagsPacket {
    pub feature_flags: Array<Identifier>,
}
