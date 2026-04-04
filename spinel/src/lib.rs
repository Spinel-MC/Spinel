pub use spinel_core as core;
pub use spinel_events as events;
pub use spinel_macros as macros;
pub use spinel_nbt as nbt;
pub use spinel_registry as registry;
pub use spinel_utils as utils;

#[cfg(feature = "server")]
pub mod server {
    pub use spinel_server::MinecraftServer;
    pub use spinel_server::ServerPacketListener;
    pub use spinel_server::entity;
    pub use spinel_server::events;
    pub use spinel_server::module_manager;
    pub use spinel_server::modules;
    pub use spinel_server::network;
}

#[cfg(feature = "client")]
pub mod client {
    pub use spinel_client::ClientPacketListener;
    pub use spinel_client::MinecraftClient;
    pub use spinel_client::entity;
    pub use spinel_client::events;
    pub use spinel_client::module_manager;
    pub use spinel_client::network;
    pub use spinel_client::registry_manager;
}

#[cfg(any(feature = "server", feature = "client"))]
pub use uuid;

pub use spinel_macros::declare_namespace;

pub mod network {
    pub use spinel_network::*;

    #[cfg(feature = "server")]
    pub use spinel_server::network::client::instance::Client;

    #[cfg(feature = "client")]
    pub use spinel_client::network::server::instance::Server;
}

pub const IS_SERVER: bool = cfg!(feature = "server");
