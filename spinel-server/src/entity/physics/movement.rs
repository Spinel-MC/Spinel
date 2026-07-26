use crate::entity::{EntityId, EntityPosition, EntitySynchronizationMode};
use spinel_core::network::clientbound::play::entity_head_look::EntityHeadLookPacket;
use spinel_core::network::clientbound::play::entity_position_and_rotation::EntityPositionAndRotationPacket;
use spinel_core::network::clientbound::play::entity_teleport::EntityTeleportPacket;
use spinel_core::network::clientbound::play::entity_velocity::EntityVelocityPacket;
use spinel_core::network::clientbound::play::spawn_entity::EntityAngle;
use spinel_network::types::{TeleportFlags, Vector3d};

pub(crate) enum EntityMovementPacket {
    Position(EntityPositionAndRotationPacket),
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
            EntitySynchronizationMode::GenericSynchronization => {
                spinel_core::network::clientbound::play::entity_position::EntityPositionPacket::delta
            }
            EntitySynchronizationMode::VanillaSynchronization => {
                spinel_core::network::clientbound::play::entity_position::EntityPositionPacket::vanilla_delta
            }
        };
        Self::Position(EntityPositionAndRotationPacket {
            entity_id: entity_id.get_value(),
            delta_x: delta(position.get_x(), previous_position.get_x()),
            delta_y: delta(position.get_y(), previous_position.get_y()),
            delta_z: delta(position.get_z(), previous_position.get_z()),
            yaw: EntityAngle(position.get_yaw()),
            pitch: EntityAngle(position.get_pitch()),
            on_ground: is_on_ground,
        })
    }
}
