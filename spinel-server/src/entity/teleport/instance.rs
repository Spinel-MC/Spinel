use crate::entity::EntityPosition;
use spinel_network::types::{TeleportFlags, Vector3d, Velocity};

#[derive(Clone, Debug)]
pub struct EntityTeleport {
    teleport_position: EntityPosition,
    position: EntityPosition,
    teleport_velocity: Velocity,
    velocity: Velocity,
    chunks: Option<Vec<i64>>,
    flags: TeleportFlags,
}

impl EntityTeleport {
    pub fn resolve(
        current_position: EntityPosition,
        current_velocity: Velocity,
        teleport_position: EntityPosition,
        teleport_velocity: Velocity,
        chunks: Option<Vec<i64>>,
        flags: TeleportFlags,
    ) -> Self {
        Self {
            teleport_position,
            position: Self::resolve_position(current_position, teleport_position, flags),
            teleport_velocity,
            velocity: Self::resolve_velocity(current_velocity, teleport_velocity, flags),
            chunks,
            flags,
        }
    }

    pub fn resolve_position(
        current_position: EntityPosition,
        teleport_position: EntityPosition,
        flags: TeleportFlags,
    ) -> EntityPosition {
        let x = if flags.contains(TeleportFlags::X) {
            current_position.x() + teleport_position.x()
        } else {
            teleport_position.x()
        };
        let y = if flags.contains(TeleportFlags::Y) {
            current_position.y() + teleport_position.y()
        } else {
            teleport_position.y()
        };
        let z = if flags.contains(TeleportFlags::Z) {
            current_position.z() + teleport_position.z()
        } else {
            teleport_position.z()
        };
        let yaw = if flags.contains(TeleportFlags::Y_ROTATION) {
            current_position.yaw() + teleport_position.yaw()
        } else {
            teleport_position.yaw()
        };
        let pitch = if flags.contains(TeleportFlags::X_ROTATION) {
            current_position.pitch() + teleport_position.pitch()
        } else {
            teleport_position.pitch()
        };
        EntityPosition::new(x, y, z, yaw, pitch)
    }

    pub fn resolve_velocity(
        current_velocity: Velocity,
        teleport_velocity: Velocity,
        flags: TeleportFlags,
    ) -> Velocity {
        Velocity(Vector3d {
            x: if flags.contains(TeleportFlags::DELTA_X) {
                current_velocity.0.x + teleport_velocity.0.x
            } else {
                teleport_velocity.0.x
            },
            y: if flags.contains(TeleportFlags::DELTA_Y) {
                current_velocity.0.y + teleport_velocity.0.y
            } else {
                teleport_velocity.0.y
            },
            z: if flags.contains(TeleportFlags::DELTA_Z) {
                current_velocity.0.z + teleport_velocity.0.z
            } else {
                teleport_velocity.0.z
            },
        })
    }

    pub const fn teleport_position(&self) -> EntityPosition {
        self.teleport_position
    }

    pub const fn position(&self) -> EntityPosition {
        self.position
    }

    pub const fn teleport_velocity(&self) -> Velocity {
        self.teleport_velocity
    }

    pub const fn velocity(&self) -> Velocity {
        self.velocity
    }

    pub fn chunks(&self) -> Option<&[i64]> {
        self.chunks.as_deref()
    }

    pub fn into_chunks(self) -> Option<Vec<i64>> {
        self.chunks
    }

    pub const fn flags(&self) -> TeleportFlags {
        self.flags
    }
}
