use crate::{DynamicRegistry, RegistryKey};
use crate::trim_pattern::TrimPattern;
impl TrimPattern {
    pub const BOLT: RegistryKey<Self> = RegistryKey::vanilla_static("bolt");
    pub const COAST: RegistryKey<Self> = RegistryKey::vanilla_static("coast");
    pub const DUNE: RegistryKey<Self> = RegistryKey::vanilla_static("dune");
    pub const EYE: RegistryKey<Self> = RegistryKey::vanilla_static("eye");
    pub const FLOW: RegistryKey<Self> = RegistryKey::vanilla_static("flow");
    pub const HOST: RegistryKey<Self> = RegistryKey::vanilla_static("host");
    pub const RAISER: RegistryKey<Self> = RegistryKey::vanilla_static("raiser");
    pub const RIB: RegistryKey<Self> = RegistryKey::vanilla_static("rib");
    pub const SENTRY: RegistryKey<Self> = RegistryKey::vanilla_static("sentry");
    pub const SHAPER: RegistryKey<Self> = RegistryKey::vanilla_static("shaper");
    pub const SILENCE: RegistryKey<Self> = RegistryKey::vanilla_static("silence");
    pub const SNOUT: RegistryKey<Self> = RegistryKey::vanilla_static("snout");
    pub const SPIRE: RegistryKey<Self> = RegistryKey::vanilla_static("spire");
    pub const TIDE: RegistryKey<Self> = RegistryKey::vanilla_static("tide");
    pub const VEX: RegistryKey<Self> = RegistryKey::vanilla_static("vex");
    pub const WARD: RegistryKey<Self> = RegistryKey::vanilla_static("ward");
    pub const WAYFINDER: RegistryKey<Self> = RegistryKey::vanilla_static("wayfinder");
    pub const WILD: RegistryKey<Self> = RegistryKey::vanilla_static("wild");
}
pub fn register_trim_patterns(registry: &mut DynamicRegistry<TrimPattern>) {
    let _ = registry.register_vanilla(TrimPattern::BOLT, TrimPattern::default());
    let _ = registry.register_vanilla(TrimPattern::COAST, TrimPattern::default());
    let _ = registry.register_vanilla(TrimPattern::DUNE, TrimPattern::default());
    let _ = registry.register_vanilla(TrimPattern::EYE, TrimPattern::default());
    let _ = registry.register_vanilla(TrimPattern::FLOW, TrimPattern::default());
    let _ = registry.register_vanilla(TrimPattern::HOST, TrimPattern::default());
    let _ = registry.register_vanilla(TrimPattern::RAISER, TrimPattern::default());
    let _ = registry.register_vanilla(TrimPattern::RIB, TrimPattern::default());
    let _ = registry.register_vanilla(TrimPattern::SENTRY, TrimPattern::default());
    let _ = registry.register_vanilla(TrimPattern::SHAPER, TrimPattern::default());
    let _ = registry.register_vanilla(TrimPattern::SILENCE, TrimPattern::default());
    let _ = registry.register_vanilla(TrimPattern::SNOUT, TrimPattern::default());
    let _ = registry.register_vanilla(TrimPattern::SPIRE, TrimPattern::default());
    let _ = registry.register_vanilla(TrimPattern::TIDE, TrimPattern::default());
    let _ = registry.register_vanilla(TrimPattern::VEX, TrimPattern::default());
    let _ = registry.register_vanilla(TrimPattern::WARD, TrimPattern::default());
    let _ = registry.register_vanilla(TrimPattern::WAYFINDER, TrimPattern::default());
    let _ = registry.register_vanilla(TrimPattern::WILD, TrimPattern::default());
}
