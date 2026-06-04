mod application;
mod configuration;
mod disconnect;
mod dispatch;
mod events;
mod login;
mod play;
mod status;

pub use spinel::client::ClientPacketListener;

#[tokio::main]
async fn main() {
    application::TestClientApplication::new().run().await;
}
