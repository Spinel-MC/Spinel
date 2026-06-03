use crate::events::network::outbound_packet::OutboundEventPacket;
use crate::events::network::outbound_packet_error::OutboundPacketErrorEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::clientbound::play::disconnect::PlayDisconnectPacket;
use spinel_network::PacketSender;
use spinel_network::packet_names::PacketNameRegistry;
use std::io;

impl PacketSender for Client {
    fn send_packet(&mut self, id: i32, payload: &[u8]) -> io::Result<()> {
        let packet_name = PacketNameRegistry::get_clientbound_packet_name(self.state, id);
        if self.should_queue_outbound_packet(id) {
            self.enqueue_outbound_packet(id, payload.to_vec());
            return Ok(());
        }
        self.dispatch_outbound_packet(id, &packet_name, payload.len());
        self.write_outbound_packet(id, payload, packet_name)
    }
}

impl Client {
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
        match self.send_packet_immediately(id, payload) {
            Ok(()) => Ok(()),
            Err(error) => {
                self.dispatch_outbound_packet_error(id, packet_name, error.to_string());
                Err(error)
            }
        }
    }

    pub(crate) fn dispatch_outbound_packet(
        &mut self,
        id: i32,
        packet_name: &str,
        payload_size: usize,
    ) {
        let Some(server_ptr) = self.server_ptr else {
            return;
        };
        let mut event =
            OutboundEventPacket::new(self.state, id, packet_name.to_string(), payload_size);
        let server = unsafe { &mut *(server_ptr as *mut MinecraftServer) };
        event.dispatch(server, self);
    }

    fn dispatch_outbound_packet_error(
        &mut self,
        packet_id: i32,
        packet_name: String,
        message: String,
    ) {
        let Some(server_ptr) = self.server_ptr else {
            return;
        };
        let mut event = OutboundPacketErrorEvent::new(self.state, packet_id, packet_name, message);
        let server = unsafe { &mut *(server_ptr as *mut MinecraftServer) };
        event.dispatch(server, self);
    }
}
