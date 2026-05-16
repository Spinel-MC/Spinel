#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BlockSize {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockSize {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}
