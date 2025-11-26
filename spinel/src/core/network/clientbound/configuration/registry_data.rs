use crate as spinel;
use spinel_macros::packet_dispatcher;
use spinel_nbt::NbtCompound;
use spinel_network::types::alias::{Array, Identifier, Optional};

#[packet_dispatcher(id: "registry_data", state: ConnectionState::Configuration)]
#[derive(Clone)]
pub struct RegistryDataPacket {
    pub registry_id: Identifier,
    pub entries: Array<(Identifier, Optional<NbtCompound>)>,
}

impl RegistryDataPacket {
    pub fn new(
        registry_id: Identifier,
        entries: Array<(Identifier, Optional<NbtCompound>)>,
    ) -> Self {
        Self {
            registry_id,
            entries,
        }
    }

    pub fn from_identifiers(registry_id: String, ids: &[String]) -> Self {
        let entries = ids.iter().map(|id| (id.to_string(), None)).collect();
        Self::new(registry_id, Array(entries))
    }
}
