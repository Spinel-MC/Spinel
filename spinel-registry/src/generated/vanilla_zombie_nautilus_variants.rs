use crate::{DynamicRegistry, RegistryKey};
use crate::zombie_nautilus_variant::ZombieNautilusVariant;
impl ZombieNautilusVariant {
    pub const TEMPERATE: RegistryKey<Self> = RegistryKey::vanilla_static("temperate");
    pub const WARM: RegistryKey<Self> = RegistryKey::vanilla_static("warm");
}
pub fn register_zombie_nautilus_variants(registry: &mut DynamicRegistry<ZombieNautilusVariant>) {
    let _ = registry.register_vanilla(ZombieNautilusVariant::TEMPERATE, ZombieNautilusVariant::default());
    let _ = registry.register_vanilla(ZombieNautilusVariant::WARM, ZombieNautilusVariant::default());
}
