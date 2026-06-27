use crate::{DynamicRegistry, Identifier, RegistryKey};
use crate::enchantment::Enchantment;
use spinel_nbt::parse_snbt_compound;
use std::collections::BTreeMap;
const ENCHANTMENT_ENTRIES: &str = include_str!("../../assets/enchantments.json");
impl Enchantment {
    pub const AQUA_AFFINITY: RegistryKey<Self> = RegistryKey::vanilla_static("aqua_affinity");
    pub const BANE_OF_ARTHROPODS: RegistryKey<Self> = RegistryKey::vanilla_static("bane_of_arthropods");
    pub const BINDING_CURSE: RegistryKey<Self> = RegistryKey::vanilla_static("binding_curse");
    pub const BLAST_PROTECTION: RegistryKey<Self> = RegistryKey::vanilla_static("blast_protection");
    pub const BREACH: RegistryKey<Self> = RegistryKey::vanilla_static("breach");
    pub const CHANNELING: RegistryKey<Self> = RegistryKey::vanilla_static("channeling");
    pub const DENSITY: RegistryKey<Self> = RegistryKey::vanilla_static("density");
    pub const DEPTH_STRIDER: RegistryKey<Self> = RegistryKey::vanilla_static("depth_strider");
    pub const EFFICIENCY: RegistryKey<Self> = RegistryKey::vanilla_static("efficiency");
    pub const FEATHER_FALLING: RegistryKey<Self> = RegistryKey::vanilla_static("feather_falling");
    pub const FIRE_ASPECT: RegistryKey<Self> = RegistryKey::vanilla_static("fire_aspect");
    pub const FIRE_PROTECTION: RegistryKey<Self> = RegistryKey::vanilla_static("fire_protection");
    pub const FLAME: RegistryKey<Self> = RegistryKey::vanilla_static("flame");
    pub const FORTUNE: RegistryKey<Self> = RegistryKey::vanilla_static("fortune");
    pub const FROST_WALKER: RegistryKey<Self> = RegistryKey::vanilla_static("frost_walker");
    pub const IMPALING: RegistryKey<Self> = RegistryKey::vanilla_static("impaling");
    pub const INFINITY: RegistryKey<Self> = RegistryKey::vanilla_static("infinity");
    pub const KNOCKBACK: RegistryKey<Self> = RegistryKey::vanilla_static("knockback");
    pub const LOOTING: RegistryKey<Self> = RegistryKey::vanilla_static("looting");
    pub const LOYALTY: RegistryKey<Self> = RegistryKey::vanilla_static("loyalty");
    pub const LUCK_OF_THE_SEA: RegistryKey<Self> = RegistryKey::vanilla_static("luck_of_the_sea");
    pub const LUNGE: RegistryKey<Self> = RegistryKey::vanilla_static("lunge");
    pub const LURE: RegistryKey<Self> = RegistryKey::vanilla_static("lure");
    pub const MENDING: RegistryKey<Self> = RegistryKey::vanilla_static("mending");
    pub const MULTISHOT: RegistryKey<Self> = RegistryKey::vanilla_static("multishot");
    pub const PIERCING: RegistryKey<Self> = RegistryKey::vanilla_static("piercing");
    pub const POWER: RegistryKey<Self> = RegistryKey::vanilla_static("power");
    pub const PROJECTILE_PROTECTION: RegistryKey<Self> = RegistryKey::vanilla_static("projectile_protection");
    pub const PROTECTION: RegistryKey<Self> = RegistryKey::vanilla_static("protection");
    pub const PUNCH: RegistryKey<Self> = RegistryKey::vanilla_static("punch");
    pub const QUICK_CHARGE: RegistryKey<Self> = RegistryKey::vanilla_static("quick_charge");
    pub const RESPIRATION: RegistryKey<Self> = RegistryKey::vanilla_static("respiration");
    pub const RIPTIDE: RegistryKey<Self> = RegistryKey::vanilla_static("riptide");
    pub const SHARPNESS: RegistryKey<Self> = RegistryKey::vanilla_static("sharpness");
    pub const SILK_TOUCH: RegistryKey<Self> = RegistryKey::vanilla_static("silk_touch");
    pub const SMITE: RegistryKey<Self> = RegistryKey::vanilla_static("smite");
    pub const SOUL_SPEED: RegistryKey<Self> = RegistryKey::vanilla_static("soul_speed");
    pub const SWEEPING_EDGE: RegistryKey<Self> = RegistryKey::vanilla_static("sweeping_edge");
    pub const SWIFT_SNEAK: RegistryKey<Self> = RegistryKey::vanilla_static("swift_sneak");
    pub const THORNS: RegistryKey<Self> = RegistryKey::vanilla_static("thorns");
    pub const UNBREAKING: RegistryKey<Self> = RegistryKey::vanilla_static("unbreaking");
    pub const VANISHING_CURSE: RegistryKey<Self> = RegistryKey::vanilla_static("vanishing_curse");
    pub const WIND_BURST: RegistryKey<Self> = RegistryKey::vanilla_static("wind_burst");
}
pub fn register_enchantments(registry: &mut DynamicRegistry<Enchantment>) {
    let entries: BTreeMap<String, String> = serde_json::from_str(ENCHANTMENT_ENTRIES).expect("SpinelExtractor enchantments.json is malformed");
    for (key, entry) in entries {
        let identifier: Identifier = key.parse().expect("SpinelExtractor enchantment key is malformed");
        let raw_nbt = parse_snbt_compound(&entry).expect("SpinelExtractor enchantment payload is malformed SNBT");
        let _ = registry.register_vanilla(RegistryKey::new(identifier), Enchantment::raw(raw_nbt));
    }
}
