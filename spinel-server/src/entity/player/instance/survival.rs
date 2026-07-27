use crate::entity::metadata::definitions;
use crate::entity::player::position::PlayerPosition;
use crate::entity::{Damage, EntityPose, EntityPosition, PlayerSpawnPoint};
use crate::events::player_death::PlayerDeathEvent;
use crate::events::player_respawn::PlayerRespawnEvent;
use crate::network::client::instance::Client;
use spinel_core::network::clientbound::play::entity_status::EntityStatusPacket;
use spinel_core::network::clientbound::play::game_event::{
    GameEvent, GameEventPacket, RespawnScreenState,
};
use spinel_core::network::clientbound::play::player_combat_kill::PlayerCombatKillPacket;
use spinel_core::network::clientbound::play::respawn::RespawnPacket;
use spinel_core::network::clientbound::play::server_difficulty::ServerDifficultyPacket;
use spinel_core::network::clientbound::play::set_experience::SetExperiencePacket;
use spinel_core::network::clientbound::play::set_health::SetHealthPacket;
use spinel_core::network::clientbound::play::system_chat::SystemChatPacket;
use spinel_network::ConnectionState;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::{Identifier, Vector3d, Velocity};
use spinel_utils::component::text::TextComponent;
use std::io;

use super::death_location::PlayerDeathLocation;
use super::state::Player;

impl Player {
    pub const fn is_dead(&self) -> bool {
        self.living.is_dead()
    }

    pub fn get_last_damage(&self) -> Option<&Damage> {
        self.living.get_last_damage()
    }

    pub const fn get_health(&self) -> f32 {
        self.living.get_health()
    }

    pub const fn get_fire_ticks(&self) -> i32 {
        self.living.get_fire_ticks()
    }

    pub fn is_on_fire(&self) -> bool {
        self.metadata.get_flag(&definitions::is_on_fire())
    }

    pub fn set_fire_ticks(&mut self, fire_ticks: i32) {
        self.living.set_fire_ticks(fire_ticks);
        self.set_on_fire(self.living.get_fire_ticks() > 0);
    }

    pub(crate) fn set_fire_ticks_after_cancelled_extinguish(&mut self, fire_ticks: i32) {
        self.living.set_fire_ticks(fire_ticks);
    }

    pub(crate) fn tick_fire_ticks(&mut self) {
        self.living.tick_fire_ticks();
        self.set_on_fire(self.living.get_fire_ticks() > 0);
    }

    pub fn set_health(&mut self, health: f32) -> io::Result<()> {
        self.living.set_health(health);
        self.metadata.set(
            &definitions::living_entity::get_health(),
            MetadataValue::Float(self.living.get_health()),
        );
        self.sync_health()?;
        if self.living.get_health() <= 0.0 {
            self.kill()?;
        }
        Ok(())
    }

    pub(crate) fn apply_damage(&mut self, damage: Damage) -> io::Result<()> {
        let mut remaining_damage = damage.get_amount();
        let additional_hearts = self.get_additional_hearts();
        if additional_hearts > 0.0 {
            if remaining_damage > additional_hearts {
                remaining_damage -= additional_hearts;
                self.set_additional_hearts(0.0);
            } else {
                self.set_additional_hearts(additional_hearts - remaining_damage);
                remaining_damage = 0.0;
            }
        }
        self.living.store_last_damage(damage);
        self.living
            .set_health(self.living.get_health() - remaining_damage);
        self.metadata.set(
            &definitions::living_entity::get_health(),
            MetadataValue::Float(self.living.get_health()),
        );
        self.sync_health()
    }

    pub fn get_additional_hearts(&self) -> f32 {
        match self
            .metadata
            .get_value(&definitions::get_additional_hearts())
        {
            MetadataValue::Float(additional_hearts) => additional_hearts,
            _ => 0.0,
        }
    }

    pub fn set_additional_hearts(&mut self, additional_hearts: f32) {
        self.metadata.set(
            &definitions::get_additional_hearts(),
            MetadataValue::Float(additional_hearts.max(0.0)),
        );
    }

