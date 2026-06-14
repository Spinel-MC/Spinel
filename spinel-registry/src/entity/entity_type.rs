use crate::{Attribute, EntityBoundingBox, Identifier, RegistryKey, StaticRegistry};
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EntityType {
    id: i32,
    path: &'static str,
    translation_key: &'static str,
    packet_type: EntityPacketType,
    width: f64,
    height: f64,
    eye_height: f64,
    client_tracking_range: i32,
    fire_immune: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EntityTypeRegistryEntry {
    entity_type: EntityType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntityPacketType {
    Entity,
    Living,
    Player,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EntityAttachmentOffset {
    name: &'static str,
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EntityDefaultAttribute {
    attribute: Attribute,
    base_value: f64,
}

impl EntityDefaultAttribute {
    pub const fn new(attribute: Attribute, base_value: f64) -> Self {
        Self {
            attribute,
            base_value,
        }
    }

    pub const fn attribute(self) -> Attribute {
        self.attribute
    }

    pub const fn base_value(self) -> f64 {
        self.base_value
    }
}

impl EntityType {
    #[must_use]
    pub(crate) const fn new(
        id: i32,
        path: &'static str,
        translation_key: &'static str,
        packet_type: EntityPacketType,
        width: f64,
        height: f64,
        eye_height: f64,
        client_tracking_range: i32,
        fire_immune: bool,
    ) -> Self {
        Self {
            id,
            path,
            translation_key,
            packet_type,
            width,
            height,
            eye_height,
            client_tracking_range,
            fire_immune,
        }
    }

    #[must_use]
    pub const fn id(self) -> i32 {
        self.id
    }

    #[must_use]
    pub const fn registry(self) -> EntityTypeRegistryEntry {
        EntityTypeRegistryEntry { entity_type: self }
    }

    #[must_use]
    pub fn key(self) -> Identifier {
        Identifier::vanilla_static(self.path)
    }

    #[must_use]
    pub const fn path(self) -> &'static str {
        self.path
    }

    #[must_use]
    pub const fn translation_key(self) -> &'static str {
        self.translation_key
    }

    #[must_use]
    pub const fn packet_type(self) -> EntityPacketType {
        self.packet_type
    }

    #[must_use]
    pub const fn is_living(self) -> bool {
        matches!(
            self.packet_type,
            EntityPacketType::Living | EntityPacketType::Player
        )
    }

    #[must_use]
    pub const fn should_send_attributes(self) -> bool {
        self.is_living()
    }

    #[must_use]
    pub const fn drag(self) -> f64 {
        0.02
    }

    #[must_use]
    pub const fn acceleration(self) -> f64 {
        0.08
    }

    #[must_use]
    pub const fn horizontal_air_resistance(self) -> f64 {
        if self.is_living() { 0.91 } else { 0.98 }
    }

    #[must_use]
    pub const fn vertical_air_resistance(self) -> f64 {
        1.0 - self.drag()
    }

    #[must_use]
    pub const fn width(self) -> f64 {
        self.width
    }

    #[must_use]
    pub const fn height(self) -> f64 {
        self.height
    }

    #[must_use]
    pub const fn bounding_box(self) -> EntityBoundingBox {
        EntityBoundingBox::new(self.width, self.height, self.width)
    }

    #[must_use]
    pub const fn eye_height(self) -> f64 {
        self.eye_height
    }

    #[must_use]
    pub const fn client_tracking_range(self) -> i32 {
        self.client_tracking_range
    }

    #[must_use]
    pub const fn fire_immune(self) -> bool {
        self.fire_immune
    }

    #[must_use]
    pub fn synchronizes_position_only(self) -> bool {
        matches!(
            self.path,
            "item"
                | "falling_block"
                | "arrow"
                | "spectral_arrow"
                | "trident"
                | "llama_spit"
                | "wind_charge"
                | "fishing_bobber"
                | "snowball"
                | "egg"
                | "ender_pearl"
                | "splash_potion"
                | "lingering_potion"
                | "eye_of_ender"
                | "dragon_fireball"
                | "fireball"
                | "small_fireball"
                | "tnt"
        )
    }

    #[must_use]
    pub fn entity_attachment(self, attachment_name: &str) -> Option<[f64; 3]> {
        self.attachments()
            .iter()
            .find(|attachment| attachment.name == attachment_name)
            .map(|attachment| attachment.offset())
    }

    #[must_use]
    pub fn default_attribute(self, attribute_name: &str) -> Option<f64> {
        self.default_attributes()
            .iter()
            .find(|attribute| attribute.attribute.key() == attribute_name)
            .map(|attribute| attribute.base_value)
    }

    #[must_use]
    pub fn from_id(id: i32) -> Option<Self> {
        Self::ALL
            .iter()
            .find(|entity_type| entity_type.id == id)
            .copied()
    }

    #[must_use]
    pub fn from_key(key: &str) -> Option<Self> {
        let entity_type_path = key.strip_prefix("minecraft:").unwrap_or(key);
        Self::ALL
            .iter()
            .find(|entity_type| entity_type.path == entity_type_path)
            .copied()
    }

    pub fn static_registry() -> &'static StaticRegistry<Self> {
        static ENTITY_TYPE_REGISTRY: OnceLock<StaticRegistry<EntityType>> = OnceLock::new();
        ENTITY_TYPE_REGISTRY.get_or_init(|| {
            let mut registry = StaticRegistry::new();
            Self::ALL.iter().for_each(|entity_type| {
                let _ = registry.register(RegistryKey::new(entity_type.key()), *entity_type);
            });
            registry.freeze();
            registry
        })
    }
}

impl EntityTypeRegistryEntry {
    #[must_use]
    pub fn key(self) -> Identifier {
        self.entity_type.key()
    }

    #[must_use]
    pub const fn id(self) -> i32 {
        self.entity_type.id()
    }

    #[must_use]
    pub const fn translation_key(self) -> &'static str {
        self.entity_type.translation_key()
    }

    #[must_use]
    pub const fn width(self) -> f64 {
        self.entity_type.width()
    }

    #[must_use]
    pub const fn height(self) -> f64 {
        self.entity_type.height()
    }

    #[must_use]
    pub const fn eye_height(self) -> f64 {
        self.entity_type.eye_height()
    }

    #[must_use]
    pub const fn client_tracking_range(self) -> i32 {
        self.entity_type.client_tracking_range()
    }

    #[must_use]
    pub const fn fire_immune(self) -> bool {
        self.entity_type.fire_immune()
    }

    #[must_use]
    pub fn default_attributes(self) -> &'static [EntityDefaultAttribute] {
        self.entity_type.default_attributes()
    }

    #[must_use]
    pub const fn drag(self) -> f64 {
        self.entity_type.drag()
    }

    #[must_use]
    pub const fn acceleration(self) -> f64 {
        self.entity_type.acceleration()
    }

    #[must_use]
    pub const fn horizontal_air_resistance(self) -> f64 {
        self.entity_type.horizontal_air_resistance()
    }

    #[must_use]
    pub const fn vertical_air_resistance(self) -> f64 {
        self.entity_type.vertical_air_resistance()
    }

    #[must_use]
    pub const fn should_send_attributes(self) -> bool {
        self.entity_type.should_send_attributes()
    }

    #[must_use]
    pub fn entity_attachment(self, attachment_name: &str) -> Option<[f64; 3]> {
        self.entity_type.entity_attachment(attachment_name)
    }

    #[must_use]
    pub const fn bounding_box(self) -> EntityBoundingBox {
        self.entity_type.bounding_box()
    }
}

impl EntityAttachmentOffset {
    #[must_use]
    pub const fn new(name: &'static str, x: f64, y: f64, z: f64) -> Self {
        Self { name, x, y, z }
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        self.name
    }

    #[must_use]
    pub const fn offset(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}
