mod application;
mod configuration;
mod dispatch;
mod login;
mod status;

pub use spinel::client::ClientPacketListener;

#[tokio::main]
async fn main() {
    application::TestClientApplication::new().run().await;
}
