use crate::{DynamicRegistry, RegistryKey};
use crate::dimension_type::DimensionType;
impl DimensionType {
    pub const OVERWORLD: RegistryKey<Self> = RegistryKey::vanilla_static("overworld");
    pub const OVERWORLD_CAVES: RegistryKey<Self> = RegistryKey::vanilla_static("overworld_caves");
    pub const THE_END: RegistryKey<Self> = RegistryKey::vanilla_static("the_end");
    pub const THE_NETHER: RegistryKey<Self> = RegistryKey::vanilla_static("the_nether");
}
pub fn register_dimension_types(registry: &mut DynamicRegistry<DimensionType>) {
    let _ = registry.register_vanilla(DimensionType::OVERWORLD, DimensionType::default());
    let _ = registry.register_vanilla(DimensionType::OVERWORLD_CAVES, DimensionType::default());
    let _ = registry.register_vanilla(DimensionType::THE_END, DimensionType::default());
    let _ = registry.register_vanilla(DimensionType::THE_NETHER, DimensionType::default());
}
