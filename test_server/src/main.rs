use std::sync::{Arc, Mutex};

use spinel::{
    macros::import_module,
    server::{
        MinecraftServer,
        events::signal::ServerSignal,
    },
};
pub mod events;

import_module!("minecraft:server_list_ping");
import_module!("minecraft:login");

#[tokio::main]
async fn main() {
    let server = Arc::new(Mutex::new(MinecraftServer::new()));
    println!("Starting Spinel Server on 127.0.0.1:25565");
    let mut server_task = tokio::spawn(MinecraftServer::start_shared(
        server.clone(),
        "127.0.0.1",
        25565,
    ));

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            {
                let Ok(mut server) = server.lock() else {
                    return;
                };
                server.on_signal(ServerSignal::CtrlC);
            }

            server_task.abort();
            let _ = server_task.await;
        }
        result = &mut server_task => {
            match result {
                Ok(()) => {}
                Err(error) => eprintln!("Server task failed: {}", error),
            }
        }
    }
}
