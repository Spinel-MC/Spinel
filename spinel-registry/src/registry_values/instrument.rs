use crate::RegistryCodec;
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Debug, PartialEq)]
pub struct Instrument {
    data: NbtCompound,
    use_duration_ticks: i32,
}

impl Instrument {
    pub fn raw(data: NbtCompound) -> Self {
        let use_duration_ticks = data
            .get("use_duration")
            .and_then(|tag| match tag {
                Nbt::Int(use_duration_ticks) => Some(*use_duration_ticks),
                _ => None,
            })
            .unwrap_or(140);
        Self {
            data,
            use_duration_ticks,
        }
    }

    pub const fn use_duration_ticks(&self) -> i32 {
        self.use_duration_ticks
    }
}

impl Default for Instrument {
    fn default() -> Self {
        Self {
            data: NbtCompound::new(),
            use_duration_ticks: 140,
        }
    }
}

impl RegistryCodec for Instrument {
    fn registry_nbt(&self) -> NbtCompound {
        self.data.clone()
    }
}
