use crate::entity::EntityPose;
use crate::events::player_game_mode_change::PlayerGameModeChangeEvent;
use crate::network::client::instance::Client;
use spinel_core::entity::game_mode::GameMode;
use spinel_core::network::clientbound::play::entity_status::EntityStatusPacket;
use spinel_core::network::clientbound::play::player_info_update::PlayerInfoUpdatePacket;
use spinel_network::ConnectionState;
use std::io;

use super::state::Player;

impl Player {
    pub fn is_online(&self) -> bool {
        self.get_client().is_some_and(Client::is_online)
    }

    pub const fn can_pickup_item(&self) -> bool {
        self.living.can_pickup_item()
    }

    pub fn set_can_pickup_item(&mut self, can_pickup_item: bool) {
        self.living.set_can_pickup_item(can_pickup_item);
    }

    pub const fn has_entity_collision(&self) -> bool {
        self.has_entity_collision
    }

    pub const fn can_prevent_block_placement(&self) -> bool {
        self.prevents_block_placement
    }

    pub const fn get_permission_level(&self) -> i32 {
        self.permission_level
    }

    pub fn set_permission_level(&mut self, permission_level: i32) -> io::Result<()> {
        self.permission_level = permission_level.clamp(0, 4);
        self.send_packet(EntityStatusPacket {
            entity_id: self.get_entity_id().get_value(),
            status: (24 + self.permission_level) as i8,
        })
    }

    pub fn set_game_mode(&mut self, game_mode: GameMode) -> bool {
        let player = self as *mut Player;
        let Some(client_ptr) = self.client else {
            self.apply_game_mode(game_mode);
            return true;
        };
        let client = unsafe { &mut *(client_ptr as *mut Client) };
        let final_game_mode = if let Some(server_ptr) = client.server_ptr {
            let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
            let mut event = PlayerGameModeChangeEvent::new(player, game_mode);
            event.dispatch(server, client);
            if event.is_cancelled() {
                return false;
            }
            event.new_game_mode()
        } else {
            game_mode
        };
        self.apply_game_mode(final_game_mode);
        if !self.has_entered_world() || client.state != ConnectionState::Play {
            return true;
        }
        let self_sync_succeeded = self.sync_game_mode_state(client).is_ok();
        let viewer_sync_succeeded = self.refresh_game_mode_to_viewers();
        self_sync_succeeded && viewer_sync_succeeded
    }

    pub fn get_game_mode(&self) -> GameMode {
        self.game_mode
    }

    pub const fn is_flying(&self) -> bool {
        self.flying
    }

    pub const fn can_fly(&self) -> bool {
        self.allow_flying
    }

    pub const fn has_instant_break(&self) -> bool {
        self.instant_break
    }

    pub const fn is_invulnerable(&self) -> bool {
        self.living.is_invulnerable()
    }

    pub const fn get_flying_speed(&self) -> f32 {
        self.flying_speed
    }

    pub const fn get_field_view_modifier(&self) -> f32 {
        self.field_view_modifier
    }

    pub fn set_flying(&mut self, flying: bool) -> io::Result<()> {
        self.set_flying_state(flying);
        self.refresh_abilities()
    }

    pub fn set_flying_state(&mut self, flying: bool) {
        self.flying = flying;
    }

    pub fn refresh_flying(&mut self, flying: bool) {
        if self.flying != flying {
            if self.is_sneaking() && self.get_pose() == EntityPose::Standing {
                self.set_pose(EntityPose::Sneaking);
            } else if self.get_pose() == EntityPose::Sneaking {
                self.set_pose(EntityPose::Standing);
            }
        }
        self.flying = flying;
    }

    pub fn set_allow_flying(&mut self, allow_flying: bool) -> io::Result<()> {
        self.allow_flying = allow_flying;
        self.refresh_abilities()
    }

    pub fn set_instant_break(&mut self, instant_break: bool) -> io::Result<()> {
        self.instant_break = instant_break;
        self.refresh_abilities()
    }

    pub fn set_invulnerable(&mut self, invulnerable: bool) -> io::Result<()> {
        self.living.set_invulnerable(invulnerable);
        self.refresh_abilities()
    }

    pub fn set_flying_speed(&mut self, flying_speed: f32) -> io::Result<()> {
        self.flying_speed = flying_speed;
        self.refresh_abilities()
    }

    pub fn set_field_view_modifier(&mut self, field_view_modifier: f32) -> io::Result<()> {
        self.field_view_modifier = field_view_modifier;
        self.refresh_abilities()
    }

    pub(super) fn apply_game_mode(&mut self, game_mode: GameMode) {
        self.game_mode = game_mode;
        self.allow_flying = game_mode.allows_flying();
        self.instant_break = game_mode.has_instant_break();
        self.living.set_invulnerable(game_mode.is_invulnerable());
        self.has_entity_collision = game_mode != GameMode::Spectator;
        self.prevents_block_placement = game_mode != GameMode::Spectator;
        if game_mode == GameMode::Spectator || !game_mode.allows_flying() {
            self.flying = game_mode.allows_flying();
        }
    }

    pub(super) fn sync_game_mode_state(&self, client: &mut Client) -> io::Result<()> {
        self.get_game_mode_packet().dispatch(client)?;
        PlayerInfoUpdatePacket::update_game_mode(self.uuid, self.game_mode).dispatch(client)?;
        self.get_abilities_packet().dispatch(client)
    }

    pub(super) fn refresh_abilities(&mut self) -> io::Result<()> {
        if !self.has_entered_world() {
            return Ok(());
        }
        let packet = self.get_abilities_packet();
        let Some(client) = self.get_client_mut() else {
            return Ok(());
        };
        if client.state != ConnectionState::Play {
            return Ok(());
        }
        packet.dispatch(client)
    }

    pub(super) fn refresh_game_mode_to_viewers(&mut self) -> bool {
        let packet = PlayerInfoUpdatePacket::update_game_mode(self.uuid, self.game_mode);
        self.dispatch_to_other_play_clients(|viewer_client| packet.clone().dispatch(viewer_client))
            .is_ok()
    }
}
