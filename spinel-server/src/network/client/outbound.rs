use crate::events::network::packet::PacketEvent;
use crate::events::network::packet_error::{PacketErrorEvent, PacketErrorStage};
use crate::events::player_packet_out::PlayerPacketOutEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::clientbound::play::disconnect::PlayDisconnectPacket;
use spinel_network::packet_names::PacketNameRegistry;
use spinel_network::{PacketSender, Recipient};
use std::io;

impl PacketSender for Client {
    fn send_packet(&mut self, id: i32, payload: &[u8]) -> io::Result<()> {
        if !self.is_online() {
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "connection is closed",
            ));
        }
        let packet_name = PacketNameRegistry::get_clientbound_packet_name(self.state, id);
        if self.should_queue_outbound_packet(id) {
            self.enqueue_outbound_packet(id, payload.to_vec());
            return Ok(());
        }
        if !self.dispatch_outbound_packet(id, &packet_name, payload.len()) {
            return Ok(());
        }
        self.write_outbound_packet(id, payload, packet_name)
    }
}

impl Client {
    pub(crate) fn dispatch_queued_outbound_packet(
        &mut self,
        packet_id: i32,
        payload: &[u8],
    ) -> io::Result<()> {
        let packet_name = PacketNameRegistry::get_clientbound_packet_name(self.state, packet_id);
        if !self.dispatch_outbound_packet(packet_id, &packet_name, payload.len()) {
            return Ok(());
        }
        self.write_outbound_packet(packet_id, payload, packet_name)
    }

    fn should_queue_outbound_packet(&self, id: i32) -> bool {
        self.state == spinel_network::ConnectionState::Play
            && self.outbound_packet_queue_enabled()
            && id != PlayDisconnectPacket::get_id()
    }

    fn write_outbound_packet(
        &mut self,
        id: i32,
        payload: &[u8],
        packet_name: String,
    ) -> io::Result<()> {
        match self.send_packet_immediately(id, payload, packet_name.clone()) {
            Ok(()) => Ok(()),
            Err(error) => {
                self.dispatch_packet_error(id, packet_name, error.to_string());
                Err(error)
            }
        }
    }

    pub(crate) fn dispatch_outbound_packet(
        &mut self,
        id: i32,
        packet_name: &str,
        payload_size: usize,
    ) -> bool {
        let Some(server_ptr) = self.server_ptr else {
            return true;
        };
        let mut event = PacketEvent::new(
            Recipient::Client,
            self.state,
            id,
            packet_name.to_string(),
            payload_size,
        );
        let server = unsafe { &mut *(server_ptr as *mut MinecraftServer) };
        event.dispatch(server, self);
        if self.state != spinel_network::ConnectionState::Play {
            return true;
        }
        let Some(player) = server.world_manager.player_pointer_for_client(self) else {
            return true;
        };
        let mut player_event =
            PlayerPacketOutEvent::new(player, id, packet_name.to_string(), payload_size);
        player_event.dispatch(server, self);
        !player_event.is_cancelled()
    }

    pub(crate) fn dispatch_packet_error(
        &mut self,
        packet_id: i32,
        packet_name: String,
        message: String,
    ) {
        let Some(server_ptr) = self.server_ptr else {
            return;
        };
        let mut event = PacketErrorEvent::new(
            Recipient::Client,
            PacketErrorStage::PacketWrite,
            self.state,
            Some(packet_id),
            Some(packet_name),
            message,
        );
        let server = unsafe { &mut *(server_ptr as *mut MinecraftServer) };
        event.dispatch(server, self);
    }
}
