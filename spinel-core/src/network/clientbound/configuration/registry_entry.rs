use ::spinel_nbt::Nbt;
use ::spinel_network::encoder::NetworkBuffer;
use ::spinel_network::types::{Identifier, Optional};

#[derive(Clone, Debug)]
pub struct RegistryEntry {
    pub id: Identifier,
    pub data: Optional<Nbt>,
}

impl RegistryEntry {
    pub fn new(id: Identifier, data: Optional<Nbt>) -> Self {
        Self { id, data }
    }

    pub fn encode(&self, buffer: &mut NetworkBuffer) {
        buffer.encode(&self.id).unwrap();
        buffer.encode(&self.data.is_some()).unwrap();
        if let Some(data) = &self.data {
            data.write_unnamed(buffer).expect("Failed to write NBT");
        }
    }
}
