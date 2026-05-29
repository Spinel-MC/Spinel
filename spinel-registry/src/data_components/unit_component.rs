use crate::data_components::DataComponentValue;
use spinel_nbt::Nbt;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct UnitComponent;

impl DataComponentValue for UnitComponent {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::Compound(spinel_nbt::NbtCompound::new())
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::Compound(_) | Nbt::End => Some(Self),
            _ => None,
        }
    }
}
