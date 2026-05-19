use crate::{DynamicRegistry, RegistryKey};
use crate::timeline::Timeline;
impl Timeline {
    pub const DAY: RegistryKey<Self> = RegistryKey::vanilla_static("day");
    pub const EARLY_GAME: RegistryKey<Self> = RegistryKey::vanilla_static("early_game");
    pub const MOON: RegistryKey<Self> = RegistryKey::vanilla_static("moon");
    pub const VILLAGER_SCHEDULE: RegistryKey<Self> = RegistryKey::vanilla_static("villager_schedule");
}
pub fn register_timelines(registry: &mut DynamicRegistry<Timeline>) {
    let _ = registry.register_vanilla(Timeline::DAY, Timeline::default());
    let _ = registry.register_vanilla(Timeline::EARLY_GAME, Timeline::default());
    let _ = registry.register_vanilla(Timeline::MOON, Timeline::default());
    let _ = registry.register_vanilla(Timeline::VILLAGER_SCHEDULE, Timeline::default());
}
