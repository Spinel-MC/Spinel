use crate::entity::PlayerSpawnPoint;

#[derive(Clone, Copy)]
pub(crate) struct PlayerPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

impl PlayerPosition {
    pub(crate) const fn new(x: f64, y: f64, z: f64, yaw: f32, pitch: f32) -> Self {
        Self {
            x,
            y,
            z,
            yaw,
            pitch,
        }
    }

    pub(crate) fn get_at(&self, x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, self.yaw, self.pitch)
    }

    pub(crate) fn get_looking_at(&self, yaw: f32, pitch: f32) -> Self {
        Self::new(self.x, self.y, self.z, yaw, pitch)
    }
}

impl From<PlayerSpawnPoint> for PlayerPosition {
    fn from(spawn_point: PlayerSpawnPoint) -> Self {
        Self::new(
            spawn_point.x,
            spawn_point.y,
            spawn_point.z,
            spawn_point.yaw,
            spawn_point.pitch,
        )
    }
}
