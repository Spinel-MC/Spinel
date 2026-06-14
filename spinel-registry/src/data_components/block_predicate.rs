use crate::blocks::{Block, BlockState};
use crate::data_components::nbt_reader::{
    compound_from_nbt, string_field, string_map_from_compound,
};
use crate::data_components::{DataComponentValue, RegistryTagReference};
use crate::{Identifier, Registries};
use spinel_nbt::{Nbt, NbtCompound};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BlockPredicates {
    predicates: Vec<BlockPredicate>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DataComponentPredicates;

#[derive(Clone, Debug, PartialEq)]
pub struct BlockPredicate {
    blocks: Option<RegistryTagReference>,
    state: Option<PropertiesPredicate>,
    nbt: Option<NbtCompound>,
    components: DataComponentPredicates,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PropertiesPredicate {
    properties: HashMap<String, PropertyValuePredicate>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PropertyValuePredicate {
    Exact(Option<String>),
    Range {
        min: Option<String>,
        max: Option<String>,
    },
}

impl BlockPredicates {
    #[must_use]
    pub fn new(predicates: Vec<BlockPredicate>) -> Self {
        Self { predicates }
    }

    #[must_use]
    pub fn predicates(&self) -> &[BlockPredicate] {
        &self.predicates
    }

    pub fn test(&self, block: Block, registries: &Registries) -> bool {
        self.test_state(block.default_state(), registries)
    }

    pub fn test_state(&self, block_state: BlockState, registries: &Registries) -> bool {
        self.test_state_with_nbt(block_state, None, registries)
    }

    pub fn test_state_with_nbt(
        &self,
        block_state: BlockState,
        client_nbt: Option<&NbtCompound>,
        registries: &Registries,
    ) -> bool {
        self.predicates
            .iter()
            .any(|predicate| predicate.test_state_with_nbt(block_state, client_nbt, registries))
    }
}

impl BlockPredicate {
    #[must_use]
    pub const fn new(
        blocks: Option<RegistryTagReference>,
        state: Option<PropertiesPredicate>,
        nbt: Option<NbtCompound>,
        components: DataComponentPredicates,
    ) -> Self {
        Self {
            blocks,
            state,
            nbt,
            components,
        }
    }

    #[must_use]
    pub const fn blocks(&self) -> Option<&RegistryTagReference> {
        self.blocks.as_ref()
    }

    #[must_use]
    pub const fn state(&self) -> Option<&PropertiesPredicate> {
        self.state.as_ref()
    }

    #[must_use]
    pub const fn nbt(&self) -> Option<&NbtCompound> {
        self.nbt.as_ref()
    }

    #[must_use]
    pub const fn components(&self) -> &DataComponentPredicates {
        &self.components
    }

    pub fn test(&self, block: Block, registries: &Registries) -> bool {
        self.test_state(block.default_state(), registries)
    }

    pub fn test_state(&self, block_state: BlockState, registries: &Registries) -> bool {
        self.test_state_with_nbt(block_state, None, registries)
    }

    pub fn test_state_with_nbt(
        &self,
        block_state: BlockState,
        client_nbt: Option<&NbtCompound>,
        registries: &Registries,
    ) -> bool {
        let block = block_state.block();
        if self
            .blocks
            .as_ref()
            .is_some_and(|blocks| !block_reference_contains(blocks, block, registries))
        {
            return false;
        }
        if self
            .state
            .as_ref()
            .is_some_and(|state| !state.test(block_state))
        {
            return false;
        }
        if self.nbt.as_ref() != client_nbt {
            return false;
        }
        true
    }
}

fn block_reference_contains(
    blocks: &RegistryTagReference,
    block: Block,
    registries: &Registries,
) -> bool {
    match blocks {
        RegistryTagReference::Backed(tag_name) => registries.block_tag_contains(tag_name, &block),
        RegistryTagReference::Direct(block_names) => registries
            .block_key(&block)
            .is_some_and(|block_key| identifiers_contain(block_names, block_key)),
        RegistryTagReference::Empty => false,
    }
}

fn identifiers_contain(identifiers: &[Identifier], target: &Identifier) -> bool {
    identifiers.iter().any(|identifier| identifier == target)
}

impl PropertiesPredicate {
    #[must_use]
    pub fn new(properties: HashMap<String, PropertyValuePredicate>) -> Self {
        Self { properties }
    }

    #[must_use]
    pub fn exact(key: String, value: String) -> Self {
        Self {
            properties: HashMap::from([(key, PropertyValuePredicate::Exact(Some(value)))]),
        }
    }

    #[must_use]
    pub fn properties(&self) -> &HashMap<String, PropertyValuePredicate> {
        &self.properties
    }

    pub fn test(&self, block_state: BlockState) -> bool {
        self.properties.iter().all(|(property, predicate)| {
            block_state
                .property(property)
                .is_some_and(|value| predicate.test(value))
        })
    }
}

impl PropertyValuePredicate {
    fn test(&self, value: &str) -> bool {
        match self {
            Self::Exact(Some(expected)) => value == expected,
            Self::Exact(None) => false,
            Self::Range { min, max } => {
                min.as_ref()
                    .is_none_or(|minimum| value_is_at_least(value, minimum))
                    && max
                        .as_ref()
                        .is_none_or(|maximum| value_is_at_most(value, maximum))
            }
        }
    }
}

fn value_is_at_least(value: &str, minimum: &str) -> bool {
    match (value.parse::<i64>(), minimum.parse::<i64>()) {
        (Ok(value), Ok(minimum)) => value >= minimum,
        _ => value >= minimum,
    }
}

fn value_is_at_most(value: &str, maximum: &str) -> bool {
    match (value.parse::<i64>(), maximum.parse::<i64>()) {
        (Ok(value), Ok(maximum)) => value <= maximum,
        _ => value <= maximum,
    }
}

impl DataComponentValue for BlockPredicates {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::List(
            self.predicates
                .iter()
                .map(BlockPredicate::to_nbt)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::List(predicates) => predicates
                .iter()
                .map(BlockPredicate::from_nbt)
                .collect::<Option<Vec<_>>>()
                .map(Self::new),
            Nbt::Compound(_) => BlockPredicate::from_nbt(component_nbt).map(|predicate| Self {
                predicates: vec![predicate],
            }),
            _ => None,
        }
    }
}

impl BlockPredicate {
    fn to_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        if let Some(blocks) = &self.blocks {
            compound.insert("blocks".to_string(), blocks.to_nbt());
        }
        if let Some(state) = &self.state {
            compound.insert("state".to_string(), state.to_nbt());
        }
        if let Some(nbt) = &self.nbt {
            compound.insert("nbt".to_string(), Nbt::Compound(nbt.clone()));
        }
        Nbt::Compound(compound)
    }

    fn from_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let blocks = match compound.get("blocks") {
            Some(blocks) => Some(RegistryTagReference::from_nbt(blocks)?),
            None => None,
        };
        let state = match compound.get("state") {
            Some(state) => Some(PropertiesPredicate::from_nbt(state)?),
            None => None,
        };
        let nbt = match compound.get("nbt") {
            Some(Nbt::Compound(nbt)) => Some(nbt.clone()),
            None => None,
            _ => return None,
        };
        Some(Self {
            blocks,
            state,
            nbt,
            components: DataComponentPredicates,
        })
    }
}

