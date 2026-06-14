use crate::entity::EntityPosition;
use crate::entity::EntityView;
use crate::entity::ExperienceOrb;
use crate::entity::TimedPotionEffect;
use crate::entity::creature::CreatureEntity;
use crate::entity::generic_entity::GenericEntity;
use crate::entity::identity::EntityId;
use crate::entity::item::ItemEntity;
use crate::entity::player::Player;
use crate::entity::projectile::ProjectileEntity;
use spinel_core::network::clientbound::play::attach_entity::AttachEntityPacket;
use spinel_core::network::clientbound::play::entity_effect::EntityEffectPacket;
use spinel_core::network::clientbound::play::entity_position_sync::EntityPositionSyncPacket;
use spinel_core::network::clientbound::play::entity_velocity::EntityVelocityPacket;
use spinel_core::network::clientbound::play::remove_entity_effect::RemoveEntityEffectPacket;
use spinel_core::network::clientbound::play::set_entity_data::SetEntityDataPacket;
use spinel_core::network::clientbound::play::set_passengers::SetPassengersPacket;
use spinel_network::types::Velocity;
use spinel_registry::EntityType;
use std::collections::BTreeSet;
use uuid::Uuid;

pub enum Entity {
    Creature(CreatureEntity),
    ExperienceOrb(ExperienceOrb),
    Generic(GenericEntity),
    Item(ItemEntity),
    Player(Player),
    Projectile(ProjectileEntity),
}

impl Entity {
    pub fn new(entity_type: EntityType) -> Self {
        Self::Generic(GenericEntity::new(entity_type))
    }

    pub fn entity_id(&self) -> EntityId {
        match self {
            Self::Creature(entity) => entity.entity_id(),
            Self::ExperienceOrb(entity) => entity.entity_id(),
            Self::Generic(entity) => entity.entity_id(),
            Self::Item(entity) => entity.entity_id(),
            Self::Player(player) => player.entity_id(),
            Self::Projectile(entity) => entity.entity_id(),
        }
    }

    pub fn uuid(&self) -> Uuid {
        match self {
            Self::Creature(entity) => entity.uuid(),
            Self::ExperienceOrb(entity) => entity.uuid(),
            Self::Generic(entity) => entity.uuid(),
            Self::Item(entity) => entity.uuid(),
            Self::Player(player) => player.uuid,
            Self::Projectile(entity) => entity.uuid(),
        }
    }

    pub fn entity_type(&self) -> EntityType {
        match self {
            Self::Creature(entity) => entity.entity_type(),
            Self::ExperienceOrb(entity) => entity.entity_type(),
            Self::Generic(entity) => entity.entity_type(),
            Self::Item(entity) => entity.entity_type(),
            Self::Player(player) => player.entity_type(),
            Self::Projectile(entity) => entity.entity_type(),
        }
    }

    pub(crate) fn switch_entity_type(&mut self, entity_type: EntityType) -> bool {
        match self {
            Self::Creature(entity) => entity.switch_entity_type(entity_type),
            Self::ExperienceOrb(entity) => entity.switch_entity_type(entity_type),
            Self::Generic(entity) => entity.switch_entity_type(entity_type),
            Self::Item(entity) => entity.switch_entity_type(entity_type),
            Self::Player(player) => player.switch_entity_type(entity_type),
            Self::Projectile(entity) => entity.switch_entity_type(entity_type),
        }
        true
    }

    pub fn eye_height(&self) -> f64 {
        self.entity_type().eye_height()
    }

    pub fn bounding_box(&self) -> spinel_registry::EntityBoundingBox {
        match self {
            Self::Creature(entity) => entity.bounding_box(),
            Self::ExperienceOrb(entity) => entity.bounding_box(),
            Self::Generic(entity) => entity.bounding_box(),
            Self::Item(entity) => entity.bounding_box(),
            Self::Player(player) => player.bounding_box(),
            Self::Projectile(entity) => entity.bounding_box(),
        }
    }

