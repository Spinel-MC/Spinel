use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::{compound_from_nbt, string_map_from_compound};
use spinel_nbt::{Nbt, NbtCompound};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DebugStickState {
    state: HashMap<String, String>,
}

impl DebugStickState {
    #[must_use]
    pub fn new(state: HashMap<String, String>) -> Self {
        Self { state }
    }

    #[must_use]
    pub fn state(&self) -> &HashMap<String, String> {
        &self.state
    }

    #[must_use]
    pub fn set(&self, key: String, value: String) -> Self {
        let mut state = self.state.clone();
        state.insert(key, value);
        Self { state }
    }

    #[must_use]
    pub fn remove(&self, key: &str) -> Self {
        let mut state = self.state.clone();
        state.remove(key);
        Self { state }
    }
}

impl DataComponentValue for DebugStickState {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        for (key, value) in &self.state {
            compound.insert(key.clone(), Nbt::String(value.clone()));
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        Some(Self {
            state: string_map_from_compound(compound_from_nbt(component_nbt)?)?,
        })
    }
}
