mod aquatic;
mod definition;
pub mod definitions;
mod golem;
mod holder;
mod hostile;
mod terrestrial;
mod villager;

pub use aquatic::{PufferfishState, TropicalFishVariant};
pub use definition::{MetadataBitMaskDefinition, MetadataByteMaskDefinition, MetadataDefinition};
pub use golem::{CopperGolemState, CopperGolemWeatherState};
pub use holder::MetadataHolder;
pub use hostile::{CreeperState, SpellcasterIllagerSpell};
pub use spinel_registry::{
    AxolotlVariant, FoxVariant, HorseColor, LlamaVariant, MooshroomVariant, ParrotColor,
    RabbitVariant, SalmonSize, TropicalFishPattern,
};
pub use terrestrial::{ArmadilloState, HorseMarking, HorseVariant, PandaGene, SnifferState};
pub use villager::{VillagerData, VillagerLevel};
#[cfg(test)]
mod tests;
