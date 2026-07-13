use crate::entity::generic_entity::EntityAerodynamics;
use crate::entity::metadata::definitions;
use crate::entity::physics::{knockback_velocity, simulate_movement};
use crate::entity::player::input::PlayerInputs;
use crate::entity::{EntityId, EntityPosition, EntityTeleport};
use crate::world::{ChunkPosition, WorldSnapshot};
use spinel_core::network::clientbound::play::entity_velocity::EntityVelocityPacket;
use spinel_core::network::clientbound::play::player_look_at::{FacePoint, PlayerLookAtPacket};
use spinel_core::network::clientbound::play::sync_player_pos::SyncPlayerPositionPacket;
use spinel_network::ConnectionState;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::{TeleportFlags, Vector3d, Velocity};
use spinel_registry::{Attribute, EntityBoundingBox, EntityType};
use std::io;

use super::constants::*;
use super::state::Player;

impl Player {
    pub const fn get_velocity(&self) -> Velocity {
        self.velocity
    }

    pub(crate) fn set_velocity(&mut self, velocity: Velocity) {
        self.velocity = velocity;
    }

    pub fn take_knockback(&mut self, strength: f32, x: f64, z: f64) {
        let living_strength = strength
            * (1.0
                - self
                    .living
                    .get_attribute_value(Attribute::KNOCKBACK_RESISTANCE) as f32);
        self.velocity = knockback_velocity(self.velocity, self.on_ground, living_strength, x, z);
    }

    pub fn has_velocity(&self) -> bool {
        let velocity = self.velocity.0;
        if self.on_ground {
            return velocity.x != 0.0 || velocity.z != 0.0 || velocity.y > 0.0;
        }
        velocity.x != 0.0 || velocity.y != 0.0 || velocity.z != 0.0
    }

    pub fn get_velocity_packet(&self) -> EntityVelocityPacket {
        EntityVelocityPacket {
            entity_id: self.get_entity_id().get_value(),
            velocity: self.protocol_velocity(),
        }
    }

    pub(super) fn protocol_velocity(&self) -> Velocity {
        Velocity(Vector3d {
            x: self.velocity.0.x / SERVER_TICKS_PER_SECOND,
            y: self.velocity.0.y / SERVER_TICKS_PER_SECOND,
            z: self.velocity.0.z / SERVER_TICKS_PER_SECOND,
        })
    }

    pub const fn get_synchronization_ticks(&self) -> u64 {
        self.synchronization.get_interval_ticks()
    }

    pub fn set_synchronization_ticks(&mut self, synchronization_ticks: u64) {
        self.synchronization
            .set_interval_ticks(synchronization_ticks);
    }

    pub fn synchronize_next_tick(&mut self) {
        self.synchronization.synchronize_next_tick();
    }

    pub(in crate::entity::player) fn record_synchronization_position(
        &mut self,
        position: EntityPosition,
    ) {
        self.synchronization.record_position(position);
    }

    pub(crate) fn synchronize_entity_position_packet(
        &mut self,
    ) -> spinel_core::network::clientbound::play::entity_position_sync::EntityPositionSyncPacket
    {
        let position = self.get_position();
        self.synchronization
            .synchronize(self.entity_id, self.alive_ticks, position, self.on_ground)
    }

    pub(crate) fn get_scheduled_entity_position_sync_packet(
        &mut self,
    ) -> Option<
        spinel_core::network::clientbound::play::entity_position_sync::EntityPositionSyncPacket,
    > {
        self.synchronization
            .is_due(self.alive_ticks, self.vehicle.is_some())
            .then(|| self.synchronize_entity_position_packet())
    }

    pub const fn is_on_ground(&self) -> bool {
        self.on_ground
    }

    pub fn set_on_ground(&mut self, on_ground: bool) {
        self.on_ground = on_ground;
    }

    pub(crate) fn refresh_on_ground(&mut self, on_ground: bool) -> bool {
        self.on_ground = on_ground;
        let player_is_airborne = !self.on_ground;
        let player_is_not_flying_with_elytra = !self.is_flying_with_elytra();
        if player_is_airborne || player_is_not_flying_with_elytra {
            return false;
        }
        self.set_flying_with_elytra(false)
    }

    pub const fn get_aerodynamics(&self) -> EntityAerodynamics {
        self.aerodynamics
    }

    pub fn set_aerodynamics(&mut self, aerodynamics: EntityAerodynamics) {
        self.aerodynamics = aerodynamics;
    }

