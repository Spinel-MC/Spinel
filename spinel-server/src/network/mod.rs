pub mod configuration;
pub mod handshake;
pub mod login;
pub mod play;
pub mod status;

pub mod client;
pub mod connection_manager;
pub mod socket;

// Fix for macros expecting types in spinel::network::...
// spinel-server alias 'spinel' -> crate. crate::network is this module.
// We must expose Client here.
pub use client::instance::Client;

pub use spinel_network::types::{
    Array, Position, Slot, VarInt, VarLong, chunk::ChunkData, light::LightData,
};
pub use spinel_network::wrappers::{JsonTextComponent, NbtTextComponent};
pub use spinel_network::{ConnectionState, PacketSender, Recipient};
