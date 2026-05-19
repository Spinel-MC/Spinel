use crate::{Identifier, RegistryCodec};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZombieNautilusVariant {
    pub model: ZombieNautilusModel,
    pub asset_id: Identifier,
}

impl ZombieNautilusVariant {
    #[must_use]
    pub fn new(model: ZombieNautilusModel, asset_id: Identifier) -> Self {
        Self { model, asset_id }
    }
}

impl Default for ZombieNautilusVariant {
    fn default() -> Self {
        Self::new(
            ZombieNautilusModel::Normal,
            Identifier::minecraft("entity/nautilus/zombie_nautilus"),
        )
    }
}

impl RegistryCodec for ZombieNautilusVariant {
    fn registry_nbt(&self) -> NbtCompound {
        let mut variant_nbt = NbtCompound::new();
        if self.model != ZombieNautilusModel::Normal {
            variant_nbt.insert(
                "model".to_string(),
                Nbt::String(self.model.as_str().to_string()),
            );
        }
        variant_nbt.insert(
            "asset_id".to_string(),
            Nbt::String(self.asset_id.to_string()),
        );
        variant_nbt
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZombieNautilusModel {
    Normal,
    Warm,
}

impl ZombieNautilusModel {
    fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Warm => "warm",
        }
    }
}
