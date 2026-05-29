use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::{compound_from_nbt, i32_field_or};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Debug, PartialEq)]
pub struct Bee {
    entity_data: NbtCompound,
    ticks_in_hive: i32,
    min_ticks_in_hive: i32,
}

impl Bee {
    #[must_use]
    pub fn new(entity_data: NbtCompound, ticks_in_hive: i32, min_ticks_in_hive: i32) -> Self {
        Self {
            entity_data,
            ticks_in_hive,
            min_ticks_in_hive,
        }
    }

    #[must_use]
    pub fn entity_data(&self) -> &NbtCompound {
        &self.entity_data
    }

    #[must_use]
    pub const fn ticks_in_hive(&self) -> i32 {
        self.ticks_in_hive
    }

    #[must_use]
    pub const fn min_ticks_in_hive(&self) -> i32 {
        self.min_ticks_in_hive
    }

    #[must_use]
    pub fn with_entity_data(&self, entity_data: NbtCompound) -> Self {
        Self {
            entity_data,
            ticks_in_hive: self.ticks_in_hive,
            min_ticks_in_hive: self.min_ticks_in_hive,
        }
    }

    #[must_use]
    pub fn with_ticks_in_hive(&self, ticks_in_hive: i32) -> Self {
        Self {
            entity_data: self.entity_data.clone(),
            ticks_in_hive,
            min_ticks_in_hive: self.min_ticks_in_hive,
        }
    }

    #[must_use]
    pub fn with_min_ticks_in_hive(&self, min_ticks_in_hive: i32) -> Self {
        Self {
            entity_data: self.entity_data.clone(),
            ticks_in_hive: self.ticks_in_hive,
            min_ticks_in_hive,
        }
    }
}

impl DataComponentValue for Bee {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert(
            "entity_data".to_string(),
            Nbt::Compound(self.entity_data.clone()),
        );
        compound.insert("ticks_in_hive".to_string(), Nbt::Int(self.ticks_in_hive));
        compound.insert(
            "min_ticks_in_hive".to_string(),
            Nbt::Int(self.min_ticks_in_hive),
        );
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let entity_data = match compound.get("entity_data") {
            Some(Nbt::Compound(entity_data)) => entity_data.clone(),
            _ => return None,
        };
        Some(Self {
            entity_data,
            ticks_in_hive: i32_field_or(compound, "ticks_in_hive", 0)?,
            min_ticks_in_hive: i32_field_or(compound, "min_ticks_in_hive", 0)?,
        })
    }
}

impl DataComponentValue for Vec<Bee> {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::List(
            self.iter()
                .map(Bee::to_component_nbt)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::List(bees) => bees.iter().map(Bee::from_component_nbt).collect(),
            _ => None,
        }
    }
}
