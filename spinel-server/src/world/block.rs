#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Block {
    pub state_id: i32,
}

impl Block {
    pub const fn air() -> Self {
        Self { state_id: 0 }
    }

    pub const fn grass_block() -> Self {
        Self { state_id: 8 }
    }
}
