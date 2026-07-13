use crate::entity::player::PlayerViewerSnapshot;
use crate::network::client::instance::Client;
use spinel_core::network::clientbound::play::entity_status::EntityStatusPacket;
use spinel_core::network::clientbound::play::game_event::{GameEvent, GameEventPacket};
use spinel_core::network::clientbound::play::player_info_remove::PlayerInfoRemovePacket;
use spinel_core::network::clientbound::play::player_info_update::PlayerInfoUpdatePacket;
use spinel_core::network::clientbound::play::remove_entities::RemoveEntitiesPacket;
use spinel_core::network::clientbound::play::respawn::RespawnPacket;
use spinel_core::network::clientbound::play::server_difficulty::ServerDifficultyPacket;
use spinel_core::network::clientbound::play::set_experience::SetExperiencePacket;
use spinel_core::network::clientbound::play::set_health::SetHealthPacket;
use spinel_network::ConnectionState;
use spinel_network::types::{GlobalPos, Identifier, Position, TeleportFlags, Vector3d};
use std::io;
use uuid::Uuid;

use super::state::Player;

impl Player {
    pub(super) fn dispatch_player_info_update(
        &mut self,
        packet: PlayerInfoUpdatePacket,
    ) -> io::Result<()> {
        if !self.has_entered_world() {
            return Ok(());
        }
        let Some(client) = self.get_client_mut() else {
            return Ok(());
        };
        if client.state != ConnectionState::Play {
            return Ok(());
        }
        packet.dispatch(client)
    }

    pub(super) fn dispatch_to_viewer_clients(
        &mut self,
        mut dispatch_packet: impl FnMut(&mut Client) -> io::Result<()>,
    ) -> io::Result<()> {
        let viewer_ids = self.get_viewers();
        let Some(client_ptr) = self.client else {
            return Ok(());
        };
        let client = unsafe { &mut *(client_ptr as *mut Client) };
        let Some(server_ptr) = client.server_ptr else {
            return Ok(());
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        server
            .connection_manager
            .clients()
            .into_iter()
            .try_for_each(|viewer_client| {
                let Ok(mut viewer_client) = viewer_client.lock() else {
                    return Ok(());
                };
                let client_is_entity_viewer = viewer_client
                    .player_entity_id()
                    .is_some_and(|viewer_id| viewer_ids.contains(&viewer_id));
                if !client_is_entity_viewer || viewer_client.state != ConnectionState::Play {
                    return Ok(());
                }
                dispatch_packet(&mut viewer_client)
            })
    }

    pub(super) fn broadcast_to_play_clients(
        &mut self,
        mut dispatch_packet: impl FnMut(&mut Client) -> io::Result<()>,
    ) -> io::Result<()> {
        let Some(client_ptr) = self.client else {
            return Ok(());
        };
        let client = unsafe { &mut *(client_ptr as *mut Client) };
        let Some(server_ptr) = client.server_ptr else {
            if client.state != ConnectionState::Play {
                return Ok(());
            }
            return dispatch_packet(client);
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        server
            .connection_manager
            .clients()
            .into_iter()
            .try_for_each(|play_client| {
                let Ok(mut play_client) = play_client.lock() else {
                    return Ok(());
                };
                if play_client.state != ConnectionState::Play {
                    return Ok(());
                }
                dispatch_packet(&mut play_client)
            })
    }

    pub(super) fn refresh_skin_for_self(
        &mut self,
        player_uuid: Uuid,
        add_player_packet: PlayerInfoUpdatePacket,
        viewer_snapshot: &PlayerViewerSnapshot,
    ) -> io::Result<()> {
        let Some(client_ptr) = self.client else {
            return Ok(());
        };
        let client = unsafe { &mut *(client_ptr as *mut Client) };
        if client.state != ConnectionState::Play {
            return Ok(());
        }
        let mut respawn_packet = RespawnPacket::new(
            self.get_game_mode(),
            self.world_name
                .clone()
                .unwrap_or_else(|| Identifier::minecraft("overworld")),
        );
        respawn_packet.common_player_spawn_info.last_death_location =
            self.get_death_location().map(|death_location| GlobalPos {
                dimension: death_location.get_dimension().clone(),
                position: Position {
                    x: death_location.get_position().get_x().floor() as i32,
                    y: death_location.get_position().get_y().floor() as i32,
                    z: death_location.get_position().get_z().floor() as i32,
                },
            });
        respawn_packet.common_player_spawn_info.portal_cooldown = self.get_portal_cooldown();

        PlayerInfoRemovePacket::new(player_uuid).dispatch(client)?;
        RemoveEntitiesPacket::new(vec![self.get_entity_id().get_value()]).dispatch(client)?;
        add_player_packet.dispatch(client)?;
        respawn_packet.dispatch(client)?;
        GameEventPacket::from(GameEvent::StartWaitingForLevelChunks).dispatch(client)?;
        ServerDifficultyPacket::normal(false).dispatch(client)?;
        SetHealthPacket::new(
            self.get_health(),
            self.get_food(),
            self.get_food_saturation(),
        )
        .dispatch(client)?;
        SetExperiencePacket::new(
            self.get_experience(),
            self.get_experience_level(),
            self.get_total_experience(),
        )
        .dispatch(client)?;
        EntityStatusPacket {
            entity_id: self.get_entity_id().get_value(),
            status: (24 + self.get_permission_level()) as i8,
        }
        .dispatch(client)?;
        self.get_abilities_packet().dispatch(client)?;
        self.sync_inventory(client)?;
        self.synchronize_position_after_teleport(
            self.get_position(),
            Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            TeleportFlags::absolute(),
            true,
        )?;
        viewer_snapshot.dispatch_shared_state(client)
    }

    pub(super) fn dispatch_to_other_play_clients(
        &mut self,
        mut dispatch_packet: impl FnMut(&mut Client) -> io::Result<()>,
    ) -> io::Result<()> {
        let Some(client_ptr) = self.client else {
            return Ok(());
        };
        let client = unsafe { &mut *(client_ptr as *mut Client) };
        let Some(server_ptr) = client.server_ptr else {
            return Ok(());
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        server
            .connection_manager
            .clients()
            .into_iter()
            .try_for_each(|viewer_client| {
                let Ok(mut viewer_client) = viewer_client.lock() else {
                    return Ok(());
                };
                if viewer_client.addr == self.addr || viewer_client.state != ConnectionState::Play {
                    return Ok(());
                }
                dispatch_packet(&mut viewer_client)
            })
    }
}
