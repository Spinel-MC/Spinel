use crate::Identifier;
use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::{bool_field_or, compound_from_nbt, string_field};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PiercingWeapon {
    deals_knockback: bool,
    dismounts: bool,
    sound: Option<Identifier>,
    hit_sound: Option<Identifier>,
}

impl PiercingWeapon {
    #[must_use]
    pub const fn new(
        deals_knockback: bool,
        dismounts: bool,
        sound: Option<Identifier>,
        hit_sound: Option<Identifier>,
    ) -> Self {
        Self {
            deals_knockback,
            dismounts,
            sound,
            hit_sound,
        }
    }

    #[must_use]
    pub const fn deals_knockback(&self) -> bool {
        self.deals_knockback
    }

    #[must_use]
    pub const fn dismounts(&self) -> bool {
        self.dismounts
    }

    #[must_use]
    pub const fn sound(&self) -> Option<&Identifier> {
        self.sound.as_ref()
    }

    #[must_use]
    pub const fn hit_sound(&self) -> Option<&Identifier> {
        self.hit_sound.as_ref()
    }
}

impl DataComponentValue for PiercingWeapon {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        if !self.deals_knockback {
            compound.insert("deals_knockback".to_string(), Nbt::Byte(0));
        }
        if self.dismounts {
            compound.insert("dismounts".to_string(), Nbt::Byte(1));
        }
        if let Some(sound) = &self.sound {
            compound.insert("sound".to_string(), Nbt::String(sound.to_string()));
        }
        if let Some(hit_sound) = &self.hit_sound {
            compound.insert("hit_sound".to_string(), Nbt::String(hit_sound.to_string()));
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        Some(Self {
            deals_knockback: bool_field_or(compound, "deals_knockback", true)?,
            dismounts: bool_field_or(compound, "dismounts", false)?,
            sound: optional_identifier(compound, "sound")?,
            hit_sound: optional_identifier(compound, "hit_sound")?,
        })
    }
}

fn optional_identifier(compound: &NbtCompound, name: &str) -> Option<Option<Identifier>> {
    match compound.get(name) {
        Some(_) => Some(Some(string_field(compound, name)?.parse().ok()?)),
        None => Some(None),
    }
}
