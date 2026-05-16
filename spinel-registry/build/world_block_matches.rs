use super::BlockDefinition;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn state_table_entries(block: &BlockDefinition) -> Vec<TokenStream> {
    if block.states.is_empty() {
        let block_id = block.id;
        let state_id = block.default_state_id;
        return vec![quote! {
            BlockState {
                block_id: #block_id,
                state_id: #state_id,
                properties: &[],
            }
        }];
    }
    block
        .states
        .values()
        .map(|state| {
            let block_id = block.id;
            let state_id = state.id;
            let properties = state
                .properties
                .iter()
                .map(|(property, value)| quote! { (#property, #value) });
            quote! {
                BlockState {
                    block_id: #block_id,
                    state_id: #state_id,
                    properties: &[#(#properties),*],
                }
            }
        })
        .collect()
}

pub(crate) fn property_table_entries(block: &BlockDefinition) -> Vec<TokenStream> {
    let block_id = block.id;
    block
        .states
        .values()
        .flat_map(|state| {
            state.properties.iter().map(move |(property, value)| {
                quote! {
                    BlockProperty {
                        block_id: #block_id,
                        name: #property,
                        value: #value,
                    }
                }
            })
        })
        .collect()
}
