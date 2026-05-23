use crate::Identifier;
use spinel_nbt::{Nbt, NbtCompound};
use spinel_utils::component::text::TextComponent;
use std::collections::BTreeMap;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataComponentType<T> {
    id: i32,
    key: Identifier,
    marker: PhantomData<T>,
}

impl<T> DataComponentType<T> {
    #[must_use]
    pub const fn new(id: i32, key: Identifier) -> Self {
        Self {
            id,
            key,
            marker: PhantomData,
        }
    }

    #[must_use]
    pub const fn id(&self) -> i32 {
        self.id
    }

    #[must_use]
    pub const fn key(&self) -> &Identifier {
        &self.key
    }
}

pub trait DataComponentValue: Clone + PartialEq {
    fn to_component_nbt(&self) -> Nbt;
    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self>;
}

#[derive(Clone, Debug, PartialEq)]
enum DataComponentPatch {
    Present(Nbt),
    Removed,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DataComponentMap {
    components: BTreeMap<i32, DataComponentPatch>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataComponentEntry {
    pub component_id: i32,
    pub component_key: Identifier,
    pub component_nbt: Nbt,
}

impl DataComponentMap {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn common_item_components() -> Self {
        Self::new()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    pub fn set<T>(&mut self, component: DataComponentType<T>, value: T)
    where
        T: DataComponentValue,
    {
        self.components.insert(
            component.id,
            DataComponentPatch::Present(value.to_component_nbt()),
        );
    }

    #[must_use]
    pub fn with<T>(&self, component: DataComponentType<T>, value: T) -> Self
    where
        T: DataComponentValue,
    {
        let mut components = self.clone();
        components.set(component, value);
        components
    }

    #[must_use]
    pub fn remove<T>(&self, component: DataComponentType<T>) -> Self {
        let mut components = self.clone();
        components
            .components
            .insert(component.id, DataComponentPatch::Removed);
        components
    }

    #[must_use]
    pub fn has<T>(&self, prototype: &DataComponentMap, component: DataComponentType<T>) -> bool {
        match self.components.get(&component.id) {
            Some(DataComponentPatch::Present(_)) => true,
            Some(DataComponentPatch::Removed) => false,
            None => prototype.has_without_fallback(component),
        }
    }

    #[must_use]
    pub fn get<T>(&self, prototype: &DataComponentMap, component: DataComponentType<T>) -> Option<T>
    where
        T: DataComponentValue,
    {
        match self.components.get(&component.id) {
            Some(DataComponentPatch::Present(component_nbt)) => {
                T::from_component_nbt(component_nbt)
            }
            Some(DataComponentPatch::Removed) => None,
            None => prototype.get_without_fallback(component),
        }
    }

    #[must_use]
    pub fn get_or<T>(
        &self,
        prototype: &DataComponentMap,
        component: DataComponentType<T>,
        default_value: T,
    ) -> T
    where
        T: DataComponentValue,
    {
        self.get(prototype, component).unwrap_or(default_value)
    }

    #[must_use]
    pub fn diff(prototype: &DataComponentMap, patch: &DataComponentMap) -> Self {
        let components = patch
            .components
            .iter()
            .filter_map(|(component_id, component_patch)| {
                let prototype_patch = prototype.components.get(component_id);
                match (component_patch, prototype_patch) {
                    (DataComponentPatch::Removed, None) => None,
                    (
                        DataComponentPatch::Present(component_nbt),
                        Some(DataComponentPatch::Present(prototype_nbt)),
                    ) if component_nbt == prototype_nbt => None,
                    _ => Some((*component_id, component_patch.clone())),
                }
            })
            .collect();
        Self { components }
    }

    #[must_use]
    pub fn entries(&self) -> Vec<DataComponentEntry> {
        self.components
            .iter()
            .filter_map(|(component_id, component_patch)| match component_patch {
                DataComponentPatch::Present(component_nbt) => {
                    DataComponentRegistry::from_id(*component_id).map(|component| {
                        DataComponentEntry {
                            component_id: *component_id,
                            component_key: component.key,
                            component_nbt: component_nbt.clone(),
                        }
                    })
                }
                DataComponentPatch::Removed => None,
            })
            .collect()
    }

    #[must_use]
    pub fn removed_component_ids(&self) -> Vec<i32> {
        self.components
            .iter()
            .filter_map(|(component_id, component_patch)| match component_patch {
                DataComponentPatch::Present(_) => None,
                DataComponentPatch::Removed => Some(*component_id),
            })
            .collect()
    }

    #[must_use]
    pub fn to_nbt_patch(&self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        for (component_id, component_patch) in &self.components {
            if let Some(component) = DataComponentRegistry::from_id(*component_id) {
                match component_patch {
                    DataComponentPatch::Present(component_nbt) => {
                        compound.insert(component.key.to_string(), component_nbt.clone());
                    }
                    DataComponentPatch::Removed => {
                        compound.insert(format!("!{}", component.key), NbtCompound::new());
                    }
                }
            }
        }
        compound
    }

    #[must_use]
    pub fn from_nbt_patch(compound: NbtCompound) -> Self {
        let components = compound
            .0
            .into_iter()
            .filter_map(|(component_key, component_nbt)| {
                let component_was_removed = component_key.starts_with('!');
                let normalized_component_key = component_key
                    .strip_prefix('!')
                    .unwrap_or(&component_key)
                    .to_string();
                DataComponentRegistry::from_key(&normalized_component_key).map(|component| {
                    let patch = match component_was_removed {
                        true => DataComponentPatch::Removed,
                        false => DataComponentPatch::Present(component_nbt),
                    };
                    (component.id, patch)
                })
            })
            .collect();
        Self { components }
    }

    fn has_without_fallback<T>(&self, component: DataComponentType<T>) -> bool {
        matches!(
            self.components.get(&component.id),
            Some(DataComponentPatch::Present(_))
        )
    }

    fn get_without_fallback<T>(&self, component: DataComponentType<T>) -> Option<T>
    where
        T: DataComponentValue,
    {
        match self.components.get(&component.id) {
            Some(DataComponentPatch::Present(component_nbt)) => {
                T::from_component_nbt(component_nbt)
            }
            Some(DataComponentPatch::Removed) | None => None,
        }
    }
}

#[derive(Clone)]
struct DataComponentRegistry {
    id: i32,
    key: Identifier,
}

impl DataComponentRegistry {
    fn from_id(id: i32) -> Option<Self> {
        DATA_COMPONENTS
            .iter()
            .find(|component| component.id == id)
            .cloned()
    }

    fn from_key(key: &str) -> Option<Self> {
        DATA_COMPONENTS
            .iter()
            .find(|component| component.key.to_string() == key)
            .cloned()
    }
}

const DATA_COMPONENTS: &[DataComponentRegistry] = &[
    DataComponentRegistry {
        id: 0,
        key: Identifier::vanilla_static("custom_data"),
    },
    DataComponentRegistry {
        id: 1,
        key: Identifier::vanilla_static("max_stack_size"),
    },
    DataComponentRegistry {
        id: 2,
        key: Identifier::vanilla_static("max_damage"),
    },
    DataComponentRegistry {
        id: 3,
        key: Identifier::vanilla_static("damage"),
    },
    DataComponentRegistry {
        id: 6,
        key: Identifier::vanilla_static("custom_name"),
    },
    DataComponentRegistry {
        id: 10,
        key: Identifier::vanilla_static("item_model"),
    },
    DataComponentRegistry {
        id: 11,
        key: Identifier::vanilla_static("lore"),
    },
    DataComponentRegistry {
        id: 21,
        key: Identifier::vanilla_static("enchantment_glint_override"),
    },
];

impl DataComponentValue for i32 {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::Int(*self)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::Int(value) => Some(*value),
            _ => None,
        }
    }
}

impl DataComponentValue for bool {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::Byte(i8::from(*self))
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::Byte(value) => Some(*value != 0),
            _ => None,
        }
    }
}

impl DataComponentValue for String {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::String(self.clone())
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::String(value) => Some(value.clone()),
            _ => None,
        }
    }
}

impl DataComponentValue for NbtCompound {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::Compound(self.clone())
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::Compound(value) => Some(value.clone()),
            _ => None,
        }
    }
}

impl DataComponentValue for TextComponent {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::String(self.to_json_string())
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::String(value) => serde_json::from_str(value).ok(),
            _ => None,
        }
    }
}

impl DataComponentValue for Vec<TextComponent> {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::List(
            self.iter()
                .map(|component| Nbt::String(component.to_json_string()))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::List(components) => components
                .iter()
                .map(|component| match component {
                    Nbt::String(value) => serde_json::from_str(value).ok(),
                    _ => None,
                })
                .collect(),
            _ => None,
        }
    }
}
