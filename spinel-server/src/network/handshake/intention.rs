use crate::events::intention::IntentionEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::handshake::intention::IntentionPacket;
use spinel_macros::fn_packet_listener;
use spinel_network::ConnectionState;
use spinel_utils::component::text::TextComponent;
use spinel_utils::constants::{MINECRAFT_VERSION, PROTOCOL_VERSION};

const STATUS_INTENTION_ID: i32 = 1;
const LOGIN_INTENTION_ID: i32 = 2;
const TRANSFER_INTENTION_ID: i32 = 3;

#[fn_packet_listener(id: "intention", state: ConnectionState::Handshaking)]
pub(super) fn on_intention(
    client: &mut Client,
    packet: IntentionPacket,
    server: &mut MinecraftServer,
) -> bool {
    if packet.intention == LOGIN_INTENTION_ID {
        client.state = ConnectionState::Login;
        if packet.protocol_version != PROTOCOL_VERSION as i32 {
            let _ = client.kick(outdated_client_message());
            return true;
        }
    }

    let mut event = IntentionEvent::new(
        packet.protocol_version,
        packet.server_address,
        packet.server_port,
        packet.intention,
    );

    event.dispatch(server, client);

    if event.cancelled {
        return true;
    }

    match packet.intention {
        STATUS_INTENTION_ID => client.state = ConnectionState::Status,
        LOGIN_INTENTION_ID => client.state = ConnectionState::Login,
        TRANSFER_INTENTION_ID => client.close_connection(),
        _ => {
            return false;
        }
    }

    true
}

fn outdated_client_message() -> TextComponent {
    TextComponent::literal(format!("Outdated client! Please use {MINECRAFT_VERSION}"))
}
