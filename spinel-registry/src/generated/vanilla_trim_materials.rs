use crate::{DynamicRegistry, RegistryKey};
use crate::trim_material::TrimMaterial;
impl TrimMaterial {
    pub const AMETHYST: RegistryKey<Self> = RegistryKey::vanilla_static("amethyst");
    pub const COPPER: RegistryKey<Self> = RegistryKey::vanilla_static("copper");
    pub const DIAMOND: RegistryKey<Self> = RegistryKey::vanilla_static("diamond");
    pub const EMERALD: RegistryKey<Self> = RegistryKey::vanilla_static("emerald");
    pub const GOLD: RegistryKey<Self> = RegistryKey::vanilla_static("gold");
    pub const IRON: RegistryKey<Self> = RegistryKey::vanilla_static("iron");
    pub const LAPIS: RegistryKey<Self> = RegistryKey::vanilla_static("lapis");
    pub const NETHERITE: RegistryKey<Self> = RegistryKey::vanilla_static("netherite");
    pub const QUARTZ: RegistryKey<Self> = RegistryKey::vanilla_static("quartz");
    pub const REDSTONE: RegistryKey<Self> = RegistryKey::vanilla_static("redstone");
    pub const RESIN: RegistryKey<Self> = RegistryKey::vanilla_static("resin");
}
pub fn register_trim_materials(registry: &mut DynamicRegistry<TrimMaterial>) {
    let _ = registry.register_vanilla(TrimMaterial::AMETHYST, TrimMaterial::default());
    let _ = registry.register_vanilla(TrimMaterial::COPPER, TrimMaterial::default());
    let _ = registry.register_vanilla(TrimMaterial::DIAMOND, TrimMaterial::default());
    let _ = registry.register_vanilla(TrimMaterial::EMERALD, TrimMaterial::default());
    let _ = registry.register_vanilla(TrimMaterial::GOLD, TrimMaterial::default());
    let _ = registry.register_vanilla(TrimMaterial::IRON, TrimMaterial::default());
    let _ = registry.register_vanilla(TrimMaterial::LAPIS, TrimMaterial::default());
    let _ = registry.register_vanilla(TrimMaterial::NETHERITE, TrimMaterial::default());
    let _ = registry.register_vanilla(TrimMaterial::QUARTZ, TrimMaterial::default());
    let _ = registry.register_vanilla(TrimMaterial::REDSTONE, TrimMaterial::default());
    let _ = registry.register_vanilla(TrimMaterial::RESIN, TrimMaterial::default());
}
