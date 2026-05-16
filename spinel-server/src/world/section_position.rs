#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SectionPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl SectionPosition {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}