    pub fn world(&self) -> Option<Uuid> {
        match self {
            Self::Creature(entity) => entity.world(),
            Self::ExperienceOrb(entity) => entity.world(),
            Self::Generic(entity) => entity.world(),
            Self::Item(entity) => entity.world(),
            Self::Player(player) => player.current_world(),
            Self::Projectile(entity) => entity.world(),
        }
    }

    pub fn position(&self) -> EntityPosition {
        match self {
            Self::Creature(entity) => entity.position(),
            Self::ExperienceOrb(entity) => entity.position(),
            Self::Generic(entity) => entity.position(),
            Self::Item(entity) => entity.position(),
            Self::Player(player) => player.position(),
            Self::Projectile(entity) => entity.position(),
        }
    }

    pub fn is_removed(&self) -> bool {
        match self {
            Self::Creature(entity) => entity.is_removed(),
            Self::ExperienceOrb(entity) => entity.is_removed(),
            Self::Generic(entity) => entity.is_removed(),
            Self::Item(entity) => entity.is_removed(),
            Self::Player(_) => false,
            Self::Projectile(entity) => entity.is_removed(),
        }
    }

    pub(crate) fn set_position(&mut self, position: EntityPosition) {
        match self {
            Self::Creature(entity) => entity.set_position(position),
            Self::ExperienceOrb(entity) => entity.set_position(position),
            Self::Generic(entity) => entity.set_position(position),
            Self::Item(entity) => entity.set_position(position),
            Self::Player(player) => player.set_position(position),
            Self::Projectile(entity) => entity.set_position(position),
        }
    }

    pub fn velocity(&self) -> Velocity {
        match self {
            Self::Creature(entity) => entity.velocity(),
            Self::ExperienceOrb(entity) => entity.velocity(),
            Self::Generic(entity) => entity.velocity(),
            Self::Item(entity) => entity.velocity(),
            Self::Player(player) => player.velocity(),
            Self::Projectile(entity) => entity.velocity(),
        }
    }

    pub(crate) fn set_velocity(&mut self, velocity: Velocity) {
        match self {
            Self::Creature(entity) => entity.set_velocity(velocity),
            Self::ExperienceOrb(entity) => entity.set_velocity(velocity),
            Self::Generic(entity) => entity.set_velocity(velocity),
            Self::Item(entity) => entity.set_velocity(velocity),
            Self::Player(player) => player.set_velocity(velocity),
            Self::Projectile(entity) => entity.set_velocity(velocity),
        }
    }

    pub(crate) fn set_world(&mut self, world: Uuid) {
        match self {
            Self::Creature(entity) => entity.set_world(world),
            Self::ExperienceOrb(entity) => entity.set_world(world),
            Self::Generic(entity) => entity.set_world(world),
            Self::Item(entity) => entity.set_world(world),
            Self::Player(player) => player.set_world(world),
            Self::Projectile(entity) => entity.set_world(world),
        }
    }

    pub fn add_effect(&mut self, effect: TimedPotionEffect) -> EntityEffectPacket {
        match self {
            Self::Creature(entity) => entity.add_effect(effect),
            Self::ExperienceOrb(entity) => entity.add_effect(effect),
            Self::Generic(entity) => entity.add_effect(effect),
            Self::Item(entity) => entity.add_effect(effect),
            Self::Player(player) => player.add_effect(effect),
            Self::Projectile(entity) => entity.add_effect(effect),
        }
    }

    pub fn remove_effect(&mut self, effect_id: i32) -> Option<RemoveEntityEffectPacket> {
        match self {
            Self::Creature(entity) => entity.remove_effect(effect_id),
            Self::ExperienceOrb(entity) => entity.remove_effect(effect_id),
            Self::Generic(entity) => entity.remove_effect(effect_id),
            Self::Item(entity) => entity.remove_effect(effect_id),
            Self::Player(player) => player.remove_effect(effect_id),
            Self::Projectile(entity) => entity.remove_effect(effect_id),
        }
    }

