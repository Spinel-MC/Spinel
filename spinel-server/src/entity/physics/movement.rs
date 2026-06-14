use crate::entity::{EntityId, EntityPosition};
use spinel_core::network::clientbound::play::entity_position_and_rotation::EntityPositionAndRotationPacket;
use spinel_core::network::clientbound::play::entity_teleport::EntityTeleportPacket;
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
}

impl EntityMovement {
    pub(crate) fn new(
        entity_id: EntityId,
        position: EntityPosition,
        packet: Option<EntityMovementPacket>,
    ) -> Self {
        Self {
            entity_id,
            position,
            packet,
        }
    }

    pub(crate) const fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub(crate) const fn position(&self) -> EntityPosition {
        self.position
    }

    pub(crate) fn packet(self) -> Option<EntityMovementPacket> {
        self.packet
    }
}

impl EntityMovementPacket {
    pub(crate) fn between(
        entity_id: EntityId,
        previous_position: EntityPosition,
        position: EntityPosition,
        is_on_ground: bool,
    ) -> Self {
        let distance_x = (position.x() - previous_position.x()).abs();
        let distance_y = (position.y() - previous_position.y()).abs();
        let distance_z = (position.z() - previous_position.z()).abs();
        let requires_teleport = distance_x > 8.0 || distance_y > 8.0 || distance_z > 8.0;
        if requires_teleport {
            return Self::Teleport(EntityTeleportPacket {
                entity_id: entity_id.value(),
                position: position.as_vector(),
                delta: Vector3d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                yaw: position.yaw(),
                pitch: position.pitch(),
                flags: TeleportFlags::DELTA_COORD,
                on_ground: is_on_ground,
            });
        }
        Self::Position(EntityPositionAndRotationPacket {
            entity_id: entity_id.value(),
            delta_x: spinel_core::network::clientbound::play::entity_position::EntityPositionPacket::delta(
                position.x(),
                previous_position.x(),
            ),
            delta_y: spinel_core::network::clientbound::play::entity_position::EntityPositionPacket::delta(
                position.y(),
                previous_position.y(),
            ),
            delta_z: spinel_core::network::clientbound::play::entity_position::EntityPositionPacket::delta(
                position.z(),
                previous_position.z(),
            ),
            yaw: EntityAngle(position.yaw()),
            pitch: EntityAngle(position.pitch()),
            on_ground: is_on_ground,
        })
    }
}
