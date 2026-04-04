pub mod entity;
pub mod events;
pub mod instance;
pub mod module_manager;
pub mod network;
pub mod registry_manager;

mod listeners;

pub use entity::player::Player;
pub use instance::MinecraftClient;

use spinel_network::ConnectionState;
use spinel_utils::Priority;

pub struct ClientPacketListener {
    pub id: i32,
    pub state: ConnectionState,
    pub events: &'static [&'static str],
    pub priority: Priority,
    pub handler: fn(&mut network::server::instance::Server, client: *mut ()) -> bool,
    pub modules: &'static [&'static str],
}

spinel_events::inventory::collect!(&'static ClientPacketListener);
