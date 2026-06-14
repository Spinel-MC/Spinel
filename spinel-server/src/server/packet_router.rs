use crate::ServerPacketListener;
use crate::events::network::packet::PacketEvent;
use crate::events::network::packet_error::{PacketErrorEvent, PacketErrorStage};
use crate::events::player_packet::PlayerPacketEvent;
use crate::listeners::listeners;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_network::packet_names::PacketNameRegistry;
use spinel_network::{ConnectionState, PacketCodecRegistry, Recipient};
use std::collections::HashMap;
use std::io::Cursor;

pub struct PacketRouter {
    assigned_packet_listeners: HashMap<(ConnectionState, i32), Vec<&'static ServerPacketListener>>,
    generic_packet_listeners: HashMap<ConnectionState, Vec<&'static ServerPacketListener>>,
}

impl PacketRouter {
    pub fn new() -> Self {
        let (assigned_packet_listeners, generic_packet_listeners) = listeners();
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

    pub fn has_codec_for(&self, packet_id: i32, state: ConnectionState) -> bool {
        PacketCodecRegistry::contains(Recipient::Server, state, packet_id)
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
        let server = unsafe { &mut *server_pointer };
        let packet_name = PacketNameRegistry::get_serverbound_packet_name(client.state, packet_id);
        let Some(packet_codec) =
            PacketCodecRegistry::codec(Recipient::Server, client.state, packet_id)
        else {
            self.dispatch_packet_error(
                server,
                client,
                packet_id,
                packet_name,
                "received an undefined packet id".to_string(),
            );
            return false;
        };
        if let Err(error) = (packet_codec.decode)(&payload) {
            self.dispatch_packet_error(server, client, packet_id, packet_name, error.to_string());
            return false;
        }

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

        let mut packet_event = PacketEvent::new(
            Recipient::Server,
            client.state,
            packet_id,
            packet_name.clone(),
            payload.len(),
        );
        packet_event.dispatch(server, client);
        let payload = match self.dispatch_player_packet_event(
            server,
            client,
            packet_id,
            packet_name,
            payload,
        ) {
            Some(payload) => payload,
            None => return true,
        };
        if assigned_packet_listeners.is_empty() && generic_packet_listeners.is_empty() {
            return true;
        }
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

    fn dispatch_packet_error(
        &self,
        server: &mut MinecraftServer,
        client: &mut Client,
        packet_id: i32,
        packet_name: String,
        message: String,
    ) {
        let mut packet_error_event = PacketErrorEvent::new(
            Recipient::Server,
            PacketErrorStage::PacketDecode,
            client.state,
            Some(packet_id),
            Some(packet_name),
            message,
        );
        packet_error_event.dispatch(server, client);
    }

    fn dispatch_player_packet_event(
        &self,
        server: &mut MinecraftServer,
        client: &mut Client,
        packet_id: i32,
        packet_name: String,
        payload: Vec<u8>,
    ) -> Option<Vec<u8>> {
        if client.state != ConnectionState::Play {
            return Some(payload);
        }
        let Some(player) = server.world_manager.player_pointer_for_client(client) else {
            return Some(payload);
        };
        let mut event = PlayerPacketEvent::new(player, packet_id, packet_name, payload);
        event.dispatch(server, client);
        (!event.is_cancelled()).then(|| event.into_payload())
    }
}

impl Default for PacketRouter {
    fn default() -> Self {
        Self::new()
    }
}
