use crate::hit_target::TrackedEntity;
use crate::instance::MinecraftClient;
use crate::network::server::instance::Server;
use spinel_core::network::clientbound::play::block_update::BlockUpdatePacket;
use spinel_core::network::clientbound::play::chunk_data::ChunkDataAndUpdateLightPacket;
use spinel_core::network::clientbound::play::entity_position::EntityPositionPacket;
use spinel_core::network::clientbound::play::entity_position_and_rotation::EntityPositionAndRotationPacket;
use spinel_core::network::clientbound::play::entity_position_sync::EntityPositionSyncPacket;
use spinel_core::network::clientbound::play::entity_teleport::EntityTeleportPacket;
use spinel_core::network::clientbound::play::forget_level_chunk::ForgetLevelChunkPacket;
use spinel_core::network::clientbound::play::remove_entities::RemoveEntitiesPacket;
use spinel_core::network::clientbound::play::spawn_entity::SpawnEntityPacket;
use spinel_core::network::clientbound::play::sync_player_pos::SyncPlayerPositionPacket;
use spinel_core::network::serverbound::play::accept_teleportation::AcceptTeleportationPacket;
use spinel_macros::packet_listener;
use spinel_network::types::Vector3d;
use std::sync::Arc;

#[packet_listener(state: spinel_network::ConnectionState::Play)]
fn on_spawn_entity(
    _server: &mut Server,
    packet: SpawnEntityPacket,
    client: &mut MinecraftClient,
) -> bool {
    client.track_entity(TrackedEntity::new(
        packet.entity_id,
        packet.entity_type,
        Vector3d {
            x: packet.x,
            y: packet.y,
            z: packet.z,
        },
    ));
    true
}

#[packet_listener(state: spinel_network::ConnectionState::Play)]
fn on_remove_entities(
    _server: &mut Server,
    packet: RemoveEntitiesPacket,
    client: &mut MinecraftClient,
) -> bool {
    client.remove_tracked_entities(packet.entity_ids);
    true
}

#[packet_listener(state: spinel_network::ConnectionState::Play)]
fn on_entity_position(
    _server: &mut Server,
    packet: EntityPositionPacket,
    client: &mut MinecraftClient,
) -> bool {
    client.move_tracked_entity_by_protocol_delta(
        packet.entity_id,
        packet.delta_x,
        packet.delta_y,
        packet.delta_z,
    );
    true
}

#[packet_listener(state: spinel_network::ConnectionState::Play)]
fn on_entity_position_and_rotation(
    _server: &mut Server,
    packet: EntityPositionAndRotationPacket,
    client: &mut MinecraftClient,
) -> bool {
    client.move_tracked_entity_by_protocol_delta(
        packet.entity_id,
        packet.delta_x,
        packet.delta_y,
        packet.delta_z,
    );
    true
}

#[packet_listener(state: spinel_network::ConnectionState::Play)]
fn on_entity_teleport(
    _server: &mut Server,
    packet: EntityTeleportPacket,
    client: &mut MinecraftClient,
) -> bool {
    client.set_tracked_entity_position(packet.entity_id, packet.position);
    true
}

#[packet_listener(state: spinel_network::ConnectionState::Play)]
fn on_entity_position_sync(
    _server: &mut Server,
    packet: EntityPositionSyncPacket,
    client: &mut MinecraftClient,
) -> bool {
    client.set_tracked_entity_position(packet.entity_id, packet.position);
    true
}

#[packet_listener(state: spinel_network::ConnectionState::Play)]
fn on_block_update(
    _server: &mut Server,
    packet: BlockUpdatePacket,
    client: &mut MinecraftClient,
) -> bool {
    client.set_tracked_block_state(packet.block_position, packet.block_state);
    true
}

#[packet_listener(state: spinel_network::ConnectionState::Play)]
fn on_chunk_data(
    _server: &mut Server,
    packet: ChunkDataAndUpdateLightPacket,
    client: &mut MinecraftClient,
) -> bool {
    client.track_chunk(
        packet.chunk_x,
        packet.chunk_z,
        Arc::unwrap_or_clone(packet.chunk_data),
    );
    true
}

#[packet_listener(state: spinel_network::ConnectionState::Play)]
fn on_forget_level_chunk(
    _server: &mut Server,
    packet: ForgetLevelChunkPacket,
    client: &mut MinecraftClient,
) -> bool {
    client.remove_tracked_chunk(packet.pos.x, packet.pos.z);
    true
}

#[packet_listener(state: spinel_network::ConnectionState::Play)]
fn on_sync_player_position(
    server: &mut Server,
    packet: SyncPlayerPositionPacket,
    client: &mut MinecraftClient,
) -> bool {
    if (AcceptTeleportationPacket {
        id: packet.teleport_id,
    })
    .dispatch(server)
    .is_err()
    {
        return false;
    }
    client.synchronize_position(
        packet.x,
        packet.y,
        packet.z,
        packet.yaw,
        packet.pitch,
        false,
    );
    true
}
