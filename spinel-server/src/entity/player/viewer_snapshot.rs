use super::Player;
use crate::network::client::instance::Client;
use spinel_core::network::clientbound::play::attach_entity::AttachEntityPacket;
use spinel_core::network::clientbound::play::entity_effect::EntityEffectPacket;
use spinel_core::network::clientbound::play::entity_head_look::EntityHeadLookPacket;
use spinel_core::network::clientbound::play::entity_velocity::EntityVelocityPacket;
use spinel_core::network::clientbound::play::player_info_update::PlayerInfoUpdatePacket;
use spinel_core::network::clientbound::play::set_entity_data::SetEntityDataPacket;
use spinel_core::network::clientbound::play::set_equipment::SetEquipmentPacket;
use spinel_core::network::clientbound::play::set_passengers::SetPassengersPacket;
use spinel_core::network::clientbound::play::spawn_entity::SpawnEntityPacket;
use spinel_core::network::clientbound::play::update_attributes::UpdateAttributesPacket;
use spinel_network::types::entity_metadata::MetadataEntry;
use spinel_registry::EntityType;
use std::io::Result;

pub(crate) struct PlayerViewerSnapshot {
    player_info_packet: Option<PlayerInfoUpdatePacket>,
    spawn_packet: SpawnEntityPacket,
    metadata_entries: Vec<MetadataEntry>,
    equipment_packet: SetEquipmentPacket,
    head_look_packet: EntityHeadLookPacket,
    velocity_packet: Option<EntityVelocityPacket>,
    attributes_packet: Option<UpdateAttributesPacket>,
    effect_packets: Vec<EntityEffectPacket>,
    passenger_packet: Option<SetPassengersPacket>,
}

impl PlayerViewerSnapshot {
    pub(crate) fn from_player(player: &Player) -> Self {
        Self {
            player_info_packet: (player.get_entity_type() == EntityType::PLAYER)
                .then(|| player.get_player_info_packet()),
            spawn_packet: player.spawn_packet(),
            metadata_entries: player.get_metadata_packet().entries.0,
            equipment_packet: player.get_visible_equipment_packet(),
            head_look_packet: player.get_head_look_packet(),
            velocity_packet: player.has_velocity().then(|| player.get_velocity_packet()),
            attributes_packet: (!player.get_attributes().is_empty())
                .then(|| player.update_attributes_packet()),
            effect_packets: player.get_effect_packets(),
            passenger_packet: player
                .has_passenger()
                .then(|| player.get_passenger_packet()),
        }
    }

    pub(crate) fn get_dispatch(&self, client: &mut Client) -> Result<()> {
        self.dispatch_with_leashes(client, Vec::new())
    }

    pub(crate) fn dispatch_without_player_info(&self, client: &mut Client) -> Result<()> {
        self.dispatch_packets(client, Vec::new(), false)
    }

    pub(crate) fn dispatch_shared_state(&self, client: &mut Client) -> Result<()> {
        if let Some(velocity_packet) = &self.velocity_packet {
            velocity_packet.clone().dispatch(client)?;
        }
        SetEntityDataPacket::new(self.spawn_packet.entity_id, self.metadata_entries.clone())
            .dispatch(client)?;
        if let Some(attributes_packet) = &self.attributes_packet {
            attributes_packet.clone().dispatch(client)?;
        }
        SetEquipmentPacket::new(
            self.spawn_packet.entity_id,
            self.equipment_packet.equipment.0.clone(),
        )
        .dispatch(client)
    }

    pub(crate) fn dispatch_with_leashes(
        &self,
        client: &mut Client,
        leash_packets: Vec<AttachEntityPacket>,
    ) -> Result<()> {
        self.dispatch_packets(client, leash_packets, true)
    }

    fn dispatch_packets(
        &self,
        client: &mut Client,
        leash_packets: Vec<AttachEntityPacket>,
        should_send_player_info: bool,
    ) -> Result<()> {
        if should_send_player_info && let Some(player_info_packet) = &self.player_info_packet {
            player_info_packet.clone().dispatch(client)?;
        }
        self.spawn_packet.clone().dispatch(client)?;
        if let Some(velocity_packet) = &self.velocity_packet {
            velocity_packet.clone().dispatch(client)?;
        }
        SetEntityDataPacket::new(self.spawn_packet.entity_id, self.metadata_entries.clone())
            .dispatch(client)?;
        leash_packets
            .into_iter()
            .try_for_each(|packet| packet.dispatch(client))?;
        EntityHeadLookPacket {
            entity_id: self.head_look_packet.entity_id,
            head_yaw: self.head_look_packet.head_yaw,
        }
        .dispatch(client)?;
        SetEquipmentPacket::new(
            self.spawn_packet.entity_id,
            self.equipment_packet.equipment.0.clone(),
        )
        .dispatch(client)?;
        if let Some(attributes_packet) = &self.attributes_packet {
            attributes_packet.clone().dispatch(client)?;
        }
        self.effect_packets
            .iter()
            .cloned()
            .try_for_each(|packet| packet.dispatch(client))?;
        if let Some(passenger_packet) = &self.passenger_packet {
            passenger_packet.clone().dispatch(client)?;
        }
        Ok(())
    }
}