    pub const fn get_food(&self) -> i32 {
        self.food
    }

    pub fn set_food(&mut self, food: i32) -> io::Result<()> {
        self.food = food.clamp(0, 20);
        self.sync_health()
    }

    pub const fn get_food_saturation(&self) -> f32 {
        self.food_saturation
    }

    pub fn set_food_saturation(&mut self, food_saturation: f32) -> io::Result<()> {
        self.food_saturation = food_saturation.clamp(0.0, self.food as f32);
        self.sync_health()
    }

    pub fn set_death_location(&mut self, position: EntityPosition) {
        let dimension = self
            .world_name
            .clone()
            .unwrap_or_else(|| Identifier::minecraft("overworld"));
        self.death_location = Some(PlayerDeathLocation::new(dimension, position));
    }

    pub fn set_death_location_in_dimension(
        &mut self,
        dimension: Identifier,
        position: EntityPosition,
    ) {
        self.death_location = Some(PlayerDeathLocation::new(dimension, position));
    }

    pub fn get_death_location(&self) -> Option<&PlayerDeathLocation> {
        self.death_location.as_ref()
    }

    pub const fn is_respawn_screen_enabled(&self) -> bool {
        self.enable_respawn_screen
    }

    pub fn set_respawn_screen_enabled(&mut self, enable_respawn_screen: bool) -> io::Result<()> {
        self.enable_respawn_screen = enable_respawn_screen;
        let state = if enable_respawn_screen {
            RespawnScreenState::Enabled
        } else {
            RespawnScreenState::ImmediateRespawn
        };
        self.dispatch_game_event(GameEvent::SetRespawnScreen(state))
    }

    pub const fn get_experience(&self) -> f32 {
        self.experience
    }

    pub fn set_experience(&mut self, experience: f32) -> io::Result<()> {
        self.experience = experience.clamp(0.0, 1.0);
        self.sync_experience()
    }

    pub const fn get_experience_level(&self) -> i32 {
        self.experience_level
    }

    pub fn set_experience_level(&mut self, experience_level: i32) -> io::Result<()> {
        self.experience_level = experience_level.max(0);
        self.sync_experience()
    }

    pub const fn get_total_experience(&self) -> i32 {
        self.total_experience
    }

    pub fn set_total_experience(&mut self, total_experience: i32) -> io::Result<()> {
        self.total_experience = total_experience.max(0);
        self.sync_experience()
    }

    pub const fn get_portal_cooldown(&self) -> i32 {
        self.portal_cooldown
    }

    pub fn set_portal_cooldown(&mut self, portal_cooldown: i32) {
        self.portal_cooldown = portal_cooldown.max(0);
    }

