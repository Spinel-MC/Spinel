use crate::ServerPacketListener;
use crate::events::network::inbound_packet::InboundPacketEvent;
use crate::listeners::get_listeners;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_network::ConnectionState;
use spinel_network::packet_names::PacketNameRegistry;
use std::collections::HashMap;
use std::io::Cursor;

pub struct PacketRouter {
    assigned_packet_listeners: HashMap<(ConnectionState, i32), Vec<&'static ServerPacketListener>>,
    generic_packet_listeners: HashMap<ConnectionState, Vec<&'static ServerPacketListener>>,
}

impl PacketRouter {
    pub fn new() -> Self {
        let (assigned_packet_listeners, generic_packet_listeners) = get_listeners();
        Self {
            assigned_packet_listeners,
            generic_packet_listeners,
        }
    }

    pub fn has_listener_for(&self, packet_id: i32, state: &ConnectionState) -> bool {
        let packet_key = (*state, packet_id);
        self.assigned_packet_listeners.contains_key(&packet_key)
            || self.generic_packet_listeners.contains_key(state)
    }

    pub fn dispatch_packet(
        &self,
        server_pointer: *mut MinecraftServer,
        packet_id: i32,
        client: &mut Client,
        payload: Vec<u8>,
    ) -> bool {
        let packet_key = (client.state, packet_id);
        client.server_ptr = Some(server_pointer as usize);

        let assigned_packet_listeners = self
            .assigned_packet_listeners
            .get(&packet_key)
            .cloned()
            .unwrap_or_default();
        let generic_packet_listeners = self
            .generic_packet_listeners
            .get(&client.state)
            .cloned()
            .unwrap_or_default();

        if assigned_packet_listeners.is_empty() && generic_packet_listeners.is_empty() {
            return false;
        }

        let packet_name = PacketNameRegistry::get_serverbound_packet_name(client.state, packet_id);
        let mut inbound_packet_event =
            InboundPacketEvent::new(client.state, packet_id, packet_name, payload.len());
        let server = unsafe { &mut *server_pointer };
        inbound_packet_event.dispatch(server, client);
        client.payload_cursor = Some(Cursor::new(payload));

        for packet_listener in assigned_packet_listeners {
            if let Some(payload_cursor) = client.payload_cursor.as_mut() {
                payload_cursor.set_position(0);
            }
            (packet_listener.handler)(client, server_pointer as *mut ());
        }

        for packet_listener in generic_packet_listeners {
            if let Some(payload_cursor) = client.payload_cursor.as_mut() {
                payload_cursor.set_position(0);
            }
            (packet_listener.handler)(client, server_pointer as *mut ());
        }

        client.payload_cursor = None;
        true
    }
}

impl Default for PacketRouter {
    fn default() -> Self {
        Self::new()
    }
}
