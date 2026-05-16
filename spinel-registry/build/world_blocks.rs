use heck::ToShoutySnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use serde::Deserialize;
use std::{collections::BTreeMap, fs};

#[path = "world_block_matches.rs"]
mod world_block_matches;
use world_block_matches::{property_table_entries, state_table_entries};

pub(crate) const BLOCK_EXTRACTION_PATH: &str =
    "../../SpinelExtractor/run/spinel_extractor/blocks.json";

#[derive(Deserialize)]
struct BlockExtraction {
    blocks: BTreeMap<String, BlockDefinition>,
}

#[derive(Deserialize)]
pub(crate) struct BlockDefinition {
    id: i32,
    name: String,
    default_state_id: i32,
    states: BTreeMap<String, BlockStateDefinition>,
}

#[derive(Deserialize)]
pub(crate) struct BlockStateDefinition {
    id: i32,
    properties: BTreeMap<String, String>,
}

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed={BLOCK_EXTRACTION_PATH}");

    let block_extraction: BlockExtraction =
        serde_json::from_str(&fs::read_to_string(BLOCK_EXTRACTION_PATH).unwrap()).unwrap();

    let block_variants = block_extraction
        .blocks
        .values()
        .map(|block| Ident::new(&block.name.to_shouty_snake_case(), Span::call_site()))
        .collect::<Vec<_>>();

    let state_id_matches = block_extraction.blocks.values().map(|block| {
        let block_variant = Ident::new(&block.name.to_shouty_snake_case(), Span::call_site());
        let default_state_id = block.default_state_id;

        quote! {
            Self::#block_variant => #default_state_id
        }
    });

    let block_id_matches = block_extraction.blocks.values().map(|block| {
        let block_variant = Ident::new(&block.name.to_shouty_snake_case(), Span::call_site());
        let block_id = block.id;

        quote! {
            Self::#block_variant => #block_id
        }
    });

    let block_id_to_block_matches = block_extraction.blocks.values().map(|block| {
        let block_variant = Ident::new(&block.name.to_shouty_snake_case(), Span::call_site());
        let block_id = block.id;

        quote! {
            #block_id => Some(Self::#block_variant)
        }
    });

    let state_table_entries = block_extraction
        .blocks
        .values()
        .flat_map(state_table_entries);
    let property_table_entries = block_extraction
        .blocks
        .values()
        .flat_map(property_table_entries);

    quote! {
        struct BlockState {
            block_id: i32,
            state_id: i32,
            properties: &'static [(&'static str, &'static str)],
        }

        struct BlockProperty {
            block_id: i32,
            name: &'static str,
            value: &'static str,
        }

        const BLOCK_STATES: &[BlockState] = &[
            #(#state_table_entries),*
        ];

        const BLOCK_PROPERTIES: &[BlockProperty] = &[
            #(#property_table_entries),*
        ];

        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum Block {
            #(#block_variants,)*
            STATE {
                block_id: i32,
                state_id: i32,
            },
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum BlockPropertyError {
            UnknownProperty,
            UnknownValue,
        }

        impl Block {
            pub const fn state_id(self) -> i32 {
                match self {
                    #(#state_id_matches,)*
                    Self::STATE { state_id, .. } => state_id,
                }
            }

            pub const fn block_id(self) -> i32 {
                match self {
                    #(#block_id_matches,)*
                    Self::STATE { block_id, .. } => block_id,
                }
            }

            pub fn with_property(self, property: &str, value: &str) -> Result<Self, BlockPropertyError> {
                if !Self::has_property(self.block_id(), property) {
                    return Err(BlockPropertyError::UnknownProperty);
                }

                if !Self::has_property_value(self.block_id(), property, value) {
                    return Err(BlockPropertyError::UnknownValue);
                }

                let mut properties = self.properties().to_vec();
                properties
                    .iter_mut()
                    .filter(|block_property| block_property.0 == property)
                    .for_each(|block_property| block_property.1 = value);

                Self::from_properties(self.block_id(), &properties)
                    .ok_or(BlockPropertyError::UnknownValue)
            }

            pub fn with_properties(self, properties: &[(&str, &str)]) -> Result<Self, BlockPropertyError> {
                properties
                    .iter()
                    .try_fold(self, |block, (property, value)| block.with_property(property, value))
            }

            pub fn properties(self) -> &'static [(&'static str, &'static str)] {
                Self::properties_by_state_id(self.state_id())
            }

            pub fn get_property(self, property: &str) -> Option<&'static str> {
                self.properties()
                    .iter()
                    .find(|block_property| block_property.0 == property)
                    .map(|block_property| block_property.1)
            }

            pub fn default_state(self) -> Self {
                Self::from_block_id(self.block_id()).unwrap_or(self)
            }

            pub fn from_block_id(block_id: i32) -> Option<Self> {
                match block_id {
                    #(#block_id_to_block_matches,)*
                    _ => None,
                }
            }

            pub fn from_state_id(state_id: i32) -> Option<Self> {
                BLOCK_STATES
                    .iter()
                    .find(|block_state| block_state.state_id == state_id)
                    .and_then(|block_state| Self::from_state(block_state.block_id, block_state.state_id))
            }

            fn properties_by_state_id(state_id: i32) -> &'static [(&'static str, &'static str)] {
                BLOCK_STATES
                    .iter()
                    .find(|block_state| block_state.state_id == state_id)
                    .map(|block_state| block_state.properties)
                    .unwrap_or(&[])
            }

            fn has_property(block_id: i32, property: &str) -> bool {
                BLOCK_PROPERTIES
                    .iter()
                    .any(|block_property| block_property.block_id == block_id && block_property.name == property)
            }

            fn has_property_value(block_id: i32, property: &str, value: &str) -> bool {
                BLOCK_PROPERTIES
                    .iter()
                    .any(|block_property| {
                        block_property.block_id == block_id
                            && block_property.name == property
                            && block_property.value == value
                    })
            }

            fn from_properties(block_id: i32, properties: &[(&str, &str)]) -> Option<Self> {
                BLOCK_STATES
                    .iter()
                    .find(|block_state| {
                        block_state.block_id == block_id
                            && block_state.properties.len() == properties.len()
                            && block_state
                                .properties
                                .iter()
                                .all(|property| properties.contains(property))
                    })
                    .and_then(|block_state| Self::from_state(block_state.block_id, block_state.state_id))
            }

            fn from_state(block_id: i32, state_id: i32) -> Option<Self> {
                let default_state = Self::from_block_id(block_id)?;
                if default_state.state_id() == state_id {
                    return Some(default_state);
                }
                Some(Self::STATE { block_id, state_id })
            }
        }
    }
}
