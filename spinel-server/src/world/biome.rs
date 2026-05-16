#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Biome {
    pub registry_id: i32,
}

impl Biome {
    pub const fn plains() -> Self {
        Self { registry_id: 1 }
    }
}
