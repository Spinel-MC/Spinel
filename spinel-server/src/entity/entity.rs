use crate::entity::EntityPosition;
use crate::entity::EntityView;
use crate::entity::ExperienceOrb;
use crate::entity::TimedPotionEffect;
use crate::entity::entity_creature::EntityCreature;
use crate::entity::generic_entity::GenericEntity;
use crate::entity::identity::{EntityId, EntityPointers};
use crate::entity::item::ItemEntity;
use crate::entity::player::Player;
use crate::entity::projectile::ProjectileEntity;
use crate::scheduler::{ContextScheduler, Task, TaskSchedule};
use crate::world::World;
use spinel_core::network::clientbound::play::attach_entity::AttachEntityPacket;
use spinel_core::network::clientbound::play::entity_effect::EntityEffectPacket;
use spinel_core::network::clientbound::play::entity_position_sync::EntityPositionSyncPacket;
use spinel_core::network::clientbound::play::entity_velocity::EntityVelocityPacket;
use spinel_core::network::clientbound::play::remove_entity_effect::RemoveEntityEffectPacket;
use spinel_core::network::clientbound::play::set_entity_data::SetEntityDataPacket;
use spinel_core::network::clientbound::play::set_passengers::SetPassengersPacket;
use spinel_network::types::Velocity;
use spinel_registry::{EntityType, MobEffect, RegistryKey};
use std::collections::BTreeSet;
use uuid::Uuid;

