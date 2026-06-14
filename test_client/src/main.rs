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
    let Some(application) = application::TestClientApplication::new() else {
        println!("{}", application::REQUIRED_FLAGS);
        return;
    };

    application.run().await;
}
