use crate::Identifier;
use crate::data_components::nbt_reader::{bool_field_or, compound_from_nbt, string_field};
use crate::data_components::{DataComponentValue, RegistryTagReference};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Equippable {
    slot: EquippableSlot,
    equip_sound: Identifier,
    asset_id: Option<String>,
    camera_overlay: Option<String>,
    allowed_entities: Option<RegistryTagReference>,
    dispensable: bool,
    swappable: bool,
    damage_on_hurt: bool,
    equip_on_interact: bool,
    can_be_sheared: bool,
    shearing_sound: Identifier,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EquippableSlot {
    MainHand,
    Feet,
    Legs,
    Chest,
    Head,
    OffHand,
    Body,
    Saddle,
}

impl Equippable {
    #[must_use]
    pub fn new(
        slot: EquippableSlot,
        equip_sound: Identifier,
        asset_id: Option<String>,
        camera_overlay: Option<String>,
        allowed_entities: Option<RegistryTagReference>,
        dispensable: bool,
        swappable: bool,
        damage_on_hurt: bool,
        equip_on_interact: bool,
        can_be_sheared: bool,
        shearing_sound: Identifier,
    ) -> Self {
        Self {
            slot,
            equip_sound,
            asset_id,
            camera_overlay,
            allowed_entities,
            dispensable,
            swappable,
            damage_on_hurt,
            equip_on_interact,
            can_be_sheared,
            shearing_sound,
        }
    }

    #[must_use]
    pub const fn slot(&self) -> EquippableSlot {
        self.slot
    }

    #[must_use]
    pub const fn equip_sound(&self) -> &Identifier {
        &self.equip_sound
    }

    #[must_use]
    pub fn asset_id(&self) -> Option<&str> {
        self.asset_id.as_deref()
    }

    #[must_use]
    pub fn camera_overlay(&self) -> Option<&str> {
        self.camera_overlay.as_deref()
    }

    #[must_use]
    pub const fn allowed_entities(&self) -> Option<&RegistryTagReference> {
        self.allowed_entities.as_ref()
    }

    #[must_use]
    pub const fn dispensable(&self) -> bool {
        self.dispensable
    }

    #[must_use]
    pub const fn swappable(&self) -> bool {
        self.swappable
    }

    #[must_use]
    pub const fn damage_on_hurt(&self) -> bool {
        self.damage_on_hurt
    }

    #[must_use]
    pub const fn equip_on_interact(&self) -> bool {
        self.equip_on_interact
    }

    #[must_use]
    pub const fn can_be_sheared(&self) -> bool {
        self.can_be_sheared
    }

    #[must_use]
    pub const fn shearing_sound(&self) -> &Identifier {
        &self.shearing_sound
    }

    #[must_use]
    pub fn with_slot(&self, slot: EquippableSlot) -> Self {
        Self {
            slot,
            equip_sound: self.equip_sound.clone(),
            asset_id: self.asset_id.clone(),
            camera_overlay: self.camera_overlay.clone(),
            allowed_entities: self.allowed_entities.clone(),
            dispensable: self.dispensable,
            swappable: self.swappable,
            damage_on_hurt: self.damage_on_hurt,
            equip_on_interact: self.equip_on_interact,
            can_be_sheared: self.can_be_sheared,
            shearing_sound: self.shearing_sound.clone(),
        }
    }

    #[must_use]
    pub fn with_equip_sound(&self, equip_sound: Identifier) -> Self {
        Self {
            slot: self.slot,
            equip_sound,
            asset_id: self.asset_id.clone(),
            camera_overlay: self.camera_overlay.clone(),
            allowed_entities: self.allowed_entities.clone(),
            dispensable: self.dispensable,
            swappable: self.swappable,
            damage_on_hurt: self.damage_on_hurt,
            equip_on_interact: self.equip_on_interact,
            can_be_sheared: self.can_be_sheared,
            shearing_sound: self.shearing_sound.clone(),
        }
    }

    #[must_use]
    pub fn with_asset_id(&self, asset_id: Option<String>) -> Self {
        Self {
            slot: self.slot,
            equip_sound: self.equip_sound.clone(),
            asset_id,
            camera_overlay: self.camera_overlay.clone(),
            allowed_entities: self.allowed_entities.clone(),
            dispensable: self.dispensable,
            swappable: self.swappable,
            damage_on_hurt: self.damage_on_hurt,
            equip_on_interact: self.equip_on_interact,
            can_be_sheared: self.can_be_sheared,
            shearing_sound: self.shearing_sound.clone(),
        }
    }

    #[must_use]
    pub fn with_camera_overlay(&self, camera_overlay: Option<String>) -> Self {
        Self {
            slot: self.slot,
            equip_sound: self.equip_sound.clone(),
            asset_id: self.asset_id.clone(),
            camera_overlay,
            allowed_entities: self.allowed_entities.clone(),
            dispensable: self.dispensable,
            swappable: self.swappable,
            damage_on_hurt: self.damage_on_hurt,
            equip_on_interact: self.equip_on_interact,
            can_be_sheared: self.can_be_sheared,
            shearing_sound: self.shearing_sound.clone(),
        }
    }

    #[must_use]
    pub fn with_allowed_entities(&self, allowed_entities: Option<RegistryTagReference>) -> Self {
        Self {
            slot: self.slot,
            equip_sound: self.equip_sound.clone(),
            asset_id: self.asset_id.clone(),
            camera_overlay: self.camera_overlay.clone(),
            allowed_entities,
            dispensable: self.dispensable,
            swappable: self.swappable,
            damage_on_hurt: self.damage_on_hurt,
            equip_on_interact: self.equip_on_interact,
            can_be_sheared: self.can_be_sheared,
            shearing_sound: self.shearing_sound.clone(),
        }
    }

    #[must_use]
    pub fn with_dispensable(&self, dispensable: bool) -> Self {
        Self {
            slot: self.slot,
            equip_sound: self.equip_sound.clone(),
            asset_id: self.asset_id.clone(),
            camera_overlay: self.camera_overlay.clone(),
            allowed_entities: self.allowed_entities.clone(),
            dispensable,
            swappable: self.swappable,
            damage_on_hurt: self.damage_on_hurt,
            equip_on_interact: self.equip_on_interact,
            can_be_sheared: self.can_be_sheared,
            shearing_sound: self.shearing_sound.clone(),
        }
    }

    #[must_use]
    pub fn with_swappable(&self, swappable: bool) -> Self {
        Self {
            slot: self.slot,
            equip_sound: self.equip_sound.clone(),
            asset_id: self.asset_id.clone(),
            camera_overlay: self.camera_overlay.clone(),
            allowed_entities: self.allowed_entities.clone(),
            dispensable: self.dispensable,
            swappable,
            damage_on_hurt: self.damage_on_hurt,
            equip_on_interact: self.equip_on_interact,
            can_be_sheared: self.can_be_sheared,
            shearing_sound: self.shearing_sound.clone(),
        }
    }

    #[must_use]
    pub fn with_damage_on_hurt(&self, damage_on_hurt: bool) -> Self {
        Self {
            slot: self.slot,
            equip_sound: self.equip_sound.clone(),
            asset_id: self.asset_id.clone(),
            camera_overlay: self.camera_overlay.clone(),
            allowed_entities: self.allowed_entities.clone(),
            dispensable: self.dispensable,
            swappable: self.swappable,
            damage_on_hurt,
            equip_on_interact: self.equip_on_interact,
            can_be_sheared: self.can_be_sheared,
            shearing_sound: self.shearing_sound.clone(),
        }
    }

    #[must_use]
    pub fn with_equip_on_interact(&self, equip_on_interact: bool) -> Self {
        Self {
            slot: self.slot,
            equip_sound: self.equip_sound.clone(),
            asset_id: self.asset_id.clone(),
            camera_overlay: self.camera_overlay.clone(),
            allowed_entities: self.allowed_entities.clone(),
            dispensable: self.dispensable,
            swappable: self.swappable,
            damage_on_hurt: self.damage_on_hurt,
            equip_on_interact,
            can_be_sheared: self.can_be_sheared,
            shearing_sound: self.shearing_sound.clone(),
        }
    }

    #[must_use]
    pub fn with_can_be_sheared(&self, can_be_sheared: bool) -> Self {
        Self {
            slot: self.slot,
            equip_sound: self.equip_sound.clone(),
            asset_id: self.asset_id.clone(),
            camera_overlay: self.camera_overlay.clone(),
            allowed_entities: self.allowed_entities.clone(),
            dispensable: self.dispensable,
            swappable: self.swappable,
            damage_on_hurt: self.damage_on_hurt,
            equip_on_interact: self.equip_on_interact,
            can_be_sheared,
            shearing_sound: self.shearing_sound.clone(),
        }
    }

    #[must_use]
    pub fn with_shearing_sound(&self, shearing_sound: Identifier) -> Self {
        Self {
            slot: self.slot,
            equip_sound: self.equip_sound.clone(),
            asset_id: self.asset_id.clone(),
            camera_overlay: self.camera_overlay.clone(),
            allowed_entities: self.allowed_entities.clone(),
            dispensable: self.dispensable,
            swappable: self.swappable,
            damage_on_hurt: self.damage_on_hurt,
            equip_on_interact: self.equip_on_interact,
            can_be_sheared: self.can_be_sheared,
            shearing_sound,
        }
    }
}

impl EquippableSlot {
    #[must_use]
    pub const fn nbt_name(self) -> &'static str {
        match self {
            Self::MainHand => "mainhand",
            Self::Feet => "feet",
            Self::Legs => "legs",
            Self::Chest => "chest",
            Self::Head => "head",
            Self::OffHand => "offhand",
            Self::Body => "body",
            Self::Saddle => "saddle",
        }
    }

    #[must_use]
    pub const fn protocol_id(self) -> i32 {
        match self {
            Self::MainHand => 0,
            Self::Feet => 1,
            Self::Legs => 2,
            Self::Chest => 3,
            Self::Head => 4,
            Self::OffHand => 5,
            Self::Body => 6,
            Self::Saddle => 7,
        }
    }

    #[must_use]
    pub fn from_nbt_name(name: &str) -> Option<Self> {
        match name {
            "mainhand" => Some(Self::MainHand),
            "feet" => Some(Self::Feet),
            "legs" => Some(Self::Legs),
            "chest" => Some(Self::Chest),
            "head" => Some(Self::Head),
            "offhand" => Some(Self::OffHand),
            "body" => Some(Self::Body),
            "saddle" => Some(Self::Saddle),
            _ => None,
        }
    }
}

