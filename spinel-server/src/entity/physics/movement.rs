use crate::entity::{EntityId, EntityPosition, EntitySynchronizationMode};
use spinel_core::network::clientbound::play::entity_head_look::EntityHeadLookPacket;
use spinel_core::network::clientbound::play::entity_position::EntityPositionPacket;
use spinel_core::network::clientbound::play::entity_position_and_rotation::EntityPositionAndRotationPacket;
use spinel_core::network::clientbound::play::entity_rotation::EntityRotationPacket;
use spinel_core::network::clientbound::play::entity_teleport::EntityTeleportPacket;
use spinel_core::network::clientbound::play::entity_velocity::EntityVelocityPacket;
use spinel_core::network::clientbound::play::spawn_entity::EntityAngle;
use spinel_network::types::{TeleportFlags, Vector3d};
use spinel_registry::EntityType;

pub(crate) enum EntityMovementPacket {
    Position(EntityPositionPacket),
    PositionAndRotation(EntityPositionAndRotationPacket),
    Rotation(EntityRotationPacket),
    Teleport(EntityTeleportPacket),
}

pub(crate) struct EntityMovement {
    entity_id: EntityId,
    position: EntityPosition,
    packet: Option<EntityMovementPacket>,
    velocity_packet: Option<EntityVelocityPacket>,
    head_look_packet: Option<EntityHeadLookPacket>,
}

impl EntityMovement {
    pub(crate) fn new(
        entity_id: EntityId,
        position: EntityPosition,
        packet: Option<EntityMovementPacket>,
        velocity_packet: Option<EntityVelocityPacket>,
        head_look_packet: Option<EntityHeadLookPacket>,
    ) -> Self {
        Self {
            entity_id,
            position,
            packet,
            velocity_packet,
            head_look_packet,
        }
    }

    pub(crate) const fn get_entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub(crate) const fn get_position(&self) -> EntityPosition {
        self.position
    }

    pub(crate) fn into_packets(
        self,
    ) -> (
        Option<EntityMovementPacket>,
        Option<EntityVelocityPacket>,
        Option<EntityHeadLookPacket>,
    ) {
        (self.packet, self.velocity_packet, self.head_look_packet)
    }
}

impl EntityMovementPacket {
    pub(crate) fn between(
        entity_id: EntityId,
        previous_position: EntityPosition,
        position: EntityPosition,
        is_on_ground: bool,
        synchronization_mode: EntitySynchronizationMode,
        entity_type: EntityType,
    ) -> Self {
        let distance_x = (position.get_x() - previous_position.get_x()).abs();
        let distance_y = (position.get_y() - previous_position.get_y()).abs();
        let distance_z = (position.get_z() - previous_position.get_z()).abs();
        let requires_teleport = distance_x > 8.0 || distance_y > 8.0 || distance_z > 8.0;
        if requires_teleport {
            return Self::Teleport(EntityTeleportPacket {
                entity_id: entity_id.get_value(),
                position: position.as_vector(),
                delta: Vector3d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                yaw: position.get_yaw(),
                pitch: position.get_pitch(),
                flags: TeleportFlags::DELTA_COORD,
                on_ground: is_on_ground,
            });
        }
        let delta = match synchronization_mode {
            EntitySynchronizationMode::GenericSynchronization => EntityPositionPacket::delta,
            EntitySynchronizationMode::VanillaSynchronization => {
                EntityPositionPacket::vanilla_delta
            }
        };
        let delta_x = delta(position.get_x(), previous_position.get_x());
        let delta_y = delta(position.get_y(), previous_position.get_y());
        let delta_z = delta(position.get_z(), previous_position.get_z());
        let position_changed = delta_x != 0 || delta_y != 0 || delta_z != 0;
        let rotation_changed =
            entity_packet_angle_changed(previous_position.get_yaw(), position.get_yaw())
                || entity_packet_angle_changed(previous_position.get_pitch(), position.get_pitch());
        let should_send_vanilla_position_and_rotation = position_changed
            && (rotation_changed || entity_type_requires_position_and_rotation(entity_type));
        if synchronization_mode == EntitySynchronizationMode::GenericSynchronization
            || should_send_vanilla_position_and_rotation
        {
            return Self::PositionAndRotation(EntityPositionAndRotationPacket {
                entity_id: entity_id.get_value(),
                delta_x,
                delta_y,
                delta_z,
                yaw: EntityAngle(position.get_yaw()),
                pitch: EntityAngle(position.get_pitch()),
                on_ground: is_on_ground,
            });
        }
        if position_changed {
            return Self::Position(EntityPositionPacket {
                entity_id: entity_id.get_value(),
                delta_x,
                delta_y,
                delta_z,
                on_ground: is_on_ground,
            });
        }
        Self::Rotation(EntityRotationPacket {
            entity_id: entity_id.get_value(),
            yaw: EntityAngle(position.get_yaw()),
            pitch: EntityAngle(position.get_pitch()),
            on_ground: is_on_ground,
        })
    }
}

fn entity_packet_angle_changed(previous_angle: f32, current_angle: f32) -> bool {
    entity_packet_angle(previous_angle) != entity_packet_angle(current_angle)
}

fn entity_packet_angle(angle: f32) -> u8 {
    (angle * 256.0 / 360.0) as i32 as u8
}

fn entity_type_requires_position_and_rotation(entity_type: EntityType) -> bool {
    matches!(
        entity_type,
        EntityType::ARROW | EntityType::SPECTRAL_ARROW | EntityType::TRIDENT
    )
}
