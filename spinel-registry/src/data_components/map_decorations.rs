use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::{
    compound_from_nbt, f32_field_or, f64_field_or, string_field,
};
use spinel_nbt::{Nbt, NbtCompound};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MapDecorations {
    decorations: HashMap<String, MapDecorationEntry>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MapDecorationEntry {
    decoration_type: String,
    x: f64,
    z: f64,
    rotation: f32,
}

impl MapDecorations {
    #[must_use]
    pub fn new(decorations: HashMap<String, MapDecorationEntry>) -> Self {
        Self { decorations }
    }

    #[must_use]
    pub fn decorations(&self) -> &HashMap<String, MapDecorationEntry> {
        &self.decorations
    }

    #[must_use]
    pub fn with_decoration(
        &self,
        id: String,
        decoration_type: String,
        x: f64,
        z: f64,
        rotation: f32,
    ) -> Self {
        self.with(id, MapDecorationEntry::new(decoration_type, x, z, rotation))
    }

    #[must_use]
    pub fn with(&self, id: String, entry: MapDecorationEntry) -> Self {
        let mut decorations = self.decorations.clone();
        decorations.insert(id, entry);
        Self { decorations }
    }

    #[must_use]
    pub fn remove(&self, id: &str) -> Self {
        let mut decorations = self.decorations.clone();
        decorations.remove(id);
        Self { decorations }
    }
}

impl MapDecorationEntry {
    #[must_use]
    pub fn new(decoration_type: String, x: f64, z: f64, rotation: f32) -> Self {
        Self {
            decoration_type,
            x,
            z,
            rotation,
        }
    }

    #[must_use]
    pub fn decoration_type(&self) -> &str {
        &self.decoration_type
    }

    #[must_use]
    pub const fn x(&self) -> f64 {
        self.x
    }

    #[must_use]
    pub const fn z(&self) -> f64 {
        self.z
    }

    #[must_use]
    pub const fn rotation(&self) -> f32 {
        self.rotation
    }
}

impl DataComponentValue for MapDecorations {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        for (id, entry) in &self.decorations {
            compound.insert(id.clone(), entry.to_nbt());
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let decorations = compound
            .0
            .iter()
            .map(|(id, entry)| MapDecorationEntry::from_nbt(entry).map(|entry| (id.clone(), entry)))
            .collect::<Option<HashMap<_, _>>>()?;
        Some(Self { decorations })
    }
}

impl MapDecorationEntry {
    fn to_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert(
            "type".to_string(),
            Nbt::String(self.decoration_type.clone()),
        );
        compound.insert("x".to_string(), Nbt::Double(self.x));
        compound.insert("z".to_string(), Nbt::Double(self.z));
        compound.insert("rotation".to_string(), Nbt::Float(self.rotation));
        Nbt::Compound(compound)
    }

    fn from_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        Some(Self {
            decoration_type: string_field(compound, "type")?,
            x: f64_field_or(compound, "x", 0.0)?,
            z: f64_field_or(compound, "z", 0.0)?,
            rotation: f32_field_or(compound, "rotation", 0.0)?,
        })
    }
}
