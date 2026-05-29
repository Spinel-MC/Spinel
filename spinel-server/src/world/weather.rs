#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Weather {
    rain_level: f32,
    thunder_level: f32,
}

impl Weather {
    pub const CLEAR: Self = Self::new(0.0, 0.0);

    pub const fn new(rain_level: f32, thunder_level: f32) -> Self {
        Self {
            rain_level,
            thunder_level,
        }
    }

    pub const fn rain_level(&self) -> f32 {
        self.rain_level
    }

    pub const fn thunder_level(&self) -> f32 {
        self.thunder_level
    }

    pub fn has_rain(&self) -> bool {
        self.rain_level > 0.0
    }
}