    pub fn effect(&self, effect_id: i32) -> Option<&TimedPotionEffect> {
        match self {
            Self::Creature(entity) => entity.effect(effect_id),
            Self::ExperienceOrb(entity) => entity.effect(effect_id),
            Self::Generic(entity) => entity.effect(effect_id),
            Self::Item(entity) => entity.effect(effect_id),
            Self::Player(player) => player.effect(effect_id),
            Self::Projectile(entity) => entity.effect(effect_id),
        }
    }

    pub fn active_effects(&self) -> Vec<&TimedPotionEffect> {
        match self {
            Self::Creature(entity) => entity.active_effects(),
            Self::ExperienceOrb(entity) => entity.active_effects(),
            Self::Generic(entity) => entity.active_effects(),
            Self::Item(entity) => entity.active_effects(),
            Self::Player(player) => player.active_effects(),
            Self::Projectile(entity) => entity.active_effects(),
        }
    }

    pub fn view(&self) -> &EntityView {
        match self {
            Self::Creature(entity) => entity.view(),
            Self::ExperienceOrb(entity) => entity.view(),
            Self::Generic(entity) => entity.view(),
            Self::Item(entity) => entity.view(),
            Self::Player(player) => player.view(),
            Self::Projectile(entity) => entity.view(),
        }
    }

    pub fn view_mut(&mut self) -> &mut EntityView {
        match self {
            Self::Creature(entity) => entity.view_mut(),
            Self::ExperienceOrb(entity) => entity.view_mut(),
            Self::Generic(entity) => entity.view_mut(),
            Self::Item(entity) => entity.view_mut(),
            Self::Player(player) => player.view_mut(),
            Self::Projectile(entity) => entity.view_mut(),
        }
    }

    pub fn viewers(&self) -> std::collections::BTreeSet<EntityId> {
        self.view().viewers()
    }

    pub fn vehicle(&self) -> Option<EntityId> {
        match self {
            Self::Creature(entity) => entity.vehicle(),
            Self::ExperienceOrb(entity) => entity.vehicle(),
            Self::Generic(entity) => entity.vehicle(),
            Self::Item(entity) => entity.vehicle(),
            Self::Player(player) => player.vehicle(),
            Self::Projectile(entity) => entity.vehicle(),
        }
    }

    pub fn has_passenger(&self) -> bool {
        match self {
            Self::Creature(entity) => entity.has_passenger(),
            Self::ExperienceOrb(entity) => entity.has_passenger(),
            Self::Generic(entity) => entity.has_passenger(),
            Self::Item(entity) => entity.has_passenger(),
            Self::Player(player) => player.has_passenger(),
            Self::Projectile(entity) => entity.has_passenger(),
        }
    }

    pub fn passengers(&self) -> &BTreeSet<EntityId> {
        match self {
            Self::Creature(entity) => entity.passengers(),
            Self::ExperienceOrb(entity) => entity.passengers(),
            Self::Generic(entity) => entity.passengers(),
            Self::Item(entity) => entity.passengers(),
            Self::Player(player) => player.passengers(),
            Self::Projectile(entity) => entity.passengers(),
        }
    }

    pub(crate) fn set_vehicle(&mut self, vehicle_id: EntityId) {
        match self {
            Self::Creature(entity) => entity.set_vehicle(vehicle_id),
            Self::ExperienceOrb(entity) => entity.set_vehicle(vehicle_id),
            Self::Generic(entity) => entity.set_vehicle(vehicle_id),
            Self::Item(entity) => entity.set_vehicle(vehicle_id),
            Self::Player(player) => player.set_vehicle(vehicle_id),
            Self::Projectile(entity) => entity.set_vehicle(vehicle_id),
        }
    }

    pub(crate) fn clear_vehicle(&mut self) {
        match self {
            Self::Creature(entity) => entity.clear_vehicle(),
            Self::ExperienceOrb(entity) => entity.clear_vehicle(),
            Self::Generic(entity) => entity.clear_vehicle(),
            Self::Item(entity) => entity.clear_vehicle(),
            Self::Player(player) => player.clear_vehicle(),
            Self::Projectile(entity) => entity.clear_vehicle(),
        }
    }

