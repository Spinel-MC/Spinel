use spinel_macros::packet_listener;
use spinel_network::{Client, ConnectionState};

use crate::core::server::MinecraftServer;

use crate as spinel;

#[packet_listener(
        id: 0x02,
        state: ConnectionState::Configuration,
        module: "login",
        fields:(channel: Identifier, data: ByteArray)
)]
fn on_plugin_message(client: &mut Client, packet: Packet, server: &mut MinecraftServer) -> bool {
        //todo: I have no idea what to do here. I don't think it's important right now.
        true
}