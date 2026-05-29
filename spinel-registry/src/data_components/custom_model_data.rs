use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::{
    bool_list_field_or_empty, compound_from_nbt, f32_list_field_or_empty, i32_list_field_or_empty,
    string_list_field_or_empty,
};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomModelData {
    floats: Vec<f32>,
    flags: Vec<bool>,
    strings: Vec<String>,
    colors: Vec<i32>,
}

impl CustomModelData {
    #[must_use]
    pub fn new(floats: Vec<f32>, flags: Vec<bool>, strings: Vec<String>, colors: Vec<i32>) -> Self {
        Self {
            floats,
            flags,
            strings,
            colors,
        }
    }

    #[must_use]
    pub fn floats(&self) -> &[f32] {
        &self.floats
    }

    #[must_use]
    pub fn flags(&self) -> &[bool] {
        &self.flags
    }

    #[must_use]
    pub fn strings(&self) -> &[String] {
        &self.strings
    }

    #[must_use]
    pub fn colors(&self) -> &[i32] {
        &self.colors
    }
}

impl DataComponentValue for CustomModelData {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        if !self.floats.is_empty() {
            compound.insert(
                "floats".to_string(),
                Nbt::List(
                    self.floats
                        .iter()
                        .copied()
                        .map(Nbt::Float)
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                ),
            );
        }
        if !self.flags.is_empty() {
            compound.insert(
                "flags".to_string(),
                Nbt::List(
                    self.flags
                        .iter()
                        .map(|flag| Nbt::Byte(i8::from(*flag)))
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                ),
            );
        }
        if !self.strings.is_empty() {
            compound.insert(
                "strings".to_string(),
                Nbt::List(
                    self.strings
                        .iter()
                        .cloned()
                        .map(Nbt::String)
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                ),
            );
        }
        if !self.colors.is_empty() {
            compound.insert(
                "colors".to_string(),
                Nbt::List(
                    self.colors
                        .iter()
                        .copied()
                        .map(Nbt::Int)
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                ),
            );
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        Some(Self {
            floats: f32_list_field_or_empty(compound, "floats")?,
            flags: bool_list_field_or_empty(compound, "flags")?,
            strings: string_list_field_or_empty(compound, "strings")?,
            colors: i32_list_field_or_empty(compound, "colors")?,
        })
    }
}
