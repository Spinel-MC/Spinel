use crate::entity::EquipmentSlot;
use spinel_core::network::clientbound::play::entity_animation::{
    EntityAnimation, EntityAnimationPacket,
};
use spinel_core::network::clientbound::play::entity_head_look::EntityHeadLookPacket;
use spinel_core::network::clientbound::play::game_event::{GameEvent, GameEventPacket};
use spinel_core::network::clientbound::play::player_abilities::PlayerAbilitiesPacket;
use spinel_core::network::clientbound::play::player_info_update::{
    PlayerInfoActions, PlayerInfoEntry, PlayerInfoUpdatePacket,
};
use spinel_core::network::clientbound::play::set_equipment::{
    EntityEquipmentEntry, SetEquipmentPacket,
};
use spinel_core::network::clientbound::play::spawn_entity::{EntityAngle, SpawnEntityPacket};
use spinel_network::types::{Array, Slot, Vector3d, Velocity};

use super::constants::*;
use super::hand::PlayerHand;
use super::state::Player;

impl Player {
    pub(crate) fn get_player_info_packet(&self) -> PlayerInfoUpdatePacket {
        let properties = self
            .skin
            .as_ref()
            .map(|skin| vec![skin.get_property()])
            .unwrap_or_default();
        let display_hat = self.settings.displayed_skin_parts & HAT_DISPLAYED_SKIN_PART_MASK
            == HAT_DISPLAYED_SKIN_PART_MASK;
        PlayerInfoUpdatePacket {
            actions: PlayerInfoActions::all(),
            entries: Array(vec![PlayerInfoEntry {
                uuid: self.uuid,
                username: self.username.clone(),
                properties,
                listed: self.listed,
                latency: self.latency,
                game_mode: self.game_mode,
                display_name: self.display_name.clone(),
                list_order: 0,
                display_hat,
            }]),
        }
    }

    pub(crate) fn get_game_mode_packet(&self) -> GameEventPacket {
        GameEventPacket::from(GameEvent::ChangeGameMode(self.game_mode))
    }

    pub(crate) fn get_abilities_packet(&self) -> PlayerAbilitiesPacket {
        let mut flags = 0;
        if self.living.is_invulnerable() {
            flags |= PlayerAbilitiesPacket::INVULNERABLE;
        }
        if self.flying {
            flags |= PlayerAbilitiesPacket::FLYING;
        }
        if self.allow_flying {
            flags |= PlayerAbilitiesPacket::ALLOW_FLYING;
        }
        if self.instant_break {
            flags |= PlayerAbilitiesPacket::INSTANT_BREAK;
        }
        PlayerAbilitiesPacket::new(flags, self.flying_speed, self.field_view_modifier)
    }

    pub(crate) fn spawn_packet(&self) -> SpawnEntityPacket {
        SpawnEntityPacket {
            entity_id: self.get_entity_id().get_value(),
            uuid: self.uuid,
            entity_type: self.entity_type.id(),
            x: self.position.x,
            y: self.position.y,
            z: self.position.z,
            velocity: Velocity(Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            pitch: EntityAngle(self.position.pitch),
            yaw: EntityAngle(self.position.yaw),
            head_yaw: EntityAngle(self.position.yaw),
            data: 0,
        }
    }

    pub(crate) fn get_visible_equipment_packet(&self) -> SetEquipmentPacket {
        SetEquipmentPacket::new(
            self.get_entity_id().get_value(),
            vec![
                self.equipment_entry(EquipmentSlot::MainHand),
                self.equipment_entry(EquipmentSlot::OffHand),
                self.equipment_entry(EquipmentSlot::Boots),
                self.equipment_entry(EquipmentSlot::Leggings),
                self.equipment_entry(EquipmentSlot::Chestplate),
                self.equipment_entry(EquipmentSlot::Helmet),
                self.equipment_entry(EquipmentSlot::Body),
            ],
        )
    }

    pub(super) fn equipment_entry(&self, equipment_slot: EquipmentSlot) -> EntityEquipmentEntry {
        EntityEquipmentEntry {
            slot: equipment_slot.get_entity_equipment_slot(),
            item: Slot::from_item_stack(&self.get_equipment(equipment_slot)),
        }
    }

    pub(crate) fn get_head_look_packet(&self) -> EntityHeadLookPacket {
        EntityHeadLookPacket {
            entity_id: self.get_entity_id().get_value(),
            head_yaw: EntityAngle(self.position.yaw),
        }
    }

    pub(crate) fn get_animation_packet(&self, hand: PlayerHand) -> EntityAnimationPacket {
        let animation = match hand {
            PlayerHand::Main => EntityAnimation::SwingMainArm,
            PlayerHand::Off => EntityAnimation::SwingOffHand,
        };
        EntityAnimationPacket {
            entity_id: self.get_entity_id().get_value(),
            animation,
        }
    }
}
