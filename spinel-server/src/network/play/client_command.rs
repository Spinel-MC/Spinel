use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::client_command::ClientCommandPacket;
use spinel_macros::packet_listener;
use spinel_network::types::{TeleportFlags, Vector3d};

#[packet_listener]
fn on_client_command(
    client: &mut Client,
    packet: ClientCommandPacket,
    server: &mut MinecraftServer,
) -> bool {
    if packet.action != ClientCommandPacket::PERFORM_RESPAWN {
        return true;
    }

    let Some(player) = server.world_manager.player_pointer_for_client(client) else {
        return false;
    };

    let player = unsafe { &mut *player };
    let Ok(player_respawned) = player.respawn() else {
        return false;
    };
    if !player_respawned {
        return true;
    }
    let player_position = player.get_position();
    player
        .synchronize_position_after_teleport(
            player_position,
            Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            TeleportFlags::absolute(),
            true,
        )
        .is_ok()
}
