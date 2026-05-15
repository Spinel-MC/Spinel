use crate::server::MinecraftServer;
use crate::network::client::instance::Client;
use ::spinel_core::network::clientbound::play::chunk_data::ChunkDataAndUpdateLightPacket;
use ::spinel_core::network::clientbound::play::game_event::{GameEvent, GameEventPacket};
use ::spinel_core::network::clientbound::play::login_play::LoginPlayPacket;
use ::spinel_core::network::clientbound::play::set_chunk_cache_center::SetChunkCacheCenterPacket;
use ::spinel_core::network::clientbound::play::set_default_spawn_position::SetDefaultSpawnPositionPacket;
use ::spinel_core::network::clientbound::play::set_health::SetHealthPacket;
use ::spinel_core::network::clientbound::play::sync_player_pos::{
    SyncPlayerPositionPacket, SyncPlayerPositionSpec,
};
use ::spinel_core::network::serverbound::configuration::finish_configuration::FinishConfigurationPacket;
use ::spinel_macros::packet_listener;
use ::spinel_network::ConnectionState;
use ::spinel_network::types::{GlobalPos, Identifier, Position};
use std::io;

#[packet_listener(id: "finish_configuration", state: ConnectionState::Configuration)]
fn on_finish_configuration(
    client: &mut Client,
    _packet: FinishConfigurationPacket,
    _server: &mut MinecraftServer,
) -> bool {
    println!("Client acknowledged finish configuration. Transitioning to Play state.");
    client.state = ConnectionState::Play;
    dispatch_play_packets(client).is_ok()
}

fn dispatch_play_packets(client: &mut Client) -> io::Result<()> {
    LoginPlayPacket::new_default(41).dispatch(client)?;
    SetChunkCacheCenterPacket::new(0, 0).dispatch(client)?;
    SetDefaultSpawnPositionPacket::new(
        GlobalPos {
            dimension: Identifier::minecraft("overworld"),
            position: Position { x: 8, y: 64, z: 8 },
        },
        0.0,
        0.0,
    )
    .dispatch(client)?;
    SetHealthPacket::new(20.0, 20, 5.0).dispatch(client)?;
    GameEventPacket::from(GameEvent::StartWaitingForLevelChunks).dispatch(client)?;
    ChunkDataAndUpdateLightPacket::new_stub(0, 0).dispatch(client)?;
    SyncPlayerPositionPacket::new(SyncPlayerPositionSpec {
        teleport_id: 0,
        x: 8.5,
        y: 65.0,
        z: 8.5,
        yaw: 0.0,
        pitch: 0.0,
    })
    .dispatch(client)
}
