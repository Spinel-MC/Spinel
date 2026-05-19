use spinel::{
    registry::biome::{BiomeEffects, Color, GrassColorModifier, TemperatureModifier},
    server::world::Biome,
};

pub struct ExampleBiome {}

impl ExampleBiome {
    fn to_biome() -> Biome {
        let mut builder = Biome::builder();
        builder.precipitation(true);
        builder
            .temperature(0.0)
            .temperature_modifier(TemperatureModifier::None)
            .downfall(0.0)
            .effects(Self::to_biome_effects())
            .build()
    }

    fn to_biome_effects() -> BiomeEffects {
        BiomeEffects {
            water_color: Some(Color::from_rgb_int(0x00ff00)),
            foliage_color: Some(Color::BLACK),
            dry_foliage_color: Some(Color::BLACK),
            grass_color: Some(Color::BLACK),
            grass_color_modifier: GrassColorModifier::None,
        }
    }
}

impl From<ExampleBiome> for Biome {
    fn from(_: ExampleBiome) -> Biome {
        ExampleBiome::to_biome()
    }
}