pub enum Entity {
    Creature(EntityCreature),
    ExperienceOrb(ExperienceOrb),
    Generic(GenericEntity),
    Item(ItemEntity),
    Player(Player),
    Projectile(ProjectileEntity),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EntityAcquirable {
    pointers: EntityPointers,
    world: Option<Uuid>,
}

pub enum EntityScheduleContext<'entity> {
    Generic(&'entity mut GenericEntity),
    Player(&'entity mut Player),
}

pub enum EntityScheduler<'entity> {
    Generic(&'entity mut ContextScheduler<GenericEntity>),
    Player(&'entity mut ContextScheduler<Player>),
}

impl EntityAcquirable {
    pub const fn new(pointers: EntityPointers, world: Option<Uuid>) -> Self {
        Self { pointers, world }
    }

    pub const fn get_entity_id(self) -> EntityId {
        self.pointers.get_entity_id()
    }

    pub const fn get_uuid(self) -> Uuid {
        self.pointers.get_uuid()
    }

    pub const fn get_world(self) -> Option<Uuid> {
        self.world
    }

    pub const fn is_assigned(self) -> bool {
        self.world.is_some()
    }

    pub const fn is_owned(self) -> bool {
        true
    }
}

impl EntityScheduleContext<'_> {
    pub fn get_entity_id(&self) -> EntityId {
        match self {
            Self::Generic(entity) => entity.get_entity_id(),
            Self::Player(player) => player.get_entity_id(),
        }
    }
}

impl EntityScheduler<'_> {
    pub fn get_task_count(&self) -> usize {
        match self {
            Self::Generic(scheduler) => scheduler.task_count(),
            Self::Player(scheduler) => scheduler.task_count(),
        }
    }

    pub fn schedule_next_tick(
        &mut self,
        callback: impl FnMut(EntityScheduleContext<'_>) -> TaskSchedule + Send + 'static,
    ) -> Task {
        match self {
            Self::Generic(scheduler) => {
                scheduler.schedule_next_tick(map_generic_schedule(callback))
            }
            Self::Player(scheduler) => scheduler.schedule_next_tick(map_player_schedule(callback)),
        }
    }
}

fn map_generic_schedule(
    mut callback: impl FnMut(EntityScheduleContext<'_>) -> TaskSchedule + Send + 'static,
) -> impl FnMut(&mut GenericEntity) -> TaskSchedule + Send + 'static {
    move |entity| callback(EntityScheduleContext::Generic(entity))
}

fn map_player_schedule(
    mut callback: impl FnMut(EntityScheduleContext<'_>) -> TaskSchedule + Send + 'static,
) -> impl FnMut(&mut Player) -> TaskSchedule + Send + 'static {
    move |player| callback(EntityScheduleContext::Player(player))
}

impl From<EntityCreature> for Entity {
    fn from(entity_creature: EntityCreature) -> Self {
        Self::Creature(entity_creature)
    }
}

impl Entity {
    pub fn get_pointers(&self) -> EntityPointers {
        match self {
            Self::Creature(entity) => entity.get_pointers(),
            Self::ExperienceOrb(entity) => entity.get_pointers(),
            Self::Generic(entity) => entity.get_pointers(),
            Self::Item(entity) => entity.get_pointers(),
            Self::Player(player) => player.get_pointers(),
            Self::Projectile(entity) => entity.get_pointers(),
        }
    }

    pub fn get_acquirable(&self) -> EntityAcquirable {
        EntityAcquirable::new(self.get_pointers(), self.get_world())
    }

    pub fn get_living_acquirable(&self) -> Option<EntityAcquirable> {
        match self {
            Self::Creature(_) => Some(self.get_acquirable()),
            Self::Generic(entity) if entity.get_entity_type().is_living() => {
                Some(self.get_acquirable())
            }
            Self::Player(_) => Some(self.get_acquirable()),
            _ => None,
        }
    }

    pub fn get_scheduler(&mut self) -> EntityScheduler<'_> {
        match self {
            Self::Generic(entity) => EntityScheduler::Generic(entity.get_scheduler()),
            Self::Player(player) => EntityScheduler::Player(player.get_scheduler()),
            Self::Creature(entity) => {
                EntityScheduler::Generic(entity.get_entity_mut().get_scheduler())
            }
            Self::ExperienceOrb(entity) => EntityScheduler::Generic(entity.get_scheduler()),
            Self::Item(entity) => EntityScheduler::Generic(entity.get_scheduler()),
            Self::Projectile(entity) => EntityScheduler::Generic(entity.get_scheduler()),
        }
    }
    pub fn new(entity_type: EntityType) -> Self {
        Self::Generic(GenericEntity::new(entity_type))
    }

    pub fn get_entity_id(&self) -> EntityId {
        match self {
            Self::Creature(entity) => entity.get_entity_id(),
            Self::ExperienceOrb(entity) => entity.get_entity_id(),
            Self::Generic(entity) => entity.get_entity_id(),
            Self::Item(entity) => entity.get_entity_id(),
            Self::Player(player) => player.get_entity_id(),
            Self::Projectile(entity) => entity.get_entity_id(),
        }
    }

    pub fn get_uuid(&self) -> Uuid {
        match self {
            Self::Creature(entity) => entity.get_uuid(),
            Self::ExperienceOrb(entity) => entity.get_uuid(),
            Self::Generic(entity) => entity.get_uuid(),
            Self::Item(entity) => entity.get_uuid(),
            Self::Player(player) => player.uuid,
            Self::Projectile(entity) => entity.get_uuid(),
        }
    }

    pub fn get_entity_type(&self) -> EntityType {
        match self {
            Self::Creature(entity) => entity.get_entity_type(),
            Self::ExperienceOrb(entity) => entity.get_entity_type(),
            Self::Generic(entity) => entity.get_entity_type(),
            Self::Item(entity) => entity.get_entity_type(),
            Self::Player(player) => player.get_entity_type(),
            Self::Projectile(entity) => entity.get_entity_type(),
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

    pub fn get_eye_height(&self) -> f64 {
        self.get_entity_type().get_eye_height()
    }

    pub fn get_bounding_box(&self) -> spinel_registry::EntityBoundingBox {
        match self {
            Self::Creature(entity) => entity.get_bounding_box(),
            Self::ExperienceOrb(entity) => entity.get_bounding_box(),
            Self::Generic(entity) => entity.get_bounding_box(),
            Self::Item(entity) => entity.get_bounding_box(),
            Self::Player(player) => player.get_bounding_box(),
            Self::Projectile(entity) => entity.get_bounding_box(),
        }
    }

    pub fn get_world(&self) -> Option<Uuid> {
        match self {
            Self::Creature(entity) => entity.get_world(),
            Self::ExperienceOrb(entity) => entity.get_world(),
            Self::Generic(entity) => entity.get_world(),
            Self::Item(entity) => entity.get_world(),
            Self::Player(player) => player.get_current_world(),
            Self::Projectile(entity) => entity.get_world(),
        }
    }

    pub fn get_position(&self) -> EntityPosition {
        match self {
            Self::Creature(entity) => entity.get_position(),
            Self::ExperienceOrb(entity) => entity.get_position(),
            Self::Generic(entity) => entity.get_position(),
            Self::Item(entity) => entity.get_position(),
            Self::Player(player) => player.get_position(),
            Self::Projectile(entity) => entity.get_position(),
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

    pub fn set_world(self, world: &mut World) -> bool {
        world.add_entity(self)
    }

    pub fn set_world_at(mut self, world: &mut World, position: EntityPosition) -> bool {
        self.set_position(position);
        self.set_world(world)
    }

    pub fn get_last_damage_source(&self) -> Option<&crate::entity::Damage> {
        match self {
            Self::Creature(entity) => entity.get_last_damage(),
            Self::ExperienceOrb(entity) => entity.get_last_damage(),
            Self::Generic(entity) => entity.get_last_damage(),
            Self::Item(entity) => entity.get_last_damage(),
            Self::Player(player) => player.get_last_damage(),
            Self::Projectile(entity) => entity.get_last_damage(),
        }
    }

    pub fn get_velocity(&self) -> Velocity {
        match self {
            Self::Creature(entity) => entity.get_velocity(),
            Self::ExperienceOrb(entity) => entity.get_velocity(),
            Self::Generic(entity) => entity.get_velocity(),
            Self::Item(entity) => entity.get_velocity(),
            Self::Player(player) => player.get_velocity(),
            Self::Projectile(entity) => entity.get_velocity(),
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

    pub(crate) fn assign_world(&mut self, world: Uuid) {
        match self {
            Self::Creature(entity) => entity.assign_world(world),
            Self::ExperienceOrb(entity) => entity.assign_world(world),
            Self::Generic(entity) => entity.assign_world(world),
            Self::Item(entity) => entity.assign_world(world),
            Self::Player(player) => player.assign_world(world),
            Self::Projectile(entity) => entity.assign_world(world),
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

    pub fn remove_effect(
        &mut self,
        effect_key: &RegistryKey<MobEffect>,
    ) -> Option<RemoveEntityEffectPacket> {
        match self {
            Self::Creature(entity) => entity.remove_effect(effect_key),
            Self::ExperienceOrb(entity) => entity.remove_effect(effect_key),
            Self::Generic(entity) => entity.remove_effect(effect_key),
            Self::Item(entity) => entity.remove_effect(effect_key),
            Self::Player(player) => player.remove_effect(effect_key),
            Self::Projectile(entity) => entity.remove_effect(effect_key),
        }
    }

    pub fn has_effect(&self, effect_key: &RegistryKey<MobEffect>) -> bool {
        self.get_effect(effect_key).is_some()
    }

    pub fn get_effect_level(&self, effect_key: &RegistryKey<MobEffect>) -> i32 {
        self.get_effect(effect_key)
            .map_or(-1, TimedPotionEffect::get_amplifier)
    }

    pub fn clear_effects(&mut self) -> Vec<RemoveEntityEffectPacket> {
        match self {
            Self::Creature(entity) => entity.clear_effects(),
            Self::ExperienceOrb(entity) => entity.clear_effects(),
            Self::Generic(entity) => entity.clear_effects(),
            Self::Item(entity) => entity.clear_effects(),
            Self::Player(player) => player.clear_effects(),
            Self::Projectile(entity) => entity.clear_effects(),
        }
    }
    pub fn get_effect(&self, effect_key: &RegistryKey<MobEffect>) -> Option<&TimedPotionEffect> {
        match self {
            Self::Creature(entity) => entity.get_effect(effect_key),
            Self::ExperienceOrb(entity) => entity.get_effect(effect_key),
            Self::Generic(entity) => entity.get_effect(effect_key),
            Self::Item(entity) => entity.get_effect(effect_key),
            Self::Player(player) => player.get_effect(effect_key),
            Self::Projectile(entity) => entity.get_effect(effect_key),
        }
    }

    pub fn get_active_effects(&self) -> Vec<&TimedPotionEffect> {
        match self {
            Self::Creature(entity) => entity.get_active_effects(),
            Self::ExperienceOrb(entity) => entity.get_active_effects(),
            Self::Generic(entity) => entity.get_active_effects(),
            Self::Item(entity) => entity.get_active_effects(),
            Self::Player(player) => player.get_active_effects(),
            Self::Projectile(entity) => entity.get_active_effects(),
        }
    }

    pub fn get_view(&self) -> &EntityView {
        match self {
            Self::Creature(entity) => entity.get_view(),
            Self::ExperienceOrb(entity) => entity.get_view(),
            Self::Generic(entity) => entity.get_view(),
            Self::Item(entity) => entity.get_view(),
            Self::Player(player) => player.get_view(),
            Self::Projectile(entity) => entity.get_view(),
        }
    }

    pub fn get_view_mut(&mut self) -> &mut EntityView {
        match self {
            Self::Creature(entity) => entity.get_view_mut(),
            Self::ExperienceOrb(entity) => entity.get_view_mut(),
            Self::Generic(entity) => entity.get_view_mut(),
            Self::Item(entity) => entity.get_view_mut(),
            Self::Player(player) => player.get_view_mut(),
            Self::Projectile(entity) => entity.get_view_mut(),
        }
    }

    pub fn get_viewers(&self) -> std::collections::BTreeSet<EntityId> {
        self.get_view().get_viewers()
    }

    pub fn get_vehicle(&self) -> Option<EntityId> {
        match self {
            Self::Creature(entity) => entity.get_vehicle(),
            Self::ExperienceOrb(entity) => entity.get_vehicle(),
            Self::Generic(entity) => entity.get_vehicle(),
            Self::Item(entity) => entity.get_vehicle(),
            Self::Player(player) => player.get_vehicle(),
            Self::Projectile(entity) => entity.get_vehicle(),
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

    pub fn get_passengers(&self) -> &BTreeSet<EntityId> {
        match self {
            Self::Creature(entity) => entity.get_passengers(),
            Self::ExperienceOrb(entity) => entity.get_passengers(),
            Self::Generic(entity) => entity.get_passengers(),
            Self::Item(entity) => entity.get_passengers(),
            Self::Player(player) => player.get_passengers(),
            Self::Projectile(entity) => entity.get_passengers(),
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

    pub(crate) fn attach_passenger(&mut self, passenger_id: EntityId) -> bool {
        match self {
            Self::Creature(entity) => entity.add_passenger(passenger_id),
            Self::ExperienceOrb(entity) => entity.add_passenger(passenger_id),
            Self::Generic(entity) => entity.add_passenger(passenger_id),
            Self::Item(entity) => entity.add_passenger(passenger_id),
            Self::Player(player) => player.add_passenger(passenger_id),
            Self::Projectile(entity) => entity.add_passenger(passenger_id),
        }
    }

    pub(crate) fn detach_passenger(&mut self, passenger_id: EntityId) -> bool {
        match self {
            Self::Creature(entity) => entity.remove_passenger(passenger_id),
            Self::ExperienceOrb(entity) => entity.remove_passenger(passenger_id),
            Self::Generic(entity) => entity.remove_passenger(passenger_id),
            Self::Item(entity) => entity.remove_passenger(passenger_id),
            Self::Player(player) => player.remove_passenger(passenger_id),
            Self::Projectile(entity) => entity.remove_passenger(passenger_id),
        }
    }

    pub(crate) fn get_passenger_packet(&self) -> SetPassengersPacket {
        match self {
            Self::Creature(entity) => entity.get_passenger_packet(),
            Self::ExperienceOrb(entity) => entity.get_passenger_packet(),
            Self::Generic(entity) => entity.get_passenger_packet(),
            Self::Item(entity) => entity.get_passenger_packet(),
            Self::Player(player) => player.get_passenger_packet(),
            Self::Projectile(entity) => entity.get_passenger_packet(),
        }
    }

    pub(crate) fn get_passenger_position(&self, passenger: &Self) -> EntityPosition {
        let vehicle_position = self.get_position();
        let passenger_position = passenger.get_position();
        let passenger_height_offset = if self.get_entity_type().path().contains("boat") {
            -0.1
        } else if self.get_entity_type() == EntityType::MINECART {
            0.0
        } else if matches!(
            passenger.get_entity_type(),
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
            self.get_bounding_box().get_height() * 0.75
        } else {
            self.get_bounding_box().get_height()
        };
        EntityPosition::new(
            vehicle_position.get_x(),
            vehicle_position.get_y() + passenger_height_offset,
            vehicle_position.get_z(),
            passenger_position.get_yaw(),
            passenger_position.get_pitch(),
        )
        .with_head_yaw(passenger_position.get_head_yaw())
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

    pub(crate) fn get_scheduled_position_sync_packet(
        &mut self,
    ) -> Option<EntityPositionSyncPacket> {
        match self {
            Self::Creature(entity) => entity.get_scheduled_position_sync_packet(),
            Self::ExperienceOrb(entity) => entity.get_scheduled_position_sync_packet(),
            Self::Generic(entity) => entity.get_scheduled_position_sync_packet(),
            Self::Item(entity) => entity.get_scheduled_position_sync_packet(),
            Self::Player(player) => player.get_scheduled_entity_position_sync_packet(),
            Self::Projectile(entity) => entity.get_scheduled_position_sync_packet(),
        }
    }

    pub(crate) fn get_velocity_packet(&self) -> EntityVelocityPacket {
        match self {
            Self::Creature(entity) => entity.get_velocity_packet(),
            Self::ExperienceOrb(entity) => entity.get_velocity_packet(),
            Self::Generic(entity) => entity.get_velocity_packet(),
            Self::Item(entity) => entity.get_velocity_packet(),
            Self::Player(player) => player.get_velocity_packet(),
            Self::Projectile(entity) => entity.get_velocity_packet(),
        }
    }

    pub(crate) fn get_dirty_metadata_packet(&mut self) -> Option<SetEntityDataPacket> {
        match self {
            Self::Creature(entity) => entity.get_dirty_metadata_packet(),
            Self::ExperienceOrb(entity) => entity.get_dirty_metadata_packet(),
            Self::Generic(entity) => entity.get_dirty_metadata_packet(),
            Self::Item(entity) => entity.get_dirty_metadata_packet(),
            Self::Player(player) => player.get_dirty_metadata_packet(),
            Self::Projectile(entity) => entity.get_dirty_metadata_packet(),
        }
    }

    pub fn get_leashed_entities(&self) -> &BTreeSet<EntityId> {
        match self {
            Self::Creature(entity) => entity.get_leashed_entities(),
            Self::ExperienceOrb(entity) => entity.get_leashed_entities(),
            Self::Generic(entity) => entity.get_leashed_entities(),
            Self::Item(entity) => entity.get_leashed_entities(),
            Self::Player(player) => player.get_leashed_entities(),
            Self::Projectile(entity) => entity.get_leashed_entities(),
        }
    }

    pub fn get_leash_holder(&self) -> Option<EntityId> {
        match self {
            Self::Creature(entity) => entity.get_leash_holder(),
            Self::ExperienceOrb(entity) => entity.get_leash_holder(),
            Self::Generic(entity) => entity.get_leash_holder(),
            Self::Item(entity) => entity.get_leash_holder(),
            Self::Player(player) => player.get_leash_holder(),
            Self::Projectile(entity) => entity.get_leash_holder(),
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

    pub(crate) fn get_attach_entity_packet(&self) -> AttachEntityPacket {
        match self {
            Self::Creature(entity) => entity.get_attach_entity_packet(),
            Self::ExperienceOrb(entity) => entity.get_attach_entity_packet(),
            Self::Generic(entity) => entity.get_attach_entity_packet(),
            Self::Item(entity) => entity.get_attach_entity_packet(),
            Self::Player(player) => player.get_attach_entity_packet(),
            Self::Projectile(entity) => entity.get_attach_entity_packet(),
        }
    }
}
