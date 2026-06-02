use crate::data_components::DataComponentValue;
use crate::vanilla_world_blocks::Block;
use crate::{Identifier, Material, Registries};
use spinel_nbt::Nbt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegistryTagReference {
    Backed(Identifier),
    Direct(Vec<Identifier>),
    Empty,
}

impl DataComponentValue for RegistryTagReference {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = spinel_nbt::NbtCompound::new();
        compound.insert("items".to_string(), self.to_nbt());
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::Compound(compound) => Self::from_nbt(compound.get("items")?),
            _ => Self::from_nbt(component_nbt),
        }
    }
}

impl RegistryTagReference {
    #[must_use]
    pub const fn backed(key: Identifier) -> Self {
        Self::Backed(key)
    }

    #[must_use]
    pub fn direct(keys: Vec<Identifier>) -> Self {
        Self::Direct(keys)
    }

    #[must_use]
    pub const fn empty() -> Self {
        Self::Empty
    }

    #[must_use]
    pub fn from_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::String(value) => match value.strip_prefix('#') {
                Some(tag_key) => Some(Self::Backed(tag_key.parse().ok()?)),
                None => Some(Self::Direct(vec![value.parse().ok()?])),
            },
            Nbt::List(values) => values
                .iter()
                .map(|value| match value {
                    Nbt::String(value) => value.parse().ok(),
                    _ => None,
                })
                .collect::<Option<Vec<_>>>()
                .map(Self::Direct),
            _ => None,
        }
    }

    #[must_use]
    pub fn to_nbt(&self) -> Nbt {
        match self {
            Self::Backed(key) => Nbt::String(format!("#{key}")),
            Self::Direct(keys) => Nbt::List(
                keys.iter()
                    .map(|key| Nbt::String(key.to_string()))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
            Self::Empty => Nbt::List(Vec::new().into_boxed_slice()),
        }
    }

    pub fn contains_block(&self, block: Block, registries: &Registries) -> bool {
        match self {
            Self::Backed(tag_name) => registries.block_tag_contains(tag_name, &block),
            Self::Direct(block_names) => registries.block_key(&block).is_some_and(|block_key| {
                block_names.iter().any(|block_name| block_name == block_key)
            }),
            Self::Empty => false,
        }
    }

    pub fn contains_material(&self, material: &Material, registries: &Registries) -> bool {
        match self {
            Self::Backed(tag_name) => registries.item_tag_contains(tag_name, material),
            Self::Direct(material_names) => material_names
                .iter()
                .any(|material_name| material_name == material.key()),
            Self::Empty => false,
        }
    }
}
