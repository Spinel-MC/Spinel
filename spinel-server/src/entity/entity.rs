use crate::entity::EntityPosition;
use crate::entity::EntityView;
use crate::entity::TimedPotionEffect;
use crate::entity::generic_entity::GenericEntity;
use crate::entity::identity::EntityId;
use crate::entity::item::ItemEntity;
use crate::entity::player::Player;
use spinel_core::network::clientbound::play::entity_effect::EntityEffectPacket;
use spinel_core::network::clientbound::play::remove_entity_effect::RemoveEntityEffectPacket;
use spinel_registry::EntityType;
use uuid::Uuid;

pub enum Entity {
    Generic(GenericEntity),
    Item(ItemEntity),
    Player(Player),
}

impl Entity {
    pub fn new(entity_type: EntityType) -> Self {
        Self::Generic(GenericEntity::new(entity_type))
    }

    pub fn entity_id(&self) -> EntityId {
        match self {
            Self::Generic(entity) => entity.entity_id(),
            Self::Item(entity) => entity.entity_id(),
            Self::Player(player) => player.entity_id(),
        }
    }

    pub fn uuid(&self) -> Uuid {
        match self {
            Self::Generic(entity) => entity.uuid(),
            Self::Item(entity) => entity.uuid(),
            Self::Player(player) => player.uuid,
        }
    }

    pub fn entity_type(&self) -> EntityType {
        match self {
            Self::Generic(entity) => entity.entity_type(),
            Self::Item(entity) => entity.entity_type(),
            Self::Player(_) => EntityType::PLAYER,
        }
    }

    pub fn eye_height(&self) -> f64 {
        self.entity_type().eye_height()
    }

    pub fn bounding_box(&self) -> spinel_registry::EntityBoundingBox {
        self.entity_type().bounding_box()
    }

    pub fn world(&self) -> Option<Uuid> {
        match self {
            Self::Generic(entity) => entity.world(),
            Self::Item(entity) => entity.world(),
            Self::Player(player) => player.current_world(),
        }
    }

    pub fn position(&self) -> EntityPosition {
        match self {
            Self::Generic(entity) => entity.position(),
            Self::Item(entity) => entity.position(),
            Self::Player(player) => player.position(),
        }
    }

    pub(crate) fn set_position(&mut self, position: EntityPosition) {
        match self {
            Self::Generic(entity) => entity.set_position(position),
            Self::Item(entity) => entity.set_position(position),
            Self::Player(player) => player.set_position(position),
        }
    }

    pub(crate) fn set_world(&mut self, world: Uuid) {
        match self {
            Self::Generic(entity) => entity.set_world(world),
            Self::Item(entity) => entity.set_world(world),
            Self::Player(player) => player.set_world(world),
        }
    }

    pub fn add_effect(&mut self, effect: TimedPotionEffect) -> EntityEffectPacket {
        match self {
            Self::Generic(entity) => entity.add_effect(effect),
            Self::Item(entity) => entity.add_effect(effect),
            Self::Player(player) => player.add_effect(effect),
        }
    }

    pub fn remove_effect(&mut self, effect_id: i32) -> Option<RemoveEntityEffectPacket> {
        match self {
            Self::Generic(entity) => entity.remove_effect(effect_id),
            Self::Item(entity) => entity.remove_effect(effect_id),
            Self::Player(player) => player.remove_effect(effect_id),
        }
    }

    pub fn effect(&self, effect_id: i32) -> Option<&TimedPotionEffect> {
        match self {
            Self::Generic(entity) => entity.effect(effect_id),
            Self::Item(entity) => entity.effect(effect_id),
            Self::Player(player) => player.effect(effect_id),
        }
    }

    pub fn active_effects(&self) -> Vec<&TimedPotionEffect> {
        match self {
            Self::Generic(entity) => entity.active_effects(),
            Self::Item(entity) => entity.active_effects(),
            Self::Player(player) => player.active_effects(),
        }
    }

    pub fn view(&self) -> &EntityView {
        match self {
            Self::Generic(entity) => entity.view(),
            Self::Item(entity) => entity.view(),
            Self::Player(player) => player.view(),
        }
    }

    pub fn view_mut(&mut self) -> &mut EntityView {
        match self {
            Self::Generic(entity) => entity.view_mut(),
            Self::Item(entity) => entity.view_mut(),
            Self::Player(player) => player.view_mut(),
        }
    }

    pub fn viewers(&self) -> std::collections::BTreeSet<EntityId> {
        self.view().viewers()
    }
}
