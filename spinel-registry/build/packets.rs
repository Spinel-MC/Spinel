use heck::ToShoutySnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Packets {
    serverbound: Direction,
    clientbound: Direction,
}

#[derive(Deserialize)]
struct Direction {
    handshake: Option<Vec<String>>,
    status: Option<Vec<String>>,
    login: Option<Vec<String>>,
    config: Option<Vec<String>>,
    play: Option<Vec<String>>,
}

pub fn build() -> TokenStream {
    let path = "build_assets/packets.json";
    let data = fs::read_to_string(path).expect("Failed to read packets.json");
    let packets: Packets = serde_json::from_str(&data).expect("Failed to parse packets.json");

    let serverbound = generate_direction(&packets.serverbound);
    let clientbound = generate_direction(&packets.clientbound);

    quote! {
        pub mod serverbound {
            #serverbound
        }

        pub mod clientbound {
            #clientbound
        }
    }
}

fn generate_direction(direction: &Direction) -> TokenStream {
    let handshake = generate_state(direction.handshake.as_ref());
    let status = generate_state(direction.status.as_ref());
    let login = generate_state(direction.login.as_ref());
    let config = generate_state(direction.config.as_ref());
    let play = generate_state(direction.play.as_ref());

    quote! {
        pub mod handshake {
            #handshake
        }
        pub mod status {
            #status
        }
        pub mod login {
            #login
        }
        pub mod config {
            #config
        }
        pub mod play {
            #play
        }
    }
}

fn generate_state(packets: Option<&Vec<String>>) -> TokenStream {
    if let Some(packets) = packets {
        let mut constants = TokenStream::new();
        for (id, name) in packets.iter().enumerate() {
            let const_name =
                Ident::new(&name.to_shouty_snake_case(), proc_macro2::Span::call_site());
            let id = id as i32;
            constants.extend(quote! {
                pub const #const_name: i32 = #id;
            });
        }
        constants
    } else {
        quote! {}
    }
}
