use crate::{DynamicRegistry, RegistryKey};
use crate::cat_variant::CatVariant;
impl CatVariant {
    pub const ALL_BLACK: RegistryKey<Self> = RegistryKey::vanilla_static("all_black");
    pub const BLACK: RegistryKey<Self> = RegistryKey::vanilla_static("black");
    pub const BRITISH_SHORTHAIR: RegistryKey<Self> = RegistryKey::vanilla_static("british_shorthair");
    pub const CALICO: RegistryKey<Self> = RegistryKey::vanilla_static("calico");
    pub const JELLIE: RegistryKey<Self> = RegistryKey::vanilla_static("jellie");
    pub const PERSIAN: RegistryKey<Self> = RegistryKey::vanilla_static("persian");
    pub const RAGDOLL: RegistryKey<Self> = RegistryKey::vanilla_static("ragdoll");
    pub const RED: RegistryKey<Self> = RegistryKey::vanilla_static("red");
    pub const SIAMESE: RegistryKey<Self> = RegistryKey::vanilla_static("siamese");
    pub const TABBY: RegistryKey<Self> = RegistryKey::vanilla_static("tabby");
    pub const WHITE: RegistryKey<Self> = RegistryKey::vanilla_static("white");
}
pub fn register_cat_variants(registry: &mut DynamicRegistry<CatVariant>) {
    let _ = registry.register_vanilla(CatVariant::ALL_BLACK, CatVariant::default());
    let _ = registry.register_vanilla(CatVariant::BLACK, CatVariant::default());
    let _ = registry.register_vanilla(CatVariant::BRITISH_SHORTHAIR, CatVariant::default());
    let _ = registry.register_vanilla(CatVariant::CALICO, CatVariant::default());
    let _ = registry.register_vanilla(CatVariant::JELLIE, CatVariant::default());
    let _ = registry.register_vanilla(CatVariant::PERSIAN, CatVariant::default());
    let _ = registry.register_vanilla(CatVariant::RAGDOLL, CatVariant::default());
    let _ = registry.register_vanilla(CatVariant::RED, CatVariant::default());
    let _ = registry.register_vanilla(CatVariant::SIAMESE, CatVariant::default());
    let _ = registry.register_vanilla(CatVariant::TABBY, CatVariant::default());
    let _ = registry.register_vanilla(CatVariant::WHITE, CatVariant::default());
}
