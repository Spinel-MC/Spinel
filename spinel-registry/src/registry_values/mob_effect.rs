use crate::RegistryCodec;
use spinel_nbt::NbtCompound;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobEffect {
    protocol_id: i32,
    translation_key: String,
    color: i32,
    is_instantaneous: bool,
}

impl MobEffect {
    #[must_use]
    pub fn new(
        protocol_id: i32,
        translation_key: String,
        color: i32,
        is_instantaneous: bool,
    ) -> Self {
        Self {
            protocol_id,
            translation_key,
            color,
            is_instantaneous,
        }
    }

    #[must_use]
    pub const fn get_protocol_id(&self) -> i32 {
        self.protocol_id
    }

    #[must_use]
    pub fn get_translation_key(&self) -> &str {
        &self.translation_key
    }

    #[must_use]
    pub const fn get_color(&self) -> i32 {
        self.color
    }

    #[must_use]
    pub const fn is_instantaneous(&self) -> bool {
        self.is_instantaneous
    }
}

impl Default for MobEffect {
    fn default() -> Self {
        Self::new(0, String::new(), 0, false)
    }
}

impl RegistryCodec for MobEffect {
    fn registry_nbt(&self) -> NbtCompound {
        NbtCompound::new()
    }
}
