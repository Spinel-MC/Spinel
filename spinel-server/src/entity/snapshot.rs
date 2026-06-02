use crate::entity::metadata::MetadataHolder;
use crate::entity::player::PlayerSkin;
use crate::entity::{EntityId, EntityPosition};
use spinel_core::entity::game_mode::GameMode;
use spinel_network::types::Velocity;
use spinel_registry::EntityType;
use spinel_utils::component::text::TextComponent;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct EntitySnapshot {
    entity_id: EntityId,
    uuid: Uuid,
    entity_type: EntityType,
    position: EntityPosition,
    velocity: Velocity,
    world: Option<Uuid>,
    removed: bool,
    metadata: MetadataHolder,
    custom_name: Option<TextComponent>,
}

impl EntitySnapshot {
    pub fn new(
        entity_id: EntityId,
        uuid: Uuid,
        entity_type: EntityType,
        position: EntityPosition,
        velocity: Velocity,
        world: Option<Uuid>,
        removed: bool,
        metadata: MetadataHolder,
        custom_name: Option<TextComponent>,
    ) -> Self {
        Self {
            entity_id,
            uuid,
            entity_type,
            position,
            velocity,
            world,
            removed,
            metadata,
            custom_name,
        }
    }

    pub const fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub const fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub const fn entity_type(&self) -> EntityType {
        self.entity_type
    }

    pub const fn position(&self) -> EntityPosition {
        self.position
    }

    pub const fn velocity(&self) -> Velocity {
        self.velocity
    }

    pub const fn world(&self) -> Option<Uuid> {
        self.world
    }

    pub const fn is_removed(&self) -> bool {
        self.removed
    }

    pub const fn metadata(&self) -> &MetadataHolder {
        &self.metadata
    }

    pub fn custom_name(&self) -> Option<TextComponent> {
        self.custom_name.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlayerSnapshot {
    entity_id: EntityId,
    uuid: Uuid,
    username: String,
    position: EntityPosition,
    world: Option<Uuid>,
    game_mode: GameMode,
    skin: Option<PlayerSkin>,
    display_name: Option<TextComponent>,
    statistics: Vec<(String, i32)>,
}

impl PlayerSnapshot {
    pub fn new(
        entity_id: EntityId,
        uuid: Uuid,
        username: String,
        position: EntityPosition,
        world: Option<Uuid>,
        game_mode: GameMode,
        skin: Option<PlayerSkin>,
        display_name: Option<TextComponent>,
        statistics: Vec<(String, i32)>,
    ) -> Self {
        Self {
            entity_id,
            uuid,
            username,
            position,
            world,
            game_mode,
            skin,
            display_name,
            statistics,
        }
    }

    pub const fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub const fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub const fn position(&self) -> EntityPosition {
        self.position
    }

    pub const fn world(&self) -> Option<Uuid> {
        self.world
    }

    pub const fn game_mode(&self) -> GameMode {
        self.game_mode
    }

    pub const fn skin(&self) -> Option<&PlayerSkin> {
        self.skin.as_ref()
    }

    pub const fn display_name(&self) -> Option<&TextComponent> {
        self.display_name.as_ref()
    }

    pub fn statistics(&self) -> &[(String, i32)] {
        &self.statistics
    }
}