    pub const fn get_gravity_tick_count(&self) -> u64 {
        self.gravity_tick_count
    }

    pub(crate) fn tick_gravity_counter(&mut self) {
        self.gravity_tick_count = if self.on_ground {
            0
        } else {
            self.gravity_tick_count.saturating_add(1)
        };
    }

    pub(crate) fn movement_tick(&mut self, world: &WorldSnapshot) {
        self.tick_gravity_counter();
        if self.vehicle.is_some() {
            return;
        }
        let position = self.get_position();
        let velocity_per_tick = Velocity(Vector3d {
            x: self.velocity.0.x / SERVER_TICKS_PER_SECOND,
            y: self.velocity.0.y / SERVER_TICKS_PER_SECOND,
            z: self.velocity.0.z / SERVER_TICKS_PER_SECOND,
        });
        let physics = simulate_movement(
            position,
            velocity_per_tick,
            self.get_bounding_box(),
            world,
            self.aerodynamics,
            self.has_no_gravity(),
            self.has_physics,
            self.on_ground,
            self.is_flying(),
            Attribute::STEP_HEIGHT.default_value(),
            self.previous_physics_result,
        );
        self.previous_physics_result = Some(physics);
        if !world.is_chunk_loaded(ChunkPosition::from(physics.get_new_position())) {
            return;
        }
        self.velocity = Velocity(Vector3d {
            x: physics.get_new_velocity_per_tick().0.x * SERVER_TICKS_PER_SECOND,
            y: physics.get_new_velocity_per_tick().0.y * SERVER_TICKS_PER_SECOND,
            z: physics.get_new_velocity_per_tick().0.z * SERVER_TICKS_PER_SECOND,
        });
    }

    pub const fn has_physics(&self) -> bool {
        self.has_physics
    }

    pub fn set_has_physics(&mut self, has_physics: bool) {
        self.has_physics = has_physics;
    }

    pub fn has_no_gravity(&self) -> bool {
        match self.metadata.get_value(&definitions::has_no_gravity()) {
            MetadataValue::Boolean(has_no_gravity) => has_no_gravity,
            _ => false,
        }
    }

