use crate::inventory::slot_conversion::{BOOTS_SLOT, CHESTPLATE_SLOT, HELMET_SLOT, LEGGINGS_SLOT};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    pub fn protocol_id(&self) -> i32 {
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

    pub fn legacy_protocol_id(&self) -> i32 {
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

    pub fn nbt_name(&self) -> &'static str {
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

    pub fn is_hand(&self) -> bool {
        matches!(self, Self::MainHand | Self::OffHand)
    }

    pub fn is_armor(&self) -> bool {
        matches!(
            self,
            Self::Boots | Self::Leggings | Self::Chestplate | Self::Helmet
        )
    }

    pub fn armor_slot(&self) -> i32 {
        match self {
            Self::Boots => BOOTS_SLOT,
            Self::Leggings => LEGGINGS_SLOT,
            Self::Chestplate => CHESTPLATE_SLOT,
            Self::Helmet => HELMET_SLOT,
            _ => -1,
        }
    }
}
