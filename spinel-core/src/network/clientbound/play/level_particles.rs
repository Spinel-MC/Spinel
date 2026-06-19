use spinel_macros::packet;
use spinel_network::types::{Particle, Vector3d};

#[packet(id: "level_particles", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct LevelParticlesPacket {
    pub override_limiter: bool,
    pub always_show: bool,
    pub position: Vector3d,
    pub offset_x: f32,
    pub offset_y: f32,
    pub offset_z: f32,
    pub max_speed: f32,
    pub count: i32,
    pub particle: Particle,
}
