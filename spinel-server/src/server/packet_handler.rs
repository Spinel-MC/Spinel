use crate::events::network::packet_error::{PacketErrorEvent, PacketErrorStage};
use crate::network::Client;
use crate::server::MinecraftServer;
use spinel_network::{PacketStruct, Recipient};
use spinel_utils::Priority;
use std::cmp::Reverse;
use std::collections::HashMap;

pub type ServerPacketHandler<P> = fn(&mut Client, P, &mut MinecraftServer) -> bool;

type ErasedServerPacketHandler = unsafe fn(usize, &mut Client, *mut MinecraftServer) -> bool;

#[derive(Clone, Copy)]
pub struct PacketHandlerEntry {
    priority: Priority,
    handler: usize,
    dispatch: ErasedServerPacketHandler,
}

pub struct GlobalPacketHandler {
    handlers: HashMap<(spinel_network::ConnectionState, i32), Vec<PacketHandlerEntry>>,
}

pub trait PacketHandler {
    fn register_packet_handlers(self, global_packet_handler: &mut GlobalPacketHandler);
}

impl GlobalPacketHandler {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn add_listener<P: PacketStruct + 'static>(&mut self, handler: ServerPacketHandler<P>) {
        self.add_listener_with_priority(Priority::Medium, handler);
    }

    pub fn add_listener_with_priority<P: PacketStruct + 'static>(
        &mut self,
        priority: Priority,
        handler: ServerPacketHandler<P>,
    ) {
        let packet_handlers = self
            .handlers
            .entry((P::get_state(), P::get_id()))
            .or_default();
        packet_handlers.push(PacketHandlerEntry {
            priority,
            handler: handler as usize,
            dispatch: dispatch_server_packet::<P>,
        });
        packet_handlers.sort_by_key(|packet_handler| Reverse(packet_handler.priority.to_order()));
    }

    pub fn has_listener_for(&self, state: spinel_network::ConnectionState, packet_id: i32) -> bool {
        self.handlers.contains_key(&(state, packet_id))
    }

    pub fn dispatch(&mut self, client: &mut Client, packet_id: i32, server: *mut MinecraftServer) {
        let packet_key = (client.state, packet_id);
        let Some(packet_handlers) = self.handlers.remove(&packet_key) else {
            return;
        };
        packet_handlers.iter().for_each(|packet_handler| {
            if let Some(payload_cursor) = client.payload_cursor.as_mut() {
                payload_cursor.set_position(0);
            }
            unsafe {
                (packet_handler.dispatch)(packet_handler.handler, client, server);
            }
        });
        self.handlers.insert(packet_key, packet_handlers);
    }
}

impl Default for GlobalPacketHandler {
    fn default() -> Self {
        Self::new()
    }
}

unsafe fn dispatch_server_packet<P: PacketStruct + 'static>(
    handler: usize,
    client: &mut Client,
    server: *mut MinecraftServer,
) -> bool {
    let Some(payload_cursor) = client.payload_cursor.as_mut() else {
        return false;
    };
    let packet = match P::decode(payload_cursor) {
        Ok(packet) => packet,
        Err(error) => {
            let server = unsafe { &mut *server };
            let mut packet_error_event = PacketErrorEvent::new(
                Recipient::Server,
                PacketErrorStage::PacketDecode,
                client.state,
                Some(P::get_id()),
                Some(std::any::type_name::<P>().to_string()),
                error.to_string(),
            );
            packet_error_event.dispatch(server, client);
            return false;
        }
    };
    let handler: ServerPacketHandler<P> = unsafe { std::mem::transmute(handler) };
    handler(client, packet, unsafe { &mut *server })
}
