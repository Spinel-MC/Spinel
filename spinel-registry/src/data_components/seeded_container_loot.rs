use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::{compound_from_nbt, string_field};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeededContainerLoot {
    loot_table: String,
    seed: i64,
}

impl SeededContainerLoot {
    #[must_use]
    pub fn new(loot_table: String, seed: i64) -> Self {
        Self { loot_table, seed }
    }

    #[must_use]
    pub fn loot_table(&self) -> &str {
        &self.loot_table
    }

    #[must_use]
    pub const fn seed(&self) -> i64 {
        self.seed
    }
}

impl DataComponentValue for SeededContainerLoot {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert(
            "loot_table".to_string(),
            Nbt::String(self.loot_table.clone()),
        );
        compound.insert("seed".to_string(), Nbt::Long(self.seed));
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let seed = match compound.get("seed") {
            Some(Nbt::Long(seed)) => *seed,
            Some(Nbt::Int(seed)) => i64::from(*seed),
            _ => return None,
        };
        Some(Self {
            loot_table: string_field(compound, "loot_table")?,
            seed,
        })
    }
}