    pub fn kill(&mut self) -> io::Result<()> {
        if self.living.is_dead() {
            return Ok(());
        }
        self.living.kill();
        self.set_pose(EntityPose::Dying);
        self.velocity = Velocity(Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        let generic_death_message = self.build_generic_death_message();
        let default_death_text = self
            .living
            .get_last_damage()
            .and_then(Damage::build_death_screen_text)
            .unwrap_or_else(|| generic_death_message.clone());
        let default_chat_message = self
            .living
            .get_last_damage()
            .and_then(|damage| damage.build_death_message(self.get_username()))
            .unwrap_or(generic_death_message);
        let (death_text, chat_message) =
            self.dispatch_player_death_event(default_death_text, default_chat_message);
        let entity_id = self.entity_id.get_value();
        if let Some(client) = self.get_client_mut() {
            if client.state == ConnectionState::Play
                && let Some(death_text) = death_text
            {
                PlayerCombatKillPacket::new(entity_id, death_text).dispatch(client)?;
            }
        }
        self.broadcast_death_message(chat_message)?;
        if self.current_world.is_some() {
            self.set_death_location(self.get_position());
        }
        Ok(())
    }

    pub(super) fn build_generic_death_message(&self) -> TextComponent {
        TextComponent::translatable("death.attack.generic")
            .argument(TextComponent::literal(self.get_username()))
            .build()
    }

    pub fn respawn(&mut self) -> io::Result<Option<EntityPosition>> {
        if !self.living.is_dead() {
            return Ok(None);
        }
        let respawn_dimension = self
            .world_name
            .clone()
            .unwrap_or_else(|| Identifier::minecraft("overworld"));
        let game_mode = self.game_mode;
        let permission_status = EntityStatusPacket {
            entity_id: self.get_entity_id().get_value(),
            status: (24 + self.get_permission_level()) as i8,
        };
        if let Some(client) = self.get_client_mut()
            && client.state == ConnectionState::Play
        {
            RespawnPacket::new(game_mode, respawn_dimension).dispatch(client)?;
            GameEventPacket::from(GameEvent::StartWaitingForLevelChunks).dispatch(client)?;
            ServerDifficultyPacket::normal(false).dispatch(client)?;
            SetHealthPacket::new(20.0, 20, 5.0).dispatch(client)?;
            SetExperiencePacket::new(0.0, 0, 0).dispatch(client)?;
            permission_status.dispatch(client)?;
            self.refresh_abilities()?;
        }
        let respawn_point = self.dispatch_player_respawn_event();
        self.living.revive();
        self.refresh_pose();
        let respawn_position = PlayerPosition::from(respawn_point);
        Ok(Some(EntityPosition::new(
            respawn_position.x,
            respawn_position.y,
            respawn_position.z,
            respawn_position.yaw,
            respawn_position.pitch,
        )))
    }

    pub(super) fn dispatch_player_death_event(
        &mut self,
        death_text: TextComponent,
        chat_message: TextComponent,
    ) -> (Option<TextComponent>, Option<TextComponent>) {
        let Some(client_ptr) = self.client else {
            return (Some(death_text), Some(chat_message));
        };
        let client = unsafe { &mut *(client_ptr as *mut Client) };
        let Some(server_ptr) = client.server_ptr else {
            return (Some(death_text), Some(chat_message));
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let mut event =
            PlayerDeathEvent::new(self as *mut Player, Some(death_text), Some(chat_message));
        event.dispatch(server, client);
        event.into_messages()
    }

    pub(super) fn dispatch_player_respawn_event(&mut self) -> PlayerSpawnPoint {
        let respawn_point = self.get_respawn_point();
        let Some(client_ptr) = self.client else {
            return respawn_point;
        };
        let client = unsafe { &mut *(client_ptr as *mut Client) };
        let Some(server_ptr) = client.server_ptr else {
            return respawn_point;
        };
        let server = unsafe { &mut *(server_ptr as *mut crate::server::MinecraftServer) };
        let mut event = PlayerRespawnEvent::new(self as *mut Player, respawn_point);
        event.dispatch(server, client);
        event.respawn_position()
    }

    pub(super) fn broadcast_death_message(
        &mut self,
        chat_message: Option<TextComponent>,
    ) -> io::Result<()> {
        let Some(chat_message) = chat_message else {
            return Ok(());
        };
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
            .try_for_each(|client_arc| {
                let Ok(mut viewer_client) = client_arc.lock() else {
                    return Ok(());
                };
                if viewer_client.state != ConnectionState::Play {
                    return Ok(());
                }
                SystemChatPacket::new(chat_message.clone(), false).dispatch(&mut *viewer_client)
            })
    }

    pub(in crate::entity::player) fn sync_health(&mut self) -> io::Result<()> {
        let packet =
            SetHealthPacket::new(self.living.get_health(), self.food, self.food_saturation);
        let Some(client) = self.get_client_mut() else {
            return Ok(());
        };
        if client.state != ConnectionState::Play {
            return Ok(());
        }
        packet.dispatch(client)
    }

    pub(super) fn sync_experience(&mut self) -> io::Result<()> {
        let packet = SetExperiencePacket::new(
            self.experience,
            self.experience_level,
            self.total_experience,
        );
        let Some(client) = self.get_client_mut() else {
            return Ok(());
        };
        if client.state != ConnectionState::Play {
            return Ok(());
        }
        packet.dispatch(client)
    }
}
