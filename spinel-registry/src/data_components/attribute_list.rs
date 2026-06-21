use crate::Identifier;
use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::{compound_from_nbt, f64_field, string_field};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AttributeList {
    modifiers: Vec<AttributeModifierEntry>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AttributeModifierEntry {
    attribute_type: Identifier,
    id: Identifier,
    amount: f64,
    operation: AttributeOperation,
    slot: EquipmentSlotGroup,
    display: AttributeModifierDisplay,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttributeOperation {
    AddValue,
    AddMultipliedBase,
    AddMultipliedTotal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EquipmentSlotGroup {
    Any,
    MainHand,
    OffHand,
    Hand,
    Feet,
    Legs,
    Chest,
    Head,
    Armor,
    Body,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AttributeModifierDisplay {
    Default,
    Hidden,
    Override(Nbt),
}

impl AttributeList {
    #[must_use]
    pub fn new(modifiers: Vec<AttributeModifierEntry>) -> Self {
        Self { modifiers }
    }

    #[must_use]
    pub fn get_modifiers(&self) -> &[AttributeModifierEntry] {
        &self.modifiers
    }

    #[must_use]
    pub fn with(&self, modifier: AttributeModifierEntry) -> Self {
        let mut modifiers = self.modifiers.clone();
        modifiers.push(modifier);
        Self { modifiers }
    }

    #[must_use]
    pub fn remove(&self, modifier: &AttributeModifierEntry) -> Self {
        let modifiers = self
            .modifiers
            .iter()
            .filter(|existing_modifier| *existing_modifier != modifier)
            .cloned()
            .collect();
        Self { modifiers }
    }
}

impl AttributeModifierEntry {
    #[must_use]
    pub const fn new(
        attribute_type: Identifier,
        id: Identifier,
        amount: f64,
        operation: AttributeOperation,
        slot: EquipmentSlotGroup,
        display: AttributeModifierDisplay,
    ) -> Self {
        Self {
            attribute_type,
            id,
            amount,
            operation,
            slot,
            display,
        }
    }

    #[must_use]
    pub const fn attribute_type(&self) -> &Identifier {
        &self.attribute_type
    }

    #[must_use]
    pub const fn id(&self) -> &Identifier {
        &self.id
    }

    #[must_use]
    pub const fn get_amount(&self) -> f64 {
        self.amount
    }

    #[must_use]
    pub const fn operation(&self) -> AttributeOperation {
        self.operation
    }

    #[must_use]
    pub const fn slot(&self) -> EquipmentSlotGroup {
        self.slot
    }

    #[must_use]
    pub const fn display(&self) -> &AttributeModifierDisplay {
        &self.display
    }
}

impl AttributeOperation {
    #[must_use]
    pub const fn nbt_name(self) -> &'static str {
        match self {
            Self::AddValue => "add_value",
            Self::AddMultipliedBase => "add_multiplied_base",
            Self::AddMultipliedTotal => "add_multiplied_total",
        }
    }

    #[must_use]
    pub const fn protocol_id(self) -> i32 {
        match self {
            Self::AddValue => 0,
            Self::AddMultipliedBase => 1,
            Self::AddMultipliedTotal => 2,
        }
    }

    #[must_use]
    pub fn from_nbt_name(name: &str) -> Option<Self> {
        match name {
            "add_value" => Some(Self::AddValue),
            "add_multiplied_base" => Some(Self::AddMultipliedBase),
            "add_multiplied_total" => Some(Self::AddMultipliedTotal),
            _ => None,
        }
    }
}

impl EquipmentSlotGroup {
    #[must_use]
    pub const fn nbt_name(self) -> &'static str {
        match self {
            Self::Any => "any",
            Self::MainHand => "mainhand",
            Self::OffHand => "offhand",
            Self::Hand => "hand",
            Self::Feet => "feet",
            Self::Legs => "legs",
            Self::Chest => "chest",
            Self::Head => "head",
            Self::Armor => "armor",
            Self::Body => "body",
        }
    }

    #[must_use]
    pub const fn protocol_id(self) -> i32 {
        match self {
            Self::Any => 0,
            Self::MainHand => 1,
            Self::OffHand => 2,
            Self::Hand => 3,
            Self::Feet => 4,
            Self::Legs => 5,
            Self::Chest => 6,
            Self::Head => 7,
            Self::Armor => 8,
            Self::Body => 9,
        }
    }

    #[must_use]
    pub fn from_nbt_name(name: &str) -> Option<Self> {
        match name {
            "any" => Some(Self::Any),
            "mainhand" => Some(Self::MainHand),
            "offhand" => Some(Self::OffHand),
            "hand" => Some(Self::Hand),
            "feet" => Some(Self::Feet),
            "legs" => Some(Self::Legs),
            "chest" => Some(Self::Chest),
            "head" => Some(Self::Head),
            "armor" => Some(Self::Armor),
            "body" => Some(Self::Body),
            _ => None,
        }
    }

    #[must_use]
    pub const fn contains_slot_name(self, slot_name: &str) -> bool {
        match self {
            Self::Any => true,
            Self::MainHand => matches!(slot_name.as_bytes(), b"mainhand"),
            Self::OffHand => matches!(slot_name.as_bytes(), b"offhand"),
            Self::Hand => matches!(slot_name.as_bytes(), b"mainhand" | b"offhand"),
            Self::Feet => matches!(slot_name.as_bytes(), b"feet"),
            Self::Legs => matches!(slot_name.as_bytes(), b"legs"),
            Self::Chest => matches!(slot_name.as_bytes(), b"chest"),
            Self::Head => matches!(slot_name.as_bytes(), b"head"),
            Self::Armor => matches!(slot_name.as_bytes(), b"feet" | b"legs" | b"chest" | b"head"),
            Self::Body => matches!(slot_name.as_bytes(), b"body"),
        }
    }
}

impl DataComponentValue for AttributeList {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::List(
            self.modifiers
                .iter()
                .map(AttributeModifierEntry::to_nbt)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::List(modifiers) => modifiers
                .iter()
                .map(AttributeModifierEntry::from_nbt)
                .collect::<Option<Vec<_>>>()
                .map(Self::new),
            _ => None,
        }
    }
}

impl AttributeModifierEntry {
    fn to_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert(
            "type".to_string(),
            Nbt::String(self.attribute_type.to_string()),
        );
        compound.insert("id".to_string(), Nbt::String(self.id.to_string()));
        compound.insert("amount".to_string(), Nbt::Double(self.amount));
        compound.insert(
            "operation".to_string(),
            Nbt::String(self.operation.nbt_name().to_string()),
        );
        if self.slot != EquipmentSlotGroup::Any {
            compound.insert(
                "slot".to_string(),
                Nbt::String(self.slot.nbt_name().to_string()),
            );
        }
        match &self.display {
            AttributeModifierDisplay::Default => {}
            AttributeModifierDisplay::Hidden => {
                compound.insert("display".to_string(), display_type_compound("hidden"));
            }
            AttributeModifierDisplay::Override(component) => {
                let mut display = NbtCompound::new();
                display.insert("type".to_string(), Nbt::String("override".to_string()));
                display.insert("value".to_string(), component.clone());
                compound.insert("display".to_string(), Nbt::Compound(display));
            }
        }
        Nbt::Compound(compound)
    }

    fn from_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        let slot = match compound.get("slot") {
            Some(Nbt::String(slot)) => EquipmentSlotGroup::from_nbt_name(slot)?,
            None => EquipmentSlotGroup::Any,
            _ => return None,
        };
        Some(Self {
            attribute_type: string_field(compound, "type")?.parse().ok()?,
            id: string_field(compound, "id")?.parse().ok()?,
            amount: f64_field(compound, "amount")?,
            operation: AttributeOperation::from_nbt_name(&string_field(compound, "operation")?)?,
            slot,
            display: AttributeModifierDisplay::from_nbt(compound.get("display"))?,
        })
    }
}

impl AttributeModifierDisplay {
    fn from_nbt(component_nbt: Option<&Nbt>) -> Option<Self> {
        let Some(component_nbt) = component_nbt else {
            return Some(Self::Default);
        };
        let compound = compound_from_nbt(component_nbt)?;
        match string_field(compound, "type")?.as_str() {
            "default" => Some(Self::Default),
            "hidden" => Some(Self::Hidden),
            "override" => Some(Self::Override(compound.get("value")?.clone())),
            _ => None,
        }
    }
}

fn display_type_compound(display_type: &str) -> Nbt {
    let mut compound = NbtCompound::new();
    compound.insert("type".to_string(), Nbt::String(display_type.to_string()));
    Nbt::Compound(compound)
}
