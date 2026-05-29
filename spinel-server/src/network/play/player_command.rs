use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::player_command::PlayerCommandPacket;
use spinel_macros::packet_listener;

#[packet_listener]
fn on_player_command(
    client: &mut Client,
    packet: PlayerCommandPacket,
    server: &mut MinecraftServer,
) -> bool {
    match packet.action {
        PlayerCommandPacket::START_SPRINTING => {
            server.set_player_sprinting_in_world(client, true).is_ok()
        }
        PlayerCommandPacket::STOP_SPRINTING => {
            server.set_player_sprinting_in_world(client, false).is_ok()
        }
        PlayerCommandPacket::START_FLYING_ELYTRA => server
            .start_player_flying_with_elytra_in_world(client)
            .is_ok(),
        PlayerCommandPacket::LEAVE_BED
        | PlayerCommandPacket::START_JUMP_HORSE
        | PlayerCommandPacket::STOP_JUMP_HORSE
        | PlayerCommandPacket::OPEN_HORSE_INVENTORY => true,
        _ => false,
    }
}
