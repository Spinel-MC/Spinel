use heck::ToShoutySnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct PacketField {
    name: String,
    #[serde(rename = "type")]
    field_type: String,
}

#[derive(Deserialize)]
struct PacketEntry {
    id: String,
    fields: Option<Vec<PacketField>>,
}

#[derive(Deserialize)]
struct Packets {
    serverbound: Direction,
    clientbound: Direction,
}

#[derive(Deserialize)]
struct Direction {
    handshake: Option<std::collections::HashMap<String, PacketEntry>>,
    status: Option<std::collections::HashMap<String, PacketEntry>>,
    login: Option<std::collections::HashMap<String, PacketEntry>>,
    config: Option<std::collections::HashMap<String, PacketEntry>>,
    play: Option<std::collections::HashMap<String, PacketEntry>>,
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

fn generate_state(packets: Option<&std::collections::HashMap<String, PacketEntry>>) -> TokenStream {
    if let Some(packets) = packets {
        let mut constants = TokenStream::new();
        let mut packet_vec: Vec<_> = packets.iter().collect();
        packet_vec.sort_by_key(|(_, entry)| {
            i32::from_str_radix(entry.id.trim_start_matches("0x"), 16).unwrap_or(0)
        });

        for (name, entry) in packet_vec {
            let const_name =
                Ident::new(&name.to_shouty_snake_case(), proc_macro2::Span::call_site());
            let id = i32::from_str_radix(entry.id.trim_start_matches("0x"), 16)
                .expect(&format!("Failed to parse hex ID: {}", entry.id));
            constants.extend(quote! {
                pub const #const_name: i32 = #id;
            });
        }
        constants
    } else {
        quote! {}
    }
}
