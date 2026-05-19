use crate::{DynamicRegistry, RegistryKey};
use crate::pig_variant::PigVariant;
impl PigVariant {
    pub const COLD: RegistryKey<Self> = RegistryKey::vanilla_static("cold");
    pub const TEMPERATE: RegistryKey<Self> = RegistryKey::vanilla_static("temperate");
    pub const WARM: RegistryKey<Self> = RegistryKey::vanilla_static("warm");
}
pub fn register_pig_variants(registry: &mut DynamicRegistry<PigVariant>) {
    let _ = registry.register_vanilla(PigVariant::COLD, PigVariant::default());
    let _ = registry.register_vanilla(PigVariant::TEMPERATE, PigVariant::default());
    let _ = registry.register_vanilla(PigVariant::WARM, PigVariant::default());
}
