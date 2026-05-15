use spinel::{
    macros::event_listener,
    server::{
        MinecraftServer,
        events::signal::{ServerSignal, SignalEvent},
    },
    utils::Priority,
};

#[event_listener(priority: Priority::High)]
fn on_signal(event: &mut SignalEvent, server: &mut MinecraftServer) {
    if event.signal == ServerSignal::CtrlC {
        println!("Ctrl+C received. Shutting the server down...");
        server.stop();
    }
}