    pub(crate) fn add_passenger(&mut self, passenger_id: EntityId) -> bool {
        match self {
            Self::Creature(entity) => entity.add_passenger(passenger_id),
            Self::ExperienceOrb(entity) => entity.add_passenger(passenger_id),
            Self::Generic(entity) => entity.add_passenger(passenger_id),
            Self::Item(entity) => entity.add_passenger(passenger_id),
            Self::Player(player) => player.add_passenger(passenger_id),
            Self::Projectile(entity) => entity.add_passenger(passenger_id),
        }
    }

    pub(crate) fn remove_passenger(&mut self, passenger_id: EntityId) -> bool {
        match self {
            Self::Creature(entity) => entity.remove_passenger(passenger_id),
            Self::ExperienceOrb(entity) => entity.remove_passenger(passenger_id),
            Self::Generic(entity) => entity.remove_passenger(passenger_id),
            Self::Item(entity) => entity.remove_passenger(passenger_id),
            Self::Player(player) => player.remove_passenger(passenger_id),
            Self::Projectile(entity) => entity.remove_passenger(passenger_id),
        }
    }

    pub(crate) fn passenger_packet(&self) -> SetPassengersPacket {
        match self {
            Self::Creature(entity) => entity.passenger_packet(),
            Self::ExperienceOrb(entity) => entity.passenger_packet(),
            Self::Generic(entity) => entity.passenger_packet(),
            Self::Item(entity) => entity.passenger_packet(),
            Self::Player(player) => player.passenger_packet(),
            Self::Projectile(entity) => entity.passenger_packet(),
        }
    }

    pub(crate) fn passenger_position(&self, passenger: &Self) -> EntityPosition {
        let vehicle_position = self.position();
        let passenger_position = passenger.position();
        let passenger_height_offset = if self.entity_type().path().contains("boat") {
            -0.1
        } else if self.entity_type() == EntityType::MINECART {
            0.0
        } else if matches!(
            passenger.entity_type(),
            EntityType::ZOMBIE
                | EntityType::HUSK
                | EntityType::DROWNED
                | EntityType::SKELETON
                | EntityType::STRAY
                | EntityType::WITHER_SKELETON
                | EntityType::PIGLIN
                | EntityType::PIGLIN_BRUTE
                | EntityType::ZOMBIFIED_PIGLIN
        ) {
            self.bounding_box().height() * 0.75
        } else {
            self.bounding_box().height()
        };
        EntityPosition::new(
            vehicle_position.x(),
            vehicle_position.y() + passenger_height_offset,
            vehicle_position.z(),
            passenger_position.yaw(),
            passenger_position.pitch(),
        )
        .with_head_yaw(passenger_position.head_yaw())
    }

    pub(crate) fn synchronize_position_packet(&mut self) -> EntityPositionSyncPacket {
        match self {
            Self::Creature(entity) => entity.synchronize_position_packet(),
            Self::ExperienceOrb(entity) => entity.synchronize_position_packet(),
            Self::Generic(entity) => entity.synchronize_position_packet(),
            Self::Item(entity) => entity.synchronize_position_packet(),
            Self::Player(player) => player.synchronize_entity_position_packet(),
            Self::Projectile(entity) => entity.synchronize_position_packet(),
        }
    }

    pub(crate) fn scheduled_position_sync_packet(&mut self) -> Option<EntityPositionSyncPacket> {
        match self {
            Self::Creature(entity) => entity.scheduled_position_sync_packet(),
            Self::ExperienceOrb(entity) => entity.scheduled_position_sync_packet(),
            Self::Generic(entity) => entity.scheduled_position_sync_packet(),
            Self::Item(entity) => entity.scheduled_position_sync_packet(),
            Self::Player(player) => player.scheduled_entity_position_sync_packet(),
            Self::Projectile(entity) => entity.scheduled_position_sync_packet(),
        }
    }