impl PropertiesPredicate {
    fn to_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        for (key, value) in &self.properties {
            compound.insert(key.clone(), value.to_nbt());
        }
        Nbt::Compound(compound)
    }

    fn from_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let exact_values = string_map_from_compound(compound);
        if let Some(exact_values) = exact_values {
            return Some(Self {
                properties: exact_values
                    .into_iter()
                    .map(|(key, value)| (key, PropertyValuePredicate::Exact(Some(value))))
                    .collect(),
            });
        }
        let properties = compound
            .0
            .iter()
            .map(|(key, value)| {
                PropertyValuePredicate::from_nbt(value).map(|value| (key.clone(), value))
            })
            .collect::<Option<HashMap<_, _>>>()?;
        Some(Self { properties })
    }
}

impl PropertyValuePredicate {
    fn to_nbt(&self) -> Nbt {
        match self {
            Self::Exact(Some(value)) => Nbt::String(value.clone()),
            Self::Exact(None) => Nbt::String(String::new()),
            Self::Range { min, max } => {
                let mut compound = NbtCompound::new();
                if let Some(min) = min {
                    compound.insert("min".to_string(), Nbt::String(min.clone()));
                }
                if let Some(max) = max {
                    compound.insert("max".to_string(), Nbt::String(max.clone()));
                }
                Nbt::Compound(compound)
            }
        }
    }

    fn from_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::String(value) => Some(Self::Exact(Some(value.clone()))),
            Nbt::Compound(compound) => Some(Self::Range {
                min: optional_string(compound, "min")?,
                max: optional_string(compound, "max")?,
            }),
            _ => None,
        }
    }
}

fn optional_string(compound: &NbtCompound, name: &str) -> Option<Option<String>> {
    match compound.get(name) {
        Some(_) => Some(Some(string_field(compound, name)?)),
        None => Some(None),
    }
}

impl DataComponentValue for DataComponentPredicates {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::Compound(NbtCompound::new())
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::Compound(_) | Nbt::End => Some(Self),
            _ => None,
        }
    }
}
