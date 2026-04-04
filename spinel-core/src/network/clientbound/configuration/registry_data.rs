use spinel_macros::packet;
use spinel_nbt::NbtCompound;
use spinel_network::types::{Array, Identifier, Optional};

#[packet(
    id: "registry_data",
    state: ConnectionState::Configuration,
    recipient: Recipient::Client
)]
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
        let entries = ids.iter().map(|id| (id.parse().unwrap(), None)).collect();
        Self::new(registry_id.parse().unwrap(), Array(entries))
    }
}
