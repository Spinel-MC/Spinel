use crate::data_components::nbt_reader::{compound_from_nbt, i32_field_or};
use crate::data_components::{DataComponentValue, FireworkExplosion};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FireworkList {
    flight_duration: i32,
    explosions: Vec<FireworkExplosion>,
}

impl FireworkList {
    #[must_use]
    pub fn new(flight_duration: i32, explosions: Vec<FireworkExplosion>) -> Self {
        Self {
            flight_duration,
            explosions,
        }
    }

    #[must_use]
    pub const fn flight_duration(&self) -> i32 {
        self.flight_duration
    }

    #[must_use]
    pub fn explosions(&self) -> &[FireworkExplosion] {
        &self.explosions
    }

    #[must_use]
    pub fn with_flight_duration(&self, flight_duration: i32) -> Self {
        Self {
            flight_duration,
            explosions: self.explosions.clone(),
        }
    }

    #[must_use]
    pub fn with_explosions(&self, explosions: Vec<FireworkExplosion>) -> Self {
        Self {
            flight_duration: self.flight_duration,
            explosions,
        }
    }
}

impl DataComponentValue for FireworkList {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert(
            "flight_duration".to_string(),
            Nbt::Byte(self.flight_duration as i8),
        );
        if !self.explosions.is_empty() {
            compound.insert(
                "explosions".to_string(),
                Nbt::List(
                    self.explosions
                        .iter()
                        .map(FireworkExplosion::to_component_nbt)
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                ),
            );
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let explosions = match compound.get("explosions") {
            Some(Nbt::List(explosions)) => explosions
                .iter()
                .map(FireworkExplosion::from_component_nbt)
                .collect::<Option<Vec<_>>>()?,
            None => Vec::new(),
            _ => return None,
        };
        Some(Self {
            flight_duration: i32_field_or(compound, "flight_duration", 0)?,
            explosions,
        })
    }
}
