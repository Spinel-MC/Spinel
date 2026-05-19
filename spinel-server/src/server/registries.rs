use crate::registry_cache::RegistryCache;
use crate::server::MinecraftServer;
use spinel_registry::biome::Biome;
use spinel_registry::{Identifier, RegisterError, Registries, RegistryKey};
use std::sync::atomic::Ordering;

impl MinecraftServer {
    pub fn register_biome(
        &mut self,
        key: Identifier,
        biome: Biome,
    ) -> Result<RegistryKey<Biome>, RegisterError> {
        if self.is_ticking.load(Ordering::SeqCst) {
            return Err(RegisterError::Frozen);
        }
        let biome_key = self.registries.biome_mut().register(key, biome)?;
        self.registry_cache = RegistryCache::new(&self.registries);
        Ok(biome_key)
    }

    pub fn registries(&self) -> &Registries {
        &self.registries
    }

    pub fn registries_mut(&mut self) -> &mut Registries {
        &mut self.registries
    }
}