    pub(crate) fn velocity_packet(&self) -> EntityVelocityPacket {
        match self {
            Self::Creature(entity) => entity.velocity_packet(),
            Self::ExperienceOrb(entity) => entity.velocity_packet(),
            Self::Generic(entity) => entity.velocity_packet(),
            Self::Item(entity) => entity.velocity_packet(),
            Self::Player(player) => player.velocity_packet(),
            Self::Projectile(entity) => entity.velocity_packet(),
        }
    }

    pub(crate) fn dirty_metadata_packet(&mut self) -> Option<SetEntityDataPacket> {
        match self {
            Self::Creature(entity) => entity.dirty_metadata_packet(),
            Self::ExperienceOrb(entity) => entity.dirty_metadata_packet(),
            Self::Generic(entity) => entity.dirty_metadata_packet(),
            Self::Item(entity) => entity.dirty_metadata_packet(),
            Self::Player(player) => player.dirty_metadata_packet(),
            Self::Projectile(entity) => entity.dirty_metadata_packet(),
        }
    }

    pub fn leashed_entities(&self) -> &BTreeSet<EntityId> {
        match self {
            Self::Creature(entity) => entity.leashed_entities(),
            Self::ExperienceOrb(entity) => entity.leashed_entities(),
            Self::Generic(entity) => entity.leashed_entities(),
            Self::Item(entity) => entity.leashed_entities(),
            Self::Player(player) => player.leashed_entities(),
            Self::Projectile(entity) => entity.leashed_entities(),
        }
    }

    pub fn leash_holder(&self) -> Option<EntityId> {
        match self {
            Self::Creature(entity) => entity.leash_holder(),
            Self::ExperienceOrb(entity) => entity.leash_holder(),
            Self::Generic(entity) => entity.leash_holder(),
            Self::Item(entity) => entity.leash_holder(),
            Self::Player(player) => player.leash_holder(),
            Self::Projectile(entity) => entity.leash_holder(),
        }
    }

    pub(crate) fn set_leash_holder(&mut self, leash_holder: Option<EntityId>) {
        match self {
            Self::Creature(entity) => entity.set_leash_holder(leash_holder),
            Self::ExperienceOrb(entity) => entity.set_leash_holder(leash_holder),
            Self::Generic(entity) => entity.set_leash_holder(leash_holder),
            Self::Item(entity) => entity.set_leash_holder(leash_holder),
            Self::Player(player) => player.set_leash_holder(leash_holder),
            Self::Projectile(entity) => entity.set_leash_holder(leash_holder),
        }
    }

    pub(crate) fn add_leashed_entity(&mut self, entity_id: EntityId) -> bool {
        match self {
            Self::Creature(entity) => entity.add_leashed_entity(entity_id),
            Self::ExperienceOrb(entity) => entity.add_leashed_entity(entity_id),
            Self::Generic(entity) => entity.add_leashed_entity(entity_id),
            Self::Item(entity) => entity.add_leashed_entity(entity_id),
            Self::Player(player) => player.add_leashed_entity(entity_id),
            Self::Projectile(entity) => entity.add_leashed_entity(entity_id),
        }
    }

    pub(crate) fn remove_leashed_entity(&mut self, entity_id: EntityId) -> bool {
        match self {
            Self::Creature(entity) => entity.remove_leashed_entity(entity_id),
            Self::ExperienceOrb(entity) => entity.remove_leashed_entity(entity_id),
            Self::Generic(entity) => entity.remove_leashed_entity(entity_id),
            Self::Item(entity) => entity.remove_leashed_entity(entity_id),
            Self::Player(player) => player.remove_leashed_entity(entity_id),
            Self::Projectile(entity) => entity.remove_leashed_entity(entity_id),
        }
    }

    pub(crate) fn attach_entity_packet(&self) -> AttachEntityPacket {
        match self {
            Self::Creature(entity) => entity.attach_entity_packet(),
            Self::ExperienceOrb(entity) => entity.attach_entity_packet(),
            Self::Generic(entity) => entity.attach_entity_packet(),
            Self::Item(entity) => entity.attach_entity_packet(),
            Self::Player(player) => player.attach_entity_packet(),
            Self::Projectile(entity) => entity.attach_entity_packet(),
        }
    }
}
