use crate::DataComponentValue;
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Debug, PartialEq)]
pub struct UseCooldown {
    seconds: f32,
    cooldown_group: Option<String>,
}

impl UseCooldown {
    pub fn new(seconds: f32, cooldown_group: Option<String>) -> Self {
        Self {
            seconds,
            cooldown_group,
        }
    }

    pub fn seconds(&self) -> f32 {
        self.seconds
    }

    pub fn cooldown_group(&self) -> Option<&str> {
        self.cooldown_group.as_deref()
    }

    pub fn ticks(&self) -> i32 {
        (self.seconds * 20.0).ceil().max(0.0) as i32
    }
}

impl DataComponentValue for UseCooldown {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert("seconds".to_string(), Nbt::Float(self.seconds));
        if let Some(cooldown_group) = &self.cooldown_group {
            compound.insert(
                "cooldown_group".to_string(),
                Nbt::String(cooldown_group.clone()),
            );
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let Nbt::Compound(compound) = component_nbt else {
            return None;
        };
        let seconds = match compound.get("seconds") {
            Some(Nbt::Float(seconds)) => *seconds,
            Some(Nbt::Double(seconds)) => *seconds as f32,
            Some(Nbt::Int(seconds)) => *seconds as f32,
            _ => return None,
        };
        let cooldown_group = match compound.get("cooldown_group") {
            Some(Nbt::String(cooldown_group)) => Some(cooldown_group.clone()),
            _ => None,
        };
        Some(Self::new(seconds, cooldown_group))
    }
}
