use crate::{DynamicRegistry, RegistryKey};
use crate::wolf_sound_variant::WolfSoundVariant;
impl WolfSoundVariant {
    pub const ANGRY: RegistryKey<Self> = RegistryKey::vanilla_static("angry");
    pub const BIG: RegistryKey<Self> = RegistryKey::vanilla_static("big");
    pub const CLASSIC: RegistryKey<Self> = RegistryKey::vanilla_static("classic");
    pub const CUTE: RegistryKey<Self> = RegistryKey::vanilla_static("cute");
    pub const GRUMPY: RegistryKey<Self> = RegistryKey::vanilla_static("grumpy");
    pub const PUGLIN: RegistryKey<Self> = RegistryKey::vanilla_static("puglin");
    pub const SAD: RegistryKey<Self> = RegistryKey::vanilla_static("sad");
}
pub fn register_wolf_sound_variants(registry: &mut DynamicRegistry<WolfSoundVariant>) {
    let _ = registry.register_vanilla(WolfSoundVariant::ANGRY, WolfSoundVariant::default());
    let _ = registry.register_vanilla(WolfSoundVariant::BIG, WolfSoundVariant::default());
    let _ = registry.register_vanilla(WolfSoundVariant::CLASSIC, WolfSoundVariant::default());
    let _ = registry.register_vanilla(WolfSoundVariant::CUTE, WolfSoundVariant::default());
    let _ = registry.register_vanilla(WolfSoundVariant::GRUMPY, WolfSoundVariant::default());
    let _ = registry.register_vanilla(WolfSoundVariant::PUGLIN, WolfSoundVariant::default());
    let _ = registry.register_vanilla(WolfSoundVariant::SAD, WolfSoundVariant::default());
}
