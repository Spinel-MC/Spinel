pub mod entity;
pub mod events;
pub mod instance;
pub mod module_manager;
pub mod modules;
pub mod network;
pub mod registry_cache;

mod listeners;

pub use entity::player::Player;
pub use instance::MinecraftServer;

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
