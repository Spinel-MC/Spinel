mod codec;
mod generator;

use proc_macro::TokenStream;

pub fn packet_struct_logic(attr: TokenStream, item: TokenStream) -> TokenStream {
    generator::PacketDefinitionGenerator::from_tokens(attr, item)
        .and_then(|generator| generator.generate())
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}
