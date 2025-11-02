pub mod client;
pub mod encoder;
pub mod server;
pub mod types;

pub use client::instance::{Client, ConnectionState};
pub use client::connection_manager::ConnectionManager;
pub use client::player::Player;

pub use inventory;
pub use tokio;
pub use spinel_utils::Priority;

pub struct PacketListener {
    pub id: i32,
    pub state: ConnectionState,
    pub events: &'static [&'static str],
    pub priority: Priority,
    pub handler: fn(&mut Client, server: *mut ()) -> bool,
    pub modules: &'static [&'static str], 
}

inventory::collect!(&'static PacketListener);

pub trait ServerContext: Send + Sync + 'static {
    fn on_startup(&mut self) -> bool;
    fn on_shutdown(&mut self);
    fn on_connection(&mut self, client: &mut Client) -> bool;
    fn on_disconnect(&mut self, addr: std::net::SocketAddr);
    fn connection_manager_mut(&mut self) -> &mut ConnectionManager;
    fn dispatch_packet(&mut self, packet_id: i32, client: &mut Client, payload: Vec<u8>) -> bool;
    fn has_listener_for(&self, packet_id: i32, state: &ConnectionState) -> bool;
}