use spinel::{
    macros::event_listener,
    server::{
        MinecraftServer,
        events::signal::{ServerSignal, SignalEvent},
    },
    utils::Priority,
};

pub struct SignalInfoListener;

#[event_listener]
impl SignalInfoListener {
    #[event_handler(priority: Priority::High)]
    pub fn on_signal(event: &mut SignalEvent, server: &mut MinecraftServer) {
        if event.signal == ServerSignal::CtrlC {
            println!("Ctrl+C received. Shutting the server down...");
            server.stop();
        }
    }
}
