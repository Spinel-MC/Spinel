mod ageable_mob_meta;
mod animal_meta;
mod aquatic;
mod armadillo_meta;
mod bee_meta;
mod definition;
pub mod definitions;
mod entity_meta;
mod flying_meta;
mod ghast_meta;
mod golem;
mod guardian_meta;
mod holder;
mod hostile;
mod living_entity_meta;
mod mob_meta;
mod monster_meta;
mod pathfinder_mob_meta;
mod terrestrial;
mod vex_meta;
mod villager;

pub use ageable_mob_meta::AgeableMobMeta;
pub use animal_meta::AnimalMeta;
pub use aquatic::{PufferfishState, TropicalFishVariant};
pub use armadillo_meta::ArmadilloMeta;
pub use bee_meta::BeeMeta;
pub use definition::{MetadataBitMaskDefinition, MetadataByteMaskDefinition, MetadataDefinition};
pub use entity_meta::EntityMeta;
pub use flying_meta::FlyingMeta;
pub use ghast_meta::GhastMeta;
pub use golem::{CopperGolemState, CopperGolemWeatherState};
pub use guardian_meta::GuardianMeta;
pub use holder::MetadataHolder;
pub use hostile::{CreeperState, SpellcasterIllagerSpell};
pub use living_entity_meta::LivingEntityMeta;
pub use mob_meta::MobMeta;
pub use monster_meta::MonsterMeta;
pub use pathfinder_mob_meta::PathfinderMobMeta;
pub use spinel_registry::{
    AxolotlVariant, FoxVariant, HorseColor, LlamaVariant, MooshroomVariant, ParrotColor,
    RabbitVariant, SalmonSize, TropicalFishPattern,
};
pub use terrestrial::{ArmadilloState, HorseMarking, HorseVariant, PandaGene, SnifferState};
pub use vex_meta::VexMeta;
pub use villager::{VillagerData, VillagerLevel};
#[cfg(test)]
mod tests;
