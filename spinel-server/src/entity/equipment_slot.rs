use crate::entity::Error;
use crate::inventory::slot_conversion::{BOOTS_SLOT, CHESTPLATE_SLOT, HELMET_SLOT, LEGGINGS_SLOT};
use spinel_core::network::clientbound::play::set_equipment::EntityEquipmentSlot;
use spinel_registry::EquippableSlot;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EquipmentSlot {
    MainHand,
    OffHand,
    Boots,
    Leggings,
    Chestplate,
    Helmet,
    Body,
    Saddle,
}

impl EquipmentSlot {
    pub const fn get_armors() -> &'static [Self] {
        &[Self::Boots, Self::Leggings, Self::Chestplate, Self::Helmet]
    }

    pub fn from_legacy_protocol_id(legacy_protocol_id: i32) -> Result<Self, Error> {
        match legacy_protocol_id {
            0 => Ok(Self::MainHand),
            1 => Ok(Self::OffHand),
            2 => Ok(Self::Boots),
            3 => Ok(Self::Leggings),
            4 => Ok(Self::Chestplate),
            5 => Ok(Self::Helmet),
            6 => Ok(Self::Body),
            7 => Ok(Self::Saddle),
            _ => Err(Error::UnknownLegacyEquipmentSlotProtocolId(
                legacy_protocol_id,
            )),
        }
    }
    pub fn get_protocol_id(&self) -> i32 {
        match self {
            Self::MainHand => 0,
            Self::Boots => 1,
            Self::Leggings => 2,
            Self::Chestplate => 3,
            Self::Helmet => 4,
            Self::OffHand => 5,
            Self::Body => 6,
            Self::Saddle => 7,
        }
    }

    pub fn get_legacy_protocol_id(&self) -> i32 {
        match self {
            Self::MainHand => 0,
            Self::OffHand => 1,
            Self::Boots => 2,
            Self::Leggings => 3,
            Self::Chestplate => 4,
            Self::Helmet => 5,
            Self::Body => 6,
            Self::Saddle => 7,
        }
    }

    pub fn get_nbt_name(&self) -> &'static str {
        match self {
            Self::MainHand => "mainhand",
            Self::OffHand => "offhand",
            Self::Boots => "feet",
            Self::Leggings => "legs",
            Self::Chestplate => "chest",
            Self::Helmet => "head",
            Self::Body => "body",
            Self::Saddle => "saddle",
        }
    }

    pub fn from_nbt_name(nbt_name: &str) -> Option<Self> {
        match nbt_name {
            "mainhand" => Some(Self::MainHand),
            "offhand" => Some(Self::OffHand),
            "feet" => Some(Self::Boots),
            "legs" => Some(Self::Leggings),
            "chest" => Some(Self::Chestplate),
            "head" => Some(Self::Helmet),
            "body" => Some(Self::Body),
            "saddle" => Some(Self::Saddle),
            _ => None,
        }
    }

    pub fn is_hand(&self) -> bool {
        matches!(self, Self::MainHand | Self::OffHand)
    }

    pub fn is_armor(&self) -> bool {
        matches!(
            self,
            Self::Boots | Self::Leggings | Self::Chestplate | Self::Helmet
        )
    }

    pub fn from_equippable_armor_slot(equippable_slot: EquippableSlot) -> Option<Self> {
        match equippable_slot {
            EquippableSlot::Feet => Some(Self::Boots),
            EquippableSlot::Legs => Some(Self::Leggings),
            EquippableSlot::Chest => Some(Self::Chestplate),
            EquippableSlot::Head => Some(Self::Helmet),
            _ => None,
        }
    }

    pub fn from_equippable_slot(equippable_slot: EquippableSlot) -> Option<Self> {
        match equippable_slot {
            EquippableSlot::Feet => Some(Self::Boots),
            EquippableSlot::Legs => Some(Self::Leggings),
            EquippableSlot::Chest => Some(Self::Chestplate),
            EquippableSlot::Head => Some(Self::Helmet),
            EquippableSlot::OffHand => Some(Self::OffHand),
            _ => None,
        }
    }

    pub fn get_armor_slot(&self) -> i32 {
        match self {
            Self::Boots => BOOTS_SLOT,
            Self::Leggings => LEGGINGS_SLOT,
            Self::Chestplate => CHESTPLATE_SLOT,
            Self::Helmet => HELMET_SLOT,
            _ => -1,
        }
    }

    pub fn get_entity_equipment_slot(&self) -> EntityEquipmentSlot {
        match self {
            Self::MainHand => EntityEquipmentSlot::MainHand,
            Self::OffHand => EntityEquipmentSlot::OffHand,
            Self::Boots => EntityEquipmentSlot::Boots,
            Self::Leggings => EntityEquipmentSlot::Leggings,
            Self::Chestplate => EntityEquipmentSlot::Chestplate,
            Self::Helmet => EntityEquipmentSlot::Helmet,
            Self::Body => EntityEquipmentSlot::Body,
            Self::Saddle => EntityEquipmentSlot::Saddle,
        }
    }
}
