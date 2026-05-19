use crate::{DynamicRegistry, RegistryKey};
use crate::instrument::Instrument;
impl Instrument {
    pub const ADMIRE_GOAT_HORN: RegistryKey<Self> = RegistryKey::vanilla_static("admire_goat_horn");
    pub const CALL_GOAT_HORN: RegistryKey<Self> = RegistryKey::vanilla_static("call_goat_horn");
    pub const DREAM_GOAT_HORN: RegistryKey<Self> = RegistryKey::vanilla_static("dream_goat_horn");
    pub const FEEL_GOAT_HORN: RegistryKey<Self> = RegistryKey::vanilla_static("feel_goat_horn");
    pub const PONDER_GOAT_HORN: RegistryKey<Self> = RegistryKey::vanilla_static("ponder_goat_horn");
    pub const SEEK_GOAT_HORN: RegistryKey<Self> = RegistryKey::vanilla_static("seek_goat_horn");
    pub const SING_GOAT_HORN: RegistryKey<Self> = RegistryKey::vanilla_static("sing_goat_horn");
    pub const YEARN_GOAT_HORN: RegistryKey<Self> = RegistryKey::vanilla_static("yearn_goat_horn");
}
pub fn register_instruments(registry: &mut DynamicRegistry<Instrument>) {
    let _ = registry.register_vanilla(Instrument::ADMIRE_GOAT_HORN, Instrument::default());
    let _ = registry.register_vanilla(Instrument::CALL_GOAT_HORN, Instrument::default());
    let _ = registry.register_vanilla(Instrument::DREAM_GOAT_HORN, Instrument::default());
    let _ = registry.register_vanilla(Instrument::FEEL_GOAT_HORN, Instrument::default());
    let _ = registry.register_vanilla(Instrument::PONDER_GOAT_HORN, Instrument::default());
    let _ = registry.register_vanilla(Instrument::SEEK_GOAT_HORN, Instrument::default());
    let _ = registry.register_vanilla(Instrument::SING_GOAT_HORN, Instrument::default());
    let _ = registry.register_vanilla(Instrument::YEARN_GOAT_HORN, Instrument::default());
}
