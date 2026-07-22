use crate::events::player_game_rules_request::PlayerGameRulesRequestEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::client_command::ClientCommandPacket;
use spinel_macros::fn_packet_listener;

#[fn_packet_listener]
fn on_client_command(
    client: &mut Client,
    packet: ClientCommandPacket,
    server: &mut MinecraftServer,
) -> bool {
    if packet.action == ClientCommandPacket::REQUEST_GAMERULE_VALUES {
        let Some(player) = server.world_manager.player_pointer_for_client(client) else {
            return false;
        };

        PlayerGameRulesRequestEvent::new(player).dispatch(server, client);
        return true;
    }

    if packet.action != ClientCommandPacket::PERFORM_RESPAWN {
        return true;
    }

    let Some(world_uuid) = server.world_manager.world_uuid_for_client(client) else {
        return false;
    };
    let Some(world) = server.world_manager.world_mut(world_uuid) else {
        return false;
    };
    world.respawn_player(client).is_ok()
}
