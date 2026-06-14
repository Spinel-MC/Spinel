use crate::entity::Player;
use crate::events::outgoing_transfer::OutgoingTransferEvent;
use crate::server::MinecraftServer;
use spinel_core::network::clientbound::configuration::transfer::TransferPacket;
use std::io;

impl Player {
    pub fn transfer(&mut self, host: impl Into<String>, port: i32) -> io::Result<()> {
        let player = self as *mut Player;
        let Some(client) = self.client_mut() else {
            return Ok(());
        };
        let Some(server_ptr) = client.server_ptr else {
            return Ok(());
        };
        let server = unsafe { &mut *(server_ptr as *mut MinecraftServer) };
        let mut event = OutgoingTransferEvent::new(player, host, port);
        event.dispatch(server, client);
        if event.is_cancelled() {
            return Ok(());
        }
        let (host, port) = event.into_destination();
        TransferPacket { host, port }.dispatch(client)
    }
}
