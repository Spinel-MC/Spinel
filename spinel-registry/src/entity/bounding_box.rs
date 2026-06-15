#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EntityBoundingBox {
    minimum_x: f64,
    minimum_y: f64,
    minimum_z: f64,
    maximum_x: f64,
    maximum_y: f64,
    maximum_z: f64,
}

impl EntityBoundingBox {
    pub const fn new(width: f64, height: f64, depth: f64) -> Self {
        Self {
            minimum_x: -width / 2.0,
            minimum_y: 0.0,
            minimum_z: -depth / 2.0,
            maximum_x: width / 2.0,
            maximum_y: height,
            maximum_z: depth / 2.0,
        }
    }

    pub const fn with_offset(self, x: f64, y: f64, z: f64) -> Self {
        Self {
            minimum_x: x,
            minimum_y: y,
            minimum_z: z,
            maximum_x: x + self.width(),
            maximum_y: y + self.height(),
            maximum_z: z + self.depth(),
        }
    }

    pub const fn width(self) -> f64 {
        self.maximum_x - self.minimum_x
    }

    pub const fn height(self) -> f64 {
        self.maximum_y - self.minimum_y
    }

    pub const fn depth(self) -> f64 {
        self.maximum_z - self.minimum_z
    }

    pub const fn minimum_x(self) -> f64 {
        self.minimum_x
    }

    pub const fn minimum_y(self) -> f64 {
        self.minimum_y
    }

    pub const fn minimum_z(self) -> f64 {
        self.minimum_z
    }

    pub const fn maximum_x(self) -> f64 {
        self.maximum_x
    }

    pub const fn maximum_y(self) -> f64 {
        self.maximum_y
    }

    pub const fn maximum_z(self) -> f64 {
        self.maximum_z
    }
}