    pub fn set_no_gravity(&mut self, has_no_gravity: bool) {
        self.metadata.set(
            &definitions::has_no_gravity(),
            MetadataValue::Boolean(has_no_gravity),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn ticks_frozen(&self) -> i32 {
        match self.metadata.get_value(&definitions::ticks_frozen()) {
            MetadataValue::VarInt(ticks_frozen) => ticks_frozen,
            _ => 0,
        }
    }

    pub fn set_ticks_frozen(&mut self, ticks_frozen: i32) {
        self.metadata.set(
            &definitions::ticks_frozen(),
            MetadataValue::VarInt(ticks_frozen),
        );
        self.refresh_dirty_metadata_to_viewers();
    }

    pub fn get_bounding_box(&self) -> EntityBoundingBox {
        self.get_pose()
            .get_bounding_box(EntityType::PLAYER.get_bounding_box())
            .unwrap_or_else(|| EntityType::PLAYER.get_bounding_box())
    }

    pub fn get_next_teleport_id(&mut self) -> i32 {
        self.last_sent_teleport_id += 1;
        self.last_sent_teleport_id
    }

    pub const fn get_last_sent_teleport_id(&self) -> i32 {
        self.last_sent_teleport_id
    }

    pub const fn get_last_received_teleport_id(&self) -> i32 {
        self.last_received_teleport_id
    }

    pub fn set_last_received_teleport_id(&mut self, received_teleport_id: i32) {
        if received_teleport_id < 0 {
            return;
        }
        self.last_received_teleport_id = received_teleport_id;
    }

    pub fn has_pending_teleport_confirmation(&self) -> bool {
        self.last_sent_teleport_id != self.last_received_teleport_id
    }

    pub fn synchronize_position_after_teleport(
        &mut self,
        position: EntityPosition,
        velocity: Vector3d,
        flags: TeleportFlags,
        should_confirm: bool,
    ) -> io::Result<()> {
        let teleport_id = if should_confirm {
            self.get_next_teleport_id()
        } else {
            -1
        };
        let packet = SyncPlayerPositionPacket {
            teleport_id,
            x: position.get_x(),
            y: position.get_y(),
            z: position.get_z(),
            velocity_x: velocity.x,
            velocity_y: velocity.y,
            velocity_z: velocity.z,
            yaw: position.get_yaw(),
            pitch: position.get_pitch(),
            flags,
        };
        let Some(client) = self.get_client_mut() else {
            return Ok(());
        };
        if client.state != ConnectionState::Play {
            return Ok(());
        }
        packet.dispatch(client)
    }

    pub fn teleport_with_chunks_and_flags(
        &mut self,
        position: EntityPosition,
        chunks: Option<Vec<i64>>,
        flags: TeleportFlags,
    ) -> io::Result<EntityTeleport> {
        self.teleport(position, chunks, flags, true)
    }

    pub fn teleport(
        &mut self,
        position: EntityPosition,
        chunks: Option<Vec<i64>>,
        flags: TeleportFlags,
        should_confirm: bool,
    ) -> io::Result<EntityTeleport> {
        self.teleport_with_velocity_chunks_and_flags(
            position,
            Velocity(Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            chunks,
            flags.with(TeleportFlags::DELTA_COORD),
            should_confirm,
        )
    }

    pub fn teleport_with_velocity_chunks_and_flags(
        &mut self,
        position: EntityPosition,
        velocity: Velocity,
        chunks: Option<Vec<i64>>,
        flags: TeleportFlags,
        should_confirm: bool,
    ) -> io::Result<EntityTeleport> {
        let teleport = EntityTeleport::resolve(
            self.get_position(),
            self.velocity,
            position,
            velocity,
            chunks,
            flags,
        );
        self.set_position(teleport.get_position());
        self.set_velocity(teleport.get_velocity());
        self.synchronize_position_after_teleport(position, velocity.0, flags, should_confirm)?;
        Ok(teleport)
    }

    pub(crate) fn apply_teleport(
        &mut self,
        teleport: &EntityTeleport,
        should_confirm: bool,
    ) -> io::Result<()> {
        self.set_position(teleport.get_position());
        self.set_velocity(teleport.get_velocity());
        self.synchronize_position_after_teleport(
            teleport.get_teleport_position(),
            teleport.get_teleport_velocity().0,
            teleport.get_flags(),
            should_confirm,
        )
    }

    pub(crate) fn teleport_destination(
        &self,
        position: EntityPosition,
        flags: TeleportFlags,
    ) -> EntityPosition {
        EntityTeleport::resolve_position(self.get_position(), position, flags)
    }

    pub(crate) fn look(&mut self, yaw: f32, pitch: f32) {
        self.position = self.position.looking_at(yaw, pitch);
    }

    pub fn look_at_position(&mut self, target: Vector3d) -> io::Result<()> {
        self.face_position(FacePoint::Eyes, target)
    }

    pub fn look_at_entity(
        &mut self,
        entity_id: EntityId,
        target: EntityPosition,
    ) -> io::Result<()> {
        self.face_entity(
            FacePoint::Eyes,
            target.as_vector(),
            entity_id,
            FacePoint::Eyes,
        )
    }

    pub fn face_position(&mut self, face_point: FacePoint, target: Vector3d) -> io::Result<()> {
        self.send_packet(PlayerLookAtPacket::at_position(face_point, target))
    }

    pub fn face_entity(
        &mut self,
        face_point: FacePoint,
        target: Vector3d,
        entity_id: EntityId,
        target_point: FacePoint,
    ) -> io::Result<()> {
        self.send_packet(PlayerLookAtPacket::at_entity(
            face_point,
            target,
            entity_id.get_value(),
            target_point,
        ))
    }

    pub(crate) fn refresh_input(
        &mut self,
        forward: bool,
        backward: bool,
        left: bool,
        right: bool,
        jump: bool,
        shift: bool,
        sprint: bool,
    ) -> bool {
        let old_shift = self.metadata.get_flag(&definitions::is_crouching());
        self.inputs
            .refresh(forward, backward, left, right, jump, shift, sprint);
        self.metadata.set_flag(&definitions::is_crouching(), shift);
        self.refresh_pose();
        old_shift != shift
    }

    pub const fn get_inputs(&self) -> PlayerInputs {
        self.inputs
    }

    pub fn get_eye_height(&self) -> f64 {
        self.entity_type.get_eye_height()
    }

    pub fn get_position(&self) -> crate::entity::EntityPosition {
        crate::entity::EntityPosition::new(
            self.position.x,
            self.position.y,
            self.position.z,
            self.position.yaw,
            self.position.pitch,
        )
    }
}
