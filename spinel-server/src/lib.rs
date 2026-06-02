pub mod command;
pub mod entity;
pub mod events;
pub mod inventory;
pub mod module_manager;
pub mod modules;
pub mod network;
pub mod registry_cache;
pub mod scheduler;
pub mod scoreboard;
pub mod server;
pub mod world;

mod listeners;

pub use entity::Player;
pub use server::MinecraftServer;
pub use world::World;

use spinel_network::ConnectionState;
use spinel_utils::Priority;

pub struct ServerPacketListener {
    pub id: i32,
    pub state: ConnectionState,
    pub events: &'static [&'static str],
    pub priority: Priority,
    pub handler: fn(&mut network::client::instance::Client, server: *mut ()) -> bool,
    pub modules: &'static [&'static str],
}

spinel_events::inventory::collect!(&'static ServerPacketListener);
