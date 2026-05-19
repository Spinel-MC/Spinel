use crate::{DynamicRegistry, RegistryKey};
use crate::wolf_variant::WolfVariant;
impl WolfVariant {
    pub const ASHEN: RegistryKey<Self> = RegistryKey::vanilla_static("ashen");
    pub const BLACK: RegistryKey<Self> = RegistryKey::vanilla_static("black");
    pub const CHESTNUT: RegistryKey<Self> = RegistryKey::vanilla_static("chestnut");
    pub const PALE: RegistryKey<Self> = RegistryKey::vanilla_static("pale");
    pub const RUSTY: RegistryKey<Self> = RegistryKey::vanilla_static("rusty");
    pub const SNOWY: RegistryKey<Self> = RegistryKey::vanilla_static("snowy");
    pub const SPOTTED: RegistryKey<Self> = RegistryKey::vanilla_static("spotted");
    pub const STRIPED: RegistryKey<Self> = RegistryKey::vanilla_static("striped");
    pub const WOODS: RegistryKey<Self> = RegistryKey::vanilla_static("woods");
}
pub fn register_wolf_variants(registry: &mut DynamicRegistry<WolfVariant>) {
    let _ = registry.register_vanilla(WolfVariant::ASHEN, WolfVariant::default());
    let _ = registry.register_vanilla(WolfVariant::BLACK, WolfVariant::default());
    let _ = registry.register_vanilla(WolfVariant::CHESTNUT, WolfVariant::default());
    let _ = registry.register_vanilla(WolfVariant::PALE, WolfVariant::default());
    let _ = registry.register_vanilla(WolfVariant::RUSTY, WolfVariant::default());
    let _ = registry.register_vanilla(WolfVariant::SNOWY, WolfVariant::default());
    let _ = registry.register_vanilla(WolfVariant::SPOTTED, WolfVariant::default());
    let _ = registry.register_vanilla(WolfVariant::STRIPED, WolfVariant::default());
    let _ = registry.register_vanilla(WolfVariant::WOODS, WolfVariant::default());
}
