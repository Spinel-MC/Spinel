use crate::entity::metadata::MetadataHolder;
use crate::entity::player::PlayerSkin;
use crate::entity::{EntityId, EntityPosition};
use crate::world::ChunkPosition;
use spinel_core::entity::game_mode::GameMode;
use spinel_nbt::{Tag, TagHandler, TagReadable};
use spinel_network::types::Velocity;
use spinel_registry::EntityType;
use spinel_utils::component::text::TextComponent;
use std::collections::BTreeSet;
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
    chunk: Option<ChunkPosition>,
    viewers: BTreeSet<EntityId>,
    passengers: BTreeSet<EntityId>,
    vehicle: Option<EntityId>,
    tag_handler: TagHandler,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EntityObservation {
    entity_id: EntityId,
    entity_type: EntityType,
    position: EntityPosition,
    removed: bool,
}

impl EntityObservation {
    pub const fn new(
        entity_id: EntityId,
        entity_type: EntityType,
        position: EntityPosition,
        removed: bool,
    ) -> Self {
        Self {
            entity_id,
            entity_type,
            position,
            removed,
        }
    }

    pub const fn get_entity_id(self) -> EntityId {
        self.entity_id
    }

    pub const fn get_entity_type(self) -> EntityType {
        self.entity_type
    }

    pub const fn get_position(self) -> EntityPosition {
        self.position
    }

    pub const fn is_removed(self) -> bool {
        self.removed
    }
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
        chunk: Option<ChunkPosition>,
        viewers: BTreeSet<EntityId>,
        passengers: BTreeSet<EntityId>,
        vehicle: Option<EntityId>,
        tag_handler: TagHandler,
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
            chunk,
            viewers,
            passengers,
            vehicle,
            tag_handler,
        }
    }

    pub const fn get_entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub const fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub const fn get_entity_type(&self) -> EntityType {
        self.entity_type
    }

    pub const fn get_position(&self) -> EntityPosition {
        self.position
    }

    pub const fn get_velocity(&self) -> Velocity {
        self.velocity
    }

    pub const fn get_world(&self) -> Option<Uuid> {
        self.world
    }

    pub const fn is_removed(&self) -> bool {
        self.removed
    }

    pub const fn get_metadata(&self) -> &MetadataHolder {
        &self.metadata
    }

    pub fn get_custom_name(&self) -> Option<TextComponent> {
        self.custom_name.clone()
    }

    pub const fn get_chunk(&self) -> Option<ChunkPosition> {
        self.chunk
    }

    pub fn get_viewers(&self) -> &BTreeSet<EntityId> {
        &self.viewers
    }

    pub fn get_passengers(&self) -> &BTreeSet<EntityId> {
        &self.passengers
    }

    pub const fn get_vehicle(&self) -> Option<EntityId> {
        self.vehicle
    }

    pub const fn get_tag_handler(&self) -> &TagHandler {
        &self.tag_handler
    }
}

impl TagReadable for EntitySnapshot {
    fn get_tag<T>(&self, tag: &Tag<T>) -> Option<T> {
        self.tag_handler.get_tag(tag)
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

    pub const fn get_entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub const fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub const fn get_position(&self) -> EntityPosition {
        self.position
    }

    pub const fn get_world(&self) -> Option<Uuid> {
        self.world
    }

    pub const fn get_game_mode(&self) -> GameMode {
        self.game_mode
    }

    pub const fn get_skin(&self) -> Option<&PlayerSkin> {
        self.skin.as_ref()
    }

    pub const fn get_display_name(&self) -> Option<&TextComponent> {
        self.display_name.as_ref()
    }

    pub fn get_statistics(&self) -> &[(String, i32)] {
        &self.statistics
    }
}
