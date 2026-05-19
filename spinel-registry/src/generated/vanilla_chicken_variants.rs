use crate::{DynamicRegistry, RegistryKey};
use crate::chicken_variant::ChickenVariant;
impl ChickenVariant {
    pub const COLD: RegistryKey<Self> = RegistryKey::vanilla_static("cold");
    pub const TEMPERATE: RegistryKey<Self> = RegistryKey::vanilla_static("temperate");
    pub const WARM: RegistryKey<Self> = RegistryKey::vanilla_static("warm");
}
pub fn register_chicken_variants(registry: &mut DynamicRegistry<ChickenVariant>) {
    let _ = registry.register_vanilla(ChickenVariant::COLD, ChickenVariant::default());
    let _ = registry.register_vanilla(ChickenVariant::TEMPERATE, ChickenVariant::default());
    let _ = registry.register_vanilla(ChickenVariant::WARM, ChickenVariant::default());
}
