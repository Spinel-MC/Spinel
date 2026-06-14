use serde::Deserialize;
use std::sync::LazyLock;

const PARTICLE_TYPES_JSON: &str = include_str!("../../assets/particle_types.json");

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ParticlePayloadShape {
    Unit,
    BlockState,
    ColorScale,
    ColorTransitionScale,
    ItemStack,
    AlphaColor,
    Float,
    VarInt,
    Vibration,
    Trail,
    ColorPower,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParticleType {
    id: i32,
    name: String,
    payload_shape: ParticlePayloadShape,
}

#[derive(Deserialize)]
struct ParticleTypesFile {
    particle_types: Vec<ParticleType>,
}

static PARTICLE_TYPES: LazyLock<Vec<ParticleType>> = LazyLock::new(|| {
    let particle_types_file: ParticleTypesFile = serde_json::from_str(PARTICLE_TYPES_JSON)
        .unwrap_or_else(|error| panic!("particle type asset is malformed: {error}"));
    particle_types_file
        .particle_types
        .iter()
        .enumerate()
        .for_each(|(expected_id, particle_type)| {
            assert_eq!(
                particle_type.id, expected_id as i32,
                "particle type ids must be contiguous"
            );
        });
    particle_types_file.particle_types
});

impl ParticleType {
    pub fn from_id(id: i32) -> Option<&'static Self> {
        usize::try_from(id)
            .ok()
            .and_then(|id| PARTICLE_TYPES.get(id))
    }

    pub fn from_name(name: &str) -> Option<&'static Self> {
        PARTICLE_TYPES
            .iter()
            .find(|particle_type| particle_type.name == name)
    }

    pub fn all() -> &'static [Self] {
        &PARTICLE_TYPES
    }

    pub const fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn payload_shape(&self) -> ParticlePayloadShape {
        self.payload_shape
    }
}

#[cfg(test)]
mod tests {
    use super::{ParticlePayloadShape, ParticleType};

    #[test]
    fn extracted_particle_types_are_contiguous_and_typed() {
        assert_eq!(ParticleType::all().len(), 115);
        assert_eq!(
            ParticleType::from_name("minecraft:dust")
                .unwrap()
                .payload_shape(),
            ParticlePayloadShape::ColorScale
        );
        assert_eq!(
            ParticleType::from_id(114).unwrap().name(),
            "minecraft:firefly"
        );
    }
}
