use heck::ToShoutySnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use serde::Deserialize;
use serde_json::Value;
use std::fs;

#[allow(dead_code)]
#[derive(Deserialize)]
struct PacketField {
    name: String,
    #[serde(rename = "type")]
    field_type: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct PacketEntry {
    id: String,
    fields: Option<Vec<PacketField>>,
}

#[derive(Deserialize)]
struct Packets {
    #[allow(dead_code)]
    version: Option<u32>,
    #[allow(dead_code)]
    types: Option<std::collections::HashMap<String, Value>>,
    packets: Option<Directions>,
    serverbound: Option<Direction>,
    clientbound: Option<Direction>,
}

#[derive(Deserialize)]
struct Directions {
    serverbound: Direction,
    clientbound: Direction,
}

#[derive(Deserialize)]
struct Direction {
    handshake: Option<std::collections::HashMap<String, PacketEntry>>,
    status: Option<std::collections::HashMap<String, PacketEntry>>,
    login: Option<std::collections::HashMap<String, PacketEntry>>,
    config: Option<std::collections::HashMap<String, PacketEntry>>,
    configuration: Option<std::collections::HashMap<String, PacketEntry>>,
    play: Option<std::collections::HashMap<String, PacketEntry>>,
}

impl Packets {
    fn directions(&self) -> Option<(&Direction, &Direction)> {
        if let Some(packets) = &self.packets {
            return Some((&packets.serverbound, &packets.clientbound));
        }

        Some((self.serverbound.as_ref()?, self.clientbound.as_ref()?))
    }
}

pub struct PacketModuleBuilder;

impl PacketModuleBuilder {
    pub fn build() -> TokenStream {
        let Some(packets) = Self::read_packets() else {
            return quote! {};
        };
        let Some((serverbound_packets, clientbound_packets)) = packets.directions() else {
            return quote! {};
        };

        let serverbound = Self::generate_direction(serverbound_packets);
        let clientbound = Self::generate_direction(clientbound_packets);

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
        let handshake = Self::generate_state(direction.handshake.as_ref());
        let status = Self::generate_state(direction.status.as_ref());
        let login = Self::generate_state(direction.login.as_ref());
        let config = Self::generate_state(
            direction
                .config
                .as_ref()
                .or(direction.configuration.as_ref()),
        );
        let play = Self::generate_state(direction.play.as_ref());

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

    fn generate_state(
        packets: Option<&std::collections::HashMap<String, PacketEntry>>,
    ) -> TokenStream {
        let Some(packets) = packets else {
            return quote! {};
        };

        let mut constants = TokenStream::new();
        let mut packet_vec: Vec<_> = packets.iter().collect();
        packet_vec.sort_by_key(|(_, entry)| Self::parse_packet_id(&entry.id).unwrap_or_default());

        for (name, entry) in packet_vec {
            let Some(packet_id) = Self::parse_packet_id(&entry.id) else {
                continue;
            };
            let const_name =
                Ident::new(&name.to_shouty_snake_case(), proc_macro2::Span::call_site());
            constants.extend(quote! {
                pub const #const_name: i32 = #packet_id;
            });
        }

        constants
    }

    fn parse_packet_id(id: &str) -> Option<i32> {
        i32::from_str_radix(id.trim_start_matches("0x"), 16).ok()
    }

    fn read_packets() -> Option<Packets> {
        let data = fs::read_to_string("build_assets/packets.json").ok()?;
        serde_json::from_str(&data).ok()
    }
}
