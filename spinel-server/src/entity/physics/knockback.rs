use spinel_network::types::{Vector3d, Velocity};

const SERVER_TICKS_PER_SECOND: f64 = 20.0;
const VERTICAL_KNOCKBACK_LIMIT_PER_TICK: f64 = 0.4;

pub fn knockback_velocity(
    velocity: Velocity,
    is_on_ground: bool,
    strength: f32,
    x: f64,
    z: f64,
) -> Velocity {
    if strength <= 0.0 {
        return velocity;
    }

    let direction_length = x.hypot(z);
    let normalized_x = if direction_length == 0.0 {
        0.0
    } else {
        x / direction_length
    };
    let normalized_z = if direction_length == 0.0 {
        0.0
    } else {
        z / direction_length
    };
    let scaled_strength = f64::from(strength) * SERVER_TICKS_PER_SECOND;
    let vertical_limit = VERTICAL_KNOCKBACK_LIMIT_PER_TICK * SERVER_TICKS_PER_SECOND;

    Velocity(Vector3d {
        x: velocity.0.x / 2.0 - normalized_x * scaled_strength,
        y: if is_on_ground {
            vertical_limit.min(velocity.0.y / 2.0 + scaled_strength)
        } else {
            velocity.0.y
        },
        z: velocity.0.z / 2.0 - normalized_z * scaled_strength,
    })
}