impl DataComponentValue for Equippable {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert(
            "slot".to_string(),
            Nbt::String(self.slot.nbt_name().to_string()),
        );
        if self.equip_sound.to_string() != "minecraft:item.armor.equip_generic" {
            compound.insert(
                "equip_sound".to_string(),
                Nbt::String(self.equip_sound.to_string()),
            );
        }
        if let Some(asset_id) = &self.asset_id {
            compound.insert("asset_id".to_string(), Nbt::String(asset_id.clone()));
        }
        if let Some(camera_overlay) = &self.camera_overlay {
            compound.insert(
                "camera_overlay".to_string(),
                Nbt::String(camera_overlay.clone()),
            );
        }
        if let Some(allowed_entities) = &self.allowed_entities {
            compound.insert("allowed_entities".to_string(), allowed_entities.to_nbt());
        }
        if !self.dispensable {
            compound.insert("dispensable".to_string(), Nbt::Byte(0));
        }
        if !self.swappable {
            compound.insert("swappable".to_string(), Nbt::Byte(0));
        }
        if !self.damage_on_hurt {
            compound.insert("damage_on_hurt".to_string(), Nbt::Byte(0));
        }
        if self.equip_on_interact {
            compound.insert("equip_on_interact".to_string(), Nbt::Byte(1));
        }
        if self.can_be_sheared {
            compound.insert("can_be_sheared".to_string(), Nbt::Byte(1));
        }
        if self.shearing_sound.to_string() != "minecraft:item.shears.snip" {
            compound.insert(
                "shearing_sound".to_string(),
                Nbt::String(self.shearing_sound.to_string()),
            );
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        Some(Self {
            slot: EquippableSlot::from_nbt_name(&string_field(compound, "slot")?)?,
            equip_sound: optional_identifier(
                compound,
                "equip_sound",
                Identifier::vanilla_static("item.armor.equip_generic"),
            )?,
            asset_id: optional_string(compound, "asset_id")?,
            camera_overlay: optional_string(compound, "camera_overlay")?,
            allowed_entities: optional_registry_tag(compound, "allowed_entities")?,
            dispensable: bool_field_or(compound, "dispensable", true)?,
            swappable: bool_field_or(compound, "swappable", true)?,
            damage_on_hurt: bool_field_or(compound, "damage_on_hurt", true)?,
            equip_on_interact: bool_field_or(compound, "equip_on_interact", false)?,
            can_be_sheared: bool_field_or(compound, "can_be_sheared", false)?,
            shearing_sound: optional_identifier(
                compound,
                "shearing_sound",
                Identifier::vanilla_static("item.shears.snip"),
            )?,
        })
    }
}

fn optional_identifier(
    compound: &NbtCompound,
    name: &str,
    default_value: Identifier,
) -> Option<Identifier> {
    match compound.get(name) {
        Some(_) => string_field(compound, name)?.parse().ok(),
        None => Some(default_value),
    }
}

fn optional_string(compound: &NbtCompound, name: &str) -> Option<Option<String>> {
    match compound.get(name) {
        Some(_) => Some(Some(string_field(compound, name)?)),
        None => Some(None),
    }
}

fn optional_registry_tag(
    compound: &NbtCompound,
    name: &str,
) -> Option<Option<RegistryTagReference>> {
    match compound.get(name) {
        Some(value) => Some(Some(RegistryTagReference::from_nbt(value)?)),
        None => Some(None),
    }
}
