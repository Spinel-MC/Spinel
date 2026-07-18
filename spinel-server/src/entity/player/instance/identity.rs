use crate::entity::generic_entity::EntityAerodynamics;
use crate::entity::metadata::MetadataHolder;
use crate::entity::player::PlayerViewerSnapshot;
use crate::entity::player::skin::PlayerSkin;
use crate::entity::{
    EntityCollisionRules, EntityId, EntityIdentity, EntityPointers, EntityView, PlayerSnapshot,
};
use crate::world::WorldHandle;
use spinel_core::network::clientbound::play::player_info_remove::PlayerInfoRemovePacket;
use spinel_core::network::clientbound::play::player_info_update::PlayerInfoUpdatePacket;
use spinel_core::network::clientbound::play::remove_entities::RemoveEntitiesPacket;
use spinel_registry::EntityType;
use spinel_utils::component::events::{HoverEntity, HoverEvent};
use spinel_utils::component::text::TextComponent;
use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::net::SocketAddr;
use uuid::Uuid;

use super::state::Player;

impl Player {
    pub const fn get_entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub const fn get_entity_type(&self) -> EntityType {
        self.entity_type
    }

    pub fn switch_entity_type(&mut self, entity_type: EntityType) {
        self.entity_type = entity_type;
        self.metadata = MetadataHolder::default();
        self.aerodynamics = EntityAerodynamics::from_entity_type(entity_type);
        self.refresh_collision_rules();
    }

    pub(super) fn refresh_collision_rules(&mut self) {
        let collision_rules = EntityCollisionRules::from_entity_type(self.entity_type);
        self.has_entity_collision = collision_rules.has_entity_collision();
        self.prevents_block_placement = collision_rules.can_prevent_block_placement();
    }

    pub const fn get_view(&self) -> &EntityView {
        &self.view
    }

    pub const fn get_view_mut(&mut self) -> &mut EntityView {
        &mut self.view
    }

    pub fn get_viewers(&self) -> BTreeSet<EntityId> {
        self.view.get_viewers()
    }

    pub fn is_viewer(&self, viewer_id: EntityId) -> bool {
        self.view.is_viewer(viewer_id)
    }

    pub const fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub const fn get_identity(&self) -> EntityIdentity {
        EntityIdentity::new(self.uuid)
    }

    pub const fn get_pointers(&self) -> EntityPointers {
        EntityPointers::new(self.uuid, self.entity_id)
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn as_hover_event(&self) -> HoverEvent {
        HoverEvent::ShowEntity(HoverEntity {
            id: self.entity_type.key().to_string(),
            uuid: self.uuid,
            name: Some(Box::new(TextComponent::literal(self.get_username()))),
        })
    }

    pub fn update_snapshot(&self, updater: impl FnOnce(&mut PlayerSnapshot)) -> PlayerSnapshot {
        let mut snapshot = PlayerSnapshot::new(
            self.entity_id,
            self.uuid,
            self.username.clone(),
            self.get_position(),
            self.current_world,
            self.game_mode,
            self.skin.clone(),
            self.display_name.clone(),
            self.statistics
                .iter()
                .map(|(statistic, value)| (statistic.clone(), *value))
                .collect(),
        );
        updater(&mut snapshot);
        snapshot
    }

    pub fn get_statistic_value_map(&self) -> &BTreeMap<String, i32> {
        &self.statistics
    }

    pub fn get_statistic_value(&self, statistic: &str) -> i32 {
        self.statistics.get(statistic).copied().unwrap_or_default()
    }

    pub fn set_statistic_value(&mut self, statistic: impl Into<String>, value: i32) {
        self.statistics.insert(statistic.into(), value.max(0));
    }

    pub fn increment_statistic_value(&mut self, statistic: impl Into<String>, amount: i32) -> i32 {
        let statistic = statistic.into();
        let value = self
            .get_statistic_value(&statistic)
            .saturating_add(amount)
            .max(0);
        self.statistics.insert(statistic, value);
        value
    }

    pub fn get_skin(&self) -> Option<&PlayerSkin> {
        self.skin.as_ref()
    }

    pub fn set_skin(&mut self, skin: Option<PlayerSkin>) -> io::Result<()> {
        self.skin = skin;
        if !self.has_entered_world() {
            return Ok(());
        }
        let player_uuid = self.get_uuid();
        let player_id = self.get_entity_id();
        let add_player_packet = self.get_player_info_packet();
        let viewer_snapshot = PlayerViewerSnapshot::from_player(self);

        self.refresh_skin_for_self(player_uuid, add_player_packet.clone(), &viewer_snapshot)?;
        self.broadcast_to_play_clients(|client| {
            PlayerInfoRemovePacket::new(player_uuid).dispatch(client)
        })?;
        self.dispatch_to_viewer_clients(|client| {
            RemoveEntitiesPacket::new(vec![player_id.get_value()]).dispatch(client)
        })?;
        self.broadcast_to_play_clients(|client| add_player_packet.clone().dispatch(client))?;
        self.dispatch_to_viewer_clients(|client| {
            viewer_snapshot.dispatch_without_player_info(client)
        })
    }

    pub(crate) fn apply_skin(&mut self, skin: Option<PlayerSkin>) {
        self.skin = skin;
    }

    pub fn get_display_name(&self) -> Option<&TextComponent> {
        self.display_name.as_ref()
    }

    pub fn set_display_name(&mut self, display_name: Option<TextComponent>) -> io::Result<()> {
        self.display_name = display_name;
        let packet =
            PlayerInfoUpdatePacket::update_display_name(self.uuid, self.display_name.clone());
        self.broadcast_to_play_clients(|client| packet.clone().dispatch(client))
    }

    pub const fn get_protocol_version(&self) -> i32 {
        self.protocol_version
    }

    pub const fn get_address(&self) -> SocketAddr {
        self.addr
    }

    pub fn get_world(&self) -> Option<WorldHandle> {
        let client = self.get_client()?;
        let server = client.server_ptr?;
        let current_world = self.current_world?;
        Some(WorldHandle::new(server, current_world))
    }

    pub const fn is_listed(&self) -> bool {
        self.listed
    }

    pub const fn get_latency(&self) -> i32 {
        self.latency
    }

    pub fn set_listed(&mut self, listed: bool) -> io::Result<()> {
        self.listed = listed;
        let packet = PlayerInfoUpdatePacket::update_listed(self.uuid, listed);
        self.dispatch_player_info_update(packet)
    }

    pub fn refresh_latency(&mut self, latency: i32) -> io::Result<()> {
        self.latency = latency;
        let packet = PlayerInfoUpdatePacket::update_latency(self.uuid, latency);
        self.dispatch_player_info_update(packet)
    }
}
