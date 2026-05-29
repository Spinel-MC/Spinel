use crate::Identifier;
use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::{compound_from_nbt, string_field};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Debug, PartialEq)]
pub struct TypedCustomData {
    type_id: Identifier,
    nbt: NbtCompound,
}

impl TypedCustomData {
    #[must_use]
    pub fn new(type_id: Identifier, mut nbt: NbtCompound) -> Self {
        nbt.remove("id");
        Self { type_id, nbt }
    }

    #[must_use]
    pub const fn type_id(&self) -> &Identifier {
        &self.type_id
    }

    #[must_use]
    pub const fn nbt(&self) -> &NbtCompound {
        &self.nbt
    }

    #[must_use]
    pub fn with_nbt(&self, mut nbt: NbtCompound) -> Self {
        nbt.remove("id");
        Self {
            type_id: self.type_id.clone(),
            nbt,
        }
    }
}

impl DataComponentValue for TypedCustomData {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = self.nbt.clone();
        compound.insert("id".to_string(), Nbt::String(self.type_id.to_string()));
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        Some(Self::new(
            string_field(compound, "id")?.parse().ok()?,
            compound.clone(),
        ))
    }
}
