use crate as spinel;
use spinel_macros::packet_dispatcher;
use spinel_network::types::alias::{Array, Identifier};

#[packet_dispatcher(id: "update_enabled_features", state: ConnectionState::Configuration)]
pub struct FeatureFlagsPacket {
    pub feature_flags: Array<Identifier>,
}
