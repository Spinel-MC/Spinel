use crate::entity::EntityPosition;
use crate::entity::generic_entity::GenericEntity;
use crate::entity::identity::EntityId;
use crate::entity::player::Player;
use spinel_registry::EntityType;
use uuid::Uuid;

pub enum Entity {
    Generic(GenericEntity),
    Player(Player),
}

impl Entity {
    pub fn new(entity_type: EntityType) -> Self {
        Self::Generic(GenericEntity::new(entity_type))
    }

    pub fn entity_id(&self) -> EntityId {
        match self {
            Self::Generic(entity) => entity.entity_id(),
            Self::Player(player) => player.entity_id(),
        }
    }

    pub fn uuid(&self) -> Uuid {
        match self {
            Self::Generic(entity) => entity.uuid(),
            Self::Player(player) => player.uuid,
        }
    }

    pub fn entity_type(&self) -> EntityType {
        match self {
            Self::Generic(entity) => entity.entity_type(),
            Self::Player(_) => EntityType::PLAYER,
        }
    }

    pub fn world(&self) -> Option<Uuid> {
        match self {
            Self::Generic(entity) => entity.world(),
            Self::Player(player) => player.current_world(),
        }
    }

    pub fn position(&self) -> EntityPosition {
        match self {
            Self::Generic(entity) => entity.position(),
            Self::Player(player) => player.position(),
        }
    }

    pub(crate) fn set_position(&mut self, position: EntityPosition) {
        match self {
            Self::Generic(entity) => entity.set_position(position),
            Self::Player(player) => player.set_position(position),
        }
    }

    pub(crate) fn set_world(&mut self, world: Uuid) {
        match self {
            Self::Generic(entity) => entity.set_world(world),
            Self::Player(player) => player.set_world(world),
        }
    }
}
