use crate::entity::EntityPosition;
use spinel_network::types::{TeleportFlags, Vector3d, Velocity};

#[derive(Clone, Debug)]
pub struct EntityTeleportRequest {
    position: EntityPosition,
    velocity: Velocity,
    chunks: Option<Vec<i64>>,
    flags: TeleportFlags,
    should_confirm: bool,
}

impl From<EntityPosition> for EntityTeleportRequest {
    fn from(position: EntityPosition) -> Self {
        Self {
            position,
            velocity: Velocity(Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            chunks: None,
            flags: TeleportFlags::absolute().with(TeleportFlags::DELTA_COORD),
            should_confirm: true,
        }
    }
}

impl EntityTeleportRequest {
    pub const fn get_position(&self) -> EntityPosition {
        self.position
    }

    pub const fn get_velocity(&self) -> Velocity {
        self.velocity
    }

    pub fn into_chunks(self) -> Option<Vec<i64>> {
        self.chunks
    }

    pub const fn get_flags(&self) -> TeleportFlags {
        self.flags
    }

    pub const fn should_confirm(&self) -> bool {
        self.should_confirm
    }

    pub fn with_velocity(mut self, velocity: Velocity) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn with_chunks(mut self, chunks: Vec<i64>) -> Self {
        self.chunks = Some(chunks);
        self.flags = self.flags.with(TeleportFlags::DELTA_COORD);
        self
    }

    pub const fn with_flags(mut self, flags: TeleportFlags) -> Self {
        self.flags = flags;
        self
    }

    pub const fn without_confirmation(mut self) -> Self {
        self.should_confirm = false;
        self
    }
}

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
            current_position.get_x() + teleport_position.get_x()
        } else {
            teleport_position.get_x()
        };
        let y = if flags.contains(TeleportFlags::Y) {
            current_position.get_y() + teleport_position.get_y()
        } else {
            teleport_position.get_y()
        };
        let z = if flags.contains(TeleportFlags::Z) {
            current_position.get_z() + teleport_position.get_z()
        } else {
            teleport_position.get_z()
        };
        let yaw = if flags.contains(TeleportFlags::Y_ROTATION) {
            current_position.get_yaw() + teleport_position.get_yaw()
        } else {
            teleport_position.get_yaw()
        };
        let pitch = if flags.contains(TeleportFlags::X_ROTATION) {
            current_position.get_pitch() + teleport_position.get_pitch()
        } else {
            teleport_position.get_pitch()
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

    pub const fn get_teleport_position(&self) -> EntityPosition {
        self.teleport_position
    }

    pub const fn get_position(&self) -> EntityPosition {
        self.position
    }

    pub const fn get_teleport_velocity(&self) -> Velocity {
        self.teleport_velocity
    }

    pub const fn get_velocity(&self) -> Velocity {
        self.velocity
    }

    pub fn get_chunks(&self) -> Option<&[i64]> {
        self.chunks.as_deref()
    }

    pub fn into_chunks(self) -> Option<Vec<i64>> {
        self.chunks
    }

    pub const fn get_flags(&self) -> TeleportFlags {
        self.flags
    }
}
