use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::{compound_from_nbt, string_map_from_compound};
use spinel_nbt::{Nbt, NbtCompound};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ItemBlockState {
    properties: HashMap<String, String>,
}

impl ItemBlockState {
    #[must_use]
    pub fn new(properties: HashMap<String, String>) -> Self {
        Self { properties }
    }

    #[must_use]
    pub fn properties(&self) -> &HashMap<String, String> {
        &self.properties
    }

    #[must_use]
    pub fn with(&self, key: String, value: String) -> Self {
        let mut properties = self.properties.clone();
        properties.insert(key, value);
        Self { properties }
    }
}

impl DataComponentValue for ItemBlockState {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        for (key, value) in &self.properties {
            compound.insert(key.clone(), Nbt::String(value.clone()));
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        Some(Self {
            properties: string_map_from_compound(compound_from_nbt(component_nbt)?)?,
        })
    }
}
