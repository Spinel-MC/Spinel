#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SetEntityViewRequest {
    yaw: f32,
    pitch: f32,
    head_rotation: f32,
}

impl SetEntityViewRequest {
    pub const fn new(yaw: f32, pitch: f32) -> Self {
        Self {
            yaw,
            pitch,
            head_rotation: yaw,
        }
    }

    pub const fn with_head_rotation(self, head_rotation: f32) -> Self {
        Self {
            head_rotation,
            ..self
        }
    }

    pub const fn get_yaw(self) -> f32 {
        self.yaw
    }

    pub const fn get_pitch(self) -> f32 {
        self.pitch
    }

    pub const fn get_head_rotation(self) -> f32 {
        self.head_rotation
    }
}
