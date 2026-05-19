use crate::{DynamicRegistry, RegistryKey};
use crate::frog_variant::FrogVariant;
impl FrogVariant {
    pub const COLD: RegistryKey<Self> = RegistryKey::vanilla_static("cold");
    pub const TEMPERATE: RegistryKey<Self> = RegistryKey::vanilla_static("temperate");
    pub const WARM: RegistryKey<Self> = RegistryKey::vanilla_static("warm");
}
pub fn register_frog_variants(registry: &mut DynamicRegistry<FrogVariant>) {
    let _ = registry.register_vanilla(FrogVariant::COLD, FrogVariant::default());
    let _ = registry.register_vanilla(FrogVariant::TEMPERATE, FrogVariant::default());
    let _ = registry.register_vanilla(FrogVariant::WARM, FrogVariant::default());
}
