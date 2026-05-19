use crate::{DynamicRegistry, RegistryKey};
use crate::cow_variant::CowVariant;
impl CowVariant {
    pub const COLD: RegistryKey<Self> = RegistryKey::vanilla_static("cold");
    pub const TEMPERATE: RegistryKey<Self> = RegistryKey::vanilla_static("temperate");
    pub const WARM: RegistryKey<Self> = RegistryKey::vanilla_static("warm");
}
pub fn register_cow_variants(registry: &mut DynamicRegistry<CowVariant>) {
    let _ = registry.register_vanilla(CowVariant::COLD, CowVariant::default());
    let _ = registry.register_vanilla(CowVariant::TEMPERATE, CowVariant::default());
    let _ = registry.register_vanilla(CowVariant::WARM, CowVariant::default());
}
