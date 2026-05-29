#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EntityBoundingBox {
    width: f64,
    height: f64,
    depth: f64,
}

impl EntityBoundingBox {
    pub const fn new(width: f64, height: f64, depth: f64) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }

    pub const fn width(self) -> f64 {
        self.width
    }

    pub const fn height(self) -> f64 {
        self.height
    }

    pub const fn depth(self) -> f64 {
        self.depth
    }
}
